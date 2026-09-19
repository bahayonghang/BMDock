# C06 — Search-to-reader and bounded frontend work: implementation plan

## Dependency order
C03 backend query contract, C04 interaction correctness and C05 layout.
Implementation was authorized after planning and serialized after C05. C03 owns
Rust/shared DTOs; C06 owns their frontend consumers. Final code review passes;
remaining native/performance clauses are not complete.

## Steps
- [x] Confirm C03/C04/C05 handoff, supported text/title/permalink and filter-only semantics, actual cursor tokens and size 50. Preserve frozen C01 budgets without claiming they are met.
- [x] Implement M1 success-bound owner-note/row selection and M2 demand diagnostics; typed command traces and same-owner/stale/failure regressions pass.
- [x] Implement M3 three-page/150-row state bound, cursor-history ordering, previous tokens and focus fallback. Production-function checks pass; mounted/native keyboard, focus and DOM row measurements remain below as pending acceptance.
- [x] Implement M5 closed-scan/preview removal and explicit revision-bound diagnostics; closed CRUD textarea also unmounts. Thirty host/SSR revisions of each 1 MiB/5 MiB body and stale-snapshot tests pass, with unconditional escaping and retained sessions.
- [ ] Complete M4/M5 measured acceptance under C01's protocol. Frontend behavior/build/source checks pass; final integrated query/fixture receipts, native query-to-paint/DOM counts and 30 committed-edit p95 <=50 ms/100 ms for 1 MiB/5 MiB remain incomplete. Full matrix remains user-deferred.

## Owned files and coordination
Search and informational content-preview call sites/modules in App.tsx/contentSafety.ts, relevant i18n.ts and focused styles/tests. C03 owns Rust query/DTO changes; agree a single writer for ipc.ts. Preserve C04/C05 seams and unconditional safe escaping; no new persistence, dependencies or backend index.

## Validation
Each AC is independently checked through its design mechanism mapping. C01 selects the executable behavior-check route; no new tooling is installed here. Run the repository-supported frontend TypeScript/build command and relevant existing Python contract checks. Do not substitute source-string assertions for request-order, editing or focus behavior. Report passed, failed, unavailable and native-UNVERIFIED evidence separately; web mocks cannot close native acceptance.

Final independent code checkpoint: 43/43 behavior/SSR tests, 34/34 source/boundary
checks, TypeScript/Vite 39-module build and diff whitespace check PASS. No separate
frontend lint command is configured. See
[implementation review](research/implementation-review.md) and
[owner evidence](research/implementation-evidence.md). C07 may proceed with this
code verdict while native/performance acceptance remains open.

## Rollback
Revert owned search/loading and informational scan/preview scheduling changes only, retaining C04/C05 edit sessions and safe escaping. Do not lose dirty text or execute raw content during rollback. Coordinate any DTO rollback with C03; no data migration or persistence is introduced.

## Exit
All ACs need named evidence; retain incomplete status for unmet native/performance criteria. Do not broaden implementation, commit or release scope. Filter semantics, previous-cursor navigation and numerical budgets depend on C01/C03. If size50 is unsupported, resolve that contract rather than silently fetching all data. Native timing requires later manual or explicitly authorized operation.

C06 executes its own authorized native typing acceptance with C01's measurement method before completion. C07 owns the integrated audit and reuses these receipts when they apply to the final revision, rerunning only invalidated scenarios. Do not require C07 to complete before C06 can produce its own evidence.
