"""C03 paired production-query evidence, using only generated G0 fixtures.

The headless desktop driver owns MCP. This stdlib harness measures the same outer
JSONL roundtrip for both lanes and retains inner owner/dispatch clocks separately.
It never drives a native window or opens the engine database.
"""
from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
import platform
import queue
import subprocess
import threading
import time
from contextlib import contextmanager
from pathlib import Path
from typing import Any

from scripts.client_baseline import (COLD_SAMPLES, WARM_SAMPLES, corpus_snapshot,
    decode_tool, load_fixture, note_identifier, note_path, peak_rss,
    populate_fixture, query_labels, summarize, utc_now, verify_engine)
from scripts.core import (ROOT, PROJECT, create_sandbox, file_sha256, fingerprint,
                          isolated_env, profile, read_json, verify_sandbox, write_json)

WORKSPACE = "bmdock-workspace"
SOURCES = tuple("apps/bmdock-desktop/src-tauri/src/" + name for name in (
    "main.rs", "engine_session.rs", "engine_queries.rs", "query_driver.rs",
    "ipc.rs", "library.rs", "supervisor.rs")) + (
    "Cargo.lock", "apps/bmdock-desktop/src-tauri/Cargo.toml", "scripts/client_queries.py",
    "scripts/client_baseline.py", "scripts/core.py", "scripts/engine_worker.py")


def desktop_binary() -> Path:
    return ROOT / "target/debug" / ("bmdock-app.exe" if os.name == "nt" else "bmdock-app")


def fingerprints() -> dict[str, Any]:
    return {"binary_sha256": file_sha256(desktop_binary()),
            "source_sha256": {name: file_sha256(ROOT / name) for name in SOURCES}}


def output_path(path: Path) -> Path:
    path = path.resolve()
    if path.parent != (ROOT / "execution/evidence").resolve() or not path.name.startswith("c03-"):
        raise ValueError("Output must be execution/evidence/c03-*.json")
    if path.suffix != ".json" or path.exists():
        raise FileExistsError("Use a fresh c03-*.json evidence filename")
    return path


def lane_order(index: int) -> tuple[str, str]:
    return ("direct", "adapter") if index % 2 == 0 else ("adapter", "direct")


def command(session: dict[str, Any], *, query: str | None = None,
            identifier: str | None = None, cursor: str | None = None,
            generation: int = 0, options: dict[str, Any] | None = None) -> dict[str, Any]:
    args = {"workspace": WORKSPACE, "project": PROJECT, "expected_session": session}
    if query is not None:
        args.update(query=query, cursor=cursor, request_generation=generation,
                    options=options or {"mode": "text", "entity_types": ["entity"]})
        # Deliberately omit page_size to exercise the production default of 50.
        return {"command": "search_notes", "args": args}
    if identifier is None:
        raise ValueError("A query or exact note identifier is required")
    args["identifier"] = identifier
    return {"command": "read_note", "args": args}


def direct_payload(event: dict[str, Any]) -> dict[str, Any]:
    kind, payload = decode_tool({"result": event.get("response")})
    if kind != "structured" or not isinstance(payload, dict) or payload.get("error"):
        raise AssertionError(f"Direct call has no successful structured payload: {kind}")
    return payload


def validate_close(receipt: dict[str, Any]) -> None:
    shutdown = receipt["shutdown"]
    if (not shutdown["transport_cancelled"] or not shutdown["child_exited"]
            or shutdown["forced"] or shutdown["timeout_unknown"] or shutdown["exit_code"] != 0):
        raise AssertionError(f"Owned driver did not shut down normally: {shutdown}")
    if receipt["stopped"]["status"] != "stopped":
        raise AssertionError("Driver runtime did not stop")


