# C03 design

## Ownership and evidence

Use C02's session owner for all production read I/O. Official Basic Memory owns index/rank/query semantics. Desktop owns explicit typed request identity, bounded execution, and faithful result projection. Parent design D2/D4 is authoritative for shared defaults and metrics.

Evidence: desktop `library.rs:3759-3790`, `:4659-4703` show fixture scan/order only; `main.rs:35-66` shows the global lock to avoid. Pinned release `mcp/tools/search.py:703-731`, `:1212-1240` defines structured search and guidance caveat; `schemas/search.py:156` defines count exactness/has_more; `repository/sqlite_search_repository.py:1052-1074` ranks before LIMIT/OFFSET. Parent `research/official-and-projects.md` records profile-only compact/CJK/temporal fields, full exact-note reads, and hybrid scores possibly above 1.

## Complete request identity

Use one explicit request value containing profile, session generation, workspace/project, operation, normalized query or note identifier, mode, every filter, and page/cursor. Normalization must preserve query-language meaning; do not lowercase, strip operators, or change CJK tokenization in the client. Include page size and any compact/field selection in identity. Explicit refresh advances request generation. Backend admission validates the route and scalar limits once; transport decoding validates result shape once. Avoid repeating scalar probes across frontend, IPC, adapter, and session.

C04 owns consumer supersession and stale completion suppression. Backend owns session validity, capacity, and result origin; it does not maintain a second copy of UI generations. DTO changes are coordinated with C04/C06 before implementation.

## Indexed search and paging

Request an explicit supported search mode and project. Preserve ordered upstream entries; do not re-rank fixture/engine scores together, clamp scores to [0,1], or sort each fetched page. Retain result-kind identity so observations/relations do not become invented notes. Exact totals and unknown totals are separate; use upstream has_more rather than deriving it from an approximate zero count.

Keep upstream page numbering separate from fixture numeric offsets. Any frontend cursor wrapper encodes only the adapter's actual paging contract; do not promise immutable snapshots when the engine exposes offset/page pagination over a changing index. An immutable test corpus establishes no-loss traversal; mutations trigger refresh rather than continuing stale pagination.

Default requested list/search page size is 50, consistent with parent D2 and existing bounded limits. C06 retains at most three pages/150 rows and preserves selected-note identity outside the row window. The backend does not automatically prefetch or read every row body. Main-preview compact is usable only when its captured schema supports it and the consuming view does not require omitted excerpts. Release omits that field entirely.

The chosen unsliced exact read_note includes frontmatter and omits main-preview-only start_line/end_line. Do not advertise page_size as note-body chunking. Fetch bodies only on selection; measure the separate 1 MiB/5 MiB cases and surface any configured size refusal explicitly rather than silently truncating raw-source content. Preserve the delivered string and distinguish upstream normalization from original disk bytes. No new arbitrary size/configuration surface is required without a measured problem.

## Execution and cancellation

Reuse C02's short-lock handle/snapshot approach. Queue/in-flight bookkeeping belongs to the session owner, with a concrete small cap established by C01's supported workload and controlled tests before closure. Avoid an unbounded queue, a lock around the whole read, and recursive prefetch. A bounded channel or equivalent simple owner-local admission is sufficient; no global task scheduler is required.

A blocked query must not prevent runtime/control snapshots. Honor pinned SDK cancellation semantics. Logical consumer cancellation may discard results without actually stopping engine work; report that honestly. Define owner behavior for queue overflow, session shutdown, and a cancelled last consumer. Read retries are not automatic performance policy.

In-flight singleflight is optional only when baseline command traces show duplicate identical reads. Its key includes all identity dimensions; multiple consumers must not cancel one another's still-needed work. Clear entries on completion/error/session replacement. Do not retain completed search results persistently.

## Freshness and errors

Explicit refresh bypasses any transient in-flight reuse from an older generation. Engine indexing is asynchronous: external fixture edit/rename/delete acceptance waits for observed engine indexing readiness, then refreshes. Do not promise immediate filesystem consistency or add a recursive watcher/index of the client’s own. Missing selected notes surface not-found/unavailable appropriately rather than displaying old cached bodies as current.

JSON output request plus transport success is not query success. Parse only captured structured shapes, check isError, and preserve guidance-text outcomes as visible error/unavailable. Do not parse success-sounding prose, fabricate zero matches, or support unobserved envelope aliases. Preserve existing IPC/runtime error-category separation from C02.

## Performance protocol and rollback

Use parent D4 unchanged: deterministic corpora around 4 KiB per note plus separate 1 MiB/5 MiB notes, independent 30-query labels, per-profile five cold/30 warm runs, raw samples and nearest-rank p95, direct-engine/adapter paired order, payload/RSS/command counts. Cold is restarted process with no application cache; disclose uncontrolled OS disk cache. C01 freezes proposed overhead budget <= max(50 ms, 0.25 * direct-engine p95). C07 combines renderer/native evidence separately. No cached-empty baseline or different corpus may be substituted to claim success.

Rollback removes this child's query scheduler/projection changes while retaining C02's isolated read owner and truthful unavailable behavior. Preserve pin identity, generated evidence, and typed policy. No database migration, index modification, or user data recovery is part of rollback.
