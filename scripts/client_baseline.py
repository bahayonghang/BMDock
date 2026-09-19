"""C01 reproducible fixture and direct-MCP measurements; host standard library only.

This developer harness never accepts a user vault. It reuses the G0 sandbox,
environment and process owners. Engine timings include the probe/stdio roundtrip;
they are not engine-internal execution time or native UI timing.
"""
from __future__ import annotations

import argparse
import ctypes
import hashlib
import json
import math
import os
import platform
import statistics
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from scripts.core import (MARKER, PROJECT, ROOT, create_sandbox, file_sha256,
                          fingerprint, inventory_delta, isolated_env, paginate,
                          profile, read_json, verify_sandbox, write_json)
from scripts.probe import Probe, engine_python, redact, require_protocol_version

SEED = 20260919
COLD_SAMPLES = 5
WARM_SAMPLES = 30
NATIVE_PROTOCOL = Path(".trellis/tasks/09-19-bmdock-client-optimization/research/native-input-latency-protocol.md")
SCALES = {"notes-100": (100, 4096), "notes-1000": (1000, 4096),
          "notes-10000": (10000, 4096), "large-1mib": (1, 1024 ** 2),
          "large-5mib": (1, 5 * 1024 ** 2)}
# Hand-authored topic judgments precede retrieval; no substring-derived gold set.
TOPICS = (
    ("thermalpress", "热压温度", "The press temperature controls resin curing.", "How does heat affect resin curing?"),
    ("moisturesensor", "含水率", "A moisture sensor measures water remaining in timber.", "Which instrument measures water in wood?"),
    ("conveyorspeed", "输送速度", "Conveyor velocity sets the residence time of each board.", "How long does a board remain on the belt?"),
    ("gluebalance", "施胶量", "Glue dosage balances bond strength and material cost.", "What trades adhesive expense against bonding strength?"),
    ("steamboiler", "蒸汽锅炉", "The boiler supplies steam for industrial heating.", "Where does the factory obtain heating steam?"),
    ("bearingvibration", "轴承振动", "Rising bearing vibration warns of mechanical wear.", "What signal can warn of worn rotating equipment?"),
    ("qualitydensity", "板材密度", "Panel density is a quality indicator of wood composites.", "Which mass per volume measure describes panel quality?"),
    ("energyrecovery", "余热回收", "Recovered exhaust heat reduces purchased fuel demand.", "How can waste heat reduce fuel purchases?"),
    ("dryerkiln", "干燥窑", "The drying kiln removes moisture before pressing.", "Which stage removes water before the press?"),
    ("processdelay", "过程时滞", "Transport delay separates a control action from its observation.", "Why can a control adjustment take time to appear?"),
)


def utc_now() -> str:
    return datetime.now(timezone.utc).isoformat()


def note_path(index: int) -> str:
    # Duplicate basenames in nested directories exercise full route identities.
    return f"group-{index // 100:03d}/item-{index % 100:03d}/note.md"


def note_identifier(index: int) -> str:
    return f"corpus/note-{index:05d}"