class Driver:
    """JSONL controller; the desktop binary is the sole MCP process owner."""

    def __init__(self, profile_id: str, sandbox: Path):
        self.closed = False
        self.sequence = 0
        self.started_at = utc_now()
        self.stderr_path = sandbox / f"c03-driver-{time.time_ns()}.stderr.log"
        self.stderr = self.stderr_path.open("w", encoding="utf-8")
        self.lines: queue.Queue[str | BaseException | None] = queue.Queue()
        try:
            self.process = subprocess.Popen(
                [str(desktop_binary()), "--query-driver", profile_id, str(sandbox)],
                cwd=ROOT, env=isolated_env(sandbox), stdin=subprocess.PIPE,
                stdout=subprocess.PIPE, stderr=self.stderr, text=True, encoding="utf-8",
                creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
        except BaseException:
            self.stderr.close()
            raise
        threading.Thread(target=self._read, daemon=True).start()
        before = time.perf_counter_ns()
        try:
            self.ready, _ = self._receive(150)
            self.startup_ms = (time.perf_counter_ns() - before) / 1e6
            self.session = self.ready["session"]
            if (self.ready.get("event") != "ready" or self.session["profile"] != profile_id
                    or self.ready["engine_sha"] != profile(profile_id)["commit"]):
                raise AssertionError("Unexpected driver/profile handshake")
        except BaseException:
            self.abort()
            raise

    def _read(self) -> None:
        try:
            assert self.process.stdout is not None
            for line in self.process.stdout:
                self.lines.put(line)
        except BaseException as error:
            self.lines.put(error)
        finally:
            self.lines.put(None)

    def _receive(self, timeout: float) -> tuple[dict[str, Any], int]:
        try:
            line = self.lines.get(timeout=timeout)
        except queue.Empty as error:
            raise TimeoutError(f"Query driver response exceeded {timeout}s") from error
        if isinstance(line, BaseException):
            raise line
        if line is None:
            self.stderr.flush()
            raise RuntimeError("Query driver EOF: " + self.stderr_path.read_text(encoding="utf-8")[-2000:])
        value = json.loads(line)
        if not isinstance(value, dict):
            raise AssertionError("Driver output is not an object")
        return value, len(line.encode("utf-8"))

    def exchange(self, request: dict[str, Any], timeout: float = 140) -> dict[str, Any]:
        self.sequence += 1
        request = {"id": str(self.sequence), **request}
        encoded = json.dumps(request, ensure_ascii=False, separators=(",", ":")) + "\n"
        started_at = utc_now()
        before = time.perf_counter_ns()
        assert self.process.stdin is not None
        self.process.stdin.write(encoded)
        self.process.stdin.flush()
        response, reply_bytes = self._receive(timeout)
        elapsed = (time.perf_counter_ns() - before) / 1e6
        if response.get("id") != request["id"]:
            raise AssertionError("Mismatched driver response identity")
        response.update(outer_elapsed_ms=elapsed, outer_reply_bytes=reply_bytes,
                        outer_request_bytes=len(encoded.encode("utf-8")), started_at=started_at)
        return response

    def call(self, lane: str, request: dict[str, Any]) -> dict[str, Any]:
        result = self.exchange({"lane": lane, "command": request})
        result.update(typed_command=request, driver_session=self.session,
                      driver_started_at=self.started_at)
        if result.get("lane") != lane:
            raise AssertionError("Driver changed measurement lane")
        response = result.get("response", {})
        if lane == "adapter" and response.get("kind") != "error":
            if response.get("session") != self.session:
                raise AssertionError("Adapter returned another session's result")
            if request["command"] in ("search_notes", "preview_context", "list_activity"):
                identity = response.get("request", {})
                if (identity.get("arguments") != result["request"]["arguments"]
                        or identity.get("session") != self.session
                        or identity.get("request_generation") != request["args"]["request_generation"]
                        or identity.get("workspace") != WORKSPACE or identity.get("project") != PROJECT
                        or identity.get("operation") != request["command"]):
                    raise AssertionError("Adapter query identity omitted or changed request dimensions")
        return result

    def close(self) -> dict[str, Any]:
        try:
            receipt = self.exchange({"control": "shutdown"}, 110)
            assert self.process.stdin is not None
            self.process.stdin.close()
            self.process.wait(timeout=10)
            validate_close(receipt)
            if self.process.returncode != 0:
                raise AssertionError("Driver exited with failure after close")
            self.closed = True
            return receipt
        finally:
            if not self.closed:
                self.abort()
            if self.process.stdout is not None:
                self.process.stdout.close()
            self.stderr.close()

    def abort(self) -> None:
        if self.process.poll() is None:
            # EOF asks the actual owner to close; never enumerate/kill other PIDs.
            if self.process.stdin is not None and not self.process.stdin.closed:
                self.process.stdin.close()
            try:
                # An outstanding read can use 90s before EOF is consumed, then
                # SDK close/normal wait/forced reap each have a 30s owner budget.
                self.process.wait(timeout=210)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=10)
        self.closed = True
        if self.process.stdout is not None:
            self.process.stdout.close()
        self.stderr.close()


@contextmanager
def owned_driver(profile_id: str, sandbox: Path, sessions: list[dict[str, Any]]):
    driver = Driver(profile_id, sandbox)
    record = {"started_at": driver.started_at, "startup_ms": driver.startup_ms,
              "ready": driver.ready, "stderr_path": driver.stderr_path.relative_to(ROOT).as_posix()}
    sessions.append(record)
    try:
        yield driver
    finally:
        try:
            record["shutdown"] = driver.close()
        except BaseException as error:
            record["shutdown_error"] = str(error)
            raise


