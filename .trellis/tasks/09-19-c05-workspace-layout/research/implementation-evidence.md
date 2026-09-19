# C05 focused workspace implementation evidence

Date: 2026-09-19. Frontend implementation only. No browser/native UI operation,
new dependency, backend change or persistent storage was used for this task.

## Implemented mechanisms

| Requirement | Implemented mechanism | Evidence and remaining boundary |
|---|---|---|
| R1 / AC1 | Workspace and Projects are primary navigation; remaining 16 destinations remain available in native Diagnostics/Settings and Maintenance/Integration disclosures. Search/list and reader are adjacent grid regions; related content and workbench diagnostics are explicit disclosures. | React server markup checks all 18 semantic destination buttons and both disclosure groups. Actual 1200x800 reachability remains unmeasured. |
| R2-R3 / AC2 | Semantic CSS color/spacing tokens, uniform form controls, hover/focus/disabled/selected states; zero-minimum grid tracks, wrapping identifiers, narrower stacked workspace and grouped navigation. Selected tree items expose aria-current. | Source implemented for 720/1200 widths and narrower effective viewports; 200-character identifier is included in source-render fixture. No native/web viewport, zoom or overflow measurement was performed. |
| R1-R3 / AC3 | Source and session-draft mode buttons expose aria-pressed; DOM order remains search/list then reader then optional diagnostics. Existing C04 app-owned editor sessions and request guards are preserved across view changes. Pending/error/empty text is localized and compact. | Existing 27 C04 behavior checks remain passing. Mounted keyboard/focus/screen-reader behavior is UNVERIFIED. |
| R4 / AC4 | The source reader renders note.body directly as an escaped pre text child, ahead of optional provenance. Source display does not transform YAML/newlines or claim raw disk-byte equality. | Actual NotePreview server render asserts hostile script/image markup is escaped, YAML/CRLF survives string rendering, and reading does not change retained draft text. No HTML injection or remote resource element is generated. Browser textarea/IME and physical byte fidelity are separate. |
| AC5 | Existing TypeScript/Vite build plus Node behavior/server-render checks and desktop source checks. | Passed as below; native acceptance remains incomplete. |

Search-result activation uses the same guarded openNote operation as tree
selection. Directory items use the already supported optional list_tree
directory argument, with parent navigation and synchronous invalidation of old
requests. Tree/read still send the C02 expected session identity. Diagnostic
profile selection remains independent from the actual engine profile.

CSS keeps list and source regions bounded at desktop widths; below 960 CSS px
they stack. At 720 CSS px navigation wraps into compact groups. These are
implementation facts, **not** evidence that native 720x480/1200x800/200%-zoom
acceptance has passed.

## Validation

- **PASS: 32/32** `npm run test:behavior`: original 27 behavior tests plus actual
  React server-render checks for escaped full source, retained draft independence
  grouped navigation completeness, selected/unselected search-result state,
  and initial-tree restoration versus a newer user note selection.
- **PASS:** `npm run build`: TypeScript 5.9.2, Vite 7.1.7, 38 modules. Local
  installed esbuild execution used the previously approved sandbox exception.
- **PASS: 34/34** `python -m unittest tests.test_desktop_shell -q`.
- **PASS:** `git diff --check` at the validation checkpoint.

No graphical screenshot, browser session, native window, IME or accessibility
tool was used. Server markup checks cannot establish visual fit, click-to-paint,
focus movement, keyboard scrolling or screen-reader announcements. Those clauses
remain **UNVERIFIED**, pending manual or explicitly authorized live acceptance.

## Handoff

Independent C05 review found and the owner fixed two view-wiring issues:
search results now receive the selected note identity and expose aria-current
with the same style as the tree; replacing a directory clears its prior paging
error. Directory loading/failure now keeps the reader/editor and parent-folder
navigation available, with a local list status/error instead of replacing the
whole workspace. This made a previously hidden interleaving possible: tree
completion could restore retained A while user-selected B was still loading.
The actual initial-tree operation is now an exported, App-used loadTree function
which gates restoration using captured detail intent. Its deferred regression
failed when the guard was temporarily removed (31 pass / 1 fail), and passed
after restoration (32/32). Final owner build, 34 source checks and diff check
passed. The list scrollport also has 0.75rem focus gutters at every breakpoint
so source styles do not clip the 3px outline plus 3px offset.
The final independent check is pending.

C06 owns demand-driven data loading,
search contract presentation and measured rendering performance. Closed native
disclosures currently improve navigation but do not by themselves defer existing
catalog IPC calls or React computation; no performance gain is claimed for that.
No Markdown renderer, theme configuration or docking dependency was added.
