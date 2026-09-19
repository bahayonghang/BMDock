# Frontend audit: task-oriented workbench and interaction correctness

Date: 2026-09-19. Scope: source inspection and first-party web references only. No product code was changed; no browser or native UI was operated. Native rendering, screen-reader behavior, IME, interaction timing, and performance baselines remain **UNVERIFIED**. Priorities below are planning priorities, not measured incident severity.

## Current implementation and evidence boundary

The live package uses React 19.1.1, TypeScript 5.9.2, Vite 7.1.7, Tauri API 2.11.1 and CLI 2.11.4 (`apps/bmdock-desktop/package.json:12`). The supplied root description that no frontend exists is stale. `App.tsx` owns navigation, workbench request state, editor state and numerous diagnostic panels in one file. `shell.ts` owns initial snapshot loading; `ipc.ts` provides typed commands and fixture routing; `styles.css` is a single dark stylesheet with repeated literal colors and no token layer. `i18n.ts` owns the Chinese catalog and `main.tsx:7` applies its locale to the document.

The native window is configured at 1200 x 800, minimum 720 x 480 (`apps/bmdock-desktop/src-tauri/tauri.conf.json:17`). Existing positive foundations include native buttons, labels, landmark navigation, skip link, visible keyboard focus, explicit error categories, and text-only note rendering. Preserve these. Fixture-only route types (`ipc.ts:4-15`) are deliberate policy: UI redesign must not silently enable user vault access or equate fixture operations with official Basic Memory MCP capability.

The scoped spec index currently lives at `.trellis/spec/bmdock-probe/backend/index.md`; its typed IPC guide explicitly distinguishes fixture features and unverified native UX. No frontend-specific spec directory was found during the scoped listing. The `codebase-design` skill was used to recommend small request/editor ownership seams, rather than introducing a generic state platform.

## Findings

### FE-01 — P1: delayed reads can replace the user's latest selection or query

**Source fact:** `App.tsx:718-755` reads a note, immediately replaces the selected note, then sequentially loads relations, graph and preview without a selection generation. `App.tsx:1618-1646` commits search results without query identity; `App.tsx:1709-1739` commits pagination against the captured previous page. Profile switching launches three similar unguarded requests (`App.tsx:674-678`).

**Reproduction reasoning:** delay note A; select B and complete B; complete A. A can replace B. Alternatively allow A's relation/graph calls to finish after B's note, producing mixed-note panels. Search A, then B, then finish A's pending next page: the old query can replace B. These are source-confirmed missing guards; actual native scheduling reproduction is outstanding.

**Smallest improvement:** own read generations by route/profile/query/selected note within the relevant state module; ignore obsolete completion, including errors and follow-on requests. Keep previous data only with an explicit refreshing state. This is logical cancellation; do not imply Tauri has cancelled backend work. Parallelize independent note details only after backend concurrency capability is verified.

**Acceptance:** deterministic deferred IPC tests complete A/B in both orders and assert all displayed note details share B's identity; stale success/error/pagination/profile results cannot mutate the active view; current errors retain their existing categories. Navigating away prevents follow-on obsolete detail requests.

### FE-02 — P1: local edits can be silently overwritten or discarded

**Source fact:** `NoteCrudPanel` resets editor state when seed props change (`App.tsx:4456-4467`). `DraftEditor` does the same (`4927-4936`). `persistDraft` overwrites visible body with the response body after saving (`5048-5050`). Navigation conditionally replaces the entire workbench (`211-263`); reload replaces it with a loading section (`269-301`, `507-510`). The local `dirty` calculation (`4938`) is display information, not a navigation guard.

**Reproduction reasoning:** type edit A, save it, type additional edit B before save resolves; the A response sets the body back to A. Selecting another note or navigating to Runtime and back also removes unsaved workbench text. This does not prove already persisted disk data is lost; the risk is unsaved local text.

**Smallest improvement:** retain the active edit session above transient view unmounts; associate save acknowledgements with the submitted revision, update the persisted baseline but never overwrite newer typing. For replacing a dirty session, provide explicit save/discard/stay behavior using the existing draft contract. Keep draft persistence distinct from engine persistence.

**Acceptance:** deferred save followed by typing preserves new text and leaves it dirty; selection and section changes cannot discard text silently; failed save retains content; successful save marks only the submitted revision persisted. Include Chinese composition and CRLF fixture cases; native IME acceptance requires a later native session.

### FE-03 — P1: operation state is missing, allowing duplicate work and misleading empty states

**Source fact:** search treats null data and zero results as the same empty state (`App.tsx:1813-1817`) and has no pending state; submit and load-more remain enabled (`1853`, `1898`). CRUD disabled rules check identifier validity rather than in-flight state (`4566-4624`). Draft save/load similarly remain enabled while pending (`4990-5005`). `openNote` and tree pagination failures promote the entire workbench to error (`742-743`, `1148-1150`), removing otherwise usable content.

