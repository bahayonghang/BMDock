# Backend query, runtime, and performance audit

Date: 2026-09-19. Scope: read-only source inspection; planning only. No benchmark, native GUI, official engine process, or user vault was opened by this audit. Paths below are repository-relative. Line anchors identify the inspected checkout and may move during implementation.

## Executive finding

The desktop executable currently has **no connected production Basic Memory backend**. Its runtime installs empty library, backup, and draft stores. The filesystem-backed library used to demonstrate search, graph, CRUD, and recall is compiled only for Rust tests. Optimizing that test library cannot establish a faster or functional desktop client.

The smallest useful sequence is: define truthful runtime capability states; connect the existing typed desktop boundary to one explicit, isolated official-engine profile; retain engine-owned indexed search and persistence; then measure and optimize the actual IPC-to-render path. Keep `release` and `main-preview` evidence separate. Do not create a second client-side database, vector index, search engine, or general configuration system merely to replace fixture scans.

## Actual data paths

```text
Desktop UI search
  App.tsx runSearch
  -> invokeTyped("ipc_invoke", typed command)
  -> main.rs sync ipc_invoke [one AppState Mutex]
  -> ipc.rs route validation + dispatch
  -> EmptyLibrary / NoteLibrary default search
  -> empty result; engine_search=false

Rust fixture tests
  dispatch_with_library(... FixtureLibrary ...)
  -> read fixture directory / files synchronously
  -> local substring search, wiki-link parsing, direct fixture writes
  -> fixture observation DTOs; engine_* flags remain false

Separate G0 contract tooling
  scripts/probe.py -> bmdock-probe
  -> rmcp stdio session -> scripts/engine_worker.py -> official engine
  -> sandbox file observation / shutdown receipt
```

Evidence: `apps/bmdock-desktop/src/App.tsx:1618`, `apps/bmdock-desktop/src/ipc.ts:1294`, `apps/bmdock-desktop/src-tauri/src/main.rs:35`, `apps/bmdock-desktop/src-tauri/src/main.rs:70`, `apps/bmdock-desktop/src-tauri/src/ipc.rs:927`, `apps/bmdock-desktop/src-tauri/src/library.rs:3325`, `apps/bmdock-desktop/src-tauri/src/library.rs:3600`, `apps/bmdock-desktop/src-tauri/src/library.rs:4536`, `crates/bmdock-probe/src/main.rs:98`.

The production desktop manifest currently depends on Tauri, serde, and serde_json, without rmcp: `apps/bmdock-desktop/src-tauri/Cargo.toml:12`. Reuse proven process/session concepts from the probe through a deliberate ownership seam; do not infer that the probe is already the desktop transport.

## Prioritized evidence table

