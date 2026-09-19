# C02: Official engine read session and desktop connection

## Goal

Make desktop reads operate through a real, long-lived official Basic Memory session against a generated isolated fixture, with truthful connection and failure states. Replace the empty production read path without enabling user-vault access or note mutations.

## Background and dependency

C01 `09-19-c01-baseline-contracts` supplied its runtime/capability and pinned-payload
handoff. The user authorized implementation on 2026-09-19; scoped C02 code and
live receipts are reviewed in `research/implementation-review.md`. C03/C07 own
the final combined revision and remaining performance/native acceptance.

At the planning baseline, desktop installed empty adapters (`main.rs:70-79`)
and FixtureLibrary was test-only (`library.rs:3600`, `:4536`). The independent G0
probe already had an rmcp handshake/close path (`crates/bmdock-probe/src/main.rs:98-164`).
Parent `research/backend-audit.md` B01/B02/B10 records that original gap;
`research/implementation-review.md` records the implemented owned session.

## Requirements

- **R1: Real isolated reads.** Desktop typed reads use one selected pinned engine profile and generated fixture. Repeated reads reuse the connected session. No real project, global Basic Memory configuration, or private memory vault is opened.
- **R2: Truthful state and identity.** Every read belongs to a profile, explicit fixture route, and session generation. Connected-empty, unavailable, malformed result, and session failure remain distinguishable. Handshake and discovery precede connected/capability claims.
- **R3: Responsive ownership.** Engine I/O does not retain the global app-state lock or prevent runtime-state queries. One owner controls transport, child, pending reads, and bounded shutdown. Old-session completions cannot belong to a new session.
- **R4: Typed read boundary.** Decode profile-supported structured payloads captured in C01. Preserve error meaning; do not expose raw callTool, executable paths, merged profile schemas, user-vault routes, or note mutations.
- **R5: Reviewable evidence.** Separate fake, official-engine, native-window, and fault-injection evidence. Passing unit fakes do not establish live lifecycle or GUI responsiveness.

## Acceptance criteria

| Mapping | Observable acceptance |
|---|---|
| R1 -> AC1 | A seeded generated fixture is listed and a known nested UTF-8 note is read through production desktop dispatch/session wiring. Full-source reads explicitly include frontmatter and preserve the delivered string; YAML/CRLF fixture comparisons record any upstream normalization separately from client preservation. Two sequential reads reuse its generation and child. Run separately for release and main-preview. |
| R2 -> AC2 | Disconnected, starting, connected-empty, connected-with-data, failed, and stopped cases remain distinct. Wrong route/profile/session requests are rejected or classified stale. Connected requires actual handshake; discovery drift fails closed per profile. |
| R3 -> AC3 | A controlled pending read does not hold the global lock or delay runtime-state access until it finishes. Closing/replacing a session cancels or retires pending reads under documented budgets, records actual close outcomes, and never attributes old results to a new generation. |
| R4 -> AC4 | Captured success, upstream isError, malformed structured payload, transport failure, and timeout map to documented typed outcomes. Raw tool/path/process arguments, user routes, writes/imports/restores/providers/global-config changes remain denied. |
| R5 -> AC5 | Evidence records OS, engine SHA, sandbox identity, production dispatch path, command, and close result without secrets. Native GUI, Windows Job Object, and unexecuted fault paths stay UNVERIFIED. Update C01 capability truth and owning specs only for connected behavior. |

## Out of scope

User-vault onboarding; note writes; production drafts/backups; cloud/provider/semantic activation; installers; another client database/index; generic backend framework; engine pin changes. C03 owns query ordering/paging/execution policy; C04 owns consumer stale-response guards.

## Planning decisions

The isolated read-only fixture is this child's complete target. Missing captured payload details are evidence prerequisites, not permission to invent fields. Direct upstream calls that bypass production desktop session ownership cannot satisfy AC1.
