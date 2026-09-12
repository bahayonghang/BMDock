# Backend Development Guidelines

> Best practices for backend development in this project.

---

## Overview

This directory contains guidelines for backend development. Fill in each file with your project's specific conventions.

---

## Guidelines Index

| Guide | Description | Status |
|-------|-------------|--------|
| [Directory Structure](./directory-structure.md) | Module organization and file layout | To fill |
| [Database Guidelines](./database-guidelines.md) | ORM patterns, queries, migrations | To fill |
| [Error Handling](./error-handling.md) | Error types, handling strategies | To fill |
| [Quality Guidelines](./quality-guidelines.md) | Code standards, forbidden patterns | To fill |
| [Logging Guidelines](./logging-guidelines.md) | Structured logging, log levels | To fill |
  | [Typed IPC and Fixture Policy](./typed-ipc-policy.md) | T06 command/event DTOs, T09 read-only preflight/discovery, T10 explicit fixture routing, T11 paginated tree/note read, T12 backup inventory and fixture restore, T13 Windows runtime prototype, T14 draft persistence and editor session, T15 typed fixture note CRUD, T16 same-target conflict / unknown-result coordination, T17 host drain `begin_shutdown`, T18 editor content safety (textarea/`<pre>`, exact-byte CRLF, `executed=false`), T19 `list_relations` fixture wiki-link relations plus observation classified_as, T20 `expand_graph` one-hop fixture neighborhood plus bounded progressive expansion, T21 typed `search_notes` fixture lexical search with hybrid lexical/semantic scores (`semantic_enabled=false`), T22 `preview_context` fixture markdown snippet plus `list_activity` fixture mtimes (`engine_activity=false`), T23 `inspect_search` fixture lexical inspector with `model_loaded=false` / `embedding_backend=none`, T24 `run_recall_benchmark` fixture Chinese recall@k plus bounded in-process elapsed_ms (`native_gui=false`), T25 `schema_validate` BMDock-owned fixture markdown/JSON-like frontmatter validation (`engine_schema=false`), T26 `list_resources` / `list_prompts` BMDock-owned fixture markdown/sidecar catalogs (`engine_resources=false`, `engine_prompts=false`), T27 `inspect_tools` / `list_cli_inventory` BMDock-owned controlled-tool comparison plus named CLI leaf catalog (`engine_tools=false`, `engine_cli=false`), T28 `import_notes` BMDock-owned fixture markdown import (`engine_import=false`), T29 `inspect_api_audit` BMDock-owned named-leaf API/CLI gap audit (`full_api_coverage=false`), T30 `inspect_extras` / `ingest_document` BMDock-owned fixture extra catalog plus sidecar ingest (`extras_enabled=false` unless extras exist on disk), T31 typed `inspect_cloud` FAIL-CLOSED local-only cloud/remote-auth status (`cloud_enabled=false`, `remote_auth=false`, `credentials_present=false`, `cloud_allowed=false`), T32 typed `inspect_sync` / `list_shares` FAIL-CLOSED local-only cloud sync/share status (`sync_enabled=false`, `sharing_enabled=false`, `remote_restore=false`, `last_sync=none`; recovery remains T12 `restore_fixture`), T33 typed `inspect_hooks` FAIL-CLOSED local-only official Agent/hook status (`hooks_enabled=false`, `agent_connected=false`, `files_written=false`; claiming installed/connected without a live official agent session is unsupported), T34 typed `inspect_providers` FAIL-CLOSED local-only provider/backend-tier status (`provider_enabled=false`, `semantic_enabled=false`, `model_loaded=false`, `files_written=false`; missing openai/anthropic/huggingface/cloud embeddings stay explicit unavailable/unverified; claiming a live provider session is unsupported), T35 typed `inspect_routes` FAIL-CLOSED local-only conditional capability and cross-route regression (`cloud_allowed=false`, `sync_enabled=false`, `sharing_enabled=false`, `hooks_enabled=false`, `provider_enabled=false`, `semantic_enabled=false`, `cross_project_search_allowed=false`, `full_api_coverage=false`; empty route catalog; claiming connected/synced/installed or a cross-project/live remote route is unsupported), T36 typed `inspect_privacy` FAIL-CLOSED local-only security/SBOM/privacy status (`telemetry=false`, `cloud_allowed=false`, `provider_enabled=false`, `html_executed=false`, `secrets_stored=false`, `env_tokens_read=false`, `remote_hosts_contacted=false`; LICENSE/NOTICE/SBOM present; `vulnerability_scan=UNVERIFIED`; claiming G7 or a live privacy-cleared review is unsupported), fixture boundary, and T07 runtime snapshot projection | Current |
| [Engine Supervisor and Runtime State](./supervisor-state.md) | T07 lifecycle, spawn policy-before-process, failure, and shutdown contract | Current |

---

## How to Fill These Guidelines

For each guideline file:

1. Document your project's **actual conventions** (not ideals)
2. Include **code examples** from your codebase
3. List **forbidden patterns** and why
4. Add **common mistakes** your team has made

The goal is to help AI assistants and new team members understand how YOUR project works.

---

**Language**: All documentation should be written in **English**.
