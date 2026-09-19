# C07 implementation plan

Integrates C01-C06 under the user's representative-first decision. Owns
acceptance evidence and documentation; partial child criteria remain partial.

1. Freeze source revision, dependency versions and C01 manifest; verify all child acceptance evidence and remaining limitations. C05/C06 supply their own native acceptance before completion; reuse applicable same-revision receipts and rerun only scenarios invalidated by later integration changes.
2. Run the actual desktop read journey for both profiles and focused behavioral regressions. Recheck physical fixture identity, route denial and payload error handling.
3. Collect paired direct/adapter measurements and command/row budgets. Compute per-profile quality and timing without merging profiles.
4. Obtain native evidence only with explicit UI authorization or attributable manual user execution. Run C01's 30-edit input-to-presented-frame protocol on 1 MiB/5 MiB notes and check p95 <=50 ms/100 ms respectively; record the actual trace method, not a paint proxy. Leave unavailable native/IME/accessibility items UNVERIFIED.
5. Publish parent AC mapping, failed/skipped/passed status and bounded decision. No commit/archive/release unless separately requested.

Checks: child-specific behavioral checks; `cargo test -p bmdock-app --locked --offline`; `python -m unittest tests.test_desktop_shell -q`; frontend build; actual desktop isolated-engine harness and C01 benchmark commands recorded verbatim; `git diff --check`. Existing `just ci`, unit and product-gate status must be reported honestly rather than inferred.

Rollback only owned acceptance harness changes if needed; keep measurement records as evidence. Failed measurements trigger a bounded correction in the owning child, not threshold changes or global rewrites.

## Recorded delivery

Code integration and agreed representative measurements are complete. The
authoritative `research/integration-acceptance.md` maps all parent/child criteria,
raw receipts, final checks and remaining native/full-matrix gaps. Final combined
Python checks pass 68 tests; reviewed Rust and frontend results pass 233 and 43
respectively. Two query profiles contribute 140 exact-parity samples; four large
source cases pass. Independent query review rechecked all ten final receipts and
their current source/binary fingerprints.

- [x] AC1: typed/headless generated-fixture journey and failure boundaries.
- [ ] AC2: representative budgets pass; full protocol/performance acceptance partial.
- [ ] AC3: function/source request and retention checks pass; mounted/native evidence missing.
- [ ] AC4: native layout/IME/presented-frame evidence unverified.
- [x] AC5: final evidence map and bounded do-not-release decision published.

Task status remains `in_progress`; no archival or commit is implied.
