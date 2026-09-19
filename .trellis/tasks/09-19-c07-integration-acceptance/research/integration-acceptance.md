# C07 integrated implementation and representative acceptance

Recorded 2026-09-19 on local `main`, base `251c74bca81c335dbfc1ea5bcc4bca4e7e9aef21`.
This records the pre-delivery measurement snapshot. Verdict: **scoped code and representative
evidence pass; full acceptance remains PARTIAL; do not release**.

The user approved implementation, then chose code and representative benchmarks
first. The complete 428-cell matrix is deliberately deferred. Native operation
was not authorized or performed. Neither that deferral nor this report changes
the frozen thresholds, engine pins or historical product gates.

## Result and ownership

The generated-fixture path now connects the desktop's actual `dispatch_host`
to an owned official MCP session. Indexing/ranking stay in Basic Memory. Typed
results retain signed scores, distinct row/owning-note identities, upstream
order, page/count exactness and profile-specific capabilities. Eight retained
RPC permits bound work after a consumer disappears; controls do not wait on a
host-wide read lock. UUID selection and exact-target validation prevent stale
permalink fallback from opening a different note.

The frontend uses shared latest-intent guards and revision-aware edit sessions.
Search and the source reader are adjacent; secondary content is demand-loaded.
Search retains three page bodies/150 hits with actual cursor history. Explicit
diagnostic snapshots replace repeated classification of hidden large bodies.
Saved acknowledgements cannot erase newer text. This protects the existing
in-memory drafts; it does not enable engine writes or promise restart persistence.

Code owners and independent reviewers contributed distinct evidence: C01/C03
harness owner `c01_baseline`, Rust owner `c02_session`, frontend specialist
`c01_frontend_seam` (`frontend_ui_engineer`), query reviewer `c01_check` and
frontend reviewer `c04_check`. Their child implementation reviews retain findings
and fixes; planning GO in `.trellis/reviews/` is not implementation acceptance.

## Authoritative receipts and revision

`execution/evidence/c03-representative-summary.json` identifies all ten final
receipts, both immutable profile SHAs and every source fingerprint. The accepted
desktop binary SHA-256 is
`67dabbe272e5fa193ab51751b67c9281687879b77a53c742df75257c3e6095ea`.
The C03 harness hash is
`cfb82b7f5396e1d7ad377c58263cf8c4195a59d26f155638f0c58896a5fdb7e9`.
Final records report unchanged source, binary and canonical fixture bytes during
measurement. The independent C03 reviewer checked all ten receipts against that binary
and 13 source hashes, recomputed all representative numbers below and reviewed
this report's parent/child acceptance map. No remaining scoped finding was found.

| Profile | Current receipts under `execution/evidence/` |
|---|---|
| release, `c0bd87c6` | `c03-release-functional-v4.json`, `c03-release-freshness-v4.json`, `c03-release-literal-00.measure.json`, `c03-release-large-1mib-source.json`, `c03-release-large-5mib-source.json` |
| main-preview, `3452c821` | `c03-main-preview-functional-v4.json`, `c03-main-preview-freshness-v4.json`, `c03-main-preview-literal-00.measure.json`, `c03-main-preview-large-1mib-source.json`, `c03-main-preview-large-5mib-source.json` |

Both profiles pass root/leaf directory selection, UUID note read, search, grouped
context and activity through production dispatch, with one MCP data call per
operation and normal owned shutdown. Each profile's 30 entity-filtered synthetic
query labels has direct/adapter recall@10 of 1.0 in literal, Chinese-mixed and
paraphrase categories. This establishes parity for those labels, not general
Chinese relevance or enabled semantic retrieval. Mixed search kinds, above-one
scores and approximate-zero totals have source-shaped unit coverage; live label
searches do not cover those cases. Wrong routes, stale sessions and malformed
results are checked separately by typed Rust/consumer regressions.

External edit/rename/delete tests use separate disposable sandboxes and restore
their original bytes. Three repeated index observations do not prove a snapshot.
The release edit refresh observed score delta `-0.014787493529005769` while ordered
identity/count semantics agreed. Only the three mutable refresh pairs allow this
recorded score difference. Immutable functional/timed pairs still require exact
scores, order and identity. Earlier failed freshness reports remain preserved.
Context excludes only generated timestamp/timeframe metadata from equality;
activity retains unknown count/continuation as null.

For each profile, 1 MiB and 5 MiB reads deliver exactly 1,048,576 and 5,242,880
UTF-8 bytes respectively, with direct = adapter = physical generated source.
These four one-off source checks are not large-note timing acceptance or a
universal claim about file encodings, editor normalization or disk writes.

## Representative measurements

The scenario is `notes-100/literal-00`, separately measured per profile. Every
lane has five fresh-owner cold and thirty primed warm samples: 35 pairs/profile,
70 pairs/140 measurements total. All pairs pass exact immutable semantics and
one-MCP-call checks. Startup/index readiness/shutdown are outside query timing;
OS disk cache and shared host load are uncontrolled. Nearest-rank p95 and median
follow frozen C01 rules. Raw samples and commands remain in each receipt.

