# Focused Client Workspace

## 1. Scope / Trigger

This is the C05 view contract for `apps/bmdock-desktop/src/App.tsx`,
`src/styles.css`, `src/i18n.ts` and the frontend behavior/SSR checks. Apply it
when changing navigation, directory browsing, list/reader layout, source
presentation, semantic controls or responsive styles. Preserve
[workbench interaction ownership](workbench-interactions.md).
C06 adds [bounded search and demand loading](client-search-experience.md) while
preserving this layout and safe-source contract.

The implementation provides a focused workspace with existing React/CSS and
escaped plain text. It adds no Markdown renderer, dependency, theme platform,
docking system or durable editor storage. Source review and SSR evidence do not
establish native viewport, zoom, focus, IME or screen-reader acceptance.

## 2. Signatures

Stable view/function anchors are `App`, `WorkbenchLibrary`, `SearchPanel`,
`NotePreview`, `DraftEditor` and `loadTree`; later layout edits may move lines.

```ts
NotePreview({ note }: { note: NoteReadDto | null }): React.JSX.Element;

SearchPanel(props: {
  selectedIdentifier: string | null;
  selectedResultIdentifier?: string | null;
  onResultFocus?: (identifier: string) => void;
  pending: boolean;
  search: SearchPageDto | null;
  error: WorkbenchError | null;
  onSearch: (query: string, options: SearchOptions) => void;
  onPreview: (identifier: string, query: string, resultIdentity?: string) => void;
  onPrevious?: () => void;
  previousAvailable?: boolean;
  onLoadMore: () => void;
}): React.JSX.Element;

loadTree(
  setEntries: (entries: TreeEntryDto[]) => void,
  setNextCursor: (cursor: string | null) => void,
  setPhase: (phase: "loading" | "ready" | "empty" | "error") => void,
  setError: (error: WorkbenchError | null) => void,
  retainedIdentifier: string | undefined,
  restoreSelection: (identifier: string) => void,
  current: () => boolean,
  detailsCurrent: () => boolean,
  invoke?: typeof invokeTyped,
): Promise<boolean>;
```

Directory browsing uses the existing typed `list_tree` arguments `directory?`,
`cursor?`, `page_size?` and connected `expected_session?`. The view's `invokeRead`
adds the current logical directory and actual engine session to tree requests;
it does not expose filesystem paths or change fixture routing.

## 3. Contracts

`App` retains all 18 existing destinations. Workspace and Projects are primary;
Diagnostics/Settings and Maintenance/Integrations use native `details/summary`
groups. Destination controls remain buttons with `aria-current="page"` on the
active destination. The skip link, navigation and main landmark remain present.
Do not substitute a non-keyboard clickable element for a semantic control.

`WorkbenchLibrary` puts search and the directory list before the reader in DOM
and visual order. The reader offers a button group for read/source and session
draft modes using `aria-pressed`. Related content and workbench diagnostics use
optional disclosures after the main workflow. Tree selection follows delivered
note identity; C06 search selection also retains distinct engine result identity
because multiple hits can own the same note. `aria-current="true"` is published
with successful note delivery, not before a read that may fail. Activating a
search result and a note in the directory uses the same guarded `openNote` path,
reading the optional owning UUID when available. Opening a directory uses its logical identifier; parent browsing
removes one logical path segment.

Directory identity joins request identity. Directory replacement invalidates
older work and clears the old paging error. Keep the grid, retained reader/editor
and parent-directory action available while tree loading fails or remains
pending; report loading/errors locally. Clearing list data does not discard the
app-owned editor session. Reader/draft mode switches likewise do not reseed it.

`loadTree` checks `current` after the typed response and before publishing error
or ready state. Restoring a retained note is an automatic follow-on: it is allowed
only while the captured `detailsCurrent` is still true. A user can now select B
while a tree request retaining A is pending; tree completion must not reopen A.
After awaiting `loadTree`, callers recheck the current token, lifetime and disposal
before any additional follow-on. A prior true return value is not current-state
proof at the next await boundary.