def search_semantics(event: dict[str, Any]) -> dict[str, Any]:
    """Strict captured-shape projection; no rank sorting or score normalization."""
    payload = direct_payload(event)
    if not isinstance(payload.get("results"), list):
        raise AssertionError("Direct search has no results array")
    for key in ("total_is_exact", "has_more"):
        if type(payload.get(key)) is not bool:
            raise AssertionError(f"Missing search exactness/pagination: {key}")
    return {"hits": [{"identifier": row["permalink"], "result_kind": row["type"],
                      "score": row["score"]} for row in payload["results"]],
            "page": payload["current_page"], "page_size": payload["page_size"],
            "total": payload["total"], "total_is_exact": payload["total_is_exact"],
            "has_more": payload["has_more"]}


def adapter_search_semantics(event: dict[str, Any]) -> dict[str, Any]:
    payload = event["response"]
    if payload.get("kind") != "search_page" or payload.get("engine_search") is not True:
        raise AssertionError("Adapter returned no official search page")
    for key in ("total_is_exact", "has_more"):
        if type(payload.get(key)) is not bool:
            raise AssertionError(f"Adapter omitted exactness/pagination: {key}")
    expected_cursor = str(payload["page"] + 1) if payload["has_more"] else None
    if payload.get("next_cursor") != expected_cursor:
        raise AssertionError("Adapter cursor contradicts upstream has_more")
    return {"hits": [{"identifier": row["identifier"], "result_kind": row["result_kind"],
                      "score": row["score"]} for row in payload["hits"]],
            **{key: payload[key] for key in ("page", "page_size", "total", "total_is_exact", "has_more")}}


def validate_pair(direct: dict[str, Any], adapter: dict[str, Any],
                  source: bytes | None = None, *, compare_scores: bool = True) -> dict[str, Any]:
    for event in (direct, adapter):
        if event.get("command_count") != 1:
            raise AssertionError("Successful page/body operation must make exactly one MCP call")
    if direct["request"] != adapter["request"]:
        raise AssertionError("Paired lanes sent different upstream arguments")
    if source is None:
        upstream = search_semantics(direct)
        projected = adapter_search_semantics(adapter)
        compared_upstream, compared_projected = copy.deepcopy(upstream), copy.deepcopy(projected)
        if not compare_scores:
            for value in (compared_upstream, compared_projected):
                for hit in value["hits"]:
                    hit.pop("score")
        if compared_upstream != compared_projected:
            raise AssertionError(f"Search semantics changed: direct={upstream}, adapter={projected}")
        if direct["request"]["arguments"].get("page_size") != 50:
            raise AssertionError("Search default page size is not 50")
        return {"ordered_identity_paging_equal": True, "scores_required_equal": compare_scores,
                "scores_equal_observed": upstream == projected, "semantics": upstream,
                "adapter_semantics": projected,
                "score_deltas_adapter_minus_direct": [b["score"] - a["score"]
                    if isinstance(a["score"], (int, float)) and isinstance(b["score"], (int, float)) else None
                    for a, b in zip(upstream["hits"], projected["hits"])],
                "score_boundary": "Immutable corpus exact parity" if compare_scores else
                    "Mutating-index freshness visibility; consecutive calls are not an atomic score snapshot"}
    upstream = direct_payload(direct)
    projected = adapter["response"]
    text = upstream.get("content")
    original = source.decode("utf-8")
    if (not isinstance(text, str) or text not in (original, original.replace("\r\n", "\n"))
            or not text.startswith("---") or upstream.get("permalink") != note_identifier(0)):
        raise AssertionError("Direct read omitted the expected complete source/identity")
    if (projected.get("kind") != "note_read" or projected.get("body") != text
            or projected.get("identifier") != upstream["permalink"]):
        raise AssertionError("Adapter changed full note body or identity")
    args = direct["request"]["arguments"]
    if args.get("include_frontmatter") is not True or "start_line" in args or "end_line" in args:
        raise AssertionError("Read does not use the captured unsliced frontmatter contract")
    return {"delivered_source_equal": True, "delivered_utf8_sha256": hashlib.sha256(text.encode()).hexdigest(),
            "delivered_utf8_bytes": len(text.encode()), "source_bytes_equal": text.encode() == source,
            "upstream_crlf_normalized": text != original, "general_byte_fidelity_claim": False}


