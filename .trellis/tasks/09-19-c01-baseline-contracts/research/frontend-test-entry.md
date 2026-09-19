# C01 frontend behavior check

Date: 2026-09-19. Scope: AC4 minimum executable seam, not C04 interaction fixes.

## Entry and tools

From `apps/bmdock-desktop`, run `npm run test:behavior` (RTK equivalent:
`rtk npm run test:behavior`). This is independent of full CI and needs only
the already installed Node 26.7.0 and TypeScript 5.9.2. No dependency, lockfile,
global configuration, browser or native UI changes were made.

The runner type-checks the production shell/IPC modules and typed test fixtures,
compiles them into a unique `node_modules/.cache/bmdock-behavior-*` directory,
and removes that exact directory after the run. Node's built-in test runner
uses `--test-isolation=none` because the sandbox rejected its nested worker
spawn with `EPERM`. Each test creates its own deferred transport, with no
global transport replacement or Tauri mocks installed.

`readShellSnapshot` now accepts an optional typed command function; the default
is the existing `invokeTyped`, so `App.tsx` retains its unchanged production
call and cancellation guard. No new state machine was added solely for tests.
The test directly executes the same snapshot producer whose ready/error output
is consumed by `App`'s `setLoad`. While its response is withheld, the promise
remains pending; after release, the exact profile/catalog state is observable.
This proves the snapshot boundary, not an actual React render or presented frame.

## Executed checks

- **PASS: 5 behavior tests.** Pending-to-ready snapshot; reverse completion of
  two independent profile snapshots; delayed typed policy error with no further
  requests; rejected invoke; unexpected typed response mapped to schema error.
- **Verified negative control:** temporarily changed the production shell's
  capabilities-error category projection to `schema`. The unchanged tests
  produced exit 1 with 4 pass / 1 fail (`schema` actual vs `policy` expected).
  Restored the original category projection and reran: exit 0, 5/5 pass.
  This was a test-sensitivity check, not a discovered/fixed product regression.
- **PASS: production TypeScript and Vite build.** Initial sandboxed build failed
  at the installed esbuild process spawn (`EPERM`), after TypeScript passed.
  The approved local elevated build completed: 36 modules, Vite 7.1.7, 578 ms.
  Build duration is not application response-time evidence.
- **PASS: 34 existing structural checks**, using
  `python -m unittest tests.test_desktop_shell -q`; these remain source checks.
- **PASS: `git diff --check`** at the check time.

## Existing gates and handoff

The parent planning audit and C01 PRD record that the full unit entry is blocked
by the existing phase-order guard (`scripts/tasks.py:check_source`), with T05+
completion preceding G0. Full unit/gate status must be reported separately by
the parent run; this narrow command neither runs nor weakens that guard, changes
historical gate records, or establishes G0 acceptance.

C04 can reuse the deferred typed transport pattern and this command for actual
request/editor state modules when extracted from `App.tsx`. Reverse completion
here demonstrates control over response order; it does **not** prove obsolete
results cannot overwrite React selection, paging or edit sessions. Those remain
C04 requirements. Renderer mounting, DOM/keyboard/accessibility behavior, native
IME, native visual acceptance and performance/paint timing remain **UNVERIFIED**.

No additional test-only dependency is necessary for this C01 snapshot seam.
Renderer-based checks, if later needed, require their own tooling decision.
