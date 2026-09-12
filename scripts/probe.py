"""Real-process contract smoke tests. No mock substitutes and no production paths."""
from __future__ import annotations

import json
import os
import queue
import subprocess
import threading
import time
from pathlib import Path
from typing import Any

from scripts.core import (ROOT, PROJECT, create_sandbox, fingerprint, inventory_delta,
                          isolated_env, paginate, profile, read_json, run, write_json,
                          classify_tool_result)


class Probe:
    def __init__(self, binary: Path, python: Path, sandbox: Path, env: dict[str, str]):
        self._log = (sandbox / "engine.stderr.log").open("w", encoding="utf-8")
        self.process = subprocess.Popen(
            [str(binary), str(python), str(ROOT / "scripts/engine_worker.py"), str(sandbox)],
            cwd=sandbox, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
            stderr=self._log, text=True, encoding="utf-8", bufsize=1,
        )
        self._messages: queue.Queue[Any] = queue.Queue()
        self._counter = 0
        self.transcript: list[dict[str, Any]] = []
        self._closed = False
        self._thread = threading.Thread(target=self._read, daemon=True)
        self._thread.start()
        try:
            self.connected = self.receive(150)
            if self.connected.get("event") != "connected":
                raise RuntimeError(f"Unexpected handshake: {self.connected}")
        except BaseException:
            self.abort()
            raise

    def _read(self) -> None:
        assert self.process.stdout is not None
        try:
            for line in self.process.stdout:
                self._messages.put(json.loads(line))
        except Exception as error:
            self._messages.put(error)
        finally:
            self._messages.put(EOFError("Probe exited; see engine.stderr.log"))

    def receive(self, seconds: int = 110) -> dict[str, Any]:
        try:
            message = self._messages.get(timeout=seconds)
        except queue.Empty as error:
            raise TimeoutError("Probe response unknown; no automatic retry") from error
        if isinstance(message, Exception):
            raise message
        if not isinstance(message, dict):
            raise RuntimeError("Probe emitted a non-object response")
        return message

    def raw(self, method: str, params: dict[str, Any]) -> dict[str, Any]:
        if self._closed:
            raise RuntimeError("Probe already closed")
        self._counter += 1
        message = {"id": self._counter, "method": method, "params": params}
        assert self.process.stdin is not None
        self.process.stdin.write(json.dumps(message, ensure_ascii=False) + "\n")
        self.process.stdin.flush()
        response = self.receive()
        if response.get("id") != self._counter:
            raise RuntimeError("Control response ID mismatch")
        self.transcript.append({"request": message, "response": response})
        return response

    def request(self, method: str, params: dict[str, Any]) -> dict[str, Any]:
        response = self.raw(method, params)
        if "error" in response:
            raise RuntimeError(f"{method}: {response['error']}")
        result = response.get("result")
        if not isinstance(result, dict):
            raise RuntimeError(f"{method}: expected result object")
        return result

    def close(self) -> dict[str, Any]:
        if self._closed:
            raise RuntimeError("Probe already closed")
        assert self.process.stdin is not None
        self.process.stdin.close()
        try:
            message = self.receive(75)
            code = self.process.wait(timeout=10)
            if message.get("event") != "shutdown" or code != 0:
                raise RuntimeError(f"Unclean shutdown: {message}, exit={code}")
            if not message.get("sdkClosed") or message.get("process") != {"forced": False, "exitCode": 0}:
                raise RuntimeError(f"Shutdown not verified: {message}")
            return message
        finally:
            self.abort()

    def abort(self) -> None:
        if self.process.poll() is None:
            # Closing the controller requests EOF; give the owned probe its cleanup budget.
            if self.process.stdin and not self.process.stdin.closed:
                self.process.stdin.close()
            try:
                self.process.wait(timeout=75)
            except subprocess.TimeoutExpired:
                self.process.kill()
                self.process.wait(timeout=10)
        self._closed = True
        self._thread.join(timeout=2)
        if self.process.stdout:
            self.process.stdout.close()
        self._log.close()


def tool_arguments(tool: dict[str, Any], values: dict[str, Any]) -> dict[str, Any]:
    """Use only parameters actually advertised; unknown required fields stop the test."""
    schema = tool["inputSchema"]
    props = schema.get("properties", {})
    result = {key: value for key, value in values.items() if key in props}
    missing = set(schema.get("required", [])) - result.keys()
    if missing:
        raise ValueError(f"Unimplemented required parameters for {tool['name']}: {sorted(missing)}")
    return result