def retained_event(event: dict[str, Any], *, body: bool = False) -> dict[str, Any]:
    result = copy.deepcopy(event)
    result["response_sha256"] = fingerprint(event["response"])
    result["response_compact_json_bytes"] = len(json.dumps(event["response"], ensure_ascii=False,
                                                          separators=(",", ":")).encode())
    if body:
        # Full source was compared before this reduction; avoid duplicating MiB
        # payloads in every raw timing row. Digest/length preserve the receipt.
        result["response"] = {"storage": "sha256-and-length-after-full-source-comparison"}
    return result


def observe_until(call, predicate, *, timeout: float = 120, stable_key=None) -> dict[str, Any]:
    start = time.monotonic()
    attempts = 0
    previous = None
    stable_count = 0
    while True:
        attempts += 1
        result = call()
        if predicate(result):
            signature = fingerprint(stable_key(result)) if stable_key is not None else None
            stable_count = stable_count + 1 if signature == previous else 1
            previous = signature
            if stable_key is None or stable_count >= 3:
                return {"attempts": attempts, "elapsed_seconds": time.monotonic() - start,
                        "matching_consecutive_observations": stable_count,
                        "stable_semantics_sha256": signature, "last_observation": result}
        else:
            previous, stable_count = None, 0
        if time.monotonic() - start >= timeout:
            raise TimeoutError("Official index did not reach the required observed state")
        time.sleep(0.5)


