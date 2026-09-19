# C03: Query execution, pagination, and indexed performance

## Goal

Make connected fixture reads preserve official Basic Memory query meaning and ranking while bounding work and keeping runtime/control operations responsive. The client must not replace the official index with full-vault scans or fabricate semantic behavior.

## Background and dependency

Depends on C02 `09-19-c02-engine-read-session`, including its real desktop read
owner, and C01's frozen contracts and measurement manifest. Implementation was
approved on 2026-09-19. Code and representative evidence are now reviewed;
AC5 remains partial under the subsequent representative-first decision.

At the planning baseline, fixture search scanned all note bodies, kept identifier
order, and sliced afterward (baseline `library.rs:3759`, `:4667`). That code is
test-only, not a measured production bottleneck. The implemented adapter delegates
ranking and paging to the official index. Parent research records the original
source anchors; `research/implementation-review.md` records current behavior.

## Requirements

- **R1: Upstream query fidelity.** Preserve result kind, identity, order, score meaning, exactness of total, and has_more. Only send mode/filter/compact parameters supported by the selected profile. Unavailable semantic retrieval remains unavailable.
- **R2: Bounded demand-driven execution.** Query only requested pages and selected-note bodies. Default search/list view page size is 50; coordinate a maximum retained view of three pages/150 rows with C06. Bound pending/in-flight read work and preserve serviceable runtime controls during slow reads.
- **R3: Complete identity and freshness.** Every request includes profile, session generation, explicit route, operation, normalized query or note identifier, mode, all filters, and page/cursor. Route/profile/reconnect/explicit refresh invalidates older identities. No persistent search-result cache by default; external edit/delete/rename is reflected on explicit refresh after engine indexing.
- **R4: Accurate structured adaptation.** Decode evidenced envelopes at one trusted boundary. JSON-requested guidance text is not successful empty search. Large exact-note reads have their actual full-body contract; page_size must not imply body chunking. Scalar limits are checked once at the trusted boundary.
- **R5: Measurable performance and relevance.** Compare the same profile's direct engine and desktop adapter under C01's fixed protocol, preserving relevance and payload semantics. No speedup claim against EmptyLibrary, fake semantic scores, or unmeasured fixture scans.

## Acceptance criteria

| Mapping | Observable acceptance |
|---|---|
| R1 -> AC1 | Immutable-corpus adapter results have the same ordered identities and scores as direct upstream calls, including a best-ranked identifier that sorts last alphabetically. Page traversal preserves has_more and total_is_exact; unknown zero totals are not represented as definitive zero matches. Scores above 1 are preserved where upstream returns them. |
| R2 -> AC2 | Default requested page size is 50 and first-page result discovery makes no per-hit full-note requests. Pending/in-flight work respects the documented owner-local cap; overflow has a recoverable typed outcome. A delayed read leaves runtime-state/control access available. C06 can enforce 150 retained rows without forcing the backend to fetch additional pages. |
| R3 -> AC3 | Requests differing in any identity dimension cannot share stale results. Explicit refresh creates a new request generation and rereads the engine. Controlled external fixture edit, rename, and deletion become visible after observed index readiness plus refresh. No persistent result cache exists unless separately justified by evidence and reviewed invalidation rules. |
| R4 -> AC4 | Profile payload tests cover structured success, upstream isError, guidance text despite JSON request, malformed output, unsupported fields/mode, and full exact-note body. Release never receives main-preview-only compact/temporal fields. Unsupported features are not silently downgraded or mislabelled. |
| R5 -> AC5 | Paired per-profile measurements conform to C01/parent D4: fixed 100/1,000/10,000 note fixtures, five cold and 30 warm samples, p50/p95, payload bytes, peak RSS, command counts, and independently labelled queries. Proposed adapter warm-p95 overhead is <= max(50 ms, 25% of direct-engine warm p95), frozen in C01; failures remain open rather than changing thresholds after measurement. |

## Scope limits

Fixture read-only execution only. No direct Basic Memory database, second SQLite/FTS/vector index, fixture-library promotion, engine pin bump, model/provider activation, note writes, real-vault access, or general cache infrastructure. C04 owns UI consumer stale-response guards; C06 owns retained view data and rendering; C07 owns final integrated performance acceptance. Child measurements supply evidence and do not imply native UI acceptance.