def wait_note(vault: Path, sentinel: str, seconds: int = 30) -> Path:
    until = time.monotonic() + seconds
    while time.monotonic() < until:
        matches = []
        for path in vault.rglob("*.md"):
            try:
                if sentinel in path.read_text(encoding="utf-8"):
                    matches.append(path)
            except (OSError, UnicodeError):
                continue
        if len(matches) == 1:
            return matches[0]
        if len(matches) > 1:
            raise AssertionError("Fixture sentinel unexpectedly exists in multiple notes")
        time.sleep(0.1)
    raise TimeoutError("Write result did not become observable on disk; no retry was performed")


def engine_python(profile_id: str) -> Path:
    profile(profile_id)
    base = ROOT / ".work/engines" / profile_id / ".venv"
    path = base / ("Scripts/python.exe" if os.name == "nt" else "bin/python")
    if not path.is_file():
        raise FileNotFoundError(f"Missing isolated engine for {profile_id}. Run just setup first.")
    # Keep the venv symlink path; resolving it would discard the venv's pyvenv.cfg.
    return path.absolute()


def redact(value: Any, root: Path) -> Any:
    if isinstance(value, str):
        return value.replace(str(root), "<G0_SANDBOX>").replace(root.as_posix(), "<G0_SANDBOX>")
    if isinstance(value, list):
        return [redact(item, root) for item in value]
    if isinstance(value, dict):
        return {key: redact(item, root) for key, item in value.items()}
    return value


def require_protocol_version(connected: dict[str, Any], expected: str = "2025-11-25") -> str:
    """Fail closed when the negotiated MCP version is missing or mixed."""
    server = connected.get("server")
    if not isinstance(server, dict):
        raise AssertionError(f"Handshake missing server object: {connected}")
    version = server.get("protocolVersion")
    if version != expected:
        raise AssertionError(f"Unexpected protocolVersion: {version!r}")
    return version


def tool_schema_fingerprints(tools: list[Any]) -> dict[str, str]:
    """Record per-tool inputSchema fingerprints without mixing profiles."""
    fingerprints: dict[str, str] = {}
    for tool in tools:
        if not isinstance(tool, dict):
            raise AssertionError("tools/list returned a non-object tool")
        name = tool.get("name")
        schema = tool.get("inputSchema")
        if not isinstance(name, str) or not isinstance(schema, dict):
            raise AssertionError("tools/list returned a tool without name/inputSchema")
        if schema.get("type") != "object" or not isinstance(schema.get("properties", {}), dict):
            raise AssertionError(f"{name}: inputSchema is not an object schema")
        required = schema.get("required", [])
        if not isinstance(required, list) or not all(isinstance(item, str) for item in required):
            raise AssertionError(f"{name}: invalid required schema field")
        if not set(required).issubset(schema.get("properties", {})):
            raise AssertionError(f"{name}: required field missing from properties")
        fingerprints[name] = fingerprint(schema)
    return fingerprints


def require_distinct_search_and_fetch(tools: dict[str, Any]) -> dict[str, Any]:
    """search and fetch stay separate tools; neither may impersonate the other."""
    search = tools.get("search")
    fetch = tools.get("fetch")
    if not isinstance(search, dict) or not isinstance(fetch, dict):
        raise AssertionError("search and fetch must both be present as distinct runtime tools")
    if search.get("name") != "search" or fetch.get("name") != "fetch":
        raise AssertionError("search/fetch names were aliased")
    search_schema = search.get("inputSchema")
    fetch_schema = fetch.get("inputSchema")
    if not isinstance(search_schema, dict) or not isinstance(fetch_schema, dict):
        raise AssertionError("search/fetch are missing inputSchema objects")
    if search_schema == fetch_schema:
        raise AssertionError("search and fetch must not share an identical inputSchema")
    search_required = set(search_schema.get("required") or [])
    fetch_required = set(fetch_schema.get("required") or [])
    if "query" not in search_required or "id" not in fetch_required:
        raise AssertionError("search/fetch required fields drifted")
    if search_required == fetch_required:
        raise AssertionError("search and fetch required fields are not distinct")
    return {
        "search_required": sorted(search_required),
        "fetch_required": sorted(fetch_required),
    }


def require_failed_tool_call(response: dict[str, Any], label: str) -> None:
    """Cross-identity tool calls must fail as MCP tool isError, not local policy."""
    if "error" in response:
        kind = (response.get("error") or {}).get("kind")
        raise AssertionError(
            f"{label}: expected MCP tool isError, got control error {kind!r}"
        )
    result = response.get("result")
    if isinstance(result, dict) and result.get("isError") is True:
        return
    raise AssertionError(f"{label} did not fail: {response}")


