# Optimization design

## D1. Ownership

```text
React workspace: active read identity + retained edit session
       -> typed IPC and explicit route
Tauri: short state snapshot -> owned read-session handle
       -> pinned rmcp MCP session
Official Basic Memory: Markdown + official indexes -> ranked results
       -> profile-aware adapter -> bounded view data
```

Basic Memory owns indexing, ranking, semantic types and project meaning. BMDock owns routing, lifecycle, typed adaptation, local view state and presentation. Keep G0 probe commands independent. Do not promote test-only filesystem scans into production, expose arbitrary callTool or build a second index.

C02 adds only the needed read adapter/session lifecycle; C03 completes execution/performance. Existing receipt-only `begin_shutdown` is not silently repurposed: the new live-session owner needs an explicit shutdown contract and corresponding spec amendment.

## D2. Shared read contract

Identity includes profile, session generation, explicit workspace/project, operation, note identifier or normalized query, mode, all filters and page/cursor. Route/profile/reconnect changes invalidate prior identities. Compact/CJK/temporal features are profile-local, never inferred from latest online docs.

The host state lock covers validation and handle retrieval, not engine I/O, disk work or large parsing. Queue/concurrency belongs to the engine-session owner and is bounded by verified engine behavior. Runtime/control responses remain serviceable during slow reads. Start with no persistent result cache; optional in-flight dedup uses the complete identity. Explicit refresh creates a new generation.

Preserve upstream result kind, order, identity, `total_is_exact` and `has_more`. Do not sort after slicing, clamp hybrid scores to 1, or represent unavailable semantic search as lexical success. Decode observed structured envelopes; JSON-requested guidance text remains an error/unavailable result, not empty success. Exact-note `read_note` pages may not slice note bodies; respect the pinned contract.

For source/reader requests, explicitly request the verified pinned equivalent of `include_frontmatter=true`. This flow is unsliced: omit main-preview-only `start_line/end_line`; release lacks those parameters. JSON reads can otherwise omit YAML. C01 compares known UTF-8/frontmatter/CRLF fixtures with the delivered string; C02 preserves that delivered source, including frontmatter. Record engine newline/encoding normalization separately and never claim disk-byte fidelity from a text response. C05 preserves the delivered string across view changes; no real-note round-trip write is introduced.

Default page size is 50. The primary result view retains at most three pages (150 rows), replacing/evicting pages while retaining selected-note identity separately. Do not fetch full bodies for list rows. No virtualizer is required by default. External edits/renames/deletes are reflected through refresh; any future caching proposal must define invalidation before adoption.

## D3. Frontend and visual arrangement

C04 owns request generations, localized pending/error states and retained edit sessions. Logical cancellation rejects stale completion without claiming physical engine cancellation. Save acknowledgements update only the submitted revision's saved baseline, never newer typed text. Dirty content survives view changes; restart durability is not newly promised.

C05 uses a primary list-reader workspace and optional related-content region. Search is always reachable; diagnostics/maintenance are secondary destinations. Use a small semantic token set and existing native controls. At minimum size use deliberate compact/stacked regions. Preserve text escaping, raw-source fidelity, labels and visible focus.

```text
Project / connection       Search
Navigation + result list | Note reader / current edit
                         | Related content on demand
Diagnostics / maintenance are separate destinations
```

This is a planning sketch, not a rendered result. C06 adds actionable hits and demand-driven queries without restoring eager diagnostics.

## D4. Measurement contract

C01 freezes seeded synthetic corpora of 100, 1,000 and 10,000 Markdown notes, about 4 KiB each, plus separate 1 MiB and 5 MiB notes. Record actual count/bytes, seed, hardware/OS, profile commit, tool versions, index-ready state and semantic availability. Independently label 30 queries: 10 literal, 10 Chinese/mixed-language and 10 paraphrase; store expected identities. Unsupported semantic modes remain unavailable.

Per supported scenario/profile, record five cold and 30 warm observations. Cold means process restart without application cache; OS disk cache is uncontrolled and disclosed. Warm uses the same ready session. Nearest-rank p95 is sorted[ceil(.95*n)-1], rank 29 for n=30. Report raw samples, median, p95, command count, payload bytes and peak RSS; engine execution, host overhead and action-to-paint are separate.

Interleave direct-engine MCP and desktop adapter runs on the same immutable corpus. Proposed C07 budget: desktop adapter warm p95 minus direct-engine warm p95 <= max(50 ms, 0.25 * direct-engine p95). For a meaningful existing UI baseline, candidate warm p95 <= 1.10 * baseline p95. Required command-count reductions also pass. These are proposed engineering budgets, not measured promises. C01 freezes them; material changes require final-plan review before closure.

Never compute speedup against EmptyLibrary. Before adapter availability, direct MCP is the backend control and delayed typed fakes are the frontend regression baseline. Native measurements require a functional read path and authorized UI operation. Relevance must match the same profile's direct-engine output; Chinese recall is reported separately. A failing budget remains incomplete rather than changing the dataset or threshold after the run.

Large-note editing has a separate proposed budget: after the note is displayed, measure 30 committed single-character edits (10 near the beginning, 10 middle, 10 end), from input-event timestamp to the first presented frame containing the updated text using the recorded renderer trace method. Native p95 must be <=50 ms for 1 MiB and <=100 ms for 5 MiB; record raw samples and whether IME composition has committed. C01 freezes the protocol; C06 owns removing repeated informational full-body scans and duplicate previews and obtaining its native pass/fail evidence; C07 integrates applicable same-revision receipts and reruns invalidated cases. Missing native timing is incomplete, not a mock-derived pass. Initial read timing remains a separate scenario. The qualified protocol is `research/native-input-latency-protocol.md`; native acquisition and frame correlation remain unverified.

## D5. Integration and rollback

C02/C03 own Rust boundaries; C04-C06 own frontend state/views and serialize shared edits. Root coordinates DTO changes. C01/C07 own measurement artifacts. Old Trellis package defaults do not assign desktop code to the probe layer.

Rollback disables a failed adapter with an honest unavailable state, never silent fake success. Preserve typed policy, content safety, user data, immutable profiles and historical tasks. No implementation, new package, native operation or real-vault mutation is authorized by this planning turn.

| Requirement / AC | Mechanism | Owner |
|---|---|---|
| R1 / AC1 | Frozen contracts and measurement manifest | C01 |
| R2 / AC2 | Owned typed read session and route gate | C02 |
| R3 / AC3 | Short locks and bounded upstream-preserving results | C03 |
| R4 / AC4 | Request generations and submitted revision acknowledgement | C04 |
| R5 / AC5 | Primary workspace, control states and focus/overflow rules | C05 |
| R6 / AC6 | Visibility-driven requests and bounded hit views | C06 |
| R7 / AC7 | Paired runs, gold labels and separate native evidence | C07 |
