# C04 — Interaction correctness and retained edit sessions: implementation plan

## Dependency order
C01 baseline/contracts. Serialize C04 -> C05 -> C06 because all own App.tsx.
Implementation was authorized by the user's “设计没有问题，继续”. C01 contract
evidence enabled this work; the full performance matrix remains explicitly
deferred and is not marked complete by C04's checks.

## Steps
- [x] Confirm C01 route/contracts and existing test facilities; reproduce out-of-order completion and typing-during-save with deferred IPC.
- [x] Implement M1/M2 state ownership and M3 operation states without new persistence or automatic mutation retries.
- [x] Run AC1-AC3 executable behavior checks, frontend TypeScript/build and relevant Python fixture-policy tests; record native evidence limits for AC4.
- [x] Hand the reviewed session seam to C05 after C04 source fixes and independent checks pass.

## Owned files and coordination
App.tsx; narrowly extracted frontend request/session modules; relevant i18n.ts and styles.css entries; focused behavioral tests. No Rust store/backend edits. Coordinate any shared IPC declarations with C01/C03 before editing.

## Validation
Each AC is independently checked through its design mechanism mapping. C01 selects the executable behavior-check route; no new tooling is installed here. Run the repository-supported frontend TypeScript/build command and relevant existing Python contract checks. Do not substitute source-string assertions for request-order, editing or focus behavior. Report passed, failed, unavailable and native-UNVERIFIED evidence separately; web mocks cannot close native acceptance.

Completed evidence: [implementation review](research/implementation-review.md)
records 27/27 behavior tests, 34/34 structural/boundary tests, TypeScript/Vite
build and diff check passing. Tests execute production async functions and
state owners; root/component wiring and reload-confirmation identity were
source-reviewed. No mounted DOM/native interaction is claimed.

## Rollback
Revert only this task's source/tests coherently, preserving unrelated work. No durable migration exists. Runtime restart loses in-memory edits: obtain explicit handling of unsaved content before disruptive restart.

## Exit
C04 AC1-AC4 are closed at the explicitly documented production-function/state-owner
and source-wiring scope. Native IME, focus, screen-reader, app-exit and timing
remain UNVERIFIED; later-task native/performance criteria remain open. The Node
retention observation records payload/heap without a native memory claim or
silent eviction. No implementation, commit, archive or release scope is broadened.
New dependencies still require explicit authorization.
