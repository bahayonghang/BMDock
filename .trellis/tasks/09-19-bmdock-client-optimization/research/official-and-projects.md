# Research: Official Basic Memory contracts and comparable clients

- Query: Which upstream contracts and proven client patterns should guide BMDock frontend, query, and performance optimization?
- Scope: mixed; read-only local source inspection and first-party web research; planning only.
- Date: 2026-09-19
- Skills applied: `research`; `trellis-start` read for workflow context. The dispatched research role owns this file only.
- Evidence level: source-verified contracts and design recommendations. No live engine, native GUI, latency benchmark, or user-vault operation was run.

## Findings

### 1. Version and ownership boundaries

`compatibility/profiles.json:6` pins release `0.23.2` to `c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048` and 21 expected tools. `compatibility/profiles.json:13` pins main-preview to `3452c821d76c083823d020984d71e06904a1ff1e` and 27 tools. The two local engine checkouts' `.git/HEAD` files contain those exact commits. This verifies checkout identity, not runtime compatibility or an unmodified source tree. No Git operation was performed.

The current [Basic Memory MCP reference](https://docs.basicmemory.com/reference/mcp-tools-reference) also documents Cloud-only capabilities. Treat it as documentation current on the access date, not as either immutable profile's supported surface. In particular, the client must distinguish project routing, tool availability, accepted parameters, result shape, and enabled model/index state. Tool-name counts alone do not prove those contracts.

The local frontend/backend analyses should remain the source for BMDock-specific defect scope. Two inspected anchors explain the integration gap: `apps/bmdock-desktop/src-tauri/src/library.rs:3759` gathers fixture entries and reads every note for literal substring matching; `library.rs:4659` computes all hits before slicing a requested page. Thus its pagination bounds returned rows, not scan work. This is fixture behavior, not evidence of official Basic Memory search performance. The related spec expressly identifies T21 as fixture lexical search at `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md:149`.

**Planning implication:** first decide the authoritative read provider and its fixture/official-engine boundary. Do not optimize the fixture implementation into a second production search engine or report its scores as upstream semantic search.

### 2. Pinned engine contracts that affect client correctness

| Contract | Verified source | Client implication |
| --- | --- | --- |
| Explicit project identity | Release `mcp/tools/search.py:710` and `mcp/tools/read_note.py:113` accept `project_id`; `mcp/project_context.py:141` describes constrained, explicit, then default project resolution. | Carry the selected exact project identity with requests. Key request state and any cache by profile/session/project. A selected label must never silently route through the default project. |
| Distinct filter namespaces | Release `mcp/tools/search.py:733` distinguishes frontmatter `note_types`, graph `entity_types`, and observation `categories`. | UI labels must distinguish note type, result kind, and observation category. A “decision” category is not automatically `metadata_filters.category`. |
| Pagination is not an exact-count guarantee | Release `schemas/search.py:156` has `total_is_exact` and `has_more`; main-preview `schemas/search.py:303` has the same boundary. | Only display a final total/page count when exact; otherwise display loaded results and availability of another page. An unknown total of zero must not become “no results.” |
| Defaults depend on server configuration | Release `mcp/tools/search.py:56` prioritizes configured `default_search_type`, then semantic-enabled hybrid, otherwise text. | Choose an explicit supported mode for reproducible benchmarks. Distinguish unavailable semantic search from an empty successful result. |
| Stable identity accompanies discoverable results | Release `schemas/search.py:125` contains `external_id`, optional body, matched chunk, and score. | Use identity plus project for selection and follow-up reads, not rendered title or list position. Avoid eager full-note reads for every result row. |
| Page size does not chunk exact-note content | Release `mcp/tools/read_note.py:118` states that exact reads return the full note body and paging controls fallback suggestions. Main-preview separately adds explicit `start_line/end_line` at `mcp/tools/read_note.py:96`. | This plan uses unsliced exact reads, omitting line ranges. Do not use page_size as body slicing or send preview-only line-range fields to release. |
| Context is bounded graph traversal | Release `mcp/tools/build_context.py:153` supplies depth, timeframe, page size, and max-related; `:249` rejects oversized page/related limits. | Expand context on demand. Make the default seven-day filter visible or explicitly choose a suitable timeframe; an older relation absent under that filter is not proof of missing graph data. |
| Activity can enter discovery mode | Release `mcp/tools/recent_activity.py:193` permits discovery when no project resolves. | A project-local activity pane must send its project explicitly. Fixture file modification times are not equivalent to official activity. |

Paths in this table are relative to `.work/engines/<profile>/src/basic_memory/`. Immutable upstream references: [release search implementation](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/search.py), [release search schema](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/schemas/search.py), [release read implementation](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/read_note.py), [context implementation](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/build_context.py), [activity implementation](https://github.com/basicmachines-co/basic-memory/blob/c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048/src/basic_memory/mcp/tools/recent_activity.py). These source files were read locally; the web renderer failed to load the pinned search blob.

### 3. Parse structured results once at the engine boundary

The [MCP 2025-11-25 tool specification](https://modelcontextprotocol.io/specification/2025-11-25/server/tools) distinguishes `content`, `structuredContent`, optional output schemas, and tool errors. Request `output_format="json"` where the pinned tool supports it, decode a recognized wire shape, and produce a typed BMDock response. Do not scrape human-facing Markdown tables or successful-sounding prose.

There is a concrete caveat: release `mcp/tools/search.py:1233` returns a dictionary on successful JSON requests, but `:1240` can catch an error and return formatted guidance text. Therefore JSON requested + transport success does not establish a successful query. A malformed/unexpected payload must remain a visible error or unsupported response rather than an empty result set.

Full source also needs an explicit frontmatter choice. Pinned JSON `read_note` defaults can omit YAML; release `mcp/note_reads.py:97` strips frontmatter for body-only content. Capture and use `include_frontmatter=true` for the reader/source flow, omit main-preview line-range fields for this unsliced flow, and compare delivered UTF-8/YAML/CRLF text with known fixtures. Preserve engine-delivered text while reporting upstream normalization separately; a string response is not proof of disk-byte fidelity. Main-preview line ranges are source-present but outside this plan.

`scripts/core.py:153` already prioritizes `isError` and recognizes an observed FastMCP `structuredContent.result` wrapper for write receipts. That is useful wire-shape evidence, but it does not prove the identical wrapper for all read tools. Capture representative responses separately for each selected profile; support only evidenced shapes at one adapter boundary. Do not add speculative alias parsers across frontend, IPC, and engine layers.

### 4. Performance belongs to the request lifecycle and upstream index

Basic Memory's [technical architecture](https://docs.basicmemory.com/reference/technical-information) treats Markdown as authoritative and the database/search index as derived structures updated through file-change processing. Its [semantic-search guide](https://docs.basicmemory.com/concepts/semantic-search) describes incremental embeddings and optional reranking; initial model/index work and reranking can add latency. These are architectural facts, not BMDock timing results.

Pinned implementation evidence supports retaining one owned engine session instead of repeatedly paying initialization: release `mcp/server.py:84` initializes the application container/database lifecycle, starts the watch coordinator at `:177`, and stops it at `:192`. Release `repository/semantic_vector_sync.py:981` skips unchanged fingerprinted entities without embedding jobs. The [MCP lifecycle specification](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle) separates initialization, ordinary operation, and shutdown; persistence is an optimization recommendation, not a requirement to introduce HTTP or a global daemon.

For BMDock, retain the existing owner/supervisor concept, establish the negotiated session once, and measure cold startup separately from warm requests. Keep UI rendering and unrelated commands responsive while a request is outstanding. A cache, if baseline data justifies one, needs a single owner and explicit invalidation on session/profile/project changes, accepted mutations, and externally observed index changes. Begin with in-flight deduplication and bounded request state; do not add Redis or another persistent database as a planning default.

The source pin already has its own ranking: release `repository/search_repository_base.py:2473` combines normalized lexical and vector signals with `max(v, f) + bonus * min(v, f)` and `:2617` documents a potential fused score above one. Therefore do not label every score a probability/percentage, average fixture lexical scores with upstream vector values, or substitute QMD's reciprocal-rank fusion in the client. Preserve upstream order unless the product explicitly offers a separately defined sort.

The G0 harness disables semantic search and forces local routing (`scripts/core.py:112`). Its offline lexical tests cannot establish model readiness, embedding quality, hybrid latency, or reranker performance. [Routing documentation](https://docs.basicmemory.com/cloud/routing) also distinguishes route selection from authentication: having credentials does not itself select Cloud. No routing, credentials, or model configuration was changed here.

### 5. Chinese recall must be tested per profile

Main-preview contains `repository/script_ngrams.py:18` with CJK and other script ranges, NFKC normalization at `:57`, and bigram construction at `:76`. `repository/sqlite_search_repository.py:818` uses that analysis in an FTS search path. The corresponding release SQLite implementation has no `script_ngrams` path. [Immutable main-preview analyzer](https://github.com/basicmachines-co/basic-memory/blob/3452c821d76c083823d020984d71e06904a1ff1e/src/basic_memory/repository/script_ngrams.py).

Main-preview also adds `search_notes.compact` at `mcp/tools/search.py:919`, omitting bodies/excerpts while retaining discovery metadata; release's signature ends at `:792` without this parameter. Main-preview temporal arguments appear at `:883`. These are useful optional capabilities of that snapshot, not grounds for sending unsupported fields to release. `compact` reduces payload but removes the excerpt needed by a rich results list; choose deliberately per view. [Immutable main-preview search](https://github.com/basicmachines-co/basic-memory/blob/3452c821d76c083823d020984d71e06904a1ff1e/src/basic_memory/mcp/tools/search.py).

Proposed evaluation corpus: Chinese exact titles, terms embedded in unspaced sentences, one-character and multi-character terms, Chinese/English mixed identifiers, punctuation/full-width variants, and semantic paraphrases. Maintain explicit query-to-relevant-note labels; report Recall@k/MRR plus latency separately for text/vector/hybrid, profile, model, and index state. Successful substring matching in fixture notes is neither semantic recall nor proof that the release FTS tokenizer handles the same cases. No measured recall improvement is claimed.

### 6. Comparable first-party projects

The following are design comparisons, not dependency or migration recommendations. Versions are the published pages/repositories observed on 2026-09-19; no installed product was inspected.

| Project and source | Pattern worth reusing | Applicability limit |
| --- | --- | --- |
| [Basic Memory Cloud web app](https://docs.basicmemory.com/cloud/web-app) | Sidebar/project tree, note list, note detail; folder selection scopes the list. Operations/activity have a dedicated location. This supports moving daily reading/search ahead of diagnostics. | The Cloud app's all-project search default and hosted activity semantics do not establish equivalent local MCP capabilities or authorize cross-project access. |
| [Obsidian Search](https://obsidian.md/help/plugins/search) and [Tabs](https://obsidian.md/help/tabs) | Collapsible/expanded match context, understandable search conditions, resizing central/side panes, and linked outline/backlink views. Keep search context beside the selected note. | Start with one selected-note workflow and adjustable panes; a full arbitrary tab/window system is separate scope. Its query grammar must not be silently mapped onto Basic Memory. |
| [Joplin Search](https://joplinapp.org/help/apps/search/) | Documents exact/phrase/filter semantics and explicitly distinguishes CJK search behavior. Make query interpretation and limitations discoverable. | Joplin documents a non-FTS CJK fallback with speed/ranking tradeoffs on large collections. This is not evidence that BMDock should implement full-vault fallback scans. Benchmark the existing upstream profiles first. |
| [QMD](https://github.com/tobi/qmd) | Separates retrieval from full-document loading; has bounded candidate controls and explains keeping models loaded across requests. Useful for cold/warm measurements and progressive retrieval. | Its RRF/reranking pipeline, local models, and HTTP service are its architecture. Copying them would create another search product and change dependency/resource/privacy costs. No QMD latency numbers are transferred to BMDock. |
| [SiYuan](https://b3log.org/siyuan/en/) | Focused content views, breadcrumb navigation, collapsible outline, and dynamic loading demonstrate useful directions for long-document presentation. | The site's large-document smoothness claim is promotional, not a reproducible BMDock benchmark. Its block-based document model should not replace Basic Memory's Markdown and note identity contracts. |

Styling direction inferred from those references: a readable note surface with restrained metadata, consistent spacing/typography, bounded result previews, persistent selection, and a secondary context inspector. Concrete tokens, breakpoints, Markdown behavior, keyboard/IME handling, accessibility, and render profiling belong to the frontend specialist's repository audit. No screenshots or visual equivalence claims were produced by this research.

### 7. Recommended planning deliverables

1. **Typed official read contracts and scope:** profile-specific read/search/context/activity DTOs, exact project routing, recognized result envelopes, explicit unsupported/error states, and pagination conformance. Test inexact totals, two projects with similar labels, metadata/category confusion, unexpected text, and stale project-switch responses.
2. **Session and query work reduction:** persistent owned session, startup/warm timing separation, bounded in-flight work, latest-request identity, deduplication, and no full-note hydration during discovery. Test that paging does not restart the engine and that obsolete requests cannot replace current selection. Add caches only with measured justification and an invalidation contract.
3. **Daily-work UI and document presentation:** project/folder → result list → note detail; distinct inspection/activity views, visible query scope, readable Markdown, progressive long-note rendering, and keyboard/IME-friendly navigation. Validate on Chinese-heavy notes, tables/code, narrow windows, and large lists.
4. **Search quality and index status:** per-profile Chinese corpus, exact vs unknown totals, capability-aware semantic controls, visible index readiness/error conditions, and optional compact discovery only where supported. Preserve rank provenance; do not fabricate semantic metrics from fixtures.
5. **Reproducible performance and regression evidence:** synthetic fixture sizes such as 1,000 and 10,000 notes plus a long-note case; cold startup, warm search/page/read, payload bytes, filesystem work, memory, render time, and edit-to-search freshness. Separate engine and UI time; record p50/p95 and hardware/configuration. Establish baseline and freeze budgets before implementing optimization; do not import another project's benchmark as an acceptance target.

These are suggested work boundaries for the parent task, not newly created child tasks. Dependencies: read/session contracts precede official-query UI integration; performance baseline precedes claims of improvement; final cross-layer acceptance covers the integrated client.

## Files found and related specs

| File or group | Purpose |
| --- | --- |
| `compatibility/profiles.json` | Immutable upstream profiles; verified-runtime fields are not evidence of current app connectivity. |
| `apps/bmdock-desktop/src-tauri/src/library.rs:3759` | Existing fixture scan and literal matching; paginated slice at `:4659`. |
| `scripts/core.py:112`, `:153` | G0 offline environment and observed structured write-result wrapper handling. |
| `.work/engines/{release,main-preview}/src/basic_memory/mcp/tools/search.py` | Exact search arguments, JSON/text behavior, profile-specific compact/temporal capabilities. |
| `.work/engines/release/src/basic_memory/mcp/tools/{read_note,build_context,recent_activity}.py` | Full-note read semantics, bounded context traversal, activity scope. |
| `.work/engines/release/src/basic_memory/{mcp/server.py,repository/search_repository_base.py,repository/semantic_vector_sync.py}` | Session lifecycle, upstream ranking, incremental embedding work. |
| `.work/engines/main-preview/src/basic_memory/repository/script_ngrams.py` | Non-space-delimited script analysis for the preview profile. |
| `.trellis/workflow.md` | Planning artifacts and independent parent/child acceptance boundaries. |
| `.trellis/spec/bmdock-probe/backend/typed-ipc-policy.md` | Current typed IPC and fixture/official capability separation; changes require deliberate spec updates during authorized implementation. |
| `.trellis/spec/bmdock-probe/backend/supervisor-state.md` | Profile isolation, owned lifecycle, and bounded shutdown; unit fakes are not live-engine/native evidence. |

## External reference ledger

All links were accessed or source-mapped on **2026-09-19**. Pinned source links above map to locally read files; other links below were opened through web search/browsing. Statements are paraphrased; no large quotation is used.

| Source | Supported claim | Version/applicability caveat |
| --- | --- | --- |
| [Basic Memory MCP reference](https://docs.basicmemory.com/reference/mcp-tools-reference) | Tool concepts, project identity, JSON options, Cloud/local distinctions. | Living documentation; validate against pinned signatures and negotiated schemas. |
| [Semantic search](https://docs.basicmemory.com/concepts/semantic-search) | Incremental embeddings and optional reranking have separate readiness/latency concerns. | Defaults and providers can change; no BMDock measurements. |
| [Technical information](https://docs.basicmemory.com/reference/technical-information) | Markdown authority and derived index/sync architecture. | Architecture description, not a benchmark or consistency SLA. |
| [Local and Cloud routing](https://docs.basicmemory.com/cloud/routing) | Routing is distinct from credentials. | User configuration and pinned implementation decide the effective route. |
| [MCP tools](https://modelcontextprotocol.io/specification/2025-11-25/server/tools) | Structured result envelope and schema/error distinctions. | Protocol version fixed to 2025-11-25; tool-specific wrappers still require evidence. |
| [MCP lifecycle](https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle) | Initialize, operate, shutdown sequencing. | Does not mandate shared HTTP or a particular pooling strategy. |
| [Cloud web app](https://docs.basicmemory.com/cloud/web-app) | Three-pane navigation and scoped note browsing. | Hosted product behavior; not local MCP parity. |
| [Obsidian search](https://obsidian.md/help/plugins/search), [tabs](https://obsidian.md/help/tabs) | Match-context controls, linked/resizable panes. | UI pattern only; separate query language and layout complexity. |
| [Joplin search](https://joplinapp.org/help/apps/search/) | CJK tokenization tradeoffs and search semantics. | Its fallback algorithm/ranking are not Basic Memory's implementation. |
| [QMD repository](https://github.com/tobi/qmd) | Staged retrieval, candidate limits, persistent loaded models. | Unpinned current README; no copied performance claims or engine change. |
| [SiYuan product documentation](https://b3log.org/siyuan/en/) | Focus, breadcrumbs, outline, dynamic loading concepts. | Promotional capacity claims; different block storage model. |

## Caveats / Not Found

- No current BMDock real-engine latency, indexed-corpus recall, native rendering trace, or external-edit freshness measurement was produced. Source inspection identifies costs and contracts, not their measured severity.
- The pinned GitHub search page failed in the web renderer; the matching local pinned source supplied its evidence. Local `.git/HEAD` matches alone do not establish a clean upstream tree.
- Search error guidance can be text even when JSON was requested. Per-profile wire captures remain necessary before finalizing the read adapter's parser.
- Main-preview CJK/compact features are source-present; quality improvement and production readiness remain unverified. No upstream bump, migration, model download, package install, or global configuration change is recommended as implicit implementation scope.
- Historical native memory was used only to route attention to typed IPC/supervisor evidence boundaries (`MEMORY.md:5616-5618`); current claims above were checked against local source/specs. Basic Memory vault routing documents were read; no vault content was queried or changed for this external-topic research.
