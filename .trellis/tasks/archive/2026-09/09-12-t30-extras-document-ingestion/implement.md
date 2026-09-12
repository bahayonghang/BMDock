# T30 execution record: local optional extras + document ingestion

## Goal

Deliver T30 P5 (AC39, AC41): typed `inspect_extras` + `ingest_document` on `ipc_invoke`, BMDock-owned fixture extras over `{temp}/bmdock-t30-*`, production `EmptyLibrary` empty/disabled, official extras / PDF/Office ingest remaining UNVERIFIED.

## What shipped

1. Typed `inspect_extras` (`ExplicitRouteArgs` + optional `extra_id`, `deny_unknown_fields`). Extra `path`/`root` is schema. Missing route is schema. Empty `extra_id` is schema. Non-fixture routes and filesystem `extra_id` are policy and do not open the library.
2. Typed `ingest_document` (`ExplicitRouteArgs` + required `source_id` as a fixture identifier, not a filesystem path). Same fail-closed rules. Distinct from T28 `import_notes` (markdown copy). Copies a named `.txt`/`.md` extra sidecar into fixture notes.
3. AC39: extras catalog is BMDock-owned. Fixture hits match physical UTF-8. Envelope text is not disk proof. Production `EmptyLibrary` is `extras_enabled=false`, empty catalog, `classified_as: empty`. Does not scan user vaults.
4. AC41: extras/semantic/cloud stay explicit unavailable unless a fixture extra is actually on disk. `extras_enabled=false` by default. Claiming `extras_enabled=true` without disk files is `unsupported`. `semantic_enabled=false`. `model_loaded=false`.
5. Renderer: zh-CN Extra / 文档摄取 empty/error/ready. No `dangerouslySetInnerHTML`. No new npm deps. Does not start Supervisor. No rmcp.
6. Allowlist 32 → 34. `ipc.ts` union + helpers. DesktopShellTests 24th test. Dual profiles isolated. `just build` stays G0 probe.

## Files

- `apps/bmdock-desktop/src-tauri/src/library.rs`
- `apps/bmdock-desktop/src-tauri/src/ipc.rs`
- `apps/bmdock-desktop/src/ipc.ts`
- `apps/bmdock-desktop/src/App.tsx`
- `apps/bmdock-desktop/src/i18n.ts`
- `apps/bmdock-desktop/src/styles.css`
- `tests/test_desktop_shell.py`
- `.trellis/spec/bmdock-probe/backend/index.md`
- `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md`
- `docs/VERIFICATION.md`
- `execution/evidence/t30-extras-document-ingestion.json`
- `execution/status.json` (T30 `planned` → `completed` only)

## Out of scope / not done

- No git commit / push / merge / amend.
- Did not run `just contract` / `just contract-main` as T30 proof.
- Did not start Supervisor, add rmcp, spawn `engine_worker.py`, or scan user vaults.
- Did not touch `.work/engines`, secrets, parent parent-task dir, or T31–T40.
- Did not revert T05–T29/G0 so `python -m scripts.tasks unit` can pass.

## Validation commands (this session)

| Command | Exit |
| --- | --- |
| `python ./.trellis/scripts/task.py validate 09-12-t30-extras-document-ingestion` | 0 |
| `cargo fmt --all -- --check` | 0 |
| `cargo test --workspace --locked --offline` | 0 (bmdock-app 175 + bmdock-probe 5) |
| `cargo check --workspace --locked --offline` | 0 (existing T07 dead_code warnings) |
| `npm run build` in `apps/bmdock-desktop` | 0 |
| `git diff --check` | 0 |
| `python -m unittest tests.test_desktop_shell -v` | 0 (24 tests) |
| `python -m scripts.tasks unit` | 1 expected (`A later task was completed before G0`; not reverted) |
| `just contract` / `just contract-main` | not run |

Evidence: `execution/evidence/t30-extras-document-ingestion.json`.
