# C06 implementation evidence

## Delivered mechanisms

- App narrows official engine and fixture search/context/activity unions without
  inventing lexical/semantic fields. Exact query text and confirmed mode, tags,
  frontmatter note types and observation categories pass to the typed request.
  Filter-only submissions are allowed. Engine result identity stays separate
  from owner UUID; result activation reads the owner and keeps the selected row
  even when the delivered note identifier is a permalink. Tree reads use the
  optional owner UUID, while directory identity remains unchanged.
- Initial workspace work is directory-only, default 50. Primary note reads do
  not launch hidden details. Related work starts on opening its disclosure;
  each diagnostic catalog has its own demand disclosure and explicit refresh.
  Ordinary search sends no inspector request. Graph pagination remains size20.
- SearchWindow retains up to three pages / 150 hits, preserving the focused
  result's page and evicting the oldest non-active page. History retains actual
  received request cursors only. Previous navigation re-requests its observed
  cursor. Result rows are not collapsed by owner UUID. Request-owner guards
  precede window mutation, and failed query/page requests preserve loaded data.
  Removed pagination controls return focus to the persistent results summary;
  existing controls are left alone. Native focus behavior remains unverified.
- ContentSafetyFacts mounts no duplicate preview or classifier while closed.
  Opening/refresh explicitly captures a synchronous body/revision snapshot;
  further edits only compare revision identity and expose a stale message.
  Closing releases the snapshot. Escaped React text remains unconditional;
  diagnostic classification is not the HTML execution defense.

## Executable checks

- 43/43 Node behavior tests pass using compiled production functions, typed IPC
  fixtures and React server markup. This retains all previous 32 tests and adds
  official request/row semantics, exact filters and size50, read/tree command
  traces, three-page retention and cursor history, failed previous navigation,
  focus fallback invocation, and on-demand diagnostic revision checks.
- Review regressions cover preserving cursor-history render order after previous
  navigation, showing raw signed scores/result kind only in explicit details,
  committing the selected result marker together with a successful current note
  delivery, and retaining newer focus when a delayed page removes its old control.
  The fixture-mutation textarea is now unmounted unless both its disclosures are
  open, while its editor state remains in the app-owned session store.
- Distinct observation hits sharing one owner carry distinct selection intent;
  reverse completion keeps the latest hit, while duplicate same-hit activation
  remains synchronously deduplicated.
- Closed diagnostics were rendered for 30 successive revisions of each 1 MiB
  and 5 MiB body with an injected classifier that throws if invoked: zero scans,
  no mounted preformatted preview. This is server-render evidence, not native
  input-to-frame latency or mounted DOM behavior.
- Negative control: temporarily allowing four retained pages caused exactly
  one failure (40 pass / 1 fail), at the production-window bound assertion.
  Restoring the three-page bound returned 41/41 passing. Mutation was removed.
- 34/34 `tests.test_desktop_shell` checks pass.
- TypeScript + Vite build passes, 39 modules, with installed dependencies only.
  Sandboxed esbuild process creation returned EPERM; the authorized elevated
  build succeeded. `git diff --check` passes.

## Boundaries

Independent review passes: 43 behavior/SSR tests, 34 source checks, TypeScript /
Vite build and diff check; no remaining frontend code blocker. No browser/native
automation was performed.
Native keyboard/focus, IME, query-to-paint timing, DOM row counts and the 30-edit
1 MiB / 5 MiB p95 thresholds remain UNVERIFIED. No Node/SSR duration is offered
as native performance acceptance. C01/C03 representative engine/IPC benchmark
receipts are owned by the paired benchmark work; the full 428-run matrix is
deferred by user instruction. C06 performance acceptance remains partial.