| ID / priority | Finding and evidence | Mechanism / impact | Minimal recommendation |
|---|---|---|---|
| B01 / P0 prerequisite | Desktop installs `Supervisor::default()`, `EmptyLibrary`, `EmptyBackupStore`, `EmptyDraftStore`: `main.rs:72-76`. `FixtureLibrary` and implementation are `#[cfg(test)]`: `library.rs:3600`, `library.rs:4536`. | Note read/write fail as unsupported while default query methods return empty fixture DTOs. No desktop engine connection means a performance claim would currently measure the wrong backend. | Add one production engine-session owner and typed query adapter in a later implementation task. Start with an isolated generated project and one profile; expose unavailable versus connected-empty explicitly. |
| B02 / P1 integration risk | Synchronous `ipc_invoke` takes one mutable `Mutex<AppState>` and holds it through all dispatch: `main.rs:35-66`; all `NoteLibrary` methods are synchronous: `library.rs:3302`. | A future slow query, file operation, or engine call blocks unrelated status/navigation commands for its full duration. Native UI-thread impact is not measured here. Existing empty adapters hide this risk. | Snapshot route/session state under a short lock, execute engine I/O asynchronously outside it, and serialize only mutations that share a real consistency boundary. Do not simply change the function keyword to `async` while retaining the lock across I/O. |
| B03 / P1 correctness | Search results commit without a request identity check: `App.tsx:1618-1646`. Load-more merges a captured old page after await: `App.tsx:1709-1739`. | If query A completes after query B, A replaces B. A stale page from an old query can similarly restore old results. Proven as a missing guard in source; not reproduced live. | Frontend owner adds a query/project/profile generation and suppresses stale completions; allow only one in-flight request per cursor. Define cancellation separately from stale-result suppression. |
| B04 / P1 future query contract | Fixture collection sorts identifiers (`library.rs:3673`), computes lexical scores (`library.rs:3759`), and slices hits without sorting by score (`library.rs:4659-4671`). | A high-score result can sit after the first page behind lower-score alphabetic results. Sorting a displayed page afterward cannot recover global top-k. This is a fixture behavior, not an observed official-engine defect. | Production adapter must preserve upstream relevance order before pagination. Test high-score results whose identifiers sort last; use a deterministic tie-break only where the upstream contract supports it. |
| B05 / P1 evidence gap | Recall benchmark has one hardcoded query, `欢迎` (`library.rs:29`); relevance is the same substring predicate used by retrieval (`library.rs:3791`, `library.rs:4754-4759`). Timing wraps a single local `search_notes` call; graph timing covers one expansion (`library.rs:4793`). | It checks UTF-8 and fixture mechanics, not semantic or multilingual retrieval quality. No cold/warm distributions, concurrency, cancellation, IPC duration, result payload, renderer duration, or native frame evidence. | Create independent relevance labels and a repeatable isolated end-to-end benchmark with explicit workload/device/profile records. Keep existing fixture test useful but relabel its evidence scope. |
| B06 / P2 fixture-only cost | `collect_entries` reads every top-level Markdown file for its title (`library.rs:3635-3674`); `collect_lexical_hits` reads those bodies again (`library.rs:3759-3790`); each search page repeats collection (`library.rs:4667`). | For N files totaling B bytes and P pages: about 2N full-file read attempts per search page, O(B + N log N) work per page, O(P(B + N log N)) across traversal, plus canonicalization and metadata calls. Returned page size does not bound input work. | Do not promote this fixture implementation to production. Delegate production pagination to the official indexed query; add client cache only if measurements identify redundant request cost and invalidation is defined. |
| B07 / P2 fixture-only pagination | Cursors are numeric offsets without query or collection identity: `library.rs:6493-6515`; each list/search call recollects current files: `library.rs:4538`, `library.rs:4667`, `library.rs:4935`. | Insert/delete/reorder between pages can duplicate or omit results; changing page size also changes reported page arithmetic. UI deduplication hides duplicates but cannot restore omitted rows. | Bind client request context to project, profile, query, and page size; reset on known mutations. Specify upstream paging consistency honestly rather than inventing a snapshot guarantee the engine does not offer. |
| B08 / P2 fixture-only graph | Wiki-link extraction deduplicates with `Vec::iter().any` (`library.rs:3280-3299`); graph rereads and reparses the full source before slicing (`library.rs:4595-4609`). | For L distinct links, deduplication can require O(L²) comparisons per parse, repeated on each page. It discovers outgoing textual wiki links only, with depth fixed to one; this is not the official observation/relation graph. | Use official bounded graph/context capabilities for product behavior. Preserve explicit node/edge/depth/response-size budgets. Only optimize fixture parsing if benchmark/test maintenance requires it. |
| B09 / P2 fixture semantics | Directory collection skips subdirectories (`library.rs:3647-3654`), while note resolution accepts nested identifiers (`library.rs:3692-3705`). | A nested fixture note can be addressable but absent from tree/search inventory. Flat fixture success does not verify ordinary nested Basic Memory project navigation. | Include nested Markdown paths, duplicate basenames, Chinese titles, and permalink/file-path differences in adapter contract tests. Avoid using a recursive client crawl as the default fix. |
| B10 / P1 integration lifecycle | Production `Supervisor` has state and process ownership, but `Transport` only defines cancellation; implementations and connected-state calls found in this desktop subtree are test fakes (`supervisor.rs:155`, `supervisor.rs:318`). Host drain projects an existing snapshot rather than shutting down a session (`drain.rs:99-134`). | A state machine and successful fake tests do not establish official handshake, transport ownership, actual cancellation, or window-close cleanup. Slow shutdown can become another global-lock stall if connected naively. | Production session owner must own transport plus child, perform handshake before reporting connected, and implement bounded cancellation/exit with unknown write outcomes. Test process lifecycle independently of native window evidence. |

