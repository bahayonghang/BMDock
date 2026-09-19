# C05 independent implementation review

Date: 2026-09-19. Reviewer: dispatched `trellis-check` agent.

**Code verdict: no remaining C05-specific review blockers. Acceptance: PARTIAL.**
Native/viewport/focus evidence is unavailable, and the final shared-tree build
is blocked by concurrent C03 query DTO changes pending C06 integration. C06 code
handoff may proceed; this verdict does not complete C05 or pre-approve C06 edits.

## Reviewed scope

Read C05 task/context and traced actual `App`, `WorkbenchLibrary`, `loadTree`,
`SearchPanel`, `NotePreview`, reader/draft mode wiring, navigation disclosures,
directory DTO use, localization, responsive CSS and production behavior/SSR
tests. Stable function names identify code because ongoing frontend work may
move lines. The reviewer reported findings to the exclusive source owner and
did not edit product code. No native/browser operation or new dependency was used.

The main workflow is search/list before reader, with related content and
diagnostics disclosed afterward. The 18 destinations remain available through
two primary buttons and two secondary groups. App-lifetime C04 editor/request
ownership remains above these transient views. Source text uses escaped React
children; no Markdown renderer or raw HTML injection was introduced.

## Findings fixed and reviewed

| Finding | Observable mechanism and fix |
|---|---|
| Search selection had no visible/programmatic state | Unlike the tree, result buttons lacked selected identity. `SearchPanel` now receives the delivered note identifier and emits matching `aria-current`; selected/unselected SSR assertions pass. |
| Previous-directory error survived replacement | The reused `WorkbenchLibrary` effect did not reset `pageError`. New directory/tree replacement now clears it. Loading/error remains local while reader/editor and parent navigation stay available. |
| Retained tree selection could override a newer user selection | Once the grid remained active during loading, tree completion could reopen retained A while B was pending. The actual App-used exported `loadTree` now checks captured detail intent before restoration; the deferred tree/B regression passes. |
| A true tree result could be reused after owner invalidation | The caller now rechecks `loaded && !cancelled && generation === requests.generation() && current()` before post-await catalog follow-ons. This was source-reviewed. |
| Scrollport edge could clip the focus outline | Tree controls touched the zero-padding left edge of an overflowing list. CSS now retains 0.75rem padding at all breakpoints for the 3px outline plus 3px offset. Actual native focus appearance remains unverified. |

## Independent checks and concurrent integration boundary

| Check | Result |
|---|---|
| `npm run test:behavior` | **32/32 PASS**, existing C04 production-operation tests plus C05 SSR and tree-restoration regression |
| `python -m unittest tests.test_desktop_shell -q` | **34/34 PASS** |
| `git diff --check` | **PASS** at the review checkpoint |
| Earlier C05 owner `npm run build` | **PASS**, TypeScript/Vite 7.1.7, 38 modules; historical checkpoint |
| Final independent `npm run build` | **FAIL**, TypeScript during concurrent C03 DTO integration; Vite did not run |

The final TypeScript errors identify `SearchPageDto`, `ContextPreviewDto` and
`ActivityPageDto` now containing engine/fixture variants in `ipc.ts`, while
`App` still accesses fixture-only `truncated`, `observation`, `lexical_score`,
`semantic_score`, `snippet` and related fields. This is the C03-to-C06 integration
boundary, not a layout fix to bypass with casts or by reverting another owner's
types. Root and frontend owner were notified; the shared tree is not build-green
at this checkpoint. The behavior run passed immediately before that build;
concurrent work means those results do not claim a single frozen final revision.

There is no separate configured frontend lint command. The installed esbuild
execution used the existing approved sandbox exception; no download occurred.
The owner also reported a targeted negative control: removing the retained-note
restoration guard caused exactly 1 failure in 32 tests, then restoration returned
32/32. It proves sensitivity to that specific race, not a native incident.

## Acceptance mapping

| Criterion | Status |
|---|---|
| AC1 | **PARTIAL**: source and SSR support primary workflow/discoverable diagnostics; reachability at native 1200x800 has not been observed. |
| AC2 | **PARTIAL**: semantic selection, control styles, zero-minimum tracks, wrapping and focus gutters implemented; native 720x480/1200x800/200%-zoom fit and focus remain unverified. |
| AC3 | **PARTIAL**: DOM/source order and retained session mechanisms reviewed; local directory loading/error no longer removes editor access. Actual keyboard navigation, focus movement and announcements remain unverified. |
| AC4 | **PASS within source/SSR and state-owner scope**: hostile markup is escaped, complete delivered YAML/CRLF strings are retained, and rendering source leaves the retained draft unchanged. No browser execution, textarea normalization, upstream normalization or physical-byte claim is made. |
| AC5 | **PARTIAL**: 32 behavior/SSR and 34 boundary checks passed; final build awaits C03/C06 integration, and native clauses remain incomplete. |

## Remaining evidence and handoff

No screenshot, browser session, native window, visual fit/zoom measurement,
screen reader, IME, focus traversal, keyboard scrolling or network-loading trace
was produced. Source wrapping rules and 200-character SSR markup do not prove
the absence of native clipping. CRLF in a JavaScript/SSR string does not prove
textarea API or disk-byte equality. These clauses stay open until actual evidence.

Closed disclosures currently reorganize visibility; they do not by themselves
defer catalog IPC or mounted computation. C06 owns demand-driven loading,
engine-query presentation and typing work reduction. Its integration must restore
the shared build and preserve these C04/C05 regressions. Native acceptance and
the explicitly deferred full performance matrix remain pending; no task status,
gate, commit, archive or release was changed by this review.
