# C01 isolated contract and baseline handoff

Date: 2026-09-19. Scope: generated `bmdock-fixture` only, installed pinned engines, no UI or user-vault access. Commands below use the host standard library and the existing G0 sandbox/environment/probe owners. No dependency was added.

## Authoritative records

- Frozen protocol: `execution/evidence/c01-measurement-manifest-v3.json`. The preliminary unversioned and v2 manifests are superseded. V3 corrects textarea selection coordinates to normalized API text; v2 native offsets must not be used. Preliminary captures under `artifacts/client-baseline/*contract*.json` are exploratory records.
- Physical reproduction: `execution/evidence/c01-corpus-reproducibility-v3.json`. All five scales were recreated twice independently for each of two profiles: 20 isolated fixtures, 10 matching pairs. Each ordinary note is exactly 4,096 bytes; large notes are exactly 1,048,576 / 5,242,880 bytes. Counts, total bytes, sorted path/content SHA-256 records, and independent query-label hashes match. V2 and v3 physical hashes are identical; v3 changes measurement metadata, not corpus bytes.
- Final profile-local wire records: `execution/evidence/c01-release-contract.json` and `execution/evidence/c01-main-preview-contract.json`. These contain negotiated schemas, exact transmitted argument objects, handshake, installed versions, full raw envelopes, typed observations, corpus hash, and normal shutdown receipt.
- Reusable fixture manifests: `artifacts/client-baseline/c01-corpus-reproducibility-v3/{profile}-{scale}-1.fixture.json`. The sandbox lives under `.work/g0/client-baseline/`; only its generated Markdown/config/index may be touched.

The manifest freezes every scenario's arguments, five cold/30 warm observations, nearest-rank p95, all 30 independent relevance queries, page/row limits, adapter/UI budgets, and native typing protocol hash. It includes 30 literal insertion characters and normalized textarea API UTF-16 offsets for each large fixture, resetting outside measured intervals. Raw source UTF-8 bytes and UTF-16 lengths are distinct fields; normalization does not mutate the physical file. `load_fixture` rejects manifest drift and changed physical corpora; generation takes an explicit `--manifest` rather than restamping rules.

## Observed contracts, both profiles unless stated

### Connected desktop capability truth after C02

| Surface | Current verified scope | Remaining boundary |
| --- | --- | --- |
| Default launch | Honest unavailable runtime/read session | No implicit vault discovery or engine launch |
| Host generated-fixture launch | Both pinned profiles handshake/discover independently | No profile merging or real-vault authorization |
| `list_tree` / `read_note` | Production desktop dispatch, nested full-source reads, child/generation reuse | No note write/store activation |
| Runtime / close | Short-lock reads; actual normal close and SDK EOF regression | Native window close, Job Object/process tree and official forced kill UNVERIFIED |
| Search/context/activity | C01 direct-engine envelopes observed | Desktop adaptation belongs to C03, not established by C02 |

Authoritative receipts are `execution/evidence/c02-release-session-v2.json` and
`execution/evidence/c02-main-preview-session-v2.json`, independently checked at
the final C02 source/binary revision. See C02 research/implementation-review.md
and the owning `engine-read-session.md` spec. Later code changes require new
matching-revision integration evidence; old receipts are historical facts.

### Captured upstream shapes

| Operation | Exact observed projection | Adapter consequence |
| --- | --- | --- |
| Wrapper | Successful JSON payload under `result.structuredContent.result`; text blocks duplicate it. A guidance string can occupy the same wrapper with `isError=false`. | Decode once at the boundary; a wrapper is not operation success. |
| `read_note` | `title`, `permalink`, `file_path`, `content`, `frontmatter`. Explicit `include_frontmatter=true`, JSON, no line bounds. | Preserve delivered string and identity. Validate `content` is a string, not merely an object envelope. |
| Missing read | Structured object whose title/permalink/path/content/frontmatter are all null, despite `isError=false`. | Missing note, not an empty successful read. |
| `search_notes` | `results`, **`current_page`**, `page_size`, `total`, `total_is_exact`, `has_more`; each hit has `type`, `permalink`, `external_id`, `score`, and supported content/title fields. | Preserve upstream order and score; respect exactness/continuation. Do not read `page` for search. |
| `list_directory` | `nodes`, `page`, `page_size`, `total`, `has_more`; file nodes carry `file_path`, `permalink`, `external_id`; directories have null note identities. | Folder paths and note identifiers are distinct. The leaf capture proves duplicate `note.md` basenames resolve through full paths. |
| `build_context` | `results` containing `primary_result`, `observations`, `related_results`; top-level `metadata`, `page`, `page_size`, `has_more`. | Preserve node kinds and explicit depth/related limits; this is not a local snippet. |
| `recent_activity` | A list of identified items (`type`, title, permalink, path, created_at). No exact-count/page metadata appears in this list. | Do not invent a total or continuation token. |
| `list_memory_projects` | In this deliberately single-project process, a prose string describes `bmdock-fixture`; no structured catalog. | Route from explicit host fixture policy; do not parse project prose. This observed behavior is not an unresolved read-adapter payload. |
| Disabled semantic mode | Guidance string explaining disabled semantic search, with `isError=false`. | Unavailable semantic mode, not a successful empty lexical result. No model/provider was enabled. |
| Invalid page type | MCP `isError=true` and validation text. | Preserve tool error separately from control policy/schema/transport errors. |

