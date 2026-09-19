# C06 — Search-to-reader and bounded frontend work

## Goal and dependencies
C03 backend query contract, C04 interaction correctness and C05 layout. The user
approved implementation and prioritized code plus representative benchmarks.
Reviewed frontend implementation is complete at production-function/source scope;
native and performance acceptance remains PARTIAL as recorded below.

## Requirements
- R1: Ordinary results open the primary reader via the same selection flow as the list. Expose entity_type, note_type, observation and other filters only with meanings confirmed by the engine contract.
- R2: Ordinary workspace startup issues zero diagnostic/catalog/inspector requests; ordinary search issues zero inspector requests until inspection is explicitly opened.
- R3: Use default search page size 50 with at most three retained pages and 150 result rows under the confirmed C03 contract. Query/filter/profile/route changes invalidate old pages through C04.
- R4: Measure engine and frontend latency separately using C01/C03 baselines; remove informational full-body scanning and duplicate preview rendering from ordinary large-note typing. No blanket cache/virtualizer adoption or unmeasured performance claim; preserve errors, escaping and fixture policy.

## Acceptance criteria
- [ ] AC1 (R1,R4): Mouse/keyboard result activation opens the same primary note as list activation. DTO-supported title/snippet and filters have tested semantics; unsupported entity_type/note_type/observation choices remain unavailable rather than aliased.
- [x] AC2 (R2): Command trace records zero diagnostic/catalog/inspector calls on ordinary startup; one ordinary query records its search command and zero inspect_search calls. Opening a diagnostic surface loads only its requested data.
- [ ] AC3 (R3): Search defaults to 50 under the accepted C03 contract; no more than three pages/150 result rows are retained/rendered. Crossing this bound evicts an old non-active page and preserves logical navigation/focus. Query changes while loading reject old pages; failed loading preserves current usable results.
- [ ] AC4 (R4): Identical named fixtures produce recorded p50/p95 query-to-visible-result, request count, DOM row count and backend query duration separately. Informational full-body scans and duplicate previews do not rerun for each ordinary edit while their details are closed; generation-bound deferred results cannot replace a newer revision. Under the C01 30-edit protocol, native input-to-presented-frame p95 is <=50 ms for 1 MiB and <=100 ms for 5 MiB, or remains explicitly incomplete if unmeasured. Accepted C01/C03 query budgets also pass before claiming optimization complete.
- [ ] AC5 (R1-R4): Targeted behavior, frontend compile/build and query/fixture checks pass. Mock/web timings are not claimed as native acceptance.

AC2 is verified at actual production-function/source scope for optional
workbench loads: activity/resources/prompts/tools/CLI/API audit/inspector.
Required shell get_capabilities/get_runtime_state/list_projects calls remain;
no mounted/native startup trace is claimed. AC1/AC3/AC4/AC5 remain **PARTIAL**.
The final independent checkpoint passed 43/43 behavior/SSR tests, 34/34 source
checks and a TypeScript/Vite build of 39 modules. See
[independent review](research/implementation-review.md) for clause-level limits.
Native focus/keyboard/IME, measured DOM rows, query-to-paint and 30-edit p95
timings are UNVERIFIED; final integrated query/fixture acceptance remains with
the parent/C07 checkpoint. No speedup is derived from EmptyLibrary or host tests.

## Authority and exclusions
Product implementation was authorized by the subsequent user instruction.
The full performance matrix is explicitly deferred while representative
measurements proceed. No UI operation occurred in this review; native acceptance
requires manual or explicitly authorized evidence. Real-vault access, new
persistence/dependencies, global configuration, rich Markdown and generalized
infrastructure remain outside scope. This handoff does not complete the task or
authorize commit/archive/release.
