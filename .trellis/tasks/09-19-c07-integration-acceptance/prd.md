# C07: Integrated query, performance and native acceptance

## Goal and dependencies

Verify the combined client rather than separate mocked modules. Owns parent R7
and integration of parent AC1-AC6. The later user instruction prioritizes code
and representative measurements; C07 integrates C01-C06's available evidence
without treating their deferred full-matrix/native criteria as complete.

## Requirements

- R1: Prove the actual desktop read route works against both pinned official engines and retains upstream semantics.
- R2: Demonstrate request reduction and bounded overhead under the frozen C01 protocol without relevance or interaction regression.
- R3: Collect native layout/keyboard/IME/accessibility evidence where authorized and report unavailable evidence separately.
- R4: Publish a traceable bounded acceptance decision, leaving unrelated product/release gates unchanged.

## Acceptance criteria

- [x] AC1 (R1): Each profile independently passes seed -> connect -> tree/search -> choose hit -> read -> context/activity where supported -> disconnect. Captured route/result identities match direct MCP and physical seed data; wrong routes, malformed responses and connection failure do not become empty success.
- [ ] AC2 (R2): Under the C01 manifest, adapter warm p95 overhead <=max(50 ms, 25% direct-engine warm p95); meaningful UI scenarios have candidate p95 <=110% baseline. Reports include raw samples, counts, bytes, hardware, RSS and cold/warm separation. A missing baseline or failed threshold leaves the related AC incomplete.
- [ ] AC3 (R2): Opening the ordinary workspace emits zero diagnostic/catalog/inspector calls; ordinary search emits one search and zero inspector calls; retained primary rows <=150. Reversed request/page/save completions preserve latest intent and new text; supported result identities/order match direct engine and per-profile labeled recall does not regress.
- [ ] AC4 (R3): Native evidence covers 1200x800, 720x480, 200% text zoom, keyboard focus, Chinese composition and retained drafts. For 30 committed edits (10 beginning/middle/end), input-to-first-presented-updated-frame p95 is <=50 ms for a 1 MiB note and <=100 ms for a 5 MiB note using C01's frozen trace method. If native operation is not authorized/available, this AC remains UNVERIFIED rather than passed from web mocks.
- [x] AC5 (R4): Final evidence maps each parent AC and child result to a command/session artifact, distinguishes failures/skips/unverified claims, documents known G0/CI limitations and retains do-not-release while product gates are unpassed.

## Current acceptance

`research/integration-acceptance.md` records the final bounded decision. AC1
passes through actual production dispatch in a typed/headless fixture harness,
with controlled Rust tests for failure boundaries; no GUI journey is claimed.
AC2 remains partial despite both representative cases meeting the unchanged
overhead allowance: full coverage, historical C01 clock conformance, engine RSS
and meaningful UI baselines are missing. AC3 is partial because production-function
command/retention tests do not establish mounted primary rows/native activation.
AC4 is UNVERIFIED. AC5 documents these limits and preserves do-not-release.

## Exclusions

No real-vault testing, Cloud/provider activation, downloads, signing/release, global settings or rewriting historical task/gate states. No authorization for UI automation is implied by this planning task. Manual user execution is an acceptable source of clearly attributed native evidence.