| Profile / lane | Cold median / p95 (ms), n=5 | Warm median / p95 (ms), n=30 |
|---|---:|---:|
| release direct | 790.7316 / 820.7610 | 48.0508 / 74.1624 |
| release adapter | 791.0327 / 806.1926 | 56.1228 / 63.9069 |
| main-preview direct | 651.5725 / 674.0308 | 19.8303 / 24.2530 |
| main-preview adapter | 665.0372 / 682.9576 | 19.2817 / 23.0867 |

Warm p95 overhead is **-10.2555 ms** for release and **-1.1663 ms** for preview;
both pass the unchanged `max(50 ms, 25% direct warm p95)` allowance (50 ms here).
This matched clock is Python JSONL write/flush through parsed reply for both
lanes. Direct still traverses the typed driver and owned MCP transport; it is
not engine-internal time. It differs from historical C01 `Probe.raw`, so strict
C01 clock conformance remains false. Do not pool the two datasets or claim an
official engine speedup or a meaningful native UI latency improvement.

| Profile | Compact response bytes, direct / adapter | JSONL reply byte range, direct / adapter | Maximum driver peak bytes, direct / adapter |
|---|---:|---|---:|
| release | 8,938 / 1,153 | 9,208–9,220 / 1,423–1,436 | 18,190,336 / 18,227,200 |
| main-preview | 9,040 / 1,164 | 9,310–9,322 / 1,434–1,447 | 18,219,008 / 18,219,008 |

The direct FastMCP envelope includes duplicated text/structured content; the
adapter projects the DTO. Smaller serialization/parse work is not faster engine
search. PeakWorkingSetSize covers the desktop driver only, not engine descendants
or renderer. Host receipts record Windows 11 build 26200, 24 logical CPUs and
Python 3.14.7; they are not a hardware-controlled benchmark. Exact CPU/RAM model
metadata is unavailable. C01's differently named engine RSS is launcher-only.

C01 coverage remains **280 valid samples / 16 of 428 phase cells**, leaving
**412 deferred cells**. Its six accepted v3 reports include both profiles' small
reads/searches and large reads. C03's 140 paired samples do not fill C01 cells or
close its full matrix. The initial C01 release 5 MiB initialization EOF (zero
samples) and separate passing rerun remain visible; its cause is unresolved.

## Verification and command scope

| Check | Final result and boundary |
|---|---|
| `python -m unittest tests.test_client_baseline tests.test_client_session tests.test_client_queries tests.test_desktop_shell -q` | **68 PASS**: 17 baseline + 3 session + 14 query + 34 structural tests. Root ran the final combined check after timing. |
| `cargo test -p bmdock-app --locked --offline` | **233 PASS**, one deliberate subprocess helper ignored by the ordinary runner and exercised by its parent regression; final Rust owner/reviewer evidence. |
| `npm --prefix apps/bmdock-desktop run test:behavior` | **43 PASS**, production-function and SSR; independently rerun by frontend reviewer. |
| `npm --prefix apps/bmdock-desktop run build` | **PASS**, TypeScript/Vite, 39 modules; installed dependencies only. |
| Rust format / focused developer probe | **PASS**; six policy tests plus focused Clippy/build. Probe results do not substitute for desktop evidence. |
| Full desktop Clippy with `-D warnings` | **FAIL, inherited**: 103 emissions / 102 unique ipc.rs spans matched to baseline; no new warning identified. |
| `python -m scripts.tasks unit` / product gate | **FAIL, inherited**: `A later task was completed before G0`; G0–G7 unpassed. No guard/status rewrite. |
| Frontend lint | No dedicated lint command configured; no lint pass claimed. |

Focused local Python tempfile writes and Vite/esbuild spawning required narrow
approved retries outside the restricted token. No global ACL/configuration was
changed. The final combined Python run passes; the earlier restricted-token
errors are environmental execution failures, not silently removed tests.
Task validation and final whitespace results are recorded in the delivery checks
below. They establish artifact structure, not product acceptance.

Executed query commands use `python -m scripts.client_queries` with modes
`functional`, `freshness`, `source`, or `measure --scenario literal-00`, exact
C01-v3 fixture paths and fresh C03 output paths. Every raw receipt records the
literal invocation. `09-19-c03-query-execution/research/paired-query-evidence.md`
explains the harness and preserved failed attempts; never overwrite old receipts
when reproducing a run.

## Parent and child acceptance map

