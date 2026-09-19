# C05 — Focused workspace and accessible plain-text reading

## Goal and dependencies
C04 session/request ownership. Serialize C04 -> C05 -> C06; no concurrent App.tsx writers.
The user approved implementation and prioritized code plus representative
benchmarks. C05 code review has no remaining layout-specific blockers;
acceptance remains PARTIAL as recorded below.

## Requirements
- R1: Provide list-reader workspace with optional relations; group the 18 equal navigation entries into a small set of task-oriented destinations, keeping diagnostics accessible.
- R2: Unify typography, spacing and input/select/button hover/focus/selection/disabled states; preserve Chinese localization and semantic keyboard navigation.
- R3: Support native minimum 720x480, default 1200x800 and 200% text zoom with long identifiers; view changes preserve C04 unsaved sessions.
- R4: Improve escaped plain-text/source reading now and preserve the engine-delivered source string including frontmatter. Engine normalization and disk-byte equality are separate evidence. Rich Markdown is a deferred extension needing separate content/link policy and dependency approval, not a blocker.

## Acceptance criteria
- [ ] AC1 (R1): At 1200x800 users can reach search, select a list item and read it without scrolling past diagnostic catalogs; diagnostics remain keyboard-discoverable and honest about unsupported capabilities.
- [ ] AC2 (R2,R3): At 720x480, 1200x800 and 200% text zoom, Chinese/English labels and 200-character identifiers do not clip actions or cause unintended page-wide horizontal scrolling. Focus and selection are visible and programmatically exposed.
- [ ] AC3 (R1-R3): Keyboard order follows visible regions; workspace/diagnostics and reader/editor transitions retain unsaved text. Loading, empty, unavailable/offline and local error cases have clear localized status.
- [x] AC4 (R4): Hostile HTML/script is shown as escaped text without execution or remote loading; switching reader/editor preserves the delivered full-source string including YAML and existing newline characters. Fixtures identify any upstream normalization separately; the UI does not claim original disk-byte fidelity from a text response. No Markdown renderer or new dependency is needed.
- [ ] AC5 (R1-R4): Targeted compile/build and interaction tests pass; native viewport/zoom/IME/screen-reader evidence is separate from web/mock checks and remains incomplete until actually obtained.

AC1/AC2/AC3/AC5 remain **PARTIAL**, not complete. AC4 is supported specifically
by React SSR escaping, retained string/state tests and production-source review;
it is not a browser execution/network trace, textarea-normalization or disk-byte
claim. See [independent implementation review](research/implementation-review.md)
for the evidence mapping. The independent checkpoint passed 32 behavior/SSR and
34 structural/boundary tests. The final shared build failed on concurrent C03
engine/fixture query DTO unions awaiting C06 integration; an earlier C05 owner
build passed but does not replace that current failure. Native viewport, zoom,
focus, keyboard, IME and screen-reader evidence remain UNVERIFIED.

## Authority and exclusions
Product implementation was authorized by the subsequent user instruction.
Real-vault access, new persistence, dependency installation and global
configuration remain outside this work. No UI operation occurred in this review;
native acceptance requires later manual or explicitly authorized evidence.
Rich Markdown and generalized infrastructure are outside this task. C06 code
may proceed from the reviewed handoff while C05 native/integration clauses remain
open; this does not complete C05 or authorize commit/archive/release.
