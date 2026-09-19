# C07 delivery and portable-evidence review

Date: 2026-09-19. Independent reviewer: `c01_check`. Decision: **PASS for the
requested packaging and scoped task closeout**. This review does not close the
deferred performance matrix, native acceptance or historical product gates.

## Archive eligibility

Only C02 (`09-19-c02-engine-read-session`) and C04
(`09-19-c04-interaction-correctness`) satisfy all their own stated criteria.
C02's target is the generated-fixture owned read lifecycle; C04's target is
production-function/state-owner correctness with source-wiring review. Their
PRDs explicitly keep native evidence distinct. Subsequent C03/C06 integration
checks support those scoped conclusions without converting missing native
evidence into a pass. Parent/C01/C03/C05/C06/C07 must remain open.

No context JSONL or live spec references a file inside either retiring task.
Keep task IDs in parent `children` and `meta.depends_on`; Trellis deliberately
retains archived children in progress accounting. C04's `design.md` parent
research reference needs the additional archive-directory traversal after its
move; internal `research/*` links stay valid. Archive with `--no-commit` when
preserving the coordinator's explicit commit grouping.

## Portable evidence and provenance

The user selected ten raw files larger than 1 MB for exact-path Git exclusion.
They remain local and byte-identical, totaling **20,291,143 bytes**. Other,
smaller measurements remain delivery inputs. The exact ten paths, file-byte
SHA-256 values, profile identities and selected outcomes are recorded in
`execution/evidence/client-raw-receipts-summary.json`. A fresh clone cannot
independently inspect all original responses from this summary alone; retain
that availability limit or generate new evidence with the recorded harness.

The two committed replay fixtures are:

- `tests/fixtures/c01-release-query-replay.json`: 32,411 bytes.
- `tests/fixtures/c01-main-preview-query-replay.json`: 32,483 bytes.

Each keeps exactly the three structured values used by the existing replay
test: ordering probe, context and activity. Independent comparison verified
all six values equal their original captures, including full retained fields,
order and scores. Unrelated records and duplicate text wrappers are omitted;
selected content is not truncated, rewritten or replaced with synthetic data.
The existing decoder assertions remain unchanged and load these fixture paths,
with no dependency on an ignored raw file. Source path/hash metadata is
provenance, not a second file opened by the test.

## Measured revision boundary

The only `engine_queries.rs` change is the input-path literal inside its
`#[cfg(test)]` module, from the large contract receipt to the compact fixture.
Independent reconstruction by reversing that one literal recovers the exact
previous source hash:

- Before: `8e9be60973b7696d9ac5f4b213425987419b226f18c652170cfa1be5efb04606`.
- Delivered: `4fdeafa79cef422a5c8cfbde483c3067d4d9c0268197623ff679261af9ed0d7c`.

The 19,048-byte production prefix is identical, with SHA-256
`50ab293fdd99b0a8ac061f5794111cdebe4f535e73447eda4f1b38dcd6807a27`.
The previously measured runtime binary remains
`67dabbe272e5fa193ab51751b67c9281687879b77a53c742df75257c3e6095ea`.
`execution/evidence/c07-replay-portability.json` records this narrow bridge.
Historical raw reports and their measured hashes were not rewritten. The
whole-file source hash therefore differs from the measured snapshot; this
packaging check is not a new benchmark or a full fingerprint-match claim.

## Verification and commit boundaries

The reviewer independently checked all ten local raw byte hashes, summary
headers and selected functional outcomes, six exact structured values, fixture
sizes/hashes, exact ignore entries, the single test-only source delta and the
unchanged runtime binary. Source search finds no old raw-contract input path in
the desktop/tests code. The owner ran the focused Rust replay: **1 PASS, 233
filtered**. `git diff --check` passes. Full suites were not repeated for this
test-path-only change; no native operation or new performance claim was made.

Baseline/probe delivery must include the native protocol file fingerprinted by
the harness and these replay fixtures before the desktop commit. Keep Rust,
TypeScript DTOs, App/state modules and their frontend/structural tests together.
Session/query harness and retained evidence can follow, then remaining Trellis
specs/tasks and the two eligible archives. Actual commits, push and archive
receipts belong to the coordinator's delivery record; this review performed
none of those operations.
