# Client Search and Bounded Presentation

## 1. Scope / Trigger

Apply this C06 contract to search/reader activation, result paging, optional
workbench data, and informational content diagnostics in
`apps/bmdock-desktop/src/App.tsx`, `searchWindow.ts` and `contentSafety.ts`.
Preserve [request/editor ownership](workbench-interactions.md) and
[workspace/source presentation](client-workspace.md). Rust owns connected query
execution and `ipc.ts` owns the engine/fixture discriminated DTO contracts.

This implementation adds no dependency, persistent cache, virtualizer, Markdown
renderer, engine write support or real-vault access. Production-function tests
and React SSR do not establish mounted/native focus, DOM size, IME or frame timing.

## 2. Signatures

Stable production anchors are `WorkbenchLibrary`, `SearchPanel`, `openNote`,
`runSearch`, `loadMoreSearch`, `loadTree`, `DemandPanel` and `ContentSafetyFacts`.

```ts
runSearch(query: string,
  setSearch: (value: SearchPageDto | null) => void,
  setError: (value: WorkbenchError | null) => void,
  invoke?: typeof invokeTyped, options?: SearchOptions,
  cursor?: string): Promise<void>;

SearchWindow.accept(value: SearchPageDto, cursor?: string): SearchPageDto;
SearchWindow.previous(): { available: boolean; cursor: string | undefined };
SearchWindow.reset(): void;

deliverPrimaryNote(note: NoteReadDto | null, resultIdentity: string | null,
  setNote: (note: NoteReadDto | null) => void,
  setSelectedResult: (identity: string | null) => void): void;

restoreSearchFocus(control: { isConnected: boolean } | null,
  fallback: { focus(): void } | null, focusUnowned: boolean): void;

captureContentDiagnostics(body: string, revision: string,
  inspect?: typeof classifyBody): {
    body: string; revision: string; safety: ContentSafetyClass;
  };
diagnosticIsCurrent(snapshot: { revision: string }, revision: string): boolean;
```

`loadMoreSearch` receives the retained `SearchWindow` and a current-request
predicate. Its current check must precede mutation of the window, not only a
React setter. `SearchPanel` supplies query/options, hit identity, previous/next
actions and focus intent; it does not own the engine transport.

## 3. Contracts

Narrow `SearchPageDto`, `ContextPreviewDto` and `ActivityPageDto` using their
engine discriminants. Do not synthesize fixture observations, lexical/semantic
scores or modification times for engine results. Display supported title and
escaped excerpt, retain hit identity, and expose raw signed score/result kind
in explicit result details. `null` score is unavailable, not zero or a percentage.
Engine total/has-more semantics remain the backend contract; do not infer exact
totals from loaded rows or invent an activity cursor.

An engine hit's `identifier` identifies the result row; `note_identifier`
identifies its owning readable note. Distinct observations may share one owner.
Do not collapse them by owner or read their row identifiers as note targets.
Tree notes likewise use optional `note_identifier` for the read and retain their
logical directory/list identifier. A result without an owner has disabled note
activation. Both entry paths use guarded `openNote` and successful-only
`deliverPrimaryNote`; a failed read leaves the old reader and marker together.
Selection request identity includes owner and hit identity (tree uses `null`),
so different hits with one owner retain latest intent while duplicate same-hit
activation is synchronously deduplicated.

Confirmed controls are text/title/permalink mode, tags, frontmatter note types
and observation categories. Pass exact nonempty query text and typed filters
without aliasing their meanings. C03 accepts blank text with a supported filter
and sends upstream `query: null`; blank without filters is rejected. The engine
adapter owns that validation, while fixture-only query rules stay separate.
Unsupported semantic/entity choices are not exposed as working filters.

Request identity contains actual runtime profile/session generation, explicit
route, logical directory, applicable page size, exact query, options and cursor.
The typed invoke wrapper supplies `expected_session` and request generation to
connected query operations. Diagnostic tool profile is independent. C04 token
guards cover every setter, error and window mutation. A failed new query or page
keeps the previous usable window; successful new-query replacement resets it.

