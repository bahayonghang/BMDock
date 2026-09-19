# C06 — Search-to-reader and bounded frontend work: design

## Source evidence
- apps/bmdock-desktop/src/App.tsx:616 — ordinary search also calls inspect_search.
- apps/bmdock-desktop/src/App.tsx:620 — result activation previews context instead of selecting primary note.
- apps/bmdock-desktop/src/App.tsx:422 — six secondary loads after initial tree.
- apps/bmdock-desktop/src/App.tsx:1605, :1739 and :1877 — accumulated merging and rendering.
- apps/bmdock-desktop/src/ipc.ts:7 and :37 — current shared size20 and SearchNotesArgs; do not infer new semantics.

Parent research: ../09-19-bmdock-client-optimization/research/frontend-audit.md. Follow parent design.md D2/D4 and the finalized official-and-projects.md for cross-layer contracts. Source conclusions are not native acceptance.

## Mechanisms, requirements and acceptance mapping
### M1
M1 -> R1 -> AC1: Use C04 selection owner and C05 reader; title/snippet only when supported, identifier secondary, raw scores in inspection. C03 specifies whether each filter targets entity category, note metadata or observation content; never substitute one for another.

### M2
M2 -> R2 -> AC2: Remove eager activity/resource/prompt/tool/CLI/API-audit reads from ordinary workspace and load each only on its relevant surface. Maintain explicit pending/empty/error states without adding a generic cache.

### M3
M3 -> R3 -> AC3: Use default50 and a finite three-page/150-row window. Evict the oldest non-active page when advancing beyond the window; restore focus by stable identifier or a documented nearby control if its row is evicted. Cursor history stores tokens, not evicted bodies. Confirm previous navigation against C03 cursor semantics; no invented page numbers or all-results fetch. Set search-only limits, not unrelated graph/tree limits.

### M4
M4 -> R4 -> AC4,AC5: Follow parent design D4 and C01: 100/1,000/10,000-note corpora at approximately 4 KiB each, separate 1/5 MiB notes, five cold and thirty warm samples, with hardware/runtime/profile recorded. Separate engine/IPC/paint intervals. C07 candidate gates are meaningful UI p95 <=110% baseline and backend overhead <= max(50 ms,25% direct-engine p95), to be accepted after C01 measurement. Never compare real-engine work against EmptyLibrary timings.

## Ownership
Search call sites/modules and informational content preview call sites in App.tsx/contentSafety.ts, relevant i18n.ts and focused styles/tests. C03 owns Rust query/DTO changes; agree a single writer for ipc.ts. Preserve C04/C05 seams; no new persistence, dependencies or backend index.

### M5 — Large-note typing
M5 -> R4 -> AC4: Keep unconditional escaped rendering, but move informational ContentSafetyFacts whole-body classification and the duplicate full-text preview out of ordinary typing. Compute them only when their details are opened or explicitly refreshed, bind results to the submitted note revision, and show stale/updating state if the note changes. Do not retain a hidden mounted duplicate preview. This work is presentation diagnostics, not the actual HTML-execution defense. C01 freezes 30 committed-character input-to-presented-frame samples; C07 records native p95 <=50 ms for 1 MiB and <=100 ms for 5 MiB or leaves native evidence incomplete.

## Unresolved evidence and implementation choices
Filter semantics, previous-cursor navigation and numerical budgets depend on C01/C03. If size50 is unsupported, resolve that contract rather than silently fetching all data. Native timing requires later manual or explicitly authorized operation.

No generic query cache, virtualizer, docking framework or state platform is implied. Use existing tools first; future new dependencies require authorization. No mutation retry follows logical cancellation or timeout_unknown.