In this table, `main.rs`, `library.rs`, `supervisor.rs`, and `drain.rs` are under `apps/bmdock-desktop/src-tauri/src/`; `App.tsx` is under `apps/bmdock-desktop/src/`.

## Official upstream contract inspected locally

Both cached engine checkout HEADs match `compatibility/profiles.json`: release `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`; main-preview `3452c821d76c083823d020984d71e06904a1ff1e`. Release checkout status showed only an untracked `.bmdock-engine.json` marker. This audit inspected release source for the details below; the main-preview contract needs separate verification before implementation. These are version-pinned source findings, not a claim about the latest hosted service.

| Official release evidence | Consequence for the client plan |
|---|---|
| `.work/engines/release/src/basic_memory/mcp/tools/search.py:703-731`: `query`, explicit project/project_id, 1-based page, page_size, search_type, `output_format="json"`. The source explicitly distinguishes item offset from page number. | Use structured output and an explicit project route. Do not parse rendered Markdown when JSON exists; do not translate the fixture offset directly into an upstream page number. |
| Same file, `:1130-1147`: FTS, vector/semantic, hybrid, title, and permalink query choices; `:1212-1233`: typed SearchClient and structured response. | Feature availability and score semantics should come from the selected pinned engine/configuration. The fixture's constant semantic score is not a hybrid implementation. Optional semantic/provider activation is outside this optimization plan unless separately authorized. |
| `.work/engines/release/src/basic_memory/repository/sqlite_search_repository.py:988-1008`, `:1052-1074`: indexed SQLite FTS5 query with `bm25`, ordering before LIMIT/OFFSET. | Preserve engine ranking and pagination; do not add a parallel Rust full-vault scan or private SQLite access. BM25 score ordering differs from a simple higher-is-better local integer score. |
| `.work/engines/release/src/basic_memory/mcp/tools/build_context.py:153-185`, `:244-260`: bounded page_size and max_related with depth parameter. | Distinguish context retrieval from plain text snippet preview and outgoing wiki-link lists; project upstream budgets into a bounded client operation. |

Stable source links: [release search tool](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/search.py), [release SQLite search repository](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/repository/sqlite_search_repository.py), [release context tool](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/build_context.py). These URLs identify the locally inspected source; web documentation and comparable-project research are owned by the parent task.

## Persistence and safety implications

Fixture note writes use exact UTF-8 `fs::write`, then disk readback (`library.rs:5494-5536`; `content_safety.rs:64-66`). Exact-byte observation is valuable but is not an atomic crash-recovery protocol. `content_safety` is itself test-only in the production module tree (`main.rs:13`). Therefore the plan must not expose this fixture code to user projects by removing `#[cfg(test)]` and calling it a production adapter. Official engine writes should stay engine-owned; client drafts/backups need their own explicitly scoped persistence acceptance before enabling them in a user workflow.

The host's single global mutex also masks true concurrent operations. Existing per-target conflict tests must be rerun after shortening that lock, and lifecycle ownership must define what happens to an in-flight write on cancellation or project changes. An unknown write result must remain unknown until reconciled; repeated retries are not a performance optimization.

No direct user-vault database reads, SQL migrations, index creation, provider activation, global configuration edits, or dependency installation were performed or are authorized by this audit.

## Proposed backend-oriented child groupings

These are planning groupings for the parent to merge with UI and documentation work, not additional tasks created by this auditor.