`NotePreview` renders the complete delivered `note.body` as a React text child
of a focusable `pre`, with YAML and existing string newlines left intact. Never
inject note content through raw HTML or interpret it as remote-resource elements.
The source explanation distinguishes upstream normalization from original disk
bytes; provenance is available in a disclosure. Draft text and delivered read
text have different owners: rendering the original source must not rewrite a
retained draft. JavaScript/SSR retention is not textarea or disk-byte evidence.

CSS uses semantic color/spacing tokens, shared control hover/focus/disabled
states, wrapping identifiers and zero-minimum content tracks. The workspace
removes the old 46rem panel cap. The list/reader grid stacks below 960 CSS px;
navigation wraps below 720 CSS px; fact rows stack below 600 CSS px. Scrollable
list/source regions are bounded. Preserve the list's 0.75rem focus gutter at all
breakpoints so the 3px outline plus 3px offset has space inside its scrollport.
These rules express layout intent; they do not prove the native window fits.

Closed disclosures alone do not defer mounted React computation or existing
catalog IPC. C06 owns demand-driven loading, bounded result presentation and
typing work reduction. C05 makes no performance gain claim from its layout.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| Note selected through search | Same guarded read path and selected state as the tree |
| New directory after old page error | Clear the old directory's alert before reporting the new result |
| Directory load fails | Keep reader/editor and navigation available; show localized local error |
| Tree retaining A completes after user selects B | Do not restore A or replace B's pending read |
| Owner invalidated after awaited tree load | Do not launch obsolete downstream requests |
| Hostile script/image markup in note body | Emit escaped text, not executable/remote-resource elements |
| Reader and draft mode alternate | Retain app-owned draft text; do not replace it with delivered source |
| Long mixed-language identifier | Wrap within zero-minimum tracks; native clipping remains to be checked |
| C03 extends query DTOs to engine/fixture variants | Integrate/narrow their real contracts; do not hide TypeScript failures with casts |

## 5. Good / Base / Bad Cases

Good: search selects a note through `openNote`, both lists expose that note's
selection, and the reader shows its escaped full source beside the list.
Good: a failed directory request leaves an unsaved draft available. Base: an
empty directory still presents its path, parent action and localized empty state.
Bad: a late directory response reopens the previously retained note, a stale page
alert appears under another directory, or a closed diagnostic disclosure is
reported as proof that diagnostics no longer execute.

## 6. Tests Required

Run `npm run test:behavior` and `npm run build` in `apps/bmdock-desktop`, plus
`python -m unittest tests.test_desktop_shell -q` from the repository root.
Keep C04's ordering, editor retention and deduplication regressions passing.

C05 adds React SSR assertions for escaped hostile full source with YAML/CRLF,
draft independence, all grouped destinations, selected/unselected search state,
and an actual deferred `loadTree`/`openNote` interleaving. A negative control
removing the tree restoration guard must fail that interleaving. Inspect the
actual App wiring and CSS rather than treating an isolated model as the product.

At independent C05 closeout, 32 behavior/SSR tests and 34 structural/boundary
checks passed. The earlier C05 owner build passed with 38 Vite modules; the final
shared-tree build then failed on concurrent C03 engine/fixture query DTO unions
that require C06 integration. Neither an earlier build nor SSR closes that live
integration failure. See the C05 review for the dated boundary.

Native checks at 720x480, 1200x800 and 200% zoom, 200-character identifiers,
keyboard scrolling/focus, IME and screen-reader behavior remain UNVERIFIED until
actually exercised. No screenshot, browser/native session or network-loading
observation was produced by C05 review. The full performance matrix is separate.

## 7. Wrong vs Correct

Wrong: automatically restore an old retained selection after any successful
directory response, even though the user has made a newer selection.

```ts
const response = await invoke(treeCommand);
setEntries(response.entries);
restoreSelection(retainedIdentifier);
```

Correct: publish only a current tree response and guard its automatic selection
follow-on with the captured detail intent.

```ts
const response = await invoke(treeCommand);
if (!current()) return false;
// Validate the typed tree response before committing its entries.
setEntries(response.entries);
if (retainedIdentifier && detailsCurrent()) restoreSelection(retainedIdentifier);
```

For source presentation, keep `<pre>{note.body}</pre>` as escaped text and retain
the app-owned draft separately. Do not use HTML injection or seed the editor
again simply because the read/source view became visible.