def require_control_error(response: dict[str, Any], kind: str, label: str) -> None:
    if (response.get("error") or {}).get("kind") != kind:
        raise AssertionError(f"{label}: expected {kind} error envelope, got {response}")


def concurrent_fixture_writes(binary: Path, python: Path, profile_id: str, env: dict[str, str], tools: dict[str, Any]) -> dict[str, Any]:
    """Exercise two independently owned probes writing distinct notes together.

    The control protocol is deliberately request/response ordered, so concurrency
    is represented by two real engine processes sharing one generated fixture.
    Each write has a unique title and sentinel; the filesystem is the source of
    truth for completion.  No retry is performed after a process-level error.
    """
    sandbox = create_sandbox(ROOT / ".work/g0", profile_id)
    local_env = isolated_env(sandbox, env)
    probes = [Probe(binary, python, sandbox, local_env), Probe(binary, python, sandbox, local_env)]
    barrier = threading.Barrier(2)
    outcomes: list[dict[str, Any]] = [{}, {}]

    def worker(index: int) -> None:
        sentinel = f"BMDock-CONCURRENT-{index}-7f3c1d"
        title = f"BMDock Concurrent {index}"
        try:
            barrier.wait(timeout=10)
            result = probes[index].request("tools/call", {"name": "write_note", "arguments": tool_arguments(
                tools["write_note"], {"project": PROJECT, "title": title, "directory": "concurrency",
                                     "content": f"# {title}\n\n{sentinel}\n\n- [[Concurrent Target]]\n",
                                     "metadata": {"unknown_frontmatter": {"preserve": True}},
                                     "overwrite": False, "output_format": "json"})})
            classification = classify_tool_result(result)
            note = wait_note(sandbox / "vault", sentinel)
            text = note.read_text(encoding="utf-8")
            outcomes[index] = {"classification": classification, "sentinel": sentinel,
                               "path": note.relative_to(sandbox / "vault").as_posix(),
                               "frontmatter_preserved": "unknown_frontmatter" in text,
                               "wiki_link_preserved": "[[Concurrent Target]]" in text}
        except BaseException as error:
            outcomes[index] = {"error": f"{type(error).__name__}: {error}"}

    threads = [threading.Thread(target=worker, args=(index,)) for index in range(2)]
    for thread in threads:
        thread.start()
    for thread in threads:
        thread.join(timeout=45)
    for probe in probes:
        probe.close()
    if any("error" in outcome for outcome in outcomes):
        raise AssertionError(f"Concurrent fixture write failed: {outcomes}")
    if any(outcome.get("classification") != "accepted_unverified" for outcome in outcomes):
        raise AssertionError(f"Concurrent write result classification drifted: {outcomes}")
    if any(not outcome.get("frontmatter_preserved") or not outcome.get("wiki_link_preserved") for outcome in outcomes):
        raise AssertionError(f"Concurrent Markdown materialization lost content: {outcomes}")
    return {"sandbox": str(sandbox), "workers": outcomes, "status": "passed"}


def recovery_boundaries() -> dict[str, Any]:
    """Record failure boundaries that cannot be safely injected by this harness.

    Keeping these explicit prevents a normal shutdown or a successful MCP
    envelope from being misreported as recovery proof.
    """
    return {
        "lost_response": {"status": "UNVERIFIED", "reason": "No transport fault injector; response loss cannot be inferred from a timeout."},
        "cancellation_after_acceptance": {"status": "UNVERIFIED", "reason": "The probe has no cancellation control method; non-idempotent writes are never retried."},
        "rpc_timeout": {"status": "UNVERIFIED", "reason": "No deterministic slow fixture operation is available; timeout_unknown remains fail-closed in Rust."},
        "forced_kill": {"status": "UNVERIFIED", "reason": "A forced kill would destroy the owned probe before its receipt; no persistence claim is made."},
        "disk_failure": {"status": "UNVERIFIED", "reason": "Cross-platform read-only or full-disk injection is not safe in the generated fixture."},
    }


