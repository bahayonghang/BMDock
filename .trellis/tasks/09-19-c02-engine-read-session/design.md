# C02 design

## Boundaries and evidence

Desktop owns explicit routing, typed IPC, session identity, process/transport lifetime, and cancellation. Official Basic Memory owns parsing, indexes, project data, and query execution. G0 probe recipes remain independent.

Source anchors: `apps/bmdock-desktop/src-tauri/src/main.rs:24-79` (global lock/empty stores), `ipc.rs:856-945` (dispatch), `library.rs:3302` (sync trait), `supervisor.rs:175-286` (lifecycle), `drain.rs:99-134` (receipt-only drain), `crates/bmdock-probe/src/main.rs:98-164` (rmcp pattern). See parent backend audit for full paths.

## Minimal owner and async seam

Add a focused desktop `engine_session.rs` only if the existing supervisor cannot clearly own transport and typed reads. It contains service/child handles, profile, generation, and lifecycle. A small adjacent read adapter may separate decoding from lifecycle; no generic backend framework is needed.

Managed app state holds a cloneable session handle. IPC snapshots route/session under a short lock, releases it, and awaits I/O through a narrow async read-dispatch branch. Retain synchronous existing write-denied/fixture behavior as needed. Do not use block_on or hold the global lock to preserve the synchronous NoteLibrary interface. FixtureLibrary remains test-only; EmptyLibrary remains explicit unavailable behavior.

Reuse versions of rmcp/tokio already pinned in the workspace where needed. Review any manifest/lock delta explicitly; this plan does not install dependencies or change pins.

```text
Typed read -> boundary validation -> route/profile/generation snapshot
 -> release state lock -> session-owned rmcp request
 -> profile-specific structured decode -> session identity check -> typed response
```

One profile is active at a time. A host-owned launch specification comes from existing isolated harness conventions, with scrubbed environment and verified generated sandbox ownership. Renderer input never selects executable paths, environment variables, filesystem roots, or raw tool names. C01 identifies the concrete validated launch input; absent that input, app startup remains honestly unavailable. Seed fixture data before the read-only session. Demonstrate actual production desktop wiring in integration tests; a parallel test-only rmcp client does not qualify.

## Protocol and outcome projection

Maintain separate captured handshake, discovery, and read payloads for each profile. Decode the required known structured fields and isError at the adapter boundary. A profile without the required shape returns unsupported/schema evidence rather than fabricated empty/success data. JSON-requested search can still return guidance text: preserve that as error/unavailable, never an empty successful query. Do not parse presentation Markdown if verified JSON output exists. Exact read_note returns its complete note body; page_size does not chunk that body. Preserve original content and defer large-note response measurement to C03/C07.

Here complete source requires explicit `include_frontmatter=true` (or the exact captured supported equivalent), rather than the JSON body's default frontmatter exclusion. Use C01 YAML/UTF-8/CRLF response fixtures. Preserve the complete delivered string, and report engine transformations separately; do not claim filesystem byte equality when the engine normalizes newlines or encoding. Keep any byte-preserving write claim disabled because this child adds reads only.

Use an unsliced read: omit main-preview `start_line/end_line`; they are not supported release parameters. Line-range retrieval is not introduced by this child.

Keep current IPC categories policy/schema/unsupported separate from runtime failure kinds transport/process/timeout_unknown unless C01 establishes an explicit reviewed contract change. Correlate outcomes with profile and generation without leaking tokens or internal paths. Empty means a connected operation returned no records; unavailable means the backend could not perform it.

Backend verifies session ownership. C04 owns consumer request generations. C03 specifies complete query/page identity in accordance with parent D2: profile, session generation, explicit route, operation, query or note identifier, mode, all filters and page/cursor. Do not implement duplicate consumer bookkeeping in the engine owner. Parent D2/D4 supplies page50, max150 retained view rows, no persistent result cache, and the frozen paired-measurement protocol; this child establishes the session contract without inventing a different performance baseline.

## Lifecycle

Connected follows successful initialize/discovery. Stopping rejects new work, requests bounded transport cancellation, waits for the owned child within a separate bounded budget, then escalates only on documented timeout/unknown. Receipts record actual observations. Individual read cancellation must not falsely mark a surviving session stopped. If pinned SDK cancellation cannot safely cancel one read, retire its consumer and finish/time out the bounded retained operation; C03 bounds queued/in-flight work.

Existing T17 begin_shutdown is receipt-only and the current supervisor spec prohibits spawning/killing a live engine through it. Explicitly define and document the new session lifecycle owner instead of silently changing T17's meaning. Window-close integration remains separate native evidence.

## Persistence, rollback, and prerequisites

Do not expose fixture fs::write code, activate production draft/backup stores, access private databases, or lift user-vault restrictions. Official engine index/cache writes are allowed only inside the generated isolated sandbox; read-only product access prohibits user-note mutations, not temporary index maintenance.

Rollback restores the unavailable read adapter and removes owned session startup wiring while preserving typed denial policy, G0 probe, fixture tests, and evidence. There is no user-data migration or backwards-compatibility shim.

C01 must capture actual tree/read structured payloads independently for each profile. Pinned SDK cancellation requires controlled tests. Documentation alone is not acceptance. Native GUI, process-tree containment, and sleep/resume remain UNVERIFIED without direct evidence.