Search and connected tree pages request 50. Graph pages remain 20. Search alone
retains at most three page bodies and 150 result rows. Preserve the active/focused
result's page and evict the oldest non-active page. Eviction recency is separate
from display order: flatten retained pages in observed cursor-history order,
including when revisiting an earlier page. Deduplicate by result identifier.
History retains actual request tokens, including `undefined` for the first page,
and no evicted page bodies. Previous navigation reissues its observed token;
never construct a numeric cursor or fetch all results. Focus falls back to the
persistent summary only when a removed page control left focus unowned; a newer
focused input/control must keep focus.

Workspace initialization loads the tree, with only an eligible retained-note
read as a follow-on. Ordinary selection passes `loadDetails=false`; related
relations/graph/context load when their disclosure is open. Diagnostic activity,
resources, prompts, tools, CLI and API audit each load on their own disclosure or
explicit refresh. Ordinary search issues no `inspect_search`. Shell routing still
reads `get_capabilities`, `get_runtime_state` and `list_projects`; those required
shell reads are not the six removed diagnostic/catalog loads. Do not describe
the whole application startup as literally one IPC call.

Closed `ContentSafetyFacts` performs no classification and mounts no preview.
Open/refresh synchronously captures body, editor-key/revision and classification.
Later edits compare revision and show stale state without rescanning or replacing
the captured body. Closing releases the snapshot. The fixture CRUD textarea
mounts only while both diagnostics and mutation disclosures are open; retained
app-owned sessions survive that unmount. Escaped React text is unconditional;
classification is informational, not the HTML execution defense.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Engine versus fixture result | Use its discriminant and fields; no invented score/provenance |
| Two observation hits share an owner | Keep both rows; read the owner; latest row intent wins |
| New note read fails or becomes obsolete | Keep old body/marker together; ignore stale delivery |
| Blank query plus supported filter | Connected adapter sends null query with unchanged filter |
| Fourth page or earlier-page revisit | At most three bodies/150 rows, active page retained, cursor order preserved |
| Query replacement or failed page | Reject old completion; preserve usable data on failure |
| Removed pager after newer focus movement | Do not steal focus; fallback only if focus is unowned |
| Closed optional data/diagnostics | No optional IPC, scan or full-text diagnostic preview |
| Edit after diagnostic capture | Retain captured revision/body and display stale status |
| Unsupported or unknown write outcome | Preserve text and existing C04 no-retry/persistence boundary |

## 5. Good / Base / Bad Cases

Good: observation A and B share owner N; click A, then B, and resolve A last.
The delivered reader is N and only B is current. Good: revisit page 7 after page
8 while page 1 is active; retained display order is 1, 7, 8, within 150 rows.
Base: a failed page request leaves the prior results and explicit error usable.
Bad: merge by owner UUID, normalize a negative score to zero, append a revisited
page after later pages, or mount a hidden 5 MiB preview during every edit.

## 6. Tests Required

Run `npm run test:behavior` and `npm run build` in the desktop package, plus
`python -m unittest tests.test_desktop_shell -q` at the root. Use actual exported
App operations, typed deferred transports, production state owners and SSR;
source-only assertions cannot establish async ordering or editing behavior.

Cover exact options/text and size50, one search/no inspector, ordinary read
without hidden details, initial tree without diagnostic follow-ons, same-owner
distinct rows, failed/stale successful-only selection, repeated-page ordering,
three-page/150-row bounds, real previous tokens and failure retention, removed
control/newer focus, and zero closed classification/preview through 30 revisions
of each 1 MiB/5 MiB body. Retain C04/C05 regressions. Filter-only engine semantics
also require C03 boundary/live evidence; a typed frontend fixture alone is not it.

Native query-to-visible timing, DOM row counts, focus/keyboard/IME and 30 committed
edits remain separate. C01's native p95 targets are 50 ms (1 MiB) and 100 ms
(5 MiB); unmeasured values remain incomplete. Do not derive speedup from
EmptyLibrary, Node execution time, SSR output length or a successful build.

## 7. Wrong / Correct

Wrong: use `[ownerUuid]` as the complete search-selection identity, set a row
current before its read succeeds, and append pages in completion order.

Correct: include owner plus result identity, publish body/marker together under
the current note token, and order the bounded retained pages by observed cursors.

Wrong: launch every catalog after `list_tree`, classify the entire draft on each
render, or infer a native performance pass from 43 passing host tests.

Correct: admit explicit demand only, capture diagnostics at a named revision,
and report production-function/SSR checks separately from native measurements.