def run_contract(profile_id: str) -> Path:
    selected = profile(profile_id)
    python = engine_python(profile_id)
    binary = ROOT / "target/debug" / ("bmdock-probe.exe" if os.name == "nt" else "bmdock-probe")
    if not binary.is_file():
        raise FileNotFoundError("Missing probe executable; run cargo build --locked -p bmdock-probe")
    source = ROOT / ".work/engines" / profile_id
    actual_ref = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=source, text=True).strip()
    if actual_ref != selected["commit"]:
        raise RuntimeError("Upstream checkout no longer matches the selected immutable profile")
    sandbox = create_sandbox(ROOT / ".work/g0", profile_id)
    env = isolated_env(sandbox)
    report: dict[str, Any] = {
        "kind": "g0_contract_smoke", "profile": profile_id, "commit": actual_ref,
        "suite_status": "running", "gate_status": "not_passed", "checks": {},
        "limitations": ["Not the full G0 acceptance gate", "No concurrent production-write guarantee",
                        "No Windows native desktop acceptance", "Failure-injection boundaries are recorded as UNVERIFIED"],
    }
    output = ROOT / "artifacts" / f"{profile_id}.contract.json"
    process: Probe | None = None
    try:
        run([str(python), str(ROOT / "scripts/engine_worker.py"), "inventory"], cwd=sandbox, env=env, timeout=120)
        inventory = read_json(sandbox / "inventory.json")
        if selected["version"] and inventory["version"] != selected["version"]:
            raise RuntimeError(f"Unexpected installed version {inventory['version']}")
        report["inventory"] = inventory
        report["checks"]["effective_isolation"] = "passed"
        process = Probe(binary, python, sandbox, env)
        report["handshake"] = process.connected
        report["protocol_version"] = require_protocol_version(process.connected)
        report["checks"]["handshake"] = "passed"
        registry: dict[str, Any] = {}
        for method, key in [("tools/list", "tools"), ("resources/list", "resources"),
                            ("resources/templates/list", "resourceTemplates"), ("prompts/list", "prompts")]:
            rows, pages = paginate(process.request, method, key)
            registry[key] = {"items": rows, "pages": pages, "sha256": fingerprint(rows)}
        report["registry"] = registry
        tools = {tool["name"]: tool for tool in registry["tools"]["items"]}
        delta = inventory_delta(selected["expected_tools"], [t["name"] for t in registry["tools"]["items"]])
        report["tool_delta"] = delta
        if delta["missing"] or delta["added"]:
            raise AssertionError(f"Runtime capability drift requires review: {delta}")
        report["checks"]["registry_names"] = "passed"
        report["checks"]["mcp_discovery"] = "passed"
        report["search_fetch_identity"] = require_distinct_search_and_fetch(tools)
        report["checks"]["search_fetch_identity"] = "passed"

        # Validate the negotiated wire schemas before invoking any write tool.
        # This catches rmcp/engine shape drift while retaining each profile's
        # independent schema as evidence.
        report["schema_fingerprints"] = tool_schema_fingerprints(registry["tools"]["items"])
        report["checks"]["tool_input_schemas"] = "passed"

        # Exercise read-only resources and prompts through the same rmcp
        # transport used by tools. These calls are profile-specific: every
        # profile advertises at least one resource and prompt, but their URI
        # templates and payload shapes differ.
        resource = registry["resources"]["items"][0]
        resource_read = process.request("resources/read", {"uri": resource["uri"]})
        if not isinstance(resource_read.get("contents"), list) or not resource_read["contents"]:
            raise AssertionError("resources/read returned no contents")
        report["resource_read"] = resource_read
        prompt = registry["prompts"]["items"][0]
        prompt_get = process.request("prompts/get", {"name": prompt["name"], "arguments": {}})
        if not isinstance(prompt_get.get("messages"), list) or not prompt_get["messages"]:
            raise AssertionError("prompts/get returned no messages")
        report["prompt_get"] = prompt_get
        report["checks"]["resource_prompt_roundtrip"] = "passed"

        # Policy, schema, and upstream RPC failures must stay distinguishable.
        # No request is retried after an error.
        denied = process.raw("logging/setLevel", {"level": "debug"})
        require_control_error(denied, "policy", "disallowed logging/setLevel")
        # A malformed prompts/get payload is rejected while decoding rmcp's
        # typed request, before it reaches the engine.
        malformed = process.raw("prompts/get", [])  # type: ignore[arg-type]
        require_control_error(malformed, "schema", "malformed prompts/get")
        # A well-typed resources/read that the engine cannot satisfy is an
        # upstream JSON-RPC error, not a local policy/schema rejection.
        # Invalid tool arguments are a separate MCP tool isError path.
        missing_resource = process.raw(
            "resources/read", {"uri": "memory://bmdock-missing-resource"}
        )
        require_control_error(
            missing_resource, "rpc_or_transport", "missing resource resources/read"
        )
        report["errors"] = {
            "policy": denied,
            "schema": malformed,
            "rpc_or_transport": missing_resource,
        }
        report["checks"]["error_categories"] = "passed"
        negative = process.raw("tools/call", {"name": "__bmdock_missing_tool__", "arguments": {}})
        if "error" not in negative and negative.get("result", {}).get("isError") is not True:
            raise AssertionError("Unknown tool did not produce a protocol/tool error")
        report["errors"]["unknown_tool"] = negative
        report["checks"]["unknown_tool_error"] = "passed"
        search_as_fetch = process.raw(
            "tools/call", {"name": "search", "arguments": {"id": "bmdock-missing-fetch-id"}}
        )
        fetch_as_search = process.raw(
            "tools/call", {"name": "fetch", "arguments": {"query": "bmdock-identity"}}
        )
        require_failed_tool_call(search_as_fetch, "search invoked with fetch identity")
        require_failed_tool_call(fetch_as_search, "fetch invoked with search identity")
        report["search_fetch_identity"]["swapped_calls"] = {
            "search_with_fetch_id": search_as_fetch,
            "fetch_with_search_query": fetch_as_search,
        }
        report["checks"]["search_fetch_swapped_calls"] = "passed"

        def call(name: str, values: dict[str, Any]) -> dict[str, Any]:
            result = process.request("tools/call", {"name": name, "arguments": tool_arguments(tools[name], values)})
            if result.get("isError"):
                raise AssertionError(f"{name} returned isError: {result}")
            return result

        report["projects"] = call("list_memory_projects", {})
        sentinel = "BMDock-G0-中文往返-74c5231e"
        report["write_result"] = call("write_note", {
            "project": PROJECT, "title": "BMDock Roundtrip", "directory": "checks",
            "content": f"# BMDock Roundtrip\n\n{sentinel}\n\n- [experiment] 保留中文与关系 #fixture\n- depends_on [[Missing Target]]\n",
            "metadata": {"bmdock_custom": {"nested": ["中文", "keep-me"]}},
            "overwrite": False, "output_format": "json",
        })
        report["write_result_classification"] = classify_tool_result(report["write_result"])
        if report["write_result_classification"] != "accepted_unverified":
            raise AssertionError("Write MCP envelope was not classified as accepted_unverified")
        note = wait_note(sandbox / "vault", sentinel)
        first = note.read_text(encoding="utf-8")
        if "bmdock_custom" not in first or "keep-me" not in first or "[[Missing Target]]" not in first:
            raise AssertionError("Custom metadata or wiki-link did not survive create/materialize")
        note_path = note.relative_to(sandbox / "vault").as_posix()
        read = call("read_note", {"identifier": note_path, "project": PROJECT, "output_format": "json"})
        if sentinel not in json.dumps(read, ensure_ascii=False):
            raise AssertionError("Read result does not contain the created fixture")
        report["checks"]["create_read_materialize"] = "passed"
        report["read_result"] = read
        append = "BMDock-APPEND-ONCE-948aed12"
        call("edit_note", {"identifier": note_path, "project": PROJECT, "operation": "append",
                           "content": f"\n{append}\n", "output_format": "json"})
        wait_note(sandbox / "vault", append)
        if note.read_text(encoding="utf-8").count(append) != 1:
            raise AssertionError("Append was duplicated")
        report["checks"]["append_once"] = "passed"
        report["transcript"] = process.transcript
        report["shutdown"] = process.close()
        process = None
        after_close = note.read_text(encoding="utf-8")
        if sentinel not in after_close or after_close.count(append) != 1:
            raise AssertionError("Materialized fixture did not survive a normal shutdown")
        report["checks"]["shutdown_file_observation"] = "passed"
        report["fixture_after_shutdown"] = after_close
        report["concurrency"] = concurrent_fixture_writes(binary, python, profile_id, env, tools)
        report["checks"]["concurrent_fixture_writes"] = "passed"
        report["recovery"] = recovery_boundaries()
        report["checks"]["recovery_boundaries_recorded"] = "passed"
        report["suite_status"] = "passed"
        print(f"PASS {profile_id}: G0 smoke checks; full G0 gate remains unpassed", flush=True)
    except BaseException as error:
        report["suite_status"] = "failed"
        report["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        if process is not None:
            report["transcript"] = process.transcript
            process.abort()
        write_json(output, redact(report, sandbox))
        print(f"Evidence: {output}", flush=True)
    return output
