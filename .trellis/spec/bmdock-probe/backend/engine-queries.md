# Official query projection and bounded execution

## 1. Scope / trigger

C03 extends the C02 production session for `search_notes`, `preview_context`
(official `build_context`) and `list_activity` (official `recent_activity`).
`engine_queries.rs` owns typed argument/result adaptation; `engine_session.rs`
owns admission, MCP and lifecycle; the official engine owns indexes and ranking.
The old FixtureLibrary path remains test-only. No database, result cache,
provider activation or new renderer command is introduced.

## 2. Signatures

```rust
PreparedQuery::from_command(&IpcCommand) -> Option<Result<PreparedQuery, IpcError>>
PreparedQuery::tool_call(&self) -> serde_json::Value
QueryIdentity { session, workspace, project, operation, arguments, request_generation }
EngineHitDto { result_kind, identifier, note_identifier, title, excerpt, score,
               file_path, category, relation_type, from_entity, to_entity,
               to_name, created_at }
```

Search/context/activity arguments add `expected_session`, a consumer-owned
`request_generation` (default zero), and typed `options`. TS response unions use
literal `engine_search`, `engine_context`, or `engine_activity` to distinguish
official and fixture shapes; do not cast one shape into the other.

`SearchOptions` separates `entity_types`, `note_types`, `categories`, `tags`,
`metadata_filters`, `status`, `after_date` and mode. Preview-only `compact`,
`valid_at`, `valid_overlaps`, `time_kind` are admitted only for main-preview.
Context options are depth/max_related/timeframe/preview-only compact; activity
options are types/depth/timeframe. The typed declarations remain authoritative.

## 3. Contracts

- Preserve ordered hits and signed JSON scores, including negative FTS and
  values above one. A hit's own identifier is distinct from its owning note
  UUID; two observations from one note must not collapse into one row.
- Upstream search reports `current_page`; context/directory report `page`.
  The typed desktop responses expose `page` for all three. Explicit
  page cursors represent the next upstream page, not an immutable index snapshot.
  Default production list/query page size is 50. Preserve `total_is_exact` and
  `has_more`; zero with unknown exactness is not definitive zero matches.
- Captured recent activity is a list without continuation/count metadata.
  Its DTO has null `next_cursor`, `has_more`, `total`, and `total_is_exact`.
  Do not invent a cursor from its length.
- JSON argument identity includes the actual transmitted mode, filters, page
  size and optional profile fields, plus route/session/operation/consumer epoch.
  The backend echoes the epoch without a second UI generation cache. Preserve
  nonempty query syntax exactly. Blank text with a supported nonempty filter
  sends `query:null`; blank text without a filter is a schema error.
- Text/title/permalink modes are available. Semantic modes/min_similarity are
  unavailable under these offline profiles, never silently downgraded. Release
  never receives preview-only fields. `valid_at` and `valid_overlaps` exclude
  one another. Context does not pretend the fixture query filter exists upstream.
- Connected tree leaves supply `note_identifier` as a UUID in addition to their
  display permalink; that optional DTO field is absent for directories and
  fixture-only entries. Use the UUID when opening a leaf or search hit.
  Exact non-UUID reads reject a returned note when both its permalink and path
  differ from the requested identifier. This deliberately does not expose
  upstream title fallback; an old path must not open an unrelated note.
- The session admits at most eight retained RPCs. A spawned owner task holds the
  permit until response/deadline/retirement, even if its UI consumer disappears.
  Logical cancellation is not physical cancellation. Overflow is recoverable
  `unsupported`; there is no unbounded queue or automatic retry.
- Production decoding moves the SDK structured value into adaptation rather
  than serializing FastMCP's duplicate text copy. Full-source reads stay unsliced;
  list discovery issues no per-hit full-note reads. Hit excerpts retain at most
  240 Unicode scalar values; selected full-source bodies are not truncated.

## 4. Validation and error matrix

| Input/outcome | Required behavior |
|---|---|
| Wrong route/session, unknown fields or host-path input | Existing policy/schema boundary rejects it |
| Blank query plus actual supported filter | Query null; preserve filter meaning |
| Blank query without filter | Schema error |
| Semantic mode or preview-only option on release | Explicit unsupported, no fallback call |
| Unknown total zero plus has_more true | Preserve unknown exactness and continuation |
| Query guidance string or upstream isError/non-null structured error | Typed `unsupported`, not successful empty results |
| Missing or malformed structured query data | Typed `schema` error |
| One consumer aborts while eight RPCs run | Ninth request stays rejected until real completion/retirement |
| Old path resolves by title to another note | Reject; never relabel the unrelated body as current |
| External edit/rename/delete | Refresh after observed engine index readiness; no persistent result cache |

## 5. Good / base / bad cases

Good: official hits remain in rank order when the strongest identifier sorts
last alphabetically, and its owner UUID opens the intended note. Base: a
default page request makes one MCP call and returns 50 or fewer rows. Bad: clamp negative
scores, derive has_more from an approximate zero, deduplicate by owner UUID, or
release admission merely because the caller dropped its future.

## 6. Required tests and assertion points

Run focused `cargo test -p bmdock-app engine_ --locked --offline` and the desktop
suite. Assert signed scores, distinct hit/owner identity, approximate-zero paging,
filter-only/profile-local arguments, exact-target reads, bounded admission after
consumer abort, short-lock controls and unchanged error/lifecycle semantics.
Non-entity/above-one/unknown-total decoder fixtures are source-shaped synthetic
coverage unless a live report specifically observes those cases.

`scripts/client_queries.py` uses host-only `--query-driver <profile> <sandbox>`.
The driver admits typed prepared reads, not raw tools; input lines are bounded
at 64 KiB. Its direct lane uses the same owned MCP session, while the adapter
lane executes `main.rs::dispatch_host`. Both expose command counts, actual
arguments and receipts. The Python functional/measure/freshness/source modes
verify frozen C01 v3 corpus hashes and source/binary fingerprints. Freshness
mutations occur in a separate disposable fixture, restored in a finally block.
Observed index readiness is not an atomic score snapshot. Mutation-only refresh
checks compare ordered identities and paging while recording both score vectors
and their differences. Immutable functional and timed pairs still require exact
score equality; freshness evidence does not relax that requirement or the budget.

Representative paired timing uses the same outer Python JSONL roundtrip for
both lanes, five cold and thirty warm pairs with alternating order. It differs
from historical C01 Probe.raw; do not pool distributions. Preserve raw failed
attempts and thresholds. Engine-total RSS, native rendering, quiet-host timing
and the deferred complete matrix remain separate unmet evidence boundaries.

## 7. Wrong vs correct

Wrong: a smaller projected payload proves a faster engine or a synthetic decoder
fixture proves live semantic relevance. Correct: label outer transport/parse,
inner owner duration, payload shape and evidence origin separately, and leave
full performance acceptance open when only representative cases were run.
