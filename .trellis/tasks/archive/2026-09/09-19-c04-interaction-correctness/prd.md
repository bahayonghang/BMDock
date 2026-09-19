# C04 — Interaction correctness and retained edit sessions

## Goal and dependencies
C01 baseline/contracts. Serialize C04 -> C05 -> C06 because all own App.tsx.
The user approved implementation with “设计没有问题，继续” and subsequently
prioritized code plus representative benchmarks, leaving the full matrix pending.
C04 implementation and independent review are complete within the evidence scope below.

## Requirements
- R1: Only current request identity can update note, detail, search, page or profile state. Include profile, session generation, explicit route, query/confirmed filters, page/cursor and identifier as applicable.
- R2: Preserve unsaved edits across section/selection changes in the running app. Save acknowledgements must not overwrite newer typing.
- R3: Expose operation pending/error/empty states, deduplicate equivalent pending actions and retain usable content on local failure.
- R4: Preserve fixture restrictions, error categories and logical-versus-physical cancellation semantics. In-memory retention ends at app exit; do not add durable draft storage, real note writes or global configuration.

## Acceptance criteria
- [x] AC1 (R1): Deferred A/B note, query, profile and page responses in both completion orders leave only the latest identity visible; obsolete successes/errors cannot change current state or launch obsolete follow-on requests.
- [x] AC2 (R2,R4): Save revision A, type revision B, complete A: B stays visible and dirty. Selection/section navigation and return retain edits. Unsupported or failed save retains text and never claims disk persistence.
- [x] AC3 (R3,R4): Double activation emits one pending operation; pending state is announced; page/detail failure preserves unrelated loaded content and edits; timeout_unknown causes no automatic mutation retry.
- [x] AC4 (R1-R4): Executable behavioral regressions fail before the fix and pass afterward; targeted frontend build and fixture-policy checks pass. Native IME/screen-reader/app-exit evidence remains distinct.

Evidence: [independent implementation review](research/implementation-review.md)
maps each AC to production-function/state-owner tests and source-wiring review.
Final checks: 27 behavior tests, 34 structural/boundary tests, TypeScript/Vite
build and diff check passed. Native IME, focus, screen-reader, app-exit and
presented-frame evidence remain UNVERIFIED; these checkboxes do not close them.

## Authority and exclusions
Implementation was authorized by the subsequent user instruction recorded above.
Real-vault access, new persistence, dependency installation, global configuration
and UI automation remain outside this work. Native acceptance is later manual or
explicitly authorized. Rich Markdown and generalized infrastructure are outside
this task. No commit, archive, release or historical gate change is implied.
