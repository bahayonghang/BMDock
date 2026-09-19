# C05 — Focused workspace and accessible plain-text reading: implementation plan

## Dependency order
C04 session/request ownership. Serialize C04 -> C05 -> C06; no concurrent App.tsx
writers. The user approved implementation and prioritized code plus representative
benchmarks. C05's code handoff may proceed to C06 while native acceptance remains
pending; no native result or task completion is inferred from that handoff.

## Steps
- [x] Confirm completed C04 session seam; group destinations without deleting diagnostics.
- [x] Implement M1 regions and M2/M3 styles, preserving M4 safe source behavior.
- [x] Execute bounded behavior/SSR/content-safety checks and frontend build; record passing, failed and native-unavailable evidence separately.
- [ ] Obtain native viewport/zoom/keyboard/focus/IME/screen-reader acceptance through manual or explicitly authorized operation. C06 code handoff does not close this step.

## Owned files and coordination
App.tsx/view modules, styles.css, relevant i18n.ts and focused tests. Reuse C04 state ownership. No native configuration, backend, dependency or global preference edits.

## Validation
Each AC is independently checked through its design mechanism mapping. C01 selects the executable behavior-check route; no new tooling is installed here. Run the repository-supported frontend TypeScript/build command and relevant existing Python contract checks. Do not substitute source-string assertions for request-order, editing or focus behavior. Report passed, failed, unavailable and native-UNVERIFIED evidence separately; web mocks cannot close native acceptance.

[Independent review](research/implementation-review.md): 32/32 behavior/SSR,
34/34 structural/boundary checks and diff check passed. The earlier C05 owner
build passed; the final independent shared-tree build failed on concurrent C03
engine/fixture query DTO changes pending C06 integration. Do not record that
final build as green or remove the real DTO variants to conceal the failure.
AC4 has bounded source/SSR/state evidence; AC1/AC2/AC3/AC5 remain PARTIAL.

## Rollback
Revert this task's views/styles/messages/tests only, preserving C04 behavior and unrelated changes. No data migration or new persistence exists.

## Exit
C05 remains incomplete for its native/viewport/focus clauses and current query
integration build. Code review has no remaining C05-specific blockers; C06 may
continue the authorized code work and restore integrated checks. Exact layout,
zoom, focus movement, keyboard scrolling, IME and screen-reader behavior still
need actual interaction evidence. No dependency, commit, archive or release scope
is broadened. The full performance matrix remains separately deferred.
