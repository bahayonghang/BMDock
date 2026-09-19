# C03 implementation plan

Status: implementation reviewed, full performance acceptance partial. The user
approved implementation on 2026-09-19 and subsequently selected representative
benchmarks first. C02's scoped read owner and C01's frozen v3 contracts supplied
the implementation handoff; deferred criteria remain open.

## Ownership and coordination

Own the C02-created read adapter/session execution modules, query-specific `src-tauri/src/ipc.rs`/`library.rs` definitions, and focused integration/benchmark harness hooks. Coordinate shared `src/ipc.ts` DTO changes with C04/C06; frontend generation guards belong to C04 and retained rows to C06. C07 owns final combined performance report. Do not overlap C02 Rust edits before its contract handoff.

Evidence inputs: parent backend audit, official-and-projects research, parent design D2/D4, finalized C01 payload/measurement inventory. Update the specific typed IPC/session specs changed by implementation.

Context-loading note: `typed-ipc-policy.md` currently exceeds the 32 KiB injection limit. Read targeted T10/T11 route/read, T19/T20 graph, T21 search, T22 context/activity, T23 inspector, and T24 benchmark sections directly before implementation. The truncated injected prefix is not complete evidence; do not change global context limits to work around it.

## Ordered work

1. Replay captured profile payloads and direct-engine ordered-result tests through C02's adapter. Freeze typed identity, page/count/score semantics, profile-supported parameters, and exact-note body behavior. Add regression cases for guidance-text output, approximate-zero total with more results, score above one, and alphabetically last best match.
2. Implement faithful query projection and default page50 without full-body-per-row reads. Validate scalar limits once at the trust boundary. Bind request identity to all route/session/query/filter/page dimensions. Preserve errors and unavailable semantic features; do not promote fixtures or open engine databases.
3. Add bounded session-owner admission and cancellation/retirement behavior. Keep controls serviceable during slow reads. Add singleflight only if C01 traces justify it; use no persistent result cache. Test mismatched keys, capacity/overflow, cancelled consumers, and session replacement with controlled completions.
4. Exercise index-ready external edit/rename/delete plus refresh in generated fixtures; test immutable paging, selected full-note read, nested/CJK identities, profile-only compact/temporal parameters. Hand C04/C06 exact identity/outcome contracts and page/retention boundaries.
5. Run focused checks and paired C01 measurements separately per profile, preserve raw outcomes, and compare to frozen budgets. Update specs/evidence, review AC1-AC5, and hand C07 results. No threshold, dataset, model, profile pin, or scope expansion to make a failing run pass.

## Validation commands and evidence

Use actual implemented module/test names under `cargo test -p bmdock-app --locked`; first target query-adapter and session-admission tests, then the full desktop Rust suite. Run `cargo clippy -p bmdock-app --all-targets --locked -- -D warnings` and `python -m scripts.tasks unit` if relevant harness/wiring changed, with active RTK rules. Run frontend build/typecheck when DTOs changed. The C01/C02 isolated harness command must be recorded verbatim in evidence rather than assumed to exist.

Do not run setup/network installation outside authorized setup. Existing G0 contract tests verify the probe and are supplementary only when shared code changed. UI/native operation requires its own authority and cannot be inferred from backend benchmarks.

## Acceptance checklist

- [x] AC1: identical direct-engine and adapter ordered identities/scores; exactness/has_more correctly preserved; full immutable paging.
- [x] AC2: page50, no per-row body-fetch amplification, recorded queue/in-flight cap and overflow outcome, controls serviceable while read pending.
- [x] AC3: complete key isolation; refresh generation; post-index edit/delete/rename visibility; no persistent result cache.
- [x] AC4: profile-specific parameters, structured/guidance/error outcomes, full-note behavior and no fabricated semantic fallback.
- [ ] AC5: paired five-cold/30-warm measurements at all corpus sizes/profile cases, raw p50/p95/payload/RSS/counts, frozen overhead comparison and unresolved failures honestly retained.

AC1–AC4 evidence: `research/implementation-review.md`, final per-profile
functional-v4 and freshness-v4 reports, and the focused query/session regressions.
Mixed search kinds, above-one scores and unknown totals have source-shaped unit
coverage; live label searches are entity-filtered. Mutation freshness proves
visibility after observed indexing and refresh, not an atomic score snapshot.
AC5 remains partial under the user's representative-first decision; the matched
query-driver clock is not historical C01 Probe.raw clock conformance.

## Rollback

Revert this child's scheduler/projection changes to the last verified C02 read adapter without touching C02 ownership, typed admission, unrelated work, engine pins, or evidence. If the affected operation becomes unavailable, show unavailable rather than switching to a fixture result. There is no client database/index migration to roll back.
