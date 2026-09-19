# C03 implementation review

Independent reviewer: `c01_check`, with a bounded contract preflight from
`backend_audit`. The official engine remains the sole query/index/ranking owner.

## Fixed findings

- A dropped read consumer could release the eight-request admission permit
  while the upstream request still ran. The owner now retains the permit in its
  bounded task until real completion/deadline/retirement; abort/cap regression
  passes, and runtime controls remain serviceable.
- Retained tree permalinks could fall through upstream title lookup after
  deletion and return another note. Connected leaves now provide stable note
  UUIDs; non-UUID exact reads additionally reject a different returned permalink
  and path. Tree and search activation use UUID targets while row identity stays
  separate. This intentionally excludes upstream title-fallback reads.
- The old fixture guard rejected supported filter-only queries. The official
  boundary now sends query null with a real supported filter; fixture behavior
  remains unchanged. Query syntax/modes and profile-only fields stay explicit.
- `json!` would clone the structured payload despite a move comment. The owner
  now moves it into a map and drops the SDK text copy before decoding. Full
  source remains unsliced; excerpts alone have a 240-Unicode-scalar bound.
- New enum-size Clippy warnings were fixed with boxing without changing wire
  shapes. No broad legacy lint suppression or cleanup was added.

## Verification and scope

The owner ran 233 passing Rust tests with one deliberate subprocess helper
ignored by ordinary discovery (invoked by its parent regression). The reviewer
independently ran 32 focused query/session tests and audited the final source.
Full app Clippy still emits 103 diagnostics at 102 unique inherited `ipc.rs`
spans; every primary span was matched verbatim to baseline `251c74b`. No new
query/driver/session/main diagnostic remains. Formatting/build checks pass.

Both final `c03-{release,main-preview}-functional-v4.json` reports exercise
production dispatch through tree, UUID selection, full source, search, context,
activity and actual normal close. Thirty fixed label searches, rank order and
scores, the alphabetically-last strongest hit, page50 traversal and filter-only
contracts match direct owner calls. Context groups retain entity/observation/
relation shape; dynamic upstream generation/timeframe metadata is not treated
as an immutable value. Activity unknown count/continuation stays null. Each
journey data operation uses one MCP call, without per-hit body fetches.

Live label searches explicitly filter entity rows. Search observation/relation,
above-one scores and approximate-zero cases retain source-shaped synthetic
decoder tests; those tests are not live semantic-model evidence.

Both final `c03-{release,main-preview}-freshness-v4.json` reports verify edit,
rename and deletion visibility after observed indexing plus a new consumer
generation in a separate disposable fixture, with original physical bytes
restored. Earlier release score-drift failures remain retained. Three stable
polls did not prove a snapshot: mutable refresh comparisons retain score vectors
and deltas while requiring exact ordered identity/kind/page/count semantics.
Only the three mutation refresh pairs use that distinction. Immutable functional
and timed comparisons still require exact scores; no threshold was relaxed.

## Acceptance

AC1–AC4 are supported for the generated-fixture typed/headless scope. AC5 remains
partial: the user deferred the full matrix, engine-total RSS is unverified,
and representative paired outer-driver timing differs from C01 Probe.raw.
Final paired/source receipts and their numeric outcome are recorded by the
authoritative `paired-query-evidence.md` and C07 integration report, not inferred
from this source review. Native GUI/input/focus and release gates remain separate.

Final independent receipt review checked all ten authoritative reports against
the current binary, 13 source hashes and canonical v3 manifest fingerprint.
It recomputed 140 raw timing samples and immutable result parity, verified four
large-source receipts, mutation-only score-difference scope and normal shutdown.
Warm direct/adapter p95 is 74.1624/63.9069 ms for release and 24.2530/23.0867 ms
for main-preview. Both representative cases pass the unchanged 50 ms allowance;
AC5 remains partial for the limits above.