Release runtime distribution: Basic Memory `0.23.2`, FastMCP `4.0.0b1`, MCP `2.0.0`; main-preview: Basic Memory `0.0.1.dev1+3452c82`, FastMCP `4.0.3`, MCP `2.0.0`. The profile-specific checkout SHA and 21/27 tool inventories are checked separately. A FastMCP handshake version is not the Basic Memory package version.

Both full-source reads preserved all known YAML/custom Chinese metadata and 16 CRLF sequences, with equal UTF-8 bytes and hashes for this 4 KiB fixture. Default reads stripped YAML. The evidence distinguishes physical-byte equality, equality after CRLF normalization, and unchanged client decoding; one successful fixture is not a general claim that engines never normalize.

Both `ordering-probe` captures must retain observed order `[corpus/note-00099, corpus/note-00020]`: the strongest result sorts last alphabetically. Gold labels remain the original 10 literal, 10 Chinese, 10 paraphrase judgments. Lexical recall on this synthetic corpus is a diagnostic, not semantic retrieval quality or a general multilingual guarantee.

Main-preview advertises `compact` and line-range read fields; release does not. Capture records retain this profile distinction. Full-source requests omit `start_line` / `end_line` in both profiles; line ranges are not a new UI requirement. Baseline search uses full output consistently; compact capture is a separate observed capability, not retroactively substituted into the baseline.

## Commands and clocks

```powershell
python -m scripts.client_baseline --help
python -m scripts.client_baseline freeze --output execution/evidence/c01-measurement-manifest-v3.json
python -m scripts.client_baseline reproduce --manifest execution/evidence/c01-measurement-manifest-v3.json --output execution/evidence/c01-corpus-reproducibility-v3.json
python -m scripts.client_baseline capture --fixture artifacts/client-baseline/c01-corpus-reproducibility-v3/release-notes-100-1.fixture.json --output execution/evidence/c01-release-contract-rerun.json
python -m scripts.client_baseline measure --fixture artifacts/client-baseline/c01-corpus-reproducibility-v3/release-notes-100-1.fixture.json --scenario read-full --scenario literal-00 --output execution/evidence/c01-release-notes-100-v3.measure.json
```

Outputs refuse overwrite. Use a fresh reviewed output path for a rerun. Change the profile and scale explicitly, never merge engine identities. `scenarios --fixture <manifest>` lists the matrix without starting the engine; omitting repeated `--scenario` selections measures all supported cases for that corpus. `matrix --manifest <frozen> --measurement <report> ... --output <new-path>` lists every measured and missing cell.

Cold means a fresh owned engine/probe for each request sample after separate index preparation. It retains the index and does not control OS disk cache. Warm means one primed session for 30 requests. No readiness query precedes a measured cold request. Timed intervals wrap `Probe.raw` and therefore include Python/probe/stdio/JSON roundtrip; they do not isolate engine service execution. Startup/readiness durations, command count, result count, UTF-8 serialized payload bytes and OS process peak RSS are recorded separately. Peak RSS is the process high-water mark since startup, not per-request allocation. Full responses are cleared from the smoke probe transcript during measurements so large-note timing does not include accumulation of 30 retained responses.

**RSS qualification:** `execution/evidence/c01-rss-scope.json` records the observed Windows process chain: the probe's direct `childPid` is a virtualenv launcher which starts the actual managed CPython worker. Thus the historically named `engine_peak_rss` field is launcher-only. Its nonzero number must not be promoted to total engine memory; actual worker/process-tree peak RSS remains `UNVERIFIED`. Probe peak RSS is a direct per-process observation. No synthetic sum/zero is substituted, and native process-tree cleanup is not established by this observation.

The paired desktop adapter, native presented-frame timing and semantic provider/model timings remain unavailable. The native protocol is frozen at the parent research file named and hashed in the manifest; C06 owns qualified acquisition and C07 integration. No EmptyLibrary speedup or full-matrix completion is claimed.

