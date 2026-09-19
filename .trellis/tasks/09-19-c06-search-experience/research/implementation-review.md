# C06 independent implementation review

Reviewed 2026-09-19 after the C04/C05 handoff. Source review covered actual App
wiring, `searchWindow.ts`, `contentSafety.ts`, shared request/editor ownership,
typed DTO consumers, localization, styles and production-function tests. Code
fixes were routed to the exclusive frontend owner; this reviewer edited only
authorized task/spec documentation. No browser or native UI operation occurred.

## Verdict

No remaining code blocker was found in the reviewed frontend scope. The final
independent checkpoint passed 43 behavior/SSR tests, 34 source/boundary tests,
TypeScript/Vite build (39 modules), and `git diff --check`. This resolves the
historical C05 build failure caused by the C03 engine/fixture DTO integration.
It does not complete native/performance acceptance or the deferred full matrix.

## Findings fixed and reviewed

| Finding | Observable mechanism and final correction |
|---|---|
| Previous navigation reordered results | `SearchWindow.accept` re-appended an earlier page and flattened arrival order. Retained bodies now keep eviction recency separately from cursor-history display order; revisiting 7 after 8 displays 1,7,8 when 1 is active. |
| Engine score inspection was missing | The details branch showed only fixture fields. Engine rows now expose raw signed score and result kind in explicit local details; null remains unavailable, and this adds no inspector IPC. |
| Failed selection marked the wrong current row | Search updated its marker before reading B, so a failed B retained A's body with B current. App-used `deliverPrimaryNote` commits body and marker together under the note token; failure and stale completion preserve the previous pair. |
| Same-owner hits lost latest row intent | Two observations sharing one UUID had identical pending read identity, so B was deduplicated and A's marker won. Search identity now includes owner plus hit; tree uses owner plus null. Regression resolves A after B and verifies duplicate B still sends no extra read. |
| Removed pager could steal newer focus | The old fallback focused summary whenever the pager disappeared, even if the user had moved to the query input. The App now checks current document focus, and production fallback runs only when focus is unowned. Native focus remains unobserved. |
| Closed mutation editor retained a hidden body control | A full CRUD textarea mounted under closed disclosures beside the source/draft. It now mounts only while both diagnostics and mutation disclosures are open; existing retained edit sessions preserve its text. |

A cross-layer filter-only mismatch was identified against an earlier C03 source
snapshot. Refresh confirmed the concurrent C03 fix: blank text with a supported
filter is translated to upstream null query; blank without filters is rejected.
This is resolved, not a remaining frontend blocker. C03 owns the Rust regression
and live filter-only receipts; this review checked the current adapter branch
and did not rerun its engine suite or claim independent live-engine evidence.

## Verified mechanisms and command boundaries

The App narrows engine/fixture search, context and activity responses. Title and
excerpt remain escaped text, hit identity is distinct from owning-note identity,
and supported mode/tags/note-type/observation-category options pass through the
typed request. Same-owner observations remain distinct rows. Reads use an owner
identifier when supplied; fixture route and actual runtime session remain in
the request path, independent of the diagnostic tool profile.

Current App startup no longer launches the six secondary loads after tree
completion. Actual `loadTree` command tracing records one tree request with
page size 50 and no diagnostic follow-on. Actual ordinary `openNote` tracing
records one read and zero relations/graph/context commands. Actual `runSearch`
records one search, exact options/text, size 50 and zero inspector commands.
Source review confirms App uses these branches and demand disclosures. These
are injected-transport production-function traces, not a mounted/native startup
trace. Required shell reads remain get_capabilities/get_runtime_state/list_projects;
the zero-optional-catalog claim does not erase project-routing discovery.

Search bounds are three retained page bodies/150 rows, with actual cursor token
history and a selected/focused page retained across eviction. Revisit order is
logical cursor order, not request completion order. Query replacement and local
failure preserve C04 guards and usable results. Initial and subsequent tree
loads use 50; graph still uses 20. Focus fallback is tested as a production
function with source-reviewed document wiring, not mounted/native focus proof.

Content diagnostics capture a synchronous editor-key/revision snapshot only on
explicit open/refresh. Edits do not rescan that snapshot; stale state is explicit.
Closing removes it. SSR rendered 30 closed revisions of each 1 MiB and 5 MiB
body with a throwing classifier: zero scans and no full-text preview. This
proves that closed branch, not native keystroke/frame performance. Safe escaping,
retained draft semantics and no automatic retry of unknown writes are preserved.

## Independent verification

| Check | Result and limit |
|---|---|
| `npm run test:behavior` | **43/43 PASS** using compiled production functions and typed fixtures/SSR |
| `python -m unittest tests.test_desktop_shell -q` | **34/34 PASS**; structural/boundary checks, not native behavior |
| `npm run build` | **PASS**, TypeScript and Vite 7.1.7, 39 modules; installed dependencies only |
| `git diff --check` | **PASS** at the final code checkpoint |
| Dedicated frontend lint | **Not configured**; do not invent a lint pass |

The installed esbuild helper ran through the existing authorized sandbox
exception. No dependency download occurred. The owner reported an earlier
negative control permitting four pages: 40 pass/1 fail; restoring the bound
returned 41/41. That control was not independently repeated and is distinct from
this review's later 43/43 checkpoint after selection regressions were added.

## Acceptance mapping

| Criterion | Status |
|---|---|
| AC1 | **PARTIAL**: shared read path, owner/row identity, supported filters and escaped title/excerpt reviewed and tested; actual mouse/keyboard activation is not native-verified. |
| AC2 | **PASS at production-function/source scope**: optional command traces and demand admission match the mechanism; no mounted/native trace is claimed, and required shell routing reads remain. |
| AC3 | **PARTIAL**: size 50, three pages/150 rows, real tokens, ordering, stale guards and failure retention pass; focus helper/source checks do not establish mounted/native focus or measured DOM rows. |
| AC4 | **PARTIAL**: closed classification/preview work removed and explicit snapshots revision-bound; native query/paint, DOM and 30-edit p95 measurements remain incomplete. C01/C03 own separate engine/IPC receipts. |
| AC5 | **PARTIAL**: frontend behavior/build and source checks pass; final combined query/fixture and performance acceptance is owned by the parent/C07 integration checkpoint, not inferred here. |

## Remaining evidence and handoff

No native window, screenshot, viewport/zoom run, screen reader, IME trace,
mounted DOM measurement or input-to-presented-frame timing was produced.
JavaScript/SSR newline retention is not textarea normalization or physical disk
byte equality. The native p95 thresholds 50 ms/100 ms for 1 MiB/5 MiB and meaningful
query budgets remain open until measured. The user deferred the full performance
matrix while prioritizing code plus representative baselines. C07 can reuse this
code verdict and gather integrated evidence without declaring those clauses done.
This review changes no task status, gate, archive, commit or release state.