**Impact:** repeated clicks issue overlapping work; initial/unsearched/loading/no-results look similar; a detail or page failure replaces unrelated panels and editor state. Backend conflict protection is not a substitute for understandable frontend operation status.

**Smallest improvement:** explicit idle/loading/refreshing/ready/empty/error states for each user operation; a per-operation in-flight guard; localized pending and result-count announcements; preserve loaded pages on page failure. Do not add automatic retries for mutations or `timeout_unknown`.

**Acceptance:** rapid double activation issues one pending mutation/page request; disabled state has visible styling and accessible explanation; search distinguishes not-yet-run, pending and zero matches; failed load-more preserves previous entries and supports a deliberate retry; a read-detail failure does not clear dirty text.

### FE-04 — P2: workbench information architecture favors diagnostic inventory over note work

**Source fact:** 18 equal-weight navigation destinations (`App.tsx:91`) coexist with all workbench subpanels rendered sequentially (`592-712`): note, observations, relations, graph, search, inspector, recall benchmark, schema, resources, prompts, tools, CLI, API audit, context, activity, CRUD and draft. All are inside `.panel { max-width: 46rem }` (`styles.css:127-133`). The tree precedes the reader, and search precedes neither tree nor reader.

**Impact hypothesis:** users scroll through several diagnostic sections to search or edit, and wide desktop space is underused. Exact above-fold positions are unmeasured. There is no evidence that green borders themselves are inaccessible; the issue is a flat emphasis hierarchy.

**Smallest improvement:** one task-oriented workspace with a persistent search entry, bounded list region, primary note reader/editor and optional related-content area. Put inspection/benchmark/API catalogs under an explicitly opened diagnostics surface. Keep fixture/offline/provenance status visible compactly; avoid repeating implementation prose between each action and its content. Simple regions and tabs are sufficient; do not add a docking framework.

**Acceptance:** at 1200 x 800 a user can initiate search, select a result and reach its content without scrolling past diagnostics; diagnostics are reachable by keyboard and retain honest capability labels; opening diagnostics does not discard an edit session; desktop and narrow layouts have documented region/focus order.

### FE-05 — P2: narrow layout and control styling lack complete states

**Source fact:** at widths <=720px navigation becomes six columns (`styles.css:554-566`); desktop grid uses `13rem 1fr` (`79`) without a zero-minimum content track; facts use fixed 7rem/11rem label tracks (`189-196`). Flex result rows do not wrap (`382-391`). Only draft/CRUD inputs receive the custom input style (`292-305`); search fields and select controls do not. No hover or disabled rules are present in the stylesheet. Tree note buttons have no selected-note attribute (`App.tsx:553-576`).

**Impact hypothesis:** minimum native width, long identifiers, Chinese/English mixed labels and text zoom can compress controls or produce overflow. Native minimum width is 720; 320px is an optional web/zoom stress case, not a current native window promise.

**Smallest improvement:** semantic color/spacing/type tokens, shared form-control styling, selected/hover/disabled states, zero-minimum tracks, identifier wrapping and narrow stacked fact/result rows. Use a compact navigation layout at the actual minimum width. Preserve focus outline and localized labels; do not add theme configuration solely for this audit.

**Acceptance:** native or equivalent web viewport checks at 720x480 and 1200x800 plus 200% text zoom show no clipped actions or unintended page-wide horizontal scrolling with 200-character identifiers; selection is programmatically exposed; every input/select/button shares readable states; keyboard traversal follows visual order. Web evidence alone does not close native acceptance.

### FE-06 — P2: unnecessary request fan-out and unbounded accumulated rendering need measurement

**Source fact:** initial shell loading serially fetches capabilities, runtime, projects (`shell.ts:32-91`). Once the tree completes, six secondary requests are launched regardless of interest (`App.tsx:422-427`). Each search also runs its inspector (`616-618`). Note opening waits serially for relations, graph and preview (`753-755`). Tree pages accumulate by array concatenation (`1158`) and render every entry (`553`); search deduplication uses a growing `.some` scan (`1605-1615`). Editor renders run content classification and a full duplicate text preview on each body change (`1240-1265`, `4973`); classification performs whole-body scans (`contentSafety.ts:17-44`).

**Inference and limits:** redundant command count and growth patterns are verified; user-visible latency, main-thread blocking and IPC cost are not measured. React batching does not eliminate full text scanning. No evidence supports adopting virtualization or a query-cache dependency immediately.

**Smallest improvement:** load diagnostic catalogs/inspector only when opened; scope invalidation to affected content; measure first-use/read/search/typing before adding optimization. Keep list pages bounded with explicit navigation initially; if accumulating pages is required, use identity-set merges and measure whether virtualization is justified. Avoid changing fixture routing or serial engine ownership to chase frontend concurrency.

