"""Verify the production desktop read dispatcher, without creating a native window.

Only fresh generated G0 fixtures are accepted. The desktop binary owns its actual
MCP process; this harness does not implement a second read client.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import subprocess
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from scripts.client_baseline import corpus_snapshot, note_path, populate_fixture, verify_engine
from scripts.core import ROOT, create_sandbox, file_sha256, isolated_env, profile, write_json


def validate_report(report: dict[str, Any], expected: bytes, profile_id: str) -> dict[str, Any]:
    """Acceptance checks consume production DTOs, not success prose or an exit code."""
    for key in ("root", "nested"):
        if report[key].get("kind") != "tree_page":
            raise AssertionError(f"{key}: desktop did not return a connected tree page: {report[key]}")
    if not any(item["identifier"] == "group-000" and item["kind"] == "directory" for item in report["root"]["entries"]):
        raise AssertionError("Production tree omitted the generated directory")
    if not any(item["identifier"] == "corpus/note-00000" and item["kind"] == "note" for item in report["nested"]["entries"]):
        raise AssertionError("Nested production tree omitted the known note")
    identity = report["session"]
    if identity["profile"] != profile_id or type(identity["generation"]) is not int or identity["generation"] < 1:
        raise AssertionError("Invalid active session identity")
    for key in ("root", "nested", "first", "second"):
        if report[key].get("session") != identity:
            raise AssertionError(f"{key}: result belongs to another session")
    for key in ("first", "second"):
        if report[key].get("kind") != "note_read" or report[key].get("identifier") != "corpus/note-00000":
            raise AssertionError("Production dispatch did not return the exact note")
        if report[key]["observation"]["disk_verified"]:
            raise AssertionError("Engine adapter incorrectly claimed disk verification")
    first = report["first"]["body"]
    if first != report["second"]["body"] or not report["child_reused"]:
        raise AssertionError("Repeated read did not preserve body and child identity")
    original = expected.decode("utf-8")
    if first not in (original, original.replace("\r\n", "\n")) or not first.startswith("---"):
        raise AssertionError("Delivered source lost frontmatter, UTF-8, or body content")
    for key in ("stale", "wrong_route"):
        if report[key].get("kind") != "error" or report[key].get("category") != "policy":
            raise AssertionError("Route/session ownership was not enforced")
    if report["runtime"]["status"] != "connected" or report["runtime"]["profile"] != profile_id:
        raise AssertionError("Runtime did not report the actual connected profile")
    if report["runtime"]["session_generation"] != identity["generation"] or report["runtime"]["shutdown"] is not None:
        raise AssertionError("T17 drain was confused with actual engine shutdown")
    # T17 engine_spawned observes the lifecycle state; it does not mean this
    # command spawned an engine. Child reuse and still-connected runtime prove
    # the drain did not replace or close our live owner.
    if report["drain"]["child_killed"]:
        raise AssertionError("T17 drain took live process ownership")
    shutdown = report["shutdown"]
    if not shutdown["transport_cancelled"] or not shutdown["child_exited"] or shutdown["forced"] or shutdown["timeout_unknown"] or shutdown["exit_code"] != 0:
        raise AssertionError(f"Actual owned session did not close cleanly: {shutdown}")
    if report["stopped"]["status"] != "stopped":
        raise AssertionError("Runtime did not report stopped after actual close")
    return {"delivered_string_preserved": first == report["second"]["body"],
            "delivered_utf8_sha256": hashlib.sha256(first.encode("utf-8")).hexdigest(),
            "physical_source_sha256": hashlib.sha256(expected).hexdigest(),
            "physical_byte_equality_for_this_fixture": first.encode("utf-8") == expected,
            "upstream_crlf_normalized": first != original,
            "general_byte_fidelity_claim": False}


def run(profile_id: str, output: Path) -> dict[str, Any]:
    output = output.resolve()
    if not output.is_relative_to((ROOT / "execution/evidence").resolve()) and not output.is_relative_to((ROOT / "artifacts/client-session").resolve()):
        raise ValueError("Output must be task-owned evidence")
    if output.exists():
        raise FileExistsError(output)
    verify_engine(profile_id)
    binary = ROOT / "target/debug" / ("bmdock-app.exe" if os.name == "nt" else "bmdock-app")
    if not binary.is_file():
        raise FileNotFoundError("Build bmdock-app with --locked --offline first")
    sandbox = create_sandbox(ROOT / ".work/g0/client-session", profile_id)
    before = populate_fixture(sandbox, "notes-100")
    original = (sandbox / "vault" / note_path(0)).read_bytes()
    command = [str(binary), "--session-check", profile_id, str(sandbox)]
    result = subprocess.run(command, cwd=ROOT, env=isolated_env(sandbox), capture_output=True,
                            text=True, encoding="utf-8", timeout=600, check=False,
                            creationflags=subprocess.CREATE_NO_WINDOW if os.name == "nt" else 0)
    if result.returncode:
        raise RuntimeError(f"Production session check failed ({result.returncode}): {result.stderr[-2000:]}")
    report = json.loads(result.stdout)
    comparisons = validate_report(report, original, profile_id)
    if corpus_snapshot(sandbox / "vault") != before:
        raise AssertionError("Read session changed generated note bytes")
    for key in ("first", "second"):
        report[key]["body_bytes"] = len(report[key].pop("body").encode("utf-8"))
    evidence = {"schema_version": 1, "captured_at": datetime.now(timezone.utc).isoformat(),
                "status": "passed", "profile": profile_id, "engine_sha": profile(profile_id)["commit"],
                "platform": platform.platform(), "sandbox_identity": sandbox.relative_to(ROOT).as_posix(),
                "desktop_binary_sha256": file_sha256(binary),
                "source_sha256": {name: file_sha256(ROOT / name) for name in (
                    "apps/bmdock-desktop/src-tauri/src/main.rs", "apps/bmdock-desktop/src-tauri/src/engine_session.rs",
                    "apps/bmdock-desktop/src-tauri/src/ipc.rs", "apps/bmdock-desktop/src-tauri/src/supervisor.rs")},
                "command": f"python -m scripts.client_session {profile_id} --output {output.relative_to(ROOT).as_posix()}",
                "production_command": ["target/debug/bmdock-app", "--session-check", profile_id, "<GENERATED_SANDBOX>"],
                "source_comparison": comparisons, "physical_corpus_unchanged": True,
                "production_dispatch": report,
                "unverified": ["native GUI", "native window close", "Windows Job Object", "process-tree containment", "forced-kill fault injection", "sleep/resume"]}
    write_json(output, evidence)
    return evidence


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("profile", choices=("release", "main-preview"))
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    evidence = run(args.profile, args.output)
    print(json.dumps({"status": evidence["status"], "profile": args.profile, "output": str(args.output)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
