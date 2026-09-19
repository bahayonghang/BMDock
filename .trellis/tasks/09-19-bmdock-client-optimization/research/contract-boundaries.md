# Focused contract handoff

Date: 2026-09-19. This is a bounded context index for the optimization task, not a replacement specification. The full typed IPC specification is about 188 KB and is truncated by normal sub-agent injection; load the relevant source sections on demand before editing an owning command.

## Current contracts to preserve

| Boundary | Source to consult | Consequence |
|---|---|---|
| Typed IPC and explicit fixture routing | `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md:123`, `:660`, `:803`; `apps/bmdock-desktop/src-tauri/src/routing.rs:73` | No arbitrary executable/path/tool, implicit project, real-vault access or credential injection from the renderer. |
| Existing fixture query semantics | Same spec `:149`, `:189`, `:223`; `research/backend-audit.md` | Fixture lexical search, inspector and recall are not official-engine capabilities. Official reads need deliberate new adapter contracts. |
| Receipt-only shutdown | Same spec `:632`, `:693`; `.trellis/spec/bmdock-probe/backend/supervisor-state.md:17` | Existing T17 receipt is not live shutdown proof. C02 must explicitly own real session lifecycle and update its scoped contract. |
| Raw content and mutation evidence | Same spec `:142`; `research/frontend-audit.md` | Retain escaped content and exact raw-source handling. Unsaved editor state and engine persistence are different; unknown outcomes are not retried automatically. |
| Profile-local contracts | `compatibility/profiles.json`; `research/official-and-projects.md` | Preserve both pins independently; JSON guidance, full-note read, approximate totals, compact/CJK capability and score semantics require profile-specific handling. |

Keep IPC policy/schema/unsupported separate from supervisor transport/process/timeout_unknown outcomes unless the reviewed implementation explicitly changes that contract. The adapter decodes observed payloads once; frontend code does not scrape Markdown or invent success from prose.

## New plan contracts requiring reviewed implementation

Parent `design.md` D1-D5 defines the authoritative proposal: official indexed read adapter, session-owned I/O outside the host-wide lock, complete request identity, generation/revision-safe frontend state, demand-driven diagnostics, page50/max150 primary rows and no persistent result cache by default.

The generated fixture remains the only engine target. Engine index maintenance inside that sandbox is permitted by a future approved isolated run; user-note writes and global configuration remain outside scope. Existing empty draft storage gains no durability claim. Rich Markdown, a second search index and generic state/caching frameworks are deferred.

Before implementation, read the exact touched sections of the source specs, the owning child plan and parent research. Update current contracts only for authorized, tested behavior; do not infer release/native acceptance from passing local tests.
