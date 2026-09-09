"""Shared safety, evidence and registry functions for the G0 harness."""
from __future__ import annotations

import hashlib
import json
import os
import subprocess
import tempfile
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
PROJECT = "bmdock-fixture"
MARKER = ".bmdock-g0-sandbox.json"
SYSTEM_ENV = ("PATH", "SystemRoot", "WINDIR", "COMSPEC", "PATHEXT", "SystemDrive", "LANG", "LC_ALL")


def read_json(path: Path) -> Any:
    return json.loads(path.read_text(encoding="utf-8"))


def write_json(path: Path, value: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    # Only used for harness-owned files, never a Basic Memory database or user note.
    with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=path.parent, delete=False) as stream:
        temporary = Path(stream.name)
        json.dump(value, stream, ensure_ascii=False, indent=2, sort_keys=True)
        stream.write("\n")
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary, path)


def fingerprint(value: Any) -> str:
    canonical = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


def file_sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def profiles() -> dict[str, Any]:
    return read_json(ROOT / "compatibility/profiles.json")["engines"]


def profile(name: str) -> dict[str, Any]:
    try:
        result = profiles()[name]
    except KeyError as error:
        raise ValueError(f"Unknown engine profile: {name!r}; use release or main-preview") from error
    sha = result["commit"]
    if len(sha) != 40 or any(c not in "0123456789abcdef" for c in sha):
        raise ValueError("An immutable upstream commit is required")
    return result


def create_sandbox(base: Path, profile_id: str) -> Path:
    profile(profile_id)
    base.mkdir(parents=True, exist_ok=True)
    sandbox = Path(tempfile.mkdtemp(prefix=f"{profile_id}-", dir=base)).resolve()
    for relative in ("config", "vault", "home", "tmp", "cache"):
        (sandbox / relative).mkdir()
    write_json(sandbox / MARKER, {"kind": "bmdock-g0", "profile": profile_id, "root": str(sandbox)})
    write_json(sandbox / "config/config.json", {
        "projects": {PROJECT: {"path": str(sandbox / "vault"), "mode": "local"}},
        "default_project": PROJECT,
        "database_backend": "sqlite",
        "auto_update": False,
        "semantic_search_enabled": False,
    })
    return sandbox


def verify_sandbox(sandbox: Path) -> Path:
    sandbox = sandbox.resolve(strict=True)
    marker = read_json(sandbox / MARKER)
    if marker.get("kind") != "bmdock-g0" or marker.get("root") != str(sandbox):
        raise ValueError("Not an owned G0 sandbox")
    profile(marker["profile"])
    for part in ("config", "vault", "home", "tmp", "cache"):
        child = sandbox / part
        if child.is_symlink() or not child.resolve(strict=True).is_relative_to(sandbox):
            raise ValueError(f"Sandbox directory escaped: {part}")
    config = read_json(sandbox / "config/config.json")
    entries = config.get("projects", {})
    if set(entries) != {PROJECT}:
        raise ValueError("Only the generated fixture project is allowed")
    if Path(entries[PROJECT]["path"]).resolve() != (sandbox / "vault").resolve():
        raise ValueError("Fixture config points outside the generated vault")
    if entries[PROJECT].get("mode") != "local":
        raise ValueError("A fixture cannot route to Cloud")
    # Reject inherited database references, rather than attempting to sanitize them.
    allowed = {"projects", "default_project", "database_backend", "auto_update", "semantic_search_enabled"}
    if not set(config).issubset(allowed):
        raise ValueError("Unexpected config keys; recreate the sandbox rather than reuse it")
    if config.get("auto_update") is not False or config.get("semantic_search_enabled") is not False:
        raise ValueError("Self-update and embeddings must be disabled for G0")
    return sandbox


def isolated_env(sandbox: Path, source: dict[str, str] | None = None) -> dict[str, str]:
    sandbox = verify_sandbox(sandbox)
    original = os.environ if source is None else source
    env = {key: original[key] for key in SYSTEM_ENV if key in original}
    env.update({
        "HOME": str(sandbox / "home"), "USERPROFILE": str(sandbox / "home"),
        "APPDATA": str(sandbox / "home"), "LOCALAPPDATA": str(sandbox / "home"),
        "XDG_CONFIG_HOME": str(sandbox / "config"), "XDG_CACHE_HOME": str(sandbox / "cache"),
        "TMP": str(sandbox / "tmp"), "TEMP": str(sandbox / "tmp"), "TMPDIR": str(sandbox / "tmp"),
        "BASIC_MEMORY_CONFIG_DIR": str(sandbox / "config"),
        "BASIC_MEMORY_AUTO_UPDATE": "false", "BASIC_MEMORY_SEMANTIC_SEARCH_ENABLED": "false",
        "BASIC_MEMORY_FORCE_LOCAL": "true", "BASIC_MEMORY_EXPLICIT_ROUTING": "true",
        "BASIC_MEMORY_DATABASE_BACKEND": "sqlite", "BASIC_MEMORY_MCP_PROJECT": PROJECT,
        "PYTHONUTF8": "1", "PYTHONIOENCODING": "utf-8", "PYTHONUNBUFFERED": "1",
        "HF_HUB_OFFLINE": "1", "TRANSFORMERS_OFFLINE": "1", "LOGFIRE_SEND_TO_LOGFIRE": "false",
    })
    return env


def run(argv: list[str], *, cwd: Path = ROOT, env: dict[str, str] | None = None, timeout: int = 1200) -> None:
    """Run a fixed argv without a shell; preserve non-zero results."""
    print("+", " ".join(argv), flush=True)
    subprocess.run(argv, cwd=cwd, env=env, check=True, timeout=timeout)


def inventory_delta(expected: list[str], actual: list[str]) -> dict[str, list[str]]:
    if len(actual) != len(set(actual)):
        raise ValueError("Duplicate capability names in discovery")
    return {"missing": sorted(set(expected) - set(actual)), "added": sorted(set(actual) - set(expected))}


def paginate(request: Any, method: str, key: str, *, max_pages: int = 128) -> tuple[list[Any], int]:
    """Follow every cursor; repeated cursors and truncated results are failures."""
    rows: list[Any] = []
    cursor: str | None = None
    seen: set[str] = set()
    for page in range(1, max_pages + 1):
        result = request(method, {"cursor": cursor} if cursor is not None else {})
        chunk = result.get(key)
        if not isinstance(chunk, list):
            raise ValueError(f"{method}: expected list field {key}")
        rows.extend(chunk)
        cursor = result.get("nextCursor")
        if cursor is None:
            return rows, page
        if not isinstance(cursor, str) or cursor in seen:
            raise ValueError(f"{method}: invalid or repeated pagination cursor")
        seen.add(cursor)
    raise ValueError(f"{method}: exceeded {max_pages} pages; refusing partial inventory")


def classify_tool_result(result: dict[str, Any]) -> str:
    """Do not promote text or an MCP success envelope to a persisted write."""
    if result.get("isError") is True:
        return "tool_error"
    structured = result.get("structuredContent")
    if isinstance(structured, dict):
        kind = structured.get("kind")
        if kind in {"already_exists", "target_moved", "locked"}:
            return "rejected"
        if kind in {"created", "updated"}:
            return "accepted_unverified"
    return "unclassified"
