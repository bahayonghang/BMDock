# C03 representative query evidence

The user authorized code and representative benchmarks first, with the complete
performance matrix retained for later acceptance. C03 AC5 remains partial. No
native UI, engine pin, provider, dependency, real vault, or global configuration
operation is part of this harness.

## Execution surface

`python -m scripts.client_queries {functional,measure,freshness,source} --fixture
<C01-v3-fixture-manifest> --output execution/evidence/c03-<new-name>.json`
uses the existing `target/debug/bmdock-app --query-driver <profile> <sandbox>`
host-only entry. The actual production desktop owner performs MCP and adapter
dispatch; Python only controls its JSONL acceptance interface. Output cannot
replace an existing report or target a non-C03 evidence path.

The canonical inputs are
`artifacts/client-baseline/c01-corpus-reproducibility-v3/<profile>-notes-100-1.fixture.json`.
The embedded measurement manifest must equal the exact canonical
`execution/evidence/c01-measurement-manifest-v3.json` fingerprint. Canonical
fixture bytes/counts are checked before and after every run.

`functional` runs the 30 independently authored C01 labels once in each lane,
the alphabetically last best-ranked `orderingprobe`, complete immutable page50
traversal, selected unsliced full-source read, semantic rejection, and the
profile-specific compact contract. It also compares root/leaf tree entries,
UUID-based selected-note reads, grouped context and recent activity through the
same production dispatcher. Context excludes only server-generated
`metadata.generated_at` and `metadata.timeframe` from cross-call equality;
both actual values and identical input arguments remain recorded.
These live searches explicitly filter
`entity_types=["entity"]`: mixed-kind, score-above-one, and unknown-zero
exactness claims require the separately identified shape tests. Text recall for
paraphrase labels does not mean semantic retrieval is enabled.

`freshness` creates a **separate disposable C03 sandbox**, never mutating the
canonical C01 fixture. It waits for observed official index readiness after
edit, rename, and deletion, then advances search request generation and reads
again. A finally block restores original bytes and mtime even when verification
fails. Restoration of physical bytes and restored index readiness are distinct
evidence fields.
Readiness records three consecutive equal observations outside timing; these
observations do not promise definitive index completion or an atomic snapshot.
Freshness compares ordered identities, count exactness, continuation and full
source after each mutation. It records both score lists and their deltas without
requiring an atomic score snapshot across consecutive calls. Exact score/order
equality remains mandatory for every immutable functional and timed pair.
The generated live freshness notes have distinct titles. Same-title stale
permalink fallback is a separate navigation regression owned by the Rust/consumer
tests; this fixture alone does not prove that collision case.

`source` is a bounded one-off direct/adapter complete-source comparison for the
separate 1 MiB/5 MiB inputs. It does not claim five-cold/30-warm timing acceptance.

## Timing boundary and validity

`measure --scenario literal-00` runs notes-100 representative query pairs.
`--scenario read-full` accepts the separate generated note scales when requested.
Each cold lane uses a freshly started desktop driver and engine: five pairs mean
ten launches. Warm samples use one primed owner. Lane order alternates for all
five cold and all 30 warm pairs. Index preparation, startup, priming, and shutdown
are outside the timed operation; OS disk cache and shared development-host load
are uncontrolled.

The budget clock is Python `perf_counter_ns` around JSONL write/flush through
receipt and JSON parsing, for both direct and adapter. The direct control still
passes through the typed host driver and owned MCP transport; it is **not
engine-internal execution time**. This differs from C01's older `Probe.raw`
boundary, so those older numbers are never pooled or substituted. Rust inner
owner/dispatch timings are recorded only as diagnostics. The unchanged proposed
budget is `adapter warm p95 - direct warm p95 <= max(50 ms, 0.25 * direct warm p95)`.
Median and nearest-rank p95 use the C01 estimator. Failing budgets remain failed.

Outer UTF-8 JSONL response bytes and compact JSON response bytes are distinct.
The direct FastMCP envelope may duplicate raw content in text and structured
blocks, whereas the adapter emits projected DTOs. A shorter adapter response can
reduce outer parsing/transport time; this is not proof of faster official search.
Every successful page/body operation must report exactly one MCP command and
both lanes must have identical upstream arguments, ordered result identities,
scores, total exactness, and continuation semantics. Full reads compare the
delivered string before storing response digests/lengths, preserving YAML/UTF-8
and separately recording any upstream CRLF normalization.