def note_bytes(index: int, target_bytes: int, count: int) -> bytes:
    token, chinese, fact, _ = TOPICS[index % len(TOPICS)]
    title = f"{token} {chinese}" if index < 10 else f"Corpus note {index:05d}"
    if count >= 100 and index == count - 1:
        title = "Orderingprobe orderingprobe orderingprobe strongest match"
    lines = ["---", f"title: {title}", "type: note",
             f"permalink: {note_identifier(index)}", "tags: [baseline, fixture]",
             "custom_metadata: 保留原始中文", "---", f"# {title}",
             f"bmdockcorpus seed{SEED} identity{index:05d}", "", "## Observations"]
    if index < 10:
        lines.append(f"- [fact] {token}: {fact} 中文主题：{chinese}。")
    else:
        lines.append(f"- [fact] Neutral synthetic record number {index:05d}.")
    if count >= 100 and index == 20:
        lines.append("A minor aside mentions orderingprobe once.")
    if count >= 100 and index == count - 1:
        lines.append(" ".join(["orderingprobe"] * 32))
    lines.extend(["", "## Relations"])
    if index == 10:
        lines.extend(f"- relates_to [[{note_identifier(i)}]]" for i in range(min(50, count)) if i != index)
    elif count > 1:
        lines.append(f"- relates_to [[{note_identifier((index + 1) % count)}]]")
    newline = "\r\n" if index == 0 else "\n"
    data = (newline.join(lines) + newline).encode("utf-8")
    remaining = target_bytes - len(data) - len(newline)
    if remaining < 0:
        raise ValueError("Fixture target too small for its metadata and relations")
    padding = ("neutral filler " * (remaining // 15 + 1))[:remaining].encode("ascii")
    return data + padding + newline.encode("ascii")


def query_labels() -> list[dict[str, Any]]:
    rows = []
    for category, part in (("literal", 0), ("chinese_mixed", 1), ("paraphrase", 3)):
        for index, topic in enumerate(TOPICS):
            rows.append({"id": f"{category}-{index:02d}", "category": category,
                         "query": topic[part], "expected_identifiers": [note_identifier(index)],
                         "judgment": "Authored topic-to-note relevance; independent of returned hits"})
    return rows


def corpus_snapshot(vault: Path) -> dict[str, Any]:
    files = [{"path": path.relative_to(vault).as_posix(), "bytes": path.stat().st_size,
              "sha256": file_sha256(path)} for path in sorted(vault.rglob("*.md"))]
    return {"note_count": len(files), "total_bytes": sum(item["bytes"] for item in files),
            "files_sha256": fingerprint(files), "files": files}


def populate_fixture(sandbox: Path, scale: str) -> dict[str, Any]:
    """Populate a fresh verified G0 sandbox. Refuse to overwrite any note."""
    sandbox = verify_sandbox(sandbox)
    if any((sandbox / "vault").iterdir()):
        raise ValueError("Fixture vault must be empty")
    count, size = SCALES[scale]
    for index in range(count):
        path = sandbox / "vault" / note_path(index)
        path.parent.mkdir(parents=True, exist_ok=True)
        with path.open("xb") as stream:
            stream.write(note_bytes(index, size, count))
        # A stable mtime makes physical fixture recreation reproducible.
        os.utime(path, (1789776000, 1789776000))
    return corpus_snapshot(sandbox / "vault")


def protocol_contract() -> dict[str, Any]:
    return {
        "schema_version": 1, "seed": SEED, "corpus_recipe_version": 2,
        "ordering_probe": {"query": "orderingprobe", "best_identity": "Last corpus index (alphabetically last permalink)",
                           "weak_distractor_index": 20, "independent_of_thirty_gold_queries": True},
        "scales": {key: {"note_count": value[0], "target_note_bytes": value[1]} for key, value in SCALES.items()},
        "profiles": {name: profile(name)["commit"] for name in ("release", "main-preview")},
        "queries": query_labels(), "query_labels_sha256": fingerprint(query_labels()),
        "scenarios": {scale: scenarios({"scale": scale}) for scale in SCALES},
        "samples": {"cold": COLD_SAMPLES, "warm": WARM_SAMPLES,
                    "cold_definition": "Restart engine/probe for each scenario sample; index retained; OS disk cache uncontrolled",
                    "warm_definition": "One primed, index-ready process for all 30 observations of a scenario"},
        "estimator": {"median": "statistics.median", "p95": "sorted[ceil(0.95*n)-1]", "warm_p95_rank": 29},
        "budgets": {"adapter_overhead": "adapter warm p95 - direct MCP warm p95 <= max(50 ms, 0.25 * direct MCP p95)",
                    "meaningful_ui": "candidate warm p95 <= 1.10 * meaningful pre-optimization UI warm p95",
                    "primary_page_size": 50, "primary_max_rows": 150},
        "typing": {"edits": 30, "positions": {"beginning": 10, "middle": 10, "end": 10},
                   "operation": "Committed single-character edit after note display; record IME commitment",
                   "clock": "input-event timestamp to first presented frame containing updated text",
                   "trace_method": "Actual Tauri WebView CDP trace: backdated committed-input User Timing mark, changed-text paint/commit flow, matching successful native presentation feedback; qualified CDP/ETW frame and clock correlation, no rAF substitute",
                   "protocol_path": NATIVE_PROTOCOL.as_posix(), "protocol_sha256": file_sha256(ROOT / NATIVE_PROTOCOL),
                   "native_evidence_owner": "C06; C07 integrates the same-revision receipt",
                   "insertions": typing_insertions(),
                   "native_p95_ms": {"large-1mib": 50, "large-5mib": 100}},
        "availability": {"semantic": {"status": "unavailable", "reason": "Isolated G0 profiles disable embeddings; no model/provider activation authorized"},
                         "native_ui": {"status": "unavailable", "reason": "Native UI not exercised; empty desktop is not a performance baseline"},
                         "engine_internal_time": {"status": "unavailable", "reason": "Direct MCP clock includes owned probe/stdio/JSON overhead"},
                         "desktop_adapter": {"status": "unavailable", "reason": "C02/C03 integration and paired interleaving are required"}},
        "measurement_boundary": "Host perf_counter_ns around one Probe.raw; separate startup/readiness; no EmptyLibrary speedup",
        "rss_boundary": "OS process peak working set/high-water RSS since process start, sampled after response; not per-request allocation",
    }


def typing_insertions() -> dict[str, Any]:
    plans = {}
    for scale in ("large-1mib", "large-5mib"):
        text = note_bytes(0, SCALES[scale][1], 1).decode("utf-8")
        raw_length = len(text.encode("utf-16-le")) // 2
        api_text = text.replace("\r\n", "\n").replace("\r", "\n")
        length = len(api_text.encode("utf-16-le")) // 2
        offsets = {"beginning": 512, "middle": length // 2, "end": length - 2}
        plans[scale] = {"source_utf8_bytes": len(text.encode("utf-8")), "source_utf16_length": raw_length,
            "textarea_api_utf16_length": length, "coordinate_basis": "textarea API value: CRLF and CR normalized to LF; physical source unchanged",
            "reset": "Restore the same immutable fixture outside every measured insertion interval",
            "samples": [{"sample": group * 10 + index + 1, "position": position,
                         "utf16_offset": offset, "character": character, "is_composing": False}
                        for group, (position, offset) in enumerate(offsets.items())
                        for index, character in enumerate("abcdefghij")]}
    return plans


def measurement_manifest() -> dict[str, Any]:
    return {**protocol_contract(), "frozen_at": utc_now(),
            "machine": {"platform": platform.platform(), "machine": platform.machine(),
                        "processor": platform.processor(), "logical_cpus": os.cpu_count(),
                        "host_python": sys.version, "probe_binary_sha256": file_sha256(probe_binary()) if probe_binary().exists() else None}}


def validate_manifest(manifest: dict[str, Any]) -> None:
    for key, value in protocol_contract().items():
        if manifest.get(key) != value:
            raise ValueError(f"Frozen measurement contract drifted at {key}; use a reviewed new manifest")


def confined_output(path: Path) -> Path:
    path = path.resolve()
    roots = (ROOT / "artifacts/client-baseline", ROOT / "execution/evidence")
    if not any(path.is_relative_to(root.resolve()) for root in roots):
        raise ValueError("Evidence output must stay in artifacts/client-baseline or execution/evidence")
    return path


def generate(profile_id: str, scale: str, output: Path, manifest_path: Path) -> dict[str, Any]:
    output = confined_output(output)
    if output.exists():
        raise FileExistsError(f"Refusing to replace fixture manifest: {output}")
    frozen_path = manifest_path
    if not frozen_path.is_file():
        raise FileNotFoundError("Freeze a reviewed measurement manifest first")
    frozen = read_json(frozen_path)
    validate_manifest(frozen)
    sandbox = create_sandbox(ROOT / ".work/g0/client-baseline", profile_id)
    record = {"schema_version": 1, "created_at": utc_now(), "profile": profile_id,
              "engine_sha": profile(profile_id)["commit"], "scale": scale, "seed": SEED,
              "sandbox": str(sandbox), "corpus": populate_fixture(sandbox, scale),
              "labels": query_labels() if scale.startswith("notes-") else [],
              "measurement_manifest": frozen, "measurement_manifest_sha256": fingerprint(frozen)}
    write_json(output, record)
    return record


def reproduce(manifest: Path, output: Path) -> dict[str, Any]:
    """Recreate each physical corpus twice, separately for both engine profiles."""
    output = confined_output(output)
    if output.exists():
        raise FileExistsError(output)
    frozen = read_json(manifest)
    validate_manifest(frozen)
    report: dict[str, Any] = {"created_at": utc_now(), "manifest_sha256": fingerprint(frozen),
                              "status": "incomplete", "corpora": []}
    artifact_dir = ROOT / "artifacts/client-baseline" / output.stem
    try:
        for profile_id in ("release", "main-preview"):
            for scale in SCALES:
                print(f"Reproducing {profile_id} {scale} twice", flush=True)
                paths = [artifact_dir / f"{profile_id}-{scale}-{run}.fixture.json" for run in (1, 2)]
                records = [generate(profile_id, scale, path, manifest) for path in paths]
                equal = records[0]["corpus"] == records[1]["corpus"] and records[0]["labels"] == records[1]["labels"]
                report["corpora"].append({"profile": profile_id, "scale": scale, "reproduced": equal,
                    "fixture_manifests": [str(path.relative_to(ROOT)).replace("\\", "/") for path in paths],
                    "note_count": records[0]["corpus"]["note_count"], "total_bytes": records[0]["corpus"]["total_bytes"],
                    "files_sha256": records[0]["corpus"]["files_sha256"], "labels_sha256": fingerprint(records[0]["labels"])})
                if not equal:
                    raise AssertionError(f"Corpus reproduction failed: {profile_id} {scale}")
        report["status"] = "reproduced"
    finally:
        write_json(output, report)
    return report


def load_fixture(path: Path) -> tuple[dict[str, Any], Path]:
    record = read_json(path)
    sandbox = Path(record["sandbox"]).resolve()
    if not sandbox.is_relative_to((ROOT / ".work/g0/client-baseline").resolve()):
        raise ValueError("Only generated client-baseline sandboxes are accepted")
    verify_sandbox(sandbox)
    if read_json(sandbox / MARKER)["profile"] != record["profile"]:
        raise ValueError("Fixture and sandbox profile differ")
    if record["engine_sha"] != profile(record["profile"])["commit"]:
        raise ValueError("Fixture engine pin differs")
    validate_manifest(record["measurement_manifest"])
    if record.get("measurement_manifest_sha256") != fingerprint(record["measurement_manifest"]):
        raise ValueError("Fixture measurement manifest hash differs")
    if corpus_snapshot(sandbox / "vault") != record["corpus"]:
        raise ValueError("Fixture bytes/count changed; do not benchmark a mutated corpus")
    return record, sandbox


def probe_binary() -> Path:
    return ROOT / "target/debug" / ("bmdock-probe.exe" if os.name == "nt" else "bmdock-probe")


def verify_engine(profile_id: str) -> None:
    checkout = ROOT / ".work/engines" / profile_id
    actual = subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip()
    if actual != profile(profile_id)["commit"]:
        raise ValueError("Engine checkout does not match the immutable profile")
    dirty = subprocess.check_output(["git", "-C", str(checkout), "status", "--porcelain", "--untracked-files=no"], text=True)
    if dirty.strip():
        raise ValueError("Engine has tracked modifications; pin alone cannot prove its runtime")
    engine_python(profile_id)
    if not probe_binary().is_file():
        raise FileNotFoundError("Build the existing debug bmdock-probe first")


def decode_tool(response: dict[str, Any]) -> tuple[str, Any]:
    """Preserve observed FastMCP union shape, including guidance strings."""
    if "error" in response:
        return "control_error", response["error"]
    result = response.get("result")
    if not isinstance(result, dict):
        return "unexpected_envelope", result
    if result.get("isError"):
        return "tool_error", result
    structured = result.get("structuredContent")
    if isinstance(structured, dict):
        payload = structured.get("result", structured)
        return ("structured" if isinstance(payload, (dict, list)) else "guidance_text"), payload
    content = result.get("content", [])
    texts = [block["text"] for block in content if block.get("type") == "text"]
    if len(texts) == 1:
        try:
            payload = json.loads(texts[0])
        except (ValueError, TypeError):
            return "guidance_text", texts[0]
        return ("json_text" if isinstance(payload, (dict, list)) else "guidance_text"), payload
    return "unexpected_envelope", result


def search_payload(response: dict[str, Any]) -> dict[str, Any]:
    kind, payload = decode_tool(response)
    if kind not in {"structured", "json_text"} or not isinstance(payload, dict) or not isinstance(payload.get("results"), list):
        raise ValueError(f"Expected observed search object, got {kind}: {str(payload)[:300]}")
    return payload


def validate_observation(scenario: dict[str, Any], response: dict[str, Any], source: bytes | None = None) -> tuple[str, Any]:
    """A structured wrapper is not sufficient evidence of a successful operation."""
    kind, payload = decode_tool(response)
    if kind not in {"structured", "json_text"}:
        raise ValueError(f"{scenario['id']}: expected structured operation success, got {kind}")
    if isinstance(payload, dict) and payload.get("error"):
        raise ValueError(f"{scenario['id']}: structured tool failure {payload['error']}")
    tool = scenario["tool"]
    if tool == "read_note":
        if not isinstance(payload, dict) or not isinstance(payload.get("content"), str):
            raise ValueError("Read success requires delivered string content")
        if payload.get("permalink") != scenario["args"]["identifier"]:
            raise ValueError("Read result identity differs from requested exact note")
        if source is None:
            raise ValueError("Full-source success requires the known physical fixture")
        original = source.decode("utf-8")
        if payload["content"] not in {original, original.replace("\r\n", "\n")}:
            raise ValueError("Full-source result lost fixture content or frontmatter")
    elif tool == "search_notes":
        payload = search_payload(response)
        for field in ("current_page", "page_size", "total"):
            if type(payload.get(field)) is not int or payload[field] < (0 if field == "total" else 1):
                raise ValueError(f"Search success requires valid {field}")
        if type(payload.get("total_is_exact")) is not bool or type(payload.get("has_more")) is not bool:
            raise ValueError("Search success requires exactness and continuation fields")
        if any(not isinstance(row, dict) or not isinstance(row.get("permalink"), str) for row in payload["results"]):
            raise ValueError("Search success requires independently addressable result identities")
    elif tool == "build_context":
        if not isinstance(payload, dict) or not isinstance(payload.get("results"), list) or not isinstance(payload.get("metadata"), dict) or type(payload.get("has_more")) is not bool:
            raise ValueError("Context success requires results, metadata and continuation")
    elif tool == "recent_activity":
        if not isinstance(payload, list) or any(not isinstance(row, dict) or not isinstance(row.get("permalink"), str) for row in payload):
            raise ValueError("Activity success requires the observed list of identified items")
    return kind, payload


def request(probe: Probe, name: str, args: dict[str, Any]) -> dict[str, Any]:
    # Project discovery has no project argument; the process config still contains only our fixture.
    arguments = args if name == "list_memory_projects" else {"project": PROJECT, **args}
    return probe.raw("tools/call", {"name": name, "arguments": arguments})


def wait_index(probe: Probe, record: dict[str, Any], seconds: int = 600) -> dict[str, Any]:
    """Check the complete indexed identity set through paginated official search."""
    expected = {note_identifier(i) for i in range(record["corpus"]["note_count"])}
    start = time.monotonic()
    attempts = 0
    while time.monotonic() - start < seconds:
        attempts += 1
        found: set[str] = set()
        page = 1
        while True:
            payload = search_payload(request(probe, "search_notes", {"query": "bmdockcorpus", "search_type": "text",
                "entity_types": ["entity"], "page": page, "page_size": 100, "output_format": "json"}))
            rows = payload["results"]
            found.update(row["permalink"] for row in rows if isinstance(row.get("permalink"), str))
            if len(rows) < 100 or page * 100 >= len(expected):
                break
            page += 1
        if found == expected:
            return {"status": "ready", "indexed_note_count": len(found), "identity_set_sha256": fingerprint(sorted(found)),
                    "elapsed_ms": (time.monotonic() - start) * 1000, "attempts": attempts,
                    "method": "Complete bmdockcorpus entity identity set through official paginated search"}
        time.sleep(0.5)
    raise TimeoutError(f"Index not ready after {seconds}s; expected {len(expected)} identities, observed {len(found)}")


def start_probe(record: dict[str, Any], sandbox: Path) -> tuple[Probe, float]:
    start = time.perf_counter_ns()
    probe = Probe(probe_binary(), engine_python(record["profile"]), sandbox, isolated_env(sandbox))
    try:
        require_protocol_version(probe.connected)
    except BaseException:
        probe.abort()
        raise
    return probe, (time.perf_counter_ns() - start) / 1e6


def scenarios(record: dict[str, Any]) -> list[dict[str, Any]]:
    rows = [{"id": "read-full", "tool": "read_note", "args": {"identifier": note_identifier(0), "output_format": "json", "include_frontmatter": True}}]
    if record["scale"].startswith("large-"):
        return rows
    rows.extend({"id": item["id"], "tool": "search_notes", "args": {"query": item["query"], "search_type": "text",
                 "entity_types": ["entity"], "page": 1, "page_size": 50, "output_format": "json"}} for item in query_labels())
    rows.extend([
        {"id": "ordering-probe", "tool": "search_notes", "args": {"query": "orderingprobe", "search_type": "text", "entity_types": ["entity"], "page": 1, "page_size": 50, "output_format": "json"}},
        {"id": "search-page-2", "tool": "search_notes", "args": {"query": "bmdockcorpus", "search_type": "text", "entity_types": ["entity"], "page": 2, "page_size": 50, "output_format": "json"}},
        {"id": "context", "tool": "build_context", "args": {"url": "memory://corpus/note-00010", "depth": 1, "max_related": 20, "page_size": 50, "timeframe": "2026-01-01", "output_format": "json"}},
        {"id": "activity", "tool": "recent_activity", "args": {"type": "entity", "timeframe": "2026-01-01", "page_size": 50, "output_format": "json"}},
    ])
    return rows


def request_identity(record: dict[str, Any], generation: int, scenario: dict[str, Any]) -> dict[str, Any]:
    return {"profile": record["profile"], "session_generation": generation,
            "workspace": "generated-client-baseline", "project": PROJECT,
            "operation": scenario["tool"], "arguments": scenario["args"],
            "corpus_sha256": record["corpus"]["files_sha256"]}


def source_comparison(response: dict[str, Any], source: bytes) -> dict[str, Any]:
    kind, payload = decode_tool(response)
    if kind not in {"structured", "json_text"} or not isinstance(payload, dict) or not isinstance(payload.get("content"), str):
        return {"status": "unresolved", "response_kind": kind}
    text = payload["content"]
    original = source.decode("utf-8")
    return {"status": "observed", "delivered_utf8_sha256": hashlib.sha256(text.encode("utf-8")).hexdigest(),
            "disk_sha256": hashlib.sha256(source).hexdigest(), "physical_bytes_equal": text.encode("utf-8") == source,
            "equal_after_crlf_to_lf": text == original.replace("\r\n", "\n"),
            "delivered_has_frontmatter": text.startswith("---"), "delivered_has_chinese": "保留原始中文" in text,
            "disk_crlf_count": original.count("\r\n"), "delivered_crlf_count": text.count("\r\n"),
            "client_text_preservation": "JSON-decoded string captured unchanged; no newline transformation by this harness"}


def capture(fixture: Path, output: Path) -> dict[str, Any]:
    output = confined_output(output)
    if output.exists():
        raise FileExistsError(output)
    record, sandbox = load_fixture(fixture)
    verify_engine(record["profile"])
    result: dict[str, Any] = {"profile": record["profile"], "engine_sha": record["engine_sha"], "captured_at": utc_now(),
                              "corpus_sha256": record["corpus"]["files_sha256"], "status": "incomplete", "records": []}
    probe = None
    try:
        probe, result["startup_ms"] = start_probe(record, sandbox)
        tools, _ = paginate(probe.request, "tools/list", "tools")
        delta = inventory_delta(profile(record["profile"])["expected_tools"], [item["name"] for item in tools])
        if any(delta.values()):
            raise ValueError(f"Profile capability drift: {delta}")
        result["handshake"] = probe.connected
        result["installed_versions"] = json.loads(subprocess.check_output(
            [str(engine_python(record["profile"])), "-c",
             "import importlib.metadata as m,json; print(json.dumps({n:m.version(n) for n in ('basic-memory','fastmcp','mcp')}))"],
            cwd=sandbox, env=isolated_env(sandbox), text=True, encoding="utf-8", timeout=30))
        result["schemas"] = {item["name"]: item for item in tools if item["name"] in {"read_note", "search_notes", "build_context", "recent_activity", "list_memory_projects", "list_directory"}}
        result["index"] = wait_index(probe, record)
        cases = scenarios(record)
        cases.extend([
            {"id": "projects", "tool": "list_memory_projects", "args": {}},
            {"id": "directory-root", "tool": "list_directory", "args": {"dir_name": "/", "depth": 1, "page": 1, "page_size": 50, "output_format": "json"}},
            {"id": "directory-nested", "tool": "list_directory", "args": {"dir_name": "/group-000", "depth": 2, "page": 1, "page_size": 50, "output_format": "json"}},
            {"id": "directory-leaf", "tool": "list_directory", "args": {"dir_name": "/group-000/item-000", "depth": 1, "page": 1, "page_size": 50, "output_format": "json"}},
            {"id": "read-default", "tool": "read_note", "args": {"identifier": note_identifier(0), "output_format": "json"}},
            {"id": "read-missing", "tool": "read_note", "args": {"identifier": "definitelymissingbmdocknote", "output_format": "json", "include_frontmatter": True}},
            {"id": "search-empty", "tool": "search_notes", "args": {"query": "definitelymissingbmdocktoken", "output_format": "json"}},
            {"id": "search-invalid-page", "tool": "search_notes", "args": {"query": "bmdockcorpus", "page": "not-a-number", "output_format": "json"}},
            {"id": "search-semantic-unavailable", "tool": "search_notes", "args": {"query": TOPICS[0][3], "search_type": "semantic", "output_format": "json"}},
            {"id": "search-filter", "tool": "search_notes", "args": {"query": "bmdockcorpus", "note_types": ["note"], "entity_types": ["entity"], "page_size": 50, "output_format": "json"}},
        ])
        if "compact" in result["schemas"]["search_notes"]["inputSchema"].get("properties", {}):
            cases.append({"id": "search-compact", "tool": "search_notes", "args": {"query": "bmdockcorpus", "compact": True, "page_size": 50, "output_format": "json"}})
        for case in cases:
            response = request(probe, case["tool"], case["args"])
            kind, payload = decode_tool(response)
            observation = {"id": case["id"], "identity": request_identity(record, 1, case),
                           "request": {"name": case["tool"], "arguments": case["args"] if case["tool"] == "list_memory_projects" else {"project": PROJECT, **case["args"]}},
                           "response": response, "response_kind": kind}
            if case["tool"] == "search_notes" and isinstance(payload, dict):
                observation["paging"] = {key: payload.get(key) for key in ("current_page", "page_size", "total", "total_is_exact", "has_more")}
            if case["id"] in {"read-full", "read-default"}:
                observation["source_comparison"] = source_comparison(response, (sandbox / "vault" / note_path(0)).read_bytes())
            if case["id"] in {item["id"] for item in scenarios(record)}:
                try:
                    validate_observation(case, response, (sandbox / "vault" / note_path(0)).read_bytes() if case["tool"] == "read_note" else None)
                except ValueError as error:
                    observation["validation_failure"] = str(error)
            if case["id"] == "ordering-probe" and isinstance(payload, dict):
                observed = [row.get("permalink") for row in payload.get("results", [])]
                expected_best = note_identifier(record["corpus"]["note_count"] - 1)
                observation["ordering_probe"] = {"expected_best": expected_best, "weak_distractor": note_identifier(20),
                                                  "observed_order": observed, "best_sorts_last_and_ranks_first": bool(observed) and observed[0] == expected_best}
            gold = next((item for item in record["labels"] if item["id"] == case["id"]), None)
            if gold is not None and isinstance(payload, dict) and isinstance(payload.get("results"), list):
                found = {row.get("permalink") for row in payload["results"]}
                expected = set(gold["expected_identifiers"])
                observation["relevance"] = {"mode": "text", "expected": sorted(expected), "recall_at_50": len(found & expected) / len(expected)}
            result["records"].append(observation)
        result["shutdown"] = probe.close()
        probe = None
        required = {"read-full", "literal-00", "search-empty", "context", "activity", "directory-root", "directory-nested", "directory-leaf", "ordering-probe"}
        result["unresolved_required_records"] = [row["id"] for row in result["records"]
            if row["id"] in required and (row["response_kind"] not in {"structured", "json_text"} or "validation_failure" in row)]
        result["project_catalog"] = {"format": "observed single-project prose", "adapter_policy": "Use the explicit host fixture route; do not parse prose into a catalog"}
        result["status"] = "captured" if not result["unresolved_required_records"] else "captured_with_unresolved_contracts"
    except Exception as error:
        result["failure"] = {"type": type(error).__name__, "message": str(error)}
        raise
    finally:
        if probe is not None:
            probe.abort()
        result["physical_corpus_unchanged"] = corpus_snapshot(sandbox / "vault") == record["corpus"]
        write_json(output, redact(result, sandbox))
    return result


def peak_rss(pid: int) -> dict[str, Any]:
    if os.name == "nt":
        from ctypes import wintypes

        class Counters(ctypes.Structure):
            _fields_ = [("cb", wintypes.DWORD), ("PageFaultCount", wintypes.DWORD)] + [
                (name, ctypes.c_size_t) for name in ("PeakWorkingSetSize", "WorkingSetSize", "QuotaPeakPagedPoolUsage",
                "QuotaPagedPoolUsage", "QuotaPeakNonPagedPoolUsage", "QuotaNonPagedPoolUsage", "PagefileUsage", "PeakPagefileUsage")]

        kernel = ctypes.WinDLL("kernel32", use_last_error=True)
        psapi = ctypes.WinDLL("psapi", use_last_error=True)
        kernel.OpenProcess.argtypes = [wintypes.DWORD, wintypes.BOOL, wintypes.DWORD]
        kernel.OpenProcess.restype = wintypes.HANDLE
        kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        psapi.GetProcessMemoryInfo.argtypes = [wintypes.HANDLE, ctypes.POINTER(Counters), wintypes.DWORD]
        handle = kernel.OpenProcess(0x0400 | 0x0010, False, pid)
        if not handle:
            return {"bytes": None, "reason": f"OpenProcess failed: {ctypes.get_last_error()}"}
        try:
            counters = Counters()
            counters.cb = ctypes.sizeof(counters)
            if not psapi.GetProcessMemoryInfo(handle, ctypes.byref(counters), counters.cb):
                return {"bytes": None, "reason": f"GetProcessMemoryInfo failed: {ctypes.get_last_error()}"}
            return {"bytes": counters.PeakWorkingSetSize, "method": "PeakWorkingSetSize"}
        finally:
            kernel.CloseHandle(handle)
    try:
        for line in Path(f"/proc/{pid}/status").read_text().splitlines():
            if line.startswith("VmHWM:"):
                return {"bytes": int(line.split()[1]) * 1024, "method": "proc VmHWM"}
    except OSError as error:
        return {"bytes": None, "reason": str(error)}
    return {"bytes": None, "reason": "Per-process peak RSS unavailable on this OS"}


def summarize(samples: list[float]) -> dict[str, Any]:
    if not samples or any(not math.isfinite(value) or value < 0 for value in samples):
        raise ValueError("Expected nonempty finite nonnegative samples")
    return {"n": len(samples), "median_ms": statistics.median(samples),
            "p95_ms": sorted(samples)[math.ceil(.95 * len(samples)) - 1]}


def measure(fixture: Path, output: Path, scenario_ids: list[str] | None) -> dict[str, Any]:
    output = confined_output(output)
    if output.exists():
        raise FileExistsError(output)
    record, sandbox = load_fixture(fixture)
    verify_engine(record["profile"])
    selected = scenarios(record)
    if scenario_ids:
        missing = set(scenario_ids) - {item["id"] for item in selected}
        if missing:
            raise ValueError(f"Unknown scenarios: {sorted(missing)}")
        selected = [item for item in selected if item["id"] in scenario_ids]
    result: dict[str, Any] = {"status": "incomplete", "profile": record["profile"], "engine_sha": record["engine_sha"],
        "scale": record["scale"], "selected_scenarios": [item["id"] for item in selected],
        "unmeasured_scenarios": [item["id"] for item in scenarios(record) if item not in selected],
        "measured_at": utc_now(), "manifest_sha256": fingerprint(record["measurement_manifest"]),
        "corpus_sha256": record["corpus"]["files_sha256"], "samples": [], "sessions": [], "summary": {},
        "clock": record["measurement_manifest"]["measurement_boundary"], "paired_desktop_runs": "unavailable",
        "host_load": "Shared developer host; OS caches and unrelated background load uncontrolled; UTC sample timestamps permit overlap audit",
        "rss_scope": {"engine_peak_rss": "Direct connected.childPid only; on Windows this is the virtualenv launcher, not full Python worker/tree RSS",
                      "engine_worker_or_tree_peak_rss": "UNVERIFIED; see execution/evidence/c01-rss-scope.json"}}
    source = (sandbox / "vault" / note_path(0)).read_bytes() if any(item["tool"] == "read_note" for item in selected) else None
    generation = 0
    probe = None
    try:
        # Prepare the index once. Cold request measurements must not warm caches with a readiness search.
        probe, startup = start_probe(record, sandbox)
        result["index"] = wait_index(probe, record)
        result["preparation_shutdown"] = probe.close()
        probe = None
        for scenario in selected:
            print(f"Measuring {record['profile']} {record['scale']} {scenario['id']}: 5 cold + 30 warm", flush=True)
            for phase, count in (("cold", COLD_SAMPLES), ("warm", WARM_SAMPLES)):
                for index in range(count):
                    if probe is None:
                        generation += 1
                        probe, startup = start_probe(record, sandbox)
                        result["sessions"].append({"generation": generation, "startup_ms": startup, "phase": phase})
                        if phase == "warm":
                            warmup = request(probe, scenario["tool"], scenario["args"])
                            validate_observation(scenario, warmup, source)
                            probe.transcript.clear()
                    sample_started_at = utc_now()
                    before = time.perf_counter_ns()
                    response = request(probe, scenario["tool"], scenario["args"])
                    elapsed = (time.perf_counter_ns() - before) / 1e6
                    kind, payload = decode_tool(response)
                    sample = {"scenario": scenario["id"], "phase": phase, "sample": index + 1,
                        "started_at": sample_started_at,
                        "identity": request_identity(record, generation, scenario), "elapsed_ms": elapsed, "command_count": 1,
                        "payload_bytes": len(json.dumps(response, ensure_ascii=False, separators=(",", ":")).encode("utf-8")),
                        "payload_size_method": "UTF-8 compact JSON reserialization, not pipe byte count", "response_kind": kind,
                        "probe_peak_rss": peak_rss(probe.process.pid), "engine_peak_rss": peak_rss(probe.connected["childPid"]),
                        "response_sha256": fingerprint(response), "valid": False}
                    if isinstance(payload, dict) and isinstance(payload.get("results"), list):
                        sample["result_count"] = len(payload["results"])
                        sample["ordered_identifiers"] = [row.get("permalink") for row in payload["results"]]
                    if isinstance(payload, dict) and isinstance(payload.get("content"), str):
                        sample["content_utf8_bytes"] = len(payload["content"].encode("utf-8"))
                    result["samples"].append(sample)
                    # The existing probe records full responses for smoke tests. Retaining
                    # 30 large notes would benchmark transcript accumulation, not read cost.
                    probe.transcript.clear()
                    validate_observation(scenario, response, source)
                    sample["valid"] = True
                    if phase == "cold" or index == count - 1:
                        result["sessions"][-1]["shutdown"] = probe.close()
                        probe = None
                values = [row["elapsed_ms"] for row in result["samples"] if row["scenario"] == scenario["id"] and row["phase"] == phase]
                result["summary"][f"{scenario['id']}/{phase}"] = summarize(values)
            write_json(output, redact(result, sandbox))
        result["status"] = "measured_selected_scenarios"
    except Exception as error:
        result["failure"] = {"type": type(error).__name__, "message": str(error)}
        raise
    finally:
        if probe is not None:
            probe.abort()
        result["finished_at"] = utc_now()
        result["physical_corpus_unchanged"] = corpus_snapshot(sandbox / "vault") == record["corpus"]
        write_json(output, redact(result, sandbox))
    return result


def coverage(manifest_path: Path, measurements: list[Path], output: Path) -> dict[str, Any]:
    frozen = read_json(manifest_path)
    validate_manifest(frozen)
    output = confined_output(output)
    if output.exists():
        raise FileExistsError(output)
    observed: dict[tuple[str, str, str, str], set[int]] = {}
    for path in measurements:
        report = read_json(path)
        if report["manifest_sha256"] != fingerprint(frozen):
            raise ValueError("Coverage cannot merge different frozen measurement manifests")
        if report.get("status") != "measured_selected_scenarios" or report.get("failure") or report.get("physical_corpus_unchanged") is not True:
            raise ValueError("Coverage requires completed measurements on an unchanged physical corpus")
        if report.get("engine_sha") != frozen["profiles"].get(report.get("profile")):
            raise ValueError("Coverage engine SHA does not match its frozen profile")
        for sample in report["samples"]:
            if not sample.get("valid"):
                continue
            key = (report["profile"], report["scale"], sample["scenario"], sample["phase"])
            numbers = observed.setdefault(key, set())
            if sample["sample"] in numbers:
                raise ValueError(f"Duplicate measurement cell/sample: {key} {sample['sample']}")
            numbers.add(sample["sample"])
    cells = []
    for profile_id in frozen["profiles"]:
        for scale, cases in frozen["scenarios"].items():
            for case in cases:
                for phase in ("cold", "warm"):
                    required = frozen["samples"][phase]
                    numbers = observed.get((profile_id, scale, case["id"], phase), set())
                    complete = numbers == set(range(1, required + 1))
                    cells.append({"profile": profile_id, "scale": scale, "scenario": case["id"], "phase": phase,
                                  "required_samples": required, "observed_valid_samples": len(numbers),
                                  "status": "measured" if complete else "missing"})
    result = {"manifest_sha256": fingerprint(frozen), "created_at": utc_now(), "cells": cells,
              "measured_cells": sum(row["status"] == "measured" for row in cells), "total_cells": len(cells),
              "full_matrix_complete": all(row["status"] == "measured" for row in cells),
              "native_ui": "unavailable", "semantic": "unavailable", "paired_desktop": "unavailable"}
    write_json(output, result)
    return result


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest="command", required=True)
    freeze = commands.add_parser("freeze", help="Write immutable measurement rules before optimization")
    freeze.add_argument("--output", type=Path, required=True)
    gen = commands.add_parser("generate", help="Create fresh isolated synthetic corpus and fixture manifest")
    gen.add_argument("--profile", choices=("release", "main-preview"), required=True)
    gen.add_argument("--scale", choices=tuple(SCALES), required=True)
    gen.add_argument("--output", type=Path, required=True)
    gen.add_argument("--manifest", type=Path, required=True, help="Reviewed frozen measurement manifest")
    repro = commands.add_parser("reproduce", help="Recreate all five corpora twice for both profiles and compare physical hashes/labels")
    repro.add_argument("--manifest", type=Path, required=True)
    repro.add_argument("--output", type=Path, required=True)
    matrix = commands.add_parser("matrix", help="Report every measured/missing cell without inferring full acceptance")
    matrix.add_argument("--manifest", type=Path, required=True)
    matrix.add_argument("--measurement", type=Path, action="append", default=[])
    matrix.add_argument("--output", type=Path, required=True)
    for name, description in (("capture", "Record profile-local schemas, raw envelopes and source fidelity"),
                              ("measure", "Record fixed five cold/30 warm direct-MCP samples"),
                              ("scenarios", "List available scenario identities without starting an engine")):
        command = commands.add_parser(name, help=description)
        command.add_argument("--fixture", type=Path, required=True)
        if name != "scenarios":
            command.add_argument("--output", type=Path, required=True)
        if name == "measure":
            command.add_argument("--scenario", action="append", help="Repeat for a bounded subset; counts remain 5/30")
    args = parser.parse_args(argv)
    if args.command == "freeze":
        destination = confined_output(args.output)
        if destination.exists():
            raise FileExistsError("Frozen manifest already exists; use a new reviewed path")
        write_json(destination, measurement_manifest())
    elif args.command == "generate":
        result = generate(args.profile, args.scale, args.output, args.manifest)
        print(json.dumps({"fixture": str(args.output), "sandbox": result["sandbox"],
                          "note_count": result["corpus"]["note_count"], "total_bytes": result["corpus"]["total_bytes"]}))
    elif args.command == "reproduce":
        result = reproduce(args.manifest, args.output)
        print(json.dumps({"status": result["status"], "corpora": len(result["corpora"]), "output": str(args.output)}))
    elif args.command == "matrix":
        result = coverage(args.manifest, args.measurement, args.output)
        print(json.dumps({"measured_cells": result["measured_cells"], "total_cells": result["total_cells"], "full_matrix_complete": result["full_matrix_complete"]}))
    elif args.command == "capture":
        result = capture(args.fixture, args.output)
        print(json.dumps({"status": result["status"], "records": len(result["records"]), "output": str(args.output)}))
    elif args.command == "measure":
        result = measure(args.fixture, args.output, args.scenario)
        print(json.dumps({"status": result["status"], "samples": len(result["samples"]), "output": str(args.output)}))
    else:
        record, _ = load_fixture(args.fixture)
        print(json.dumps(scenarios(record), ensure_ascii=False, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