def wait_corpus(driver: Driver, count: int) -> dict[str, Any]:
    def observe():
        identifiers = []
        cursor = None
        pages = []
        for _ in range((count + 49) // 50 + 1):
            event = driver.call("direct", command(driver.session, query="bmdockcorpus", cursor=cursor))
            semantics = search_semantics(event)
            identifiers.extend(item["identifier"] for item in semantics["hits"])
            pages.append(semantics)
            if not semantics["has_more"]:
                break
            cursor = str(semantics["page"] + 1)
        return {"identifiers": identifiers, "pages": pages}
    expected = {note_identifier(index) for index in range(count)}
    return observe_until(observe, lambda value: len(value["identifiers"]) == count
                         and set(value["identifiers"]) == expected, timeout=600,
                         stable_key=lambda value: value)


def perform_pair(driver: Driver, request: dict[str, Any], index: int,
                 source: bytes | None = None, *, compare_scores: bool = True) -> dict[str, Any]:
    events = {lane: driver.call(lane, request) for lane in lane_order(index)}
    comparison = validate_pair(events["direct"], events["adapter"], source, compare_scores=compare_scores)
    return {"order": list(lane_order(index)), "comparison": comparison,
            "events": {lane: retained_event(value, body=source is not None)
                       for lane, value in events.items()}}


def functional(driver: Driver, sandbox: Path) -> dict[str, Any]:
    labels = []
    for index, label in enumerate(query_labels()):
        pair = perform_pair(driver, command(driver.session, query=label["query"]), index)
        identities = [row["identifier"] for row in pair["comparison"]["semantics"]["hits"]]
        recall = len(set(identities[:10]) & set(label["expected_identifiers"])) / len(label["expected_identifiers"])
        labels.append({"label": label, "pair": pair, "recall_at_10_both_lanes": recall})
    ordering = perform_pair(driver, command(driver.session, query="orderingprobe"), 0)
    filtered = perform_pair(driver, command(driver.session, query="",
        options={"mode": "text", "entity_types": ["entity"], "tags": ["baseline"]}), 0)
    if filtered["comparison"]["semantics"]["total"] != 100:
        raise AssertionError("Filter-only request did not discover the known fixture tag")
    ordered = [row["identifier"] for row in ordering["comparison"]["semantics"]["hits"]]
    if ordered != [note_identifier(99), note_identifier(20)]:
        raise AssertionError(f"Fixture did not exercise alphabetically last best hit: {ordered}")
    pages, identifiers, cursor = [], [], None
    for index in range(3):
        pair = perform_pair(driver, command(driver.session, query="bmdockcorpus", cursor=cursor), index)
        semantics = pair["comparison"]["semantics"]
        pages.append(pair)
        identifiers.extend(row["identifier"] for row in semantics["hits"])
        if not semantics["has_more"]:
            break
        cursor = pair["events"]["adapter"]["response"]["next_cursor"]
    if len(identifiers) != 100 or set(identifiers) != {note_identifier(i) for i in range(100)}:
        raise AssertionError("Immutable page traversal lost or duplicated identities")
    full = perform_pair(driver, command(driver.session, identifier=note_identifier(0)), 0,
                        (sandbox / "vault" / note_path(0)).read_bytes())
    semantic = driver.call("adapter", command(driver.session, query="thermalpress", options={"mode": "semantic"}))
    if semantic["response"].get("kind") != "error" or semantic["command_count"] != 0:
        raise AssertionError("Disabled semantic mode was presented as successful search")
    compact_request = command(driver.session, query="thermalpress",
                              options={"mode": "text", "entity_types": ["entity"], "compact": True})
    if driver.session["profile"] == "main-preview":
        compact = perform_pair(driver, compact_request, 0)
    else:
        compact = driver.call("adapter", compact_request)
        if compact["response"].get("kind") != "error" or compact["command_count"] != 0:
            raise AssertionError("Release received an unsupported preview-only compact request")
    return {"thirty_independent_labels": labels, "ordering_probe": ordering,
            "connected_journey": connected_journey(driver, sandbox),
            "immutable_paging": pages, "full_source": full, "semantic_unavailable": semantic,
            "profile_compact_contract": compact,
            "filter_only": filtered,
            "recall_at_10_by_category": {category: sum(row["recall_at_10_both_lanes"] for row in labels
                if row["label"]["category"] == category) / 10
                for category in ("literal", "chinese_mixed", "paraphrase")},
            "interpretation": "Entity-filtered text retrieval parity, including paraphrase labels; mixed result kinds covered by shape tests, no enabled semantic retrieval claim"}


def hit_projection(row: dict[str, Any]) -> dict[str, Any]:
    kind = row["type"]
    identifier = row.get("permalink") or f"{kind}:{row[kind + '_id']}"
    return {"result_kind": kind, "identifier": identifier,
        "note_identifier": next((row[key] for key in ("external_id", "entity_external_id", "from_entity_external_id", "entity", "file_path") if row.get(key) is not None), None),
        "excerpt": row["content"][:240] if row.get("content") is not None else None,
        **{key: row.get(key) for key in ("title", "score", "file_path", "category", "relation_type", "from_entity", "to_entity", "to_name", "created_at")}}


def connected_journey(driver: Driver, sandbox: Path) -> dict[str, Any]:
    """Bounded C07 read journey; no native renderer or all-matrix timing claim."""
    route = {"workspace": WORKSPACE, "project": PROJECT, "expected_session": driver.session}
    records = {}
    for label, directory in (("tree_root", ""), ("tree_leaf", "group-000/item-000")):
        request = {"command": "list_tree", "args": {**route, "directory": directory, "page_size": 50}}
        direct, adapter = [driver.call(lane, request) for lane in ("direct", "adapter")]
        raw = direct_payload(direct)
        dto = adapter["response"]
        expected = [{"identifier": node["directory_path"].lstrip("/"), "title": node["name"],
                     "kind": "directory"} if node["type"] == "directory" else
                    {"identifier": node["permalink"], "title": node.get("title") or node["name"],
                     "kind": "note", "note_identifier": node.get("external_id")} for node in raw["nodes"]]
        if (direct["request"] != adapter["request"] or direct["command_count"] != 1 or adapter["command_count"] != 1
                or dto.get("kind") != "tree_page" or dto["entries"] != expected
                or dto["page"] != raw["page"] or dto["next_cursor"] != (str(raw["page"] + 1) if raw["has_more"] else None)):
            raise AssertionError("Production tree changed directory/note identity or pagination")
        records[label] = {"direct": direct, "adapter": adapter, "ordered_entries_equal": True}
    selection = next(row["note_identifier"] for row in records["tree_leaf"]["adapter"]["response"]["entries"] if row["identifier"] == note_identifier(0))
    if not selection:
        raise AssertionError("Tree did not expose official note identity for selection")
    records["tree_selected_note"] = perform_pair(driver, command(driver.session, identifier=selection), 0,
                                                  (sandbox / "vault" / note_path(0)).read_bytes())
    context_request = {"command": "preview_context", "args": {**route, "identifier": note_identifier(10),
        "request_generation": 0, "options": {"timeframe": "2026-01-01", "depth": 1, "max_related": 20}}}
    direct, adapter = [driver.call(lane, context_request) for lane in ("direct", "adapter")]
    raw, dto = direct_payload(direct), adapter["response"]
    expected_groups = [{"primary_result": hit_projection(group["primary_result"]),
        "observations": [hit_projection(row) for row in group["observations"]],
        "related_results": [hit_projection(row) for row in group["related_results"]]} for group in raw["results"]]
    # These are server-generated time values from two consecutive calls, not
    # stable corpus semantics. All remaining metadata is compared exactly.
    dynamic_metadata = ("generated_at", "timeframe")
    metadata = lambda value: {key: item for key, item in value.items() if key not in dynamic_metadata}
    if (direct["request"] != adapter["request"] or direct["command_count"] != 1 or adapter["command_count"] != 1
            or dto.get("kind") != "context_preview" or dto.get("engine_context") is not True
            or dto["results"] != expected_groups or not expected_groups
            or any(dto[key] != raw[key] for key in ("page", "page_size", "has_more"))
            or metadata(dto["metadata"]) != metadata(raw["metadata"])):
        raise AssertionError("Production context changed captured group/score/metadata semantics")
    records["context"] = {"direct": direct, "adapter": adapter, "group_projection_equal": True,
                          "excluded_dynamic_metadata": list(dynamic_metadata)}
    activity_request = {"command": "list_activity", "args": {**route, "request_generation": 0,
                        "options": {"types": ["entity"], "timeframe": "2026-01-01", "depth": 1}}}
    direct, adapter = [driver.call(lane, activity_request) for lane in ("direct", "adapter")]
    kind, raw = decode_tool({"result": direct["response"]})
    dto = adapter["response"]
    if (kind != "structured" or not isinstance(raw, list) or not raw
            or direct["request"] != adapter["request"] or direct["command_count"] != 1 or adapter["command_count"] != 1
            or dto.get("kind") != "activity_page" or dto.get("engine_activity") is not True
            or dto["entries"] != [hit_projection(row) for row in raw]
            or any(dto[key] is not None for key in ("next_cursor", "has_more", "total", "total_is_exact"))):
        raise AssertionError("Production activity changed ordered entries or fabricated paging/counts")
    records["activity"] = {"direct": direct, "adapter": adapter, "ordered_projection_equal": True,
                           "unknown_paging_and_totals_preserved": True}
    return records


def measure(profile_id: str, sandbox: Path, scenario: str, report: dict[str, Any]) -> None:
    source = (sandbox / "vault" / note_path(0)).read_bytes() if scenario == "read-full" else None
    def request(driver):
        return command(driver.session, identifier=note_identifier(0)) if source is not None else command(driver.session, query=query_labels()[0]["query"])
    samples = report["samples"]
    for index in range(COLD_SAMPLES):
        print(f"{profile_id} {scenario}: cold pair {index + 1}/{COLD_SAMPLES}", flush=True)
        events = {}
        for lane in lane_order(index):
            with owned_driver(profile_id, sandbox, report["sessions"]) as driver:
                event = driver.call(lane, request(driver))
                event.update(lane=lane, phase="cold", pair=index + 1,
                             driver_peak_rss=peak_rss(driver.process.pid))
                events[lane] = event
        try:
            comparison = validate_pair(events["direct"], events["adapter"], source)
        except BaseException:
            report["failed_pair"] = events
            raise
        report["pairs"].append({"phase": "cold", "pair": index + 1,
                                "order": list(lane_order(index)), "comparison": comparison})
        samples.extend(retained_event(events[lane], body=source is not None) for lane in lane_order(index))
    with owned_driver(profile_id, sandbox, report["sessions"]) as driver:
        report["warmup"] = perform_pair(driver, request(driver), 0, source)
        for index in range(WARM_SAMPLES):
            events = {lane: driver.call(lane, request(driver)) for lane in lane_order(index)}
            try:
                comparison = validate_pair(events["direct"], events["adapter"], source)
            except BaseException:
                report["failed_pair"] = events
                raise
            report["pairs"].append({"phase": "warm", "pair": index + 1,
                                    "order": list(lane_order(index)), "comparison": comparison})
            for lane in lane_order(index):
                event = events[lane]
                event.update(lane=lane, phase="warm", pair=index + 1,
                             driver_peak_rss=peak_rss(driver.process.pid))
                samples.append(retained_event(event, body=source is not None))
    report["budget"] = paired_budget(samples)


def paired_budget(samples: list[dict[str, Any]]) -> dict[str, Any]:
    stats = {}
    for lane in ("direct", "adapter"):
        for phase, expected in (("cold", COLD_SAMPLES), ("warm", WARM_SAMPLES)):
            values = [row["outer_elapsed_ms"] for row in samples if row["lane"] == lane and row["phase"] == phase]
            if len(values) != expected:
                raise AssertionError(f"Missing fixed sample count for {lane}/{phase}")
            stats[f"{lane}_{phase}"] = summarize(values)
    direct = stats["direct_warm"]["p95_ms"]
    adapter = stats["adapter_warm"]["p95_ms"]
    allowance = max(50.0, 0.25 * direct)
    return {"outer_roundtrip": stats, "overhead_ms": adapter - direct,
            "allowed_overhead_ms": allowance, "within_frozen_budget": adapter - direct <= allowance,
            "strict_c01_probe_clock_conformance": False,
            "interpretation": "Representative paired driver roundtrip; not old Probe.raw clock or native acceptance"}


@contextmanager
def preserved_note(sandbox: Path, relative: str):
    """One reversible mutation target only in a newly generated C03 sandbox."""
    sandbox = verify_sandbox(sandbox)
    if not sandbox.is_relative_to((ROOT / ".work/g0/client-queries").resolve()):
        raise ValueError("Freshness mutations require a disposable C03 sandbox")
    original = (sandbox / "vault" / relative).resolve(strict=True)
    if not original.is_relative_to(sandbox / "vault"):
        raise ValueError("Mutation target escaped its fixture vault")
    renamed = original.with_name("c03-renamed-note.md")
    if renamed.exists():
        raise FileExistsError(renamed)
    data, stat = original.read_bytes(), original.stat()
    try:
        yield original, renamed, data
    finally:
        if renamed.exists():
            renamed.unlink()
        original.write_bytes(data)
        os.utime(original, ns=(stat.st_atime_ns, stat.st_mtime_ns))


def freshness(profile_id: str, report: dict[str, Any]) -> None:
    sandbox = create_sandbox(ROOT / ".work/g0/client-queries", profile_id)
    before = populate_fixture(sandbox, "notes-100")
    report["disposable_sandbox"] = sandbox.relative_to(ROOT).as_posix()
    report["disposable_corpus_before"] = before
    records = report["freshness"] = {}
    marker = "c03freshnessmarker"
    with owned_driver(profile_id, sandbox, report["sessions"]) as driver:
        records["initial_index_ready"] = wait_corpus(driver, 100)
        with preserved_note(sandbox, note_path(0)) as (original, renamed, data):
            original.write_bytes(data + f"\r\n{marker}\r\n".encode())
            def query_event():
                return driver.call("direct", command(driver.session, query=marker))
            records["edit_index_ready"] = observe_until(query_event, lambda event:
                note_identifier(0) in [row["identifier"] for row in search_semantics(event)["hits"]],
                stable_key=search_semantics)
            records["edit_refresh"] = perform_pair(driver,
                command(driver.session, query=marker, generation=1), 0, compare_scores=False)
            records["edited_full_source"] = perform_pair(driver,
                command(driver.session, identifier=note_identifier(0)), 0, original.read_bytes())
            original.rename(renamed)
            def read_event():
                return driver.call("direct", command(driver.session, identifier=note_identifier(0)))
            expected_path = renamed.relative_to(sandbox / "vault").as_posix()
            def renamed_ready(event):
                kind, payload = decode_tool({"result": event.get("response")})
                return kind == "structured" and isinstance(payload, dict) and payload.get("file_path") == expected_path
            records["rename_index_ready"] = observe_until(read_event, renamed_ready,
                                                          stable_key=direct_payload)
            records["rename_query_settled"] = observe_until(query_event,
                lambda event: len(search_semantics(event)["hits"]) == 1, stable_key=search_semantics)
            records["rename_refresh"] = perform_pair(driver,
                command(driver.session, query=marker, generation=2), 1, compare_scores=False)
            records["renamed_full_source"] = perform_pair(driver,
                command(driver.session, identifier=note_identifier(0)), 0, renamed.read_bytes())
            renamed.unlink()
            records["delete_index_ready"] = observe_until(query_event,
                lambda event: not search_semantics(event)["hits"], stable_key=search_semantics)
            records["delete_refresh"] = perform_pair(driver,
                command(driver.session, query=marker, generation=3), 0, compare_scores=False)
            missing = driver.call("adapter", command(driver.session, identifier=note_identifier(0)))
            if missing["response"].get("kind") != "error":
                raise AssertionError("Deleted selected note was returned as current content")
            records["deleted_note_unavailable"] = missing
        records["restore_index_ready"] = wait_corpus(driver, 100)
        records["restored_source"] = perform_pair(driver,
            command(driver.session, identifier=note_identifier(0)), 0,
            (sandbox / "vault" / note_path(0)).read_bytes())
    report["disposable_corpus_restored"] = corpus_snapshot(sandbox / "vault") == before
    if not report["disposable_corpus_restored"]:
        raise AssertionError("Disposable fixture source was not restored")


def run(mode: str, fixture: Path, output: Path, scenario: str = "literal-00") -> dict[str, Any]:
    if mode == "source":
        scenario = "read-full"
    output = output_path(output)
    record, sandbox = load_fixture(fixture)
    frozen = read_json(ROOT / "execution/evidence/c01-measurement-manifest-v3.json")
    if record["measurement_manifest_sha256"] != fingerprint(frozen):
        raise ValueError("Only the frozen C01 v3 measurement manifest is accepted")
    profile_id = record["profile"]
    if mode in ("functional", "freshness") and record["scale"] != "notes-100":
        raise ValueError("Functional/freshness evidence uses the notes-100 representative corpus")
    if mode == "measure" and scenario == "literal-00" and record["scale"] != "notes-100":
        raise ValueError("The authorized representative literal timing uses notes-100 only")
    verify_engine(profile_id)
    report = {"schema_version": 1, "status": "incomplete", "mode": mode,
        "started_at": utc_now(), "profile": profile_id, "engine_sha": profile(profile_id)["commit"],
        "platform": platform.platform(), "scenario": scenario, "scale": record["scale"],
        "host_python": platform.python_version(), "logical_cpus": os.cpu_count(),
        "command": f"python -m scripts.client_queries {mode} --fixture {fixture.as_posix()} --output {output.relative_to(ROOT).as_posix()} --scenario {scenario}",
        "fixture_manifest": fixture.resolve().relative_to(ROOT).as_posix(),
        "measurement_manifest_sha256": record["measurement_manifest_sha256"],
        "corpus_sha256": record["corpus"]["files_sha256"], "fingerprints_before": fingerprints(),
        "sessions": [], "samples": [], "pairs": [],
        "protocol": {"cold_pairs": COLD_SAMPLES, "warm_pairs": WARM_SAMPLES,
            "cold": "Fresh desktop driver and owned engine per lane; indexed corpus reused; OS disk cache uncontrolled",
            "warm": "One primed desktop owner; alternating paired direct/adapter calls; no application result cache",
            "budget_clock": "Python perf_counter_ns before JSONL write through complete parsed response; identical boundaries both lanes",
            "inner_clock": "Rust owner request for direct; production dispatch for adapter; diagnostic only, includes transport/serde",
            "payload": "Outer reply UTF-8 JSONL bytes and compact response JSON bytes; raw direct envelope may duplicate content",
            "rss": "Desktop driver process peak working set only; official engine/descendant peak RSS unavailable",
            "host_load": "Shared development host, background build/test load uncontrolled; no cross-profile speedup claim",
            "strict_c01_probe_clock_conformance": False},
        "full_matrix_complete": False,
        "user_authorized_deferral": "Code and representative benchmarks first; full performance matrix retained for acceptance",
        "unverified": ["Full 428-cell C01/C03 performance matrix", "Native renderer/typing/IME/UI",
            "Meaningful pre-optimization UI comparison", "Official engine/descendant peak RSS",
            "Semantic retrieval (disabled in isolated profiles)", "Controlled quiet-host performance"]}
    try:
        if mode == "freshness":
            freshness(profile_id, report)
        else:
            with owned_driver(profile_id, sandbox, report["sessions"]) as driver:
                report["index_ready_outside_timing"] = wait_corpus(driver, record["corpus"]["note_count"])
                if mode == "functional":
                    report["functional"] = functional(driver, sandbox)
                elif mode == "source":
                    report["source_fidelity"] = perform_pair(driver,
                        command(driver.session, identifier=note_identifier(0)), 0,
                        (sandbox / "vault" / note_path(0)).read_bytes())
                    report["timing_acceptance"] = False
            if mode == "measure":
                measure(profile_id, sandbox, scenario, report)
        report["status"] = "budget_failed" if report.get("budget", {}).get("within_frozen_budget") is False else "passed"
    except BaseException as error:
        report["status"] = "failed"
        report["error"] = {"type": type(error).__name__, "message": str(error)}
        print(f"{profile_id} {mode}: {type(error).__name__}: {error}", flush=True)
    finally:
        report["completed_at"] = utc_now()
        report["physical_corpus_unchanged"] = corpus_snapshot(sandbox / "vault") == record["corpus"]
        if "disposable_sandbox" in report:
            report["disposable_corpus_restored"] = corpus_snapshot(ROOT / report["disposable_sandbox"] / "vault") == report["disposable_corpus_before"]
        report["fingerprints_after"] = fingerprints()
        report["source_and_binary_unchanged"] = report["fingerprints_before"] == report["fingerprints_after"]
        if (not report["physical_corpus_unchanged"] or not report["source_and_binary_unchanged"]
                or report.get("disposable_corpus_restored") is False):
            report["status"] = "invalidated"
        write_json(output, report)
    return report


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("mode", choices=("functional", "measure", "freshness", "source"))
    parser.add_argument("--fixture", type=Path, required=True, help="Frozen C01 v3 fixture manifest")
    parser.add_argument("--output", type=Path, required=True, help="Fresh execution/evidence/c03-*.json")
    parser.add_argument("--scenario", choices=("literal-00", "read-full"), default="literal-00")
    args = parser.parse_args()
    report = run(args.mode, args.fixture, args.output, args.scenario)
    print(json.dumps({"status": report["status"], "output": str(args.output)}, ensure_ascii=False))
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