On 2026-09-19 the user explicitly chose to finish code and representative benchmarks while leaving the full matrix as pending acceptance. Representative scope is `notes-100` full read/literal search and separate 1 MiB/5 MiB full reads, independently for both profiles. C01 AC2 therefore remains partial; tooling handoff does not waive the remaining cells or the later paired/native criteria. The v2 release notes-100 run is historical backend-only evidence and is excluded from final v3 coverage.

The host is shared with approved C02 Rust builds/checks and live fixture testing during the measurement window. Final samples carry UTC start timestamps and reports carry run start/end timestamps. CPU utilization and background load were not controlled; C03/C07 need interleaved direct/adapter measurements under comparable conditions before making a gain claim.

The first release 5 MiB attempt (`c01-release-large-5mib-v3.measure.json`, 03:54:10–03:54:38 UTC) prepared the complete one-note index and shut down cleanly, then the first cold restart ended before initialization. The owned stderr contained `Error: ConnectionClosed("initialize response")`; the report retains `EOFError`, zero timed samples and `status=incomplete`. No cause was established. A separately named bounded rerun is permitted; it must not replace this failed attempt or silently retry inside a measured sample. C02 confirmed its cleanup targets only its own direct process handles.

## Completed representative measurements

The separately named release 5 MiB rerun completed successfully. The failed first attempt remains preserved; success on rerun does not establish its cause or a startup fix.

`execution/evidence/c01-baseline-coverage.json` binds only these six completed, unchanged-corpus v3 reports: `c01-release-notes-100-v3.measure.json`, `c01-main-preview-notes-100-v3.measure.json`, `c01-release-large-1mib-v3.measure.json`, `c01-release-large-5mib-v3-retry.measure.json`, `c01-main-preview-large-1mib-v3.measure.json`, and `c01-main-preview-large-5mib-v3.measure.json`.

| Profile | Corpus / operation | Cold p95 ms (n=5) | Warm median ms (n=30) | Warm p95 ms |
| --- | --- | ---: | ---: | ---: |
| release | 100 notes / full read | 910.55 | 86.80 | 118.01 |
| release | 100 notes / literal-00 search | 1467.65 | 56.17 | 83.15 |
| main-preview | 100 notes / full read | 1301.75 | 52.92 | 60.29 |
| main-preview | 100 notes / literal-00 search | 781.96 | 22.36 | 49.01 |
| release | 1 MiB / full read | 2080.85 | 194.88 | 231.19 |
| release | 5 MiB / full read, distinct rerun | 1839.59 | 548.80 | 624.89 |
| main-preview | 1 MiB / full read | 2440.79 | 132.91 | 145.31 |
| main-preview | 5 MiB / full read | 8283.53 | 523.66 | 840.54 |

All 280 raw samples are retained, including the slow main-preview cold sample; none is trimmed. These are direct-MCP roundtrip timings, not paint/typing timing or a version-to-version improvement claim. Shared host load and distinct runtime dependencies preclude treating this table alone as a fair profile comparison.

Coverage is **16 measured / 428 total cells; 412 missing cells**, with `full_matrix_complete=false`. Missing cells explicitly include all 1,000/10,000-note timings and the other 100-note query/context/activity scenarios. The full matrix remains user-deferred acceptance. The complete five-scale physical reproduction is a separate passing result and does not substitute for those timings.

The release 5 MiB source response reserializes to 10,486,617 UTF-8 bytes because FastMCP includes both text and structured payload copies. The delivered source remains exactly 5,242,880 bytes. Future adapter response budgets must account for this wire shape; list projections must still avoid forwarding full bodies unnecessarily.

## Validation and inherited limitations

`python -m unittest tests.test_client_baseline -q` verifies physical reproduction/mutation detection, corpus sizes and labels, guidance/null reads, manifest drift, full-source identity, fixed 5/30 restart behavior, protocol-mismatch cleanup, p95 rank and output confinement. It requires no live engine. The focused desktop behavior check is owned by the root/frontend collaborator, not replaced by these harness tests.

Final command `python -m unittest tests.test_client_baseline tests.test_desktop_shell -q` passed **51 tests** (17 C01 harness + 34 desktop structural checks). `git diff --check` passed. Full CI/gates were not inferred from that subset.

The restricted Windows token cannot write beneath `tempfile.mkdtemp` directories on either host Python 3.14 or installed Python 3.12. The approved test/capture commands ran with a narrow local escalation; no global ACL/configuration or core sandbox helper was changed. Ordinary no-temp read/CLI checks still run normally.

The existing full-unit wrapper still fails `A later task was completed before G0`, and `gate` reports G0–G7 unpassed. These failures were not concealed or repaired by the focused C01 entry. Native UI, Windows process-tree fault cleanup, signed packaging, and real-vault durability remain unverified.
