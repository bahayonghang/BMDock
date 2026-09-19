# C02 implementation plan

Status: implemented and independently reviewed for the generated-fixture headless scope. User approved implementation on 2026-09-19; C01 contracts and representative evidence supplied the dependency. Native and product gates remain separate.

## Ownership

Rust owner: desktop src-tauri `main.rs`, `supervisor.rs`, minimal read portions of `ipc.rs`/`library.rs`, and focused session/adapter module if needed. Existing pinned dependency reuse may touch desktop Cargo.toml/Cargo.lock with explicit review. Coordinate `src/ipc.ts` contract edits before C04; C04 owns UI consumers. Host-harness edits are limited to connecting production desktop session ownership to isolated integration tests. C03 follows this child and must not edit overlapping files concurrently.

Specs: backend `supervisor-state.md`, `typed-ipc-policy.md`. Inputs: parent backend audit and C01 finalized profile payload evidence.

Context-loading note: `typed-ipc-policy.md` currently exceeds the 32 KiB injection limit. Before implementation, use targeted `rg` and reads of its T06 error/route, T10 explicit routing, T11 read, T17 drain, and T21 query sections plus `supervisor-state.md`; a truncated injected prefix is not the complete governing contract. Do not enlarge context limits globally.

## Ordered work

1. Verify C01 evidence, actual per-profile list/read payloads, and validated sandbox launch conventions. Add focused behavior tests for unavailable vs connected-empty, session identity, and pending-I/O state access.
2. Implement minimal session-owned transport/process lifecycle and asynchronous typed read dispatch. Keep locks short, connect production wiring to generated fixture, preserve denied write/private/raw-tool paths. Review manifest changes; setup remains the only repository network installation step.
3. Test profile-specific decoding and production dispatch against seeded fixture data: nested/CJK read, explicit frontmatter-inclusive unsliced source, YAML/UTF-8/CRLF comparisons, child reuse, error/malformed payload, disconnection and stale generation. Preserve delivered text and report upstream normalization separately. Use controlled completion instead of flaky sleeps for concurrency tests.
4. Run real fixture handshake/close separately for both profiles and appropriate focused/repository checks. Update specs for new ownership and its distinction from T17 drain. Unit fakes alone do not complete this child.
5. Review AC1-AC5 evidence and hand C03/C04 finalized session/identity contracts. Preserve unrelated dirt; no commit/archive/push/release-gate mutation without authority.

## Validation

Start with focused implemented tests, expected commands `cargo test -p bmdock-app engine_session --locked`, `cargo test -p bmdock-app supervisor --locked`, and actual typed-adapter test names. Then run `cargo test -p bmdock-app --locked`, `cargo clippy -p bmdock-app --all-targets --locked -- -D warnings`, and `python -m scripts.tasks unit`, using active RTK rules. Run frontend typecheck/build when shared DTOs change. Record the real new isolated integration command before acceptance; do not invent a command or count an unexecuted test as passed.

The existing `just contract` / `just contract-main` verify the independent probe if shared harness changes affect it; they cannot substitute for desktop-session integration.

## Evidence checklist and rollback

- [x] AC1: each profile's production dispatch returns known fixture data and reuses child/generation across reads.
- [x] AC2: distinct connection/empty/error states and wrong-route/profile/generation behavior.
- [x] AC3: controlled pending I/O leaves runtime-state access available; actual close receipts and stale-session rejection.
- [x] AC4: structured success/error/malformed/timeout tests and continued mutation/private-route denial.
- [x] AC5: exact environment/command/SHA records, updated specs/truth matrix, explicit native UNVERIFIED limits.

Evidence: `research/implementation-review.md`, both final v2 live receipts and
`.trellis/spec/bmdock-probe/backend/engine-read-session.md`. Full Clippy and the
historical G0 ordering gate remain failed as recorded; these checks do not
establish native acceptance or release readiness.

Before wiring, keep changes confined to focused owner/tests. If integration fails, restore only this child's production adapter/startup to unavailable, stop only its owned generated process, retain evidence, and preserve all unrelated changes. No user-data migration or global cleanup is part of rollback.
