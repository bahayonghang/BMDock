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
    import click
    from basic_memory.cli.main import app as cli_app
    from basic_memory.api.app import app as api_app
    from basic_memory.config import ConfigManager

    config = ConfigManager().config
    if config.auto_update or config.semantic_search_enabled:
        raise SystemExit("Effective upstream settings failed the isolation precondition")
    root_command = typer.main.get_command(cli_app)
    commands: list[dict[str, object]] = []

    def walk(command: click.Command, path: list[str], parent: click.Context | None = None) -> None:
        context = click.Context(command, info_name=path[-1], parent=parent)
        parameters = []
        for parameter in command.params:
            parameters.append({"name": parameter.name, "required": parameter.required,
                               "opts": list(getattr(parameter, "opts", [])), "type": str(parameter.type)})
        commands.append({"path": path, "group": isinstance(command, click.Group), "parameters": parameters})
        if isinstance(command, click.Group):
            for name in command.list_commands(context):
                child = command.get_command(context, name)
                if child is not None:
                    walk(child, path + [name], context)

    walk(root_command, ["bm"])
    openapi = api_app.openapi()
    output = {
        "version": importlib.metadata.version("basic-memory"),
        "python": sys.version.split()[0],
        "effective": {"auto_update": config.auto_update, "semantic_search_enabled": config.semantic_search_enabled},
        "cli_registry": commands,
        "openapi": openapi,
        "scope": "CLI registration and OpenAPI inventory; not proof that every capability works",
    }
    Path("inventory.json").write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
