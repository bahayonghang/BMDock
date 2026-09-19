# C04 — Interaction correctness and retained edit sessions: design

## Source evidence
- apps/bmdock-desktop/src/App.tsx:718 — note and detail requests lack active-selection identity.
- apps/bmdock-desktop/src/App.tsx:1618 and :1709 — search and captured-page responses can replace newer results.
- apps/bmdock-desktop/src/App.tsx:4456 and :4927 — seed changes reset editor text.
- apps/bmdock-desktop/src/App.tsx:5048 — draft acknowledgement replaces visible body.
- apps/bmdock-desktop/src-tauri/src/main.rs:77 and apps/bmdock-desktop/src-tauri/src/drafts.rs:58 — production uses EmptyDraftStore; save returns unsupported.

Parent research: ../09-19-bmdock-client-optimization/research/frontend-audit.md. Follow parent design.md D2/D4 and the finalized official-and-projects.md for cross-layer contracts. Source conclusions are not native acceptance.

## Mechanisms, requirements and acceptance mapping
### M1
M1 -> R1 -> AC1: A request owner validates generation and full identity before each result/error/follow-on update. Page identity includes parent query generation. Invalidation ignores completion; it does not claim backend work was cancelled.

### M2
M2 -> R2,R4 -> AC2: Session state above transient panels retains per-note text and submitted revisions during this process. Acknowledgement updates only the submitted baseline, not newer text. Explicit discard clears a session. No localStorage or filesystem persistence; unsupported store responses never create a persistence badge.

### M3
M3 -> R3,R4 -> AC3: Per-operation pending slots and local error states preserve prior content with a refreshing indication. Existing typed error/unknown-result distinctions survive mapping.

### M4
M4 -> R1-R4 -> AC4: Deferred typed-IPC tests exercise the same state/selection seam used by views, followed by targeted build and fixture-policy checks.

## Ownership
App.tsx; narrowly extracted frontend request/session modules; relevant i18n.ts and styles.css entries; focused behavioral tests. No Rust store/backend edits. Coordinate any shared IPC declarations with C01/C03 before editing.

## Unresolved evidence and implementation choices
C01 chooses the behavior-check route; C04 implements the tests. New dependencies require future authorization. Measure retained edit-session memory rather than silently evicting drafts. Native IME/screen-reader evidence is UNVERIFIED.

No generic query cache, virtualizer, docking framework or state platform is implied. Use existing tools first; future new dependencies require authorization. No mutation retry follows logical cancellation or timeout_unknown.
