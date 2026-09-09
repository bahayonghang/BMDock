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
                          isolated_env, paginate, profile, read_json, run, write_json)


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
                        "No Windows native desktop acceptance", "No lost-response or disk-failure injection yet"],
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
        negative = process.raw("tools/call", {"name": "__bmdock_missing_tool__", "arguments": {}})
        if "error" not in negative and negative.get("result", {}).get("isError") is not True:
            raise AssertionError("Unknown tool did not produce a protocol/tool error")
        report["checks"]["unknown_tool_error"] = "passed"

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
