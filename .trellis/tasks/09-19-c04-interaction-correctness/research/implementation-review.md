# C04 independent implementation review

Date: 2026-09-19. Reviewer: dispatched `trellis-check` agent. Decision: **PASS
for the C04 production-function/state-owner and source-wiring scope**. No code
blockers remain in that reviewed scope. Subsequent C05/C06 changes require their
own checks; this report does not pre-approve later edits.

## Scope and method

Read the C04 PRD/design/implementation plan and context, then traced actual
`App`, `WorkbenchLibrary`, `useEditorSession`, `openNote`, paging operations,
`DraftEditor`, `NoteCrudPanel`, `readShellSnapshot`, `WorkbenchRequests` and
`EditSessions`. Stable function names identify source ownership because C05
layout work may move line numbers. Tests invoke the named production functions
and retained stores, not a duplicate test-only request implementation.

The review was read-only on shared source. Findings were routed to the exclusive
frontend owner, fixed there and independently rechecked. No dependency, native
UI, real-vault, global configuration, commit or gate-status change was performed
by the reviewer.

## Findings fixed and rechecked

| Finding | Evidence and final mechanism |
|---|---|
| Automatic details overwrote newer explicit preview | Reproduced with actual exports: delayed A relations, explicit B preview, then A completion replaced B's error and launched graph/context. `WorkbenchRequests` now captures detail intent; `openNote` guards detail clearing, commits/errors and follow-ons. Two delayed interleaving regressions pass. |
| Diagnostic profile paging could mix profiles | Reproduced `loadMoreCli(main-preview, releasePage)` publishing release and preview leaves while claiming `mixed_profiles: false`. `WorkbenchLibrary` clears old profile data; `loadMoreCli` rejects wrong current/response profile. |
| Old graph page could be admitted after replacement began | `WorkbenchRequests.run` now refuses page admission while its parent replacement is pending, with an actual graph interleaving regression and a page-owner admission matrix. Initial tree uses `loadTree`; all paged owners were checked. |
| Reload confirmation and response-driven delete target lacked complete identity | `DraftEditor` confirmation now matches editor key plus revision. `EditSessions.update` clears delete confirmation on identifier changes. Ordinary identifier-input changes already reset delete confirmation; that initially suspected ordinary-input defect was retracted. The fixed additional case is a response-driven move target. |
| Successful retry retained a stale page error | `loadMoreTree` clears the error on success; a regression passes. The owner also retained the last delivered selected note during reread so a local note-read failure does not remove access to its editor. |

## Acceptance mapping

| Criterion | Result and evidence |
|---|---|
| AC1 | PASS: deferred note/query orders, stale error/cleanup and follow-on tests; profile/page regressions; source review of navigation/runtime invalidation, actual session identity and connected `expected_session` routing. |
| AC2 | PASS: save-A/type-B/ack-A, unsupported/rejected save, revision-safe reload and retained session tests; source review of app-lifetime ownership, selected-note return and confirmation identity. No durable save claim is added. |
| AC3 | PASS: synchronous page/mutation deduplication and no-retry tests, local-error result retention and successful retry test; source-reviewed pending announcements and editor availability. |
| AC4 | PASS within stated scope: final executable regressions/build/boundary checks below; historical negative controls establish sensitivity. Native evidence remains distinct and unavailable. |

## Independent final verification

| Command | Result |
|---|---|
| `npm run test:behavior` in `apps/bmdock-desktop` | **27/27 PASS** |
| `npm run build` in `apps/bmdock-desktop` | **PASS**, TypeScript plus Vite 7.1.7; 38 modules |
| `python -m unittest tests.test_desktop_shell -q` | **34/34 PASS** |
| `git diff --check` | **PASS** |

There is no separate configured frontend lint command. The first restricted
build reached Vite but esbuild's installed helper hit `spawn EPERM`; the approved
local execution passed. No download or dependency change was used to resolve it.

The owner's earlier negative control temporarily restored unguarded completion
and response-body overwrite: 8 expected failures in 19 tests, followed by 19/19
after restoration. Later additions reached the independently verified 27/27.
This was a controlled restoration of the relevant former behavior, not a run of
the full historical commit. See [implementation evidence](implementation-evidence.md).

## Explicit evidence limits

No mounted DOM, browser or native UI operation occurred. Native layout/focus,
screen reader, IME composition, app-exit behavior and input-to-presented-frame
timing remain **UNVERIFIED**. Reload-confirmation key/revision binding was
source-reviewed, not exercised through a mounted component. CRLF/Chinese tests
prove JavaScript string retention, not textarea normalization or disk bytes.

The final Node run retained 6,291,456 UTF-8 bytes across 1 MiB and 5 MiB current
editor bodies and observed heap delta 11,575,464 bytes; GC was uncontrolled.
This is a bounded host observation, not steady-state/native RSS or typing timing.
Production mutation/draft availability remains the backend's responsibility;
C04 does not infer persistence from a successful-looking envelope.

The full performance matrix remains pending under the user's explicit deferral.
The inherited `A later task was completed before G0` failure and release gates
were not repaired, weakened or reclassified by these focused checks.
