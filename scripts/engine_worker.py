"""Executed only by the installed upstream Python inside a generated G0 sandbox.

The inventory mode imports official registries but does not execute CLI callbacks.
The serve mode delegates to the official CLI unchanged. No private SQL is used.
"""
from __future__ import annotations

import importlib.metadata
import json
import os
import sys
from pathlib import Path
from typing import Any


def collect_cli_commands(root: Any) -> list[dict[str, Any]]:
    """Inspect registered commands without invoking callbacks.

    Typer may use its bundled Click classes. Use each command's public context
    and group methods rather than isinstance against a different Click package.
    """
    commands: list[dict[str, Any]] = []

    def walk(command: Any, path: list[str], parent: Any, ancestors: frozenset[int]) -> None:
        if id(command) in ancestors or len(commands) >= 10000 or len(path) > 32:
            raise ValueError("Cyclic or unbounded CLI registry")
        context = command.context_class(command, info_name=path[-1], parent=parent)
        group = callable(getattr(command, "list_commands", None)) and callable(getattr(command, "get_command", None))
        parameters = [{"name": parameter.name, "required": parameter.required,
                       "opts": list(getattr(parameter, "opts", [])), "type": str(parameter.type)}
                      for parameter in command.params]
        commands.append({"path": path, "group": group, "parameters": parameters})
        if group:
            names = command.list_commands(context)
            if len(names) != len(set(names)):
                raise ValueError("Duplicate CLI command names")
            for name in names:
                child = command.get_command(context, name)
                if child is None:
                    raise ValueError(f"Advertised CLI child is missing: {name}")
                walk(child, path + [name], context, ancestors | {id(command)})

    walk(root, ["bm"], None, frozenset())
    return commands


def main() -> None:
    if len(sys.argv) != 2 or sys.argv[1] not in {"inventory", "serve"}:
        raise SystemExit("usage: engine_worker.py inventory|serve")
    mode = sys.argv[1]
    config_dir = Path(os.environ["BASIC_MEMORY_CONFIG_DIR"])
    if not (config_dir.parent / ".bmdock-g0-sandbox.json").is_file():
        raise SystemExit("Refusing a non-sandbox configuration")
    # Prevent ambient CLI arguments from being interpreted by Basic Memory imports.
    sys.argv = ["basic-memory", "mcp", "--transport", "stdio"]
    if mode == "serve":
        from basic_memory.cli.main import app
        app()
        return

    import typer.main
    from basic_memory.cli.main import app as cli_app
    from basic_memory.api.app import app as api_app
    from basic_memory.config import ConfigManager

    config = ConfigManager().config
    if config.auto_update or config.semantic_search_enabled:
        raise SystemExit("Effective upstream settings failed the isolation precondition")
    root_command = typer.main.get_command(cli_app)
    commands = collect_cli_commands(root_command)
    required_roots = {"mcp", "project", "cloud", "import"}
    root_names = {entry["path"][1] for entry in commands if len(entry["path"]) == 2}
    if not required_roots.issubset(root_names):
        raise RuntimeError(f"Incomplete CLI inventory: missing root commands {sorted(required_roots - root_names)}")
    openapi = api_app.openapi()
    output = {
        "version": importlib.metadata.version("basic-memory"),
        "python": sys.version.split()[0],
        "dependencies": {name: importlib.metadata.version(name) for name in ("typer", "click", "fastmcp", "mcp")},
        "effective": {"auto_update": config.auto_update, "semantic_search_enabled": config.semantic_search_enabled},
        "cli_registry": commands,
        "openapi": openapi,
        "scope": "CLI registration and OpenAPI inventory; not proof that every capability works",
    }
    Path("inventory.json").write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