**Acceptance:** initial ordinary workspace emits zero diagnostics/catalog/inspector commands; one ordinary search emits only its search command; deferred detail data has independent loading/error status. Record p50/p95 click-to-result, request counts, rendered row counts and keystroke-to-paint for deterministic 100/1,000/10,000-note fixtures and a 1 MiB note. Set numerical performance budgets after this baseline, before implementation closure; separate backend execution from IPC/React/native render time.

### FE-07 — P2: reader/search presentation exposes wire details but cannot render normal note semantics

**Source fact:** content is a literal escaped `<pre>` (`App.tsx:1264-1266`); graph is node/edge lists (`1546-1564`); search hits display identifiers and lexical/semantic scores (`1875-1891`) and selection triggers context preview rather than opening the primary note (`620-622`).

**Smallest improvement:** prioritize readable note title/path and supported snippet data; result activation opens the primary note consistently. Hide raw scores under inspection. Keep text mode as the initial safe reader; richer Markdown is a separately bounded enhancement requiring an explicit content/link policy and approved dependency choice. Do not replace escaped text with raw HTML. A graphical graph is optional, not necessary to improve related-note navigation.

**Acceptance:** result activation selects the same note as tree activation; title/snippet fields are used only when supported by the route contract; unknown capability stays explicit. If Markdown is later enabled, raw HTML/event handlers/scripts and remote resource fetching remain disabled, links route through an explicit policy, and raw source editing preserves bytes independently of preview.

### FE-08 — P1 validation prerequisite: existing checks do not exercise these interaction failures

**Source fact:** `tests/test_desktop_shell.py:105-150` checks source-string presence for landmarks and states; subsequent frontend sections similarly inspect source text (for example editor at `493`, search at `865`, accessibility at `2192`). `package.json:6-11` has build/dev scripts but no component test entry. These are useful contract guards, not renderer interaction tests.

**Smallest improvement:** a narrow typed-IPC test seam for deferred responses and user-visible session transitions; add actual renderer tests for race, dirty-session, paging failure and keyboard behavior. The test-tool choice must be recorded and any new dependency separately authorized at implementation time. Keep structural checks where they enforce fixture policy; do not multiply string assertions for behavior.

**Acceptance:** FE-01/02/03 regression scenarios fail before fixes and pass after; targeted TypeScript/build and existing fixture policy tests pass; native layout/IME/screen-reader and timing evidence is explicitly separate from mocked web tests. Planning adds no dependencies and claims no checks passed.

## First-party comparisons and bounded transfer

1. [Joplin search documentation](https://joplinapp.org/help/apps/search/) describes direct note navigation through Goto Anything and query/filter behavior. Its documented non-Latin search fallback can be slower on large collections. Transfer: bring note finding into the primary workflow, and measure Chinese/mixed-language queries separately. Do not copy its FTS implementation or query grammar into BMDock without the official engine contract.
2. [Obsidian deferred views](https://docs.obsidian.md/plugins/guides/defer-views) documents creating actual view content when its tab becomes visible. Transfer: defer diagnostics and their data until opened, while preserving editor session state separately. This does not require adopting Obsidian's workspace API or recreating its docking system.
3. [React useEffect reference](https://react.dev/reference/react/useEffect) explains ignoring obsolete request responses and avoiding request waterfalls. Transfer: make identity/cleanup explicit and parallelize only truly independent permitted reads. Adding a framework or generic query engine is not a prerequisite.

These references are design evidence, not proof of BMDock performance. Basic Memory engine/version-contract research is owned by the parent/backend research scope.

## Suggested bounded child tasks

| Group | Ownership and outcome | Acceptance mapping | Dependency |
|---|---|---|---|
| F1 Interaction correctness | Request identity, operation state, dirty-session and save revision semantics; targeted executable regression tests | FE-01, FE-02, FE-03, FE-08 | Existing typed IPC; coordinate route identity with backend owner |
| F2 Workspace UX and visual states | Navigation grouping, primary list/reader/editor, diagnostics disclosure, tokens, controls and keyboard/responsive layouts | FE-04, FE-05, FE-07 basic reader/result activation | F1 session ownership before moving/unmounting views |
| F3 Demand-driven work and measured performance | Lazy diagnostics, scoped invalidation, bounded rendering, reproducible baseline and budgets | FE-06 plus stale-page tests | F1 and backend measurements; avoid overlapping App.tsx edits with F2 |
| F4 Native acceptance | Authorized native window, Chinese IME, keyboard, screen reader, min-size/zoom and timing verification | Native clauses in FE-02/05/06/08 | F1-F3; UI operation explicitly authorized when executed |

Split `App.tsx` only along these owned state/view responsibilities as implementation requires. A module split is not acceptance by itself. Do not create a separate generalized design-system or caching platform task unless measurements or product requirements establish that need.