1. **Runtime connection and capability truth (B01/B10).** Own desktop engine session, selected-profile launch policy, typed adapter, disconnected/empty/error projection. AC: an isolated generated project becomes connected only after an actual official handshake; tree/read/search complete against its known data; unavailable engine is visibly distinct from zero matches; wrong route remains denied; each profile has separate receipts. No GUI or real-vault claim from a unit test.
2. **Indexed query semantics and bounded navigation (B04/B06/B07/B08/B09).** Own search/list/context/graph DTO mapping and official adapter contract tests. AC: a high-relevance result whose identifier sorts last appears in the engine-ranked first page; page traversal has no duplicates or missing expected entries on an immutable dataset; query/profile/project changes reset paging; all responses respect chosen page and graph budgets; client does not read all note bodies to construct each search page. Nested notes and CJK identifiers work through official APIs.
3. **Responsive operations and request lifecycle (B02/B03/B10).** Backend owner handles short state locks and session cancellation; frontend owner handles generation guards and single-flight pagination. AC: a deliberately delayed read cannot hold the global application-state lock through its delay; status requests complete while it is pending; A/B reversed completion leaves B visible; load-more from the previous query is ignored; cancel-after-accept writes are not blindly retried. Unit tests use controlled completion, not flaky sleeps.
4. **Performance and retrieval evidence (B05).** Own isolated dataset generator, labels, harness, metrics, and evidence report. AC: report cold and warm p50/p95, sample count, dataset bytes/count, hardware/OS, profile SHA, IPC/engine/render boundaries, response size, and cancellation outcome separately; use at least 30 samples per measured case after explicit warmup; compare baseline and proposed implementation on identical fixtures; independently labeled Chinese/English queries test relevance. Do not label default 0 ms or substring-derived recall as native production performance.
5. **Focused ownership and verification cleanup.** Extract only runtime/query modules needed by the above changes from the oversized library/IPC files, preserving existing contracts until their intentional change. AC: each touched command has an executable behavioral contract test; source-string tests are retained only where they verify repository wiring rather than substituted for runtime behavior. Refresh the specific Trellis specs that own session/query contracts; do not turn this into a whole-repository rewrite.

## Proposed measurement design, not measured results

The following were initial research suggestions. The parent `design.md` D4 and C01's final manifest supersede workload defaults and budgets: approximately 4 KiB notes, separate 1 MiB/5 MiB notes, five cold/30 warm samples per supported scenario/profile, and the specified paired adapter-overhead budget. Do not use the initial alternatives below as a competing acceptance protocol.

Use temporary generated projects under `.work/`, separately provisioned per engine profile. Suggested workload scales are 100, 1,000, and 10,000 notes with recorded total bytes; include 1 KiB typical notes, a bounded large-note case, nested folders, CJK content, duplicate basenames, and high-degree graph nodes. Treat these as candidate test sizes, not existing fixture counts. Capture initial search and follow-up pages, read-after-selection, route/status while search is delayed, repeated expansion, and stale-response suppression.

Separate three clocks: engine service execution; Tauri IPC roundtrip; renderer response-to-paint. Separately record cold engine/model startup and UI navigation. Choose numeric latency budgets only after establishing the machine and actual connected-engine baseline; this avoids inventing a performance promise from a source audit. Structural gates (bounded page payload, no global lock across I/O, no client full-vault body scan per page, preserved ranking) can be fixed before that baseline.

## Verification status

- **Verified by current source inspection:** production empty-adapter wiring; fixture-only implementations; single synchronous dispatch mutex; fixture scan/ranking/paging algorithms; absent search completion guards; pinned release indexed-search contract.
- **Not executed in this audit:** application build, Rust/Python suites, new benchmark, engine launch, network calls, native GUI operations, or fault injection. Existing test bodies were inspected, not counted as passing this turn.
- **UNVERIFIED:** end-user latency, native UI stalls, memory ceilings, Windows process-tree cleanup, real-vault durability, actual engine cancellation, semantic retrieval quality, and quantitative speedup.
