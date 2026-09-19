# C01 design

## D1. Contract capture

Use installed pinned engines and existing isolated sandbox helpers; verify checkout identity and negotiate tool schemas before capturing selected read envelopes. Explicitly select only generated fixture projects. Record success, empty, tool error, unexpected text and unavailable modes per profile. Preserve raw response records plus concise typed field mapping; do not make heuristic alias tables.

The shared request identity is profile/session-generation/workspace/project/operation/query-or-note/mode/filters/page. Compare source-present behavior with actual wire results; the observed adapter shape is authoritative. Latest online documentation is supporting context only.

Capture JSON reads both with default frontmatter behavior and explicit `include_frontmatter=true` where supported. The full-source adapter uses the latter. Compare known YAML, Chinese UTF-8 and CRLF fixtures with the returned text; report engine normalization, client text preservation and physical-byte equality as distinct observations. Pinned release strips YAML for body-only reads (`.work/engines/release/src/basic_memory/mcp/note_reads.py:97`); do not assume JSON content is the original complete file.

The chosen full-source flow is unsliced and omits main-preview `start_line/end_line`. Capture these as preview-only capabilities, not as release parameters or requirements to add a line-range UI. Upstream page_size still concerns fallback result pagination rather than note-body chunks.

## D2. Measurement

Apply parent design D4 exactly: 100/1,000/10,000 notes near 4 KiB, independent 1/5 MiB cases, 30 independently labeled queries, five cold and 30 warm samples. Store seed, physical hash/count/bytes, engine SHA, model/index readiness and raw time/count/payload/RSS data. Separate direct-MCP controls from empty current desktop behavior. Unavailable modes get no fabricated measurements.

Freeze budgets before optimization: host overhead <= max(50 ms, 25% direct-engine p95); meaningful UI warm p95 <=110% its baseline. These are proposed criteria awaiting final implementation review. A material change to them requires a reviewed manifest change before closure, not retroactive tuning.

Freeze parent D4 large-note typing too: 30 committed single-character edits (10 beginning/middle/end), input-event to first presented updated frame, trace method and IME commitment recorded; native p95 <=50 ms for 1 MiB and <=100 ms for 5 MiB. C06 owns typing-path work reduction and its native acceptance evidence; C07 integrates valid evidence at the same revision and reruns invalidated cases. Absent native authorization leaves that evidence unavailable; a typed fake cannot establish paint timing.

## D3. Minimal behavior test seam and existing gates

C01 proves a controllable typed IPC seam can delay and reverse completions. C04 owns the full race/editor regressions. First evaluate existing tools; if renderer behavior cannot be tested adequately, document the smallest test-only dependency and exact version/cost for explicit approval before adding it. Do not substitute string assertions for behavior.

Add a named desktop-focused check entry only as needed; it reports its scope. Existing `check_source` and gate failures remain visible. Update stale desktop guidance when touched, without migrating global rules or changing Trellis package registration merely for convenience.

| Requirement / AC | Design mechanism |
|---|---|
| R1 / AC1 | D1 negotiated schemas and profile-local wire captures |
| R2 / AC2, AC3 | D2 deterministic fixtures, independent labels and frozen manifest |
| R3 / AC4 | D3 delayed IPC seam and distinct desktop-check entry |
