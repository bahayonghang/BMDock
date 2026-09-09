"""Cross-platform task dispatcher; every command is argv-based, never shell=True."""
from __future__ import annotations

import argparse
import ast
import json
import os
import shutil
import subprocess
import sys
from pathlib import Path

from scripts.core import ROOT, profile, profiles, read_json, run, write_json


def require(tool: str) -> str:
    found = shutil.which(tool)
    if not found:
        raise RuntimeError(f"Required tool not found: {tool}. See README.md prerequisites; nothing was skipped.")
    return found


def doctor() -> int:
    ok = sys.version_info >= (3, 12)
    print(f"Python: {sys.version.split()[0]} (required: >=3.12)")
    for name in ("git", "uv", "cargo", "rustc", "just"):
        found = shutil.which(name)
        print(f"{name}: {found or 'MISSING'}")
        ok = ok and found is not None
    print("No Basic Memory command or user configuration was opened.")
    return 0 if ok else 2


def bootstrap_engines() -> None:
    git, uv = require("git"), require("uv")
    for name in profiles():
        selected = profile(name)
        destination = ROOT / ".work/engines" / name
        marker = destination / ".bmdock-engine.json"
        if destination.exists():
            if not marker.is_file() or read_json(marker).get("commit") != selected["commit"]:
                raise RuntimeError(f"Refusing to overwrite an unrecognized engine checkout: {destination}")
        else:
            destination.mkdir(parents=True)
            write_json(marker, {"kind": "bmdock-engine", "commit": selected["commit"]})
        if not (destination / ".git").exists():
            run([git, "init", str(destination)])
            run([git, "remote", "add", "origin", selected["repository"]], cwd=destination)
        run([git, "fetch", "--depth", "1", "origin", selected["commit"]], cwd=destination)
        run([git, "checkout", "--detach", selected["commit"]], cwd=destination)
        actual = subprocess.check_output([git, "rev-parse", "HEAD"], cwd=destination, text=True).strip()
        if actual != selected["commit"]:
            raise RuntimeError("Unexpected checkout commit")
        run([git, "diff", "--exit-code"], cwd=destination)
        if not (destination / "uv.lock").is_file():
            raise RuntimeError("Pinned upstream checkout has no uv.lock")
        # Explicit setup is the ONLY task allowed to download an engine/Python.
        run([uv, "sync", "--frozen", "--no-dev", "--no-editable", "--python", "3.12.12", "--prerelease", "allow"],
            cwd=destination, timeout=1800)


def check_source() -> None:
    for file in (ROOT / "scripts").glob("*.py"):
        ast.parse(file.read_text(encoding="utf-8"), filename=str(file))
    for name in profiles():
        selected = profile(name)
        if len(selected["expected_tools"]) != len(set(selected["expected_tools"])):
            raise RuntimeError(f"Duplicate static tools in {name}")
    status = read_json(ROOT / "execution/status.json")
    if status["gates"]["G0"]["status"] != "passed":
        if any(x["status"] == "completed" for x in status["tasks"] if int(x["id"][1:]) >= 5):
            raise RuntimeError("A later task was completed before G0")
    print("Source and phase-order checks passed")


def unit() -> None:
    check_source()
    run([sys.executable, "-m", "unittest", "discover", "-s", "tests", "-v"])


def locked_cargo(*args: str) -> None:
    if not (ROOT / "Cargo.lock").is_file():
        raise RuntimeError("Cargo.lock missing. Run just lock, review the resolution, then commit the real lockfile. CI does not invent one.")
    run([require("cargo"), *args, "--locked"], timeout=1800)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description="BMDock P0/G0 developer commands (not a desktop release)")
    parser.add_argument("command", choices=["doctor", "setup", "lock", "unit", "ci", "dev", "contract", "build", "gate"])
    parser.add_argument("profile", nargs="?", choices=["release", "main-preview"], default="release")
    args = parser.parse_args(argv)
    try:
        if args.command == "doctor":
            return doctor()
        if args.command == "setup":
            bootstrap_engines()
        elif args.command == "lock":
            run([require("cargo"), "generate-lockfile"])
            print("Generated real Cargo.lock. Review and commit it before ci/build.")
        elif args.command == "unit":
            unit()
        elif args.command == "build":
            locked_cargo("build", "--release", "-p", "bmdock-probe")
            print("Built target/release/bmdock-probe[.exe]. This is a G0 probe, NOT a desktop installer.")
        elif args.command in {"dev", "contract"}:
            if args.command == "dev":
                print("P0 dev session: run the real engine probe in a newly generated fixture sandbox. No production vault is connected.")
            locked_cargo("build", "-p", "bmdock-probe")
            from scripts.probe import run_contract
            run_contract(args.profile)
        elif args.command == "ci":
            unit()
            run([require("cargo"), "fmt", "--all", "--", "--check"])
            if not (ROOT / "Cargo.lock").is_file():
                raise RuntimeError("CI requires the committed Cargo.lock; run just lock and commit it first")
            run([require("cargo"), "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"], timeout=1800)
            locked_cargo("test", "--workspace")
            locked_cargo("build", "-p", "bmdock-probe")
            from scripts.probe import run_contract
            for name in profiles():
                run_contract(name)
        elif args.command == "gate":
            state = read_json(ROOT / "execution/status.json")
            blocked = [name for name, gate in state["gates"].items() if gate["status"] != "passed"]
            print(json.dumps({"unpassed_gates": blocked, "implementation_status": state["stage"]}, ensure_ascii=False, indent=2))
            return 2 if blocked else 0
        return 0
    except (OSError, ValueError, RuntimeError, AssertionError, TimeoutError, subprocess.SubprocessError) as error:
        print(f"ERROR: {error}", file=sys.stderr)
        return error.returncode if isinstance(error, subprocess.CalledProcessError) else 1


if __name__ == "__main__":
    # Invoke from the repository root with `python -m scripts.tasks`.
    raise SystemExit(main())