RSS samples refer only to the desktop driver process peak working set. Official
engine/descendant peak RSS is unavailable; the C01 venv-launcher measurement must
not be used as engine-total RSS. No native input/frame, renderer, or UI performance
acceptance is implied.

Reports include the profile SHA, UTC intervals, typed command, actual upstream
arguments, driver identity, startup/shutdown receipts, raw timings and payload
metrics, and before/after hashes for source and desktop binary. Source/binary
drift or corpus mutation invalidates the run. Failed attempts remain separate
reports; no automatic measured-sample retries are implemented.

## Current checks and live evidence

Offline harness suite: `python -m unittest tests.test_client_queries` — 14 tests
passed. AST parsing and CLI help passed. Final live evidence is complete for the
authorized representative scope. The authoritative concise receipt is
`execution/evidence/c03-representative-summary.json`, which lists all ten final
report paths and every final source fingerprint. Earlier functional v1/v2/v3
records are historical revisions.

| Profile | Direct warm p50 / p95 ms | Adapter warm p50 / p95 ms | p95 overhead ms | Frozen allowance ms |
|---|---:|---:|---:|---:|
| release | 48.0508 / 74.1624 | 56.1228 / 63.9069 | -10.2555 | 50.0 — passed |
| main-preview | 19.83025 / 24.2530 | 19.2817 / 23.0867 | -1.1663 | 50.0 — passed |

Each profile has 70 raw samples: five cold and 30 warm in each lane, for 35
alternating pairs. All pairs preserve ordered identity, signed score, exactness,
and continuation with one MCP command per sample. Release ran
2026-09-19 04:30:53–04:32:05 UTC; main-preview ran 04:32:17–04:33:30 UTC.
Agents held builds during these sequential timed runs; the host's other load was
not instrumented or controlled. Negative outer p95 differences are observations,
not proof of faster engine execution or a comparison between profiles.

Authoritative per-profile reports use these suffixes under
`execution/evidence/c03-<profile>-`:

- `functional-v4.json`: all 30 independent labels, text recall@10 of 1.0 for each
  ten-query category in both lanes; rank probe, complete 100-note paging, filter
  request, compact/semantic behavior, tree/UUID-selected note, context/activity.
- `freshness-v4.json`: observed edit/rename/delete after request-generation
  changes, complete source restoration and restored index observation. The
  release edit pair records a score delta of -0.014787493529005769; its visibility
  acceptance does not claim an atomic score snapshot.
- `literal-00.measure.json`: representative fixed-count paired timing above.
- `large-1mib-source.json` and `large-5mib-source.json`: one-off full-source
  comparisons preserve exactly 1,048,576 and 5,242,880 UTF-8 bytes respectively
  through both lanes, equal to these fixtures' physical source bytes. They do not
  claim all-input byte fidelity or large-note timing acceptance.

All ten final records report unchanged canonical corpus, source and binary;
both freshness fixtures report restored bytes; every final owner session closed
normally and every driver exited. No owned driver process remains running.

Final desktop binary SHA-256:
`67dabbe272e5fa193ab51751b67c9281687879b77a53c742df75257c3e6095ea`.
Final harness SHA-256:
`cfb82b7f5396e1d7ad377c58263cf8c4195a59d26f155638f0c58896a5fdb7e9`.
The summary independently compared each report's before/after fingerprints with
the then-current files. Source or binary changes after this freeze require new
affected evidence instead of relabelling these receipts.

`c03-release-freshness.json` and `c03-release-freshness-v3.json` preserve failed
attempts that demanded score equality during asynchronous index mutation.
Both restored the disposable source and left the canonical corpus unchanged.
`c03-release-freshness-v2.json` passed the earlier version; it does not establish
that three matching polls always produce a stable score snapshot. The observed
race motivated the scope correction above, approved by the root coordinator.

## Remaining acceptance

C03 AC5 is **partial**: timing covers only notes-100 literal-00 on both profiles.
The remaining labelled/scenario timings, notes-1000/notes-10000 timing, paired
large-note timing, official engine/descendant RSS, strict historical Probe.raw
clock conformance, quiet-host replication, and native renderer/input/frame
acceptance are not established. The user deferred the full matrix; no missing
cell is represented as zero or passed. C07 integrates these bounded receipts
without changing the frozen thresholds or declaring complete performance
acceptance.

Existing G0–G7 gate blockers and the pre-existing phase-order unit-wrapper
failure remain outside this child's repair scope.