| Parent criterion | Outcome | Evidence |
|---|---|---|
| AC1 contracts/baseline | **PARTIAL** | C01 review, v3 manifest, reproducibility and coverage; 412 cells deferred, RSS scope limited. |
| AC2 actual official reads | **PASS, typed/headless generated fixture** | Both functional-v4/source reports; Rust route/error/session regressions. No native GUI claim. |
| AC3 faithful responsive queries | **PASS at stated live/unit scope** | Final query receipts, exact immutable paging/scores, signed/unknown shape tests, retained cap and nonblocking-control regressions. |
| AC4 latest intent/drafts | **PASS, production functions** | C04/C06 reviews and final 43 behavior/SSR tests; native IME remains distinct. |
| AC5 responsive accessible layout | **PARTIAL** | C05 source/SSR/state review; required viewports/zoom/native focus not observed. |
| AC6 demand-driven actionable bounded results | **PARTIAL** | One ordinary search/zero inspector and demand-only secondary calls verified by App-used transport functions; 3/150 retention verified; mounted DOM/activation remains unverified. |
| AC7 integration/performance | **PARTIAL** | Current report and matched representative budget pass; historical-clock conformance/full matrix/UI p95/native evidence missing. |

| Child | Accepted result and remaining scope |
|---|---|
| C01 | AC1/3/4 pass; AC2 partial. `research/implementation-review.md`, coverage JSON and frozen v3. |
| C02 | Scoped AC1–5 pass for owned generated-fixture read lifecycle; C03 final receipts supersede old C02-v2 binary claims for current integration. |
| C03 | AC1–4 pass at declared live/unit scope; AC5 partial. `research/implementation-review.md`, `paired-query-evidence.md`, summary/raw receipts. |
| C04 | AC1–4 pass at production-function/source scope; `research/implementation-review.md`. No native event/IME claim. |
| C05 | AC4 state/SSR pass; AC1/2/3/5 partial. C06 restores the transient DTO build failure; native layout/focus still missing. |
| C06 | AC2 demand admission passes at function/source scope; AC1/3/4/5 partial for native/DOM/integrated performance clauses. Independent review records 43 tests/build. |
| C07 | AC1 typed/headless journey and AC5 evidence mapping pass; AC2/3 partial; AC4 UNVERIFIED. Full task stays open. |

Required shell startup reads (`get_capabilities`, `get_runtime_state`,
`list_projects`) are retained. Zero optional diagnostics/catalog/inspector calls
does not mean zero routing calls. Closed-diagnostic SSR checks cover thirty
revisions each of 1 MiB/5 MiB text with a throwing classifier and no duplicated
preview; they do not measure keystroke-to-presented-frame latency.

## Open acceptance and delivery checks

Native 1200x800/720x480, 200% text zoom, keyboard/screen-reader/Chinese IME,
mounted row count, and the frozen thirty-edit presented-frame p95 50/100 ms
limits remain **UNVERIFIED**. Meaningful UI candidate p95 <=110% baseline is
unmeasured. Engine-total RSS, OS process-tree forced cleanup/sleep behavior and
unrelated original product gates are not closed by normal fixture shutdown.

At the verification checkpoint all eight task statuses remained `in_progress`.
The later user instruction authorizes commit, push and archival of completed
tasks; independent review identifies C02/C04 as eligible, with all other tasks
remaining open. No dependency download, engine pin change, real-vault product
access or provider/cloud activation occurred.
Six preexisting untracked files under `09-12-bmdock-product-completion` remain
outside this change. The next bounded acceptance work is the deferred matrix or
explicitly authorized native checks, with the existing protocols and budgets.

Pre-archive delivery checks: all eight `task.py validate` invocations pass their sixteen
context manifests (104 entries). Parent/child links, seven child approvals and
`in_progress` states agree. The six preexisting files remain present. Final
`git diff --check` passes. These checks do not change the partial acceptance above.

## Portable evidence packaging

The user chose to keep the ten large raw C01/C03 JSON receipts local and ignore
them in Git. `execution/evidence/client-raw-receipts-summary.json` preserves their
identities, hashes and bounded outcomes; compact exact query replay fixtures
support Rust tests in a fresh clone. Raw reports were not rewritten or deleted.
Consequently their full contents are not available from Git alone; rerunning the
documented capture commands is required to recreate fresh evidence.

The delivery-only Rust change redirects test replay inputs to these compact
fixtures. Production code is unchanged, but the whole-file source hash differs
from the measured snapshot. Packaging verification and the runtime-prefix/hash
comparison are recorded in `research/delivery-review.md`; do not assert every
historical whole-file fingerprint equals the delivered revision.

## Authorized closeout

Work commits are `3986f64`, `1e64708`, `bd4c6da` and `b75bd15`, covering baseline,
coordinated desktop code, query/session verification and Trellis documents.
C02/C04 were archived with `task.py archive --no-commit` under
`.trellis/tasks/archive/2026-09/`; their relocation and this closeout are committed
separately. Their parent links remain intact. Parent/C01/C03/C05/C06/C07 stay
open, with C07 still active. The user authorized normal push to `origin/main`;
the coordinator verifies the remote after the final journal commit.

The ten ignored raw files remain local, not deleted. Six preexisting untracked
files remain outside the commit scope. The continuation prompt is
`../09-19-bmdock-client-optimization/research/continuation-prompt.md` relative to
the C07 task directory. It covers the remaining native and full-matrix work.
