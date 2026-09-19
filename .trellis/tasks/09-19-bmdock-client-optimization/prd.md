# Basic Memory client experience, query and performance optimization

## Goal

Deliver a correct and measurable find-open-read workflow, retain local edits, and make the existing desktop workspace usable. The parent owns requirements and final integration review. Planning was approved, then implementation was authorized by `设计没有问题，继续` on 2026-09-19. The user subsequently chose code and representative benchmarks first; full-matrix acceptance remains deferred. See `research/implementation-progress.md` for current evidence.

## Background and confirmed facts at the planning baseline

The desktop starts with `EmptyLibrary`, `EmptyDraftStore` and a default Supervisor (`apps/bmdock-desktop/src-tauri/src/main.rs:72`). Real-engine probe evidence is separate. The filesystem query library is test-only, so its scans are not measured production bottlenecks.

Frontend source confirms unguarded completions, save-response overwrite risks and diagnostic-heavy stacked views. Exact evidence is in `research/frontend-audit.md`; backend and pinned-version findings are in `research/backend-audit.md` and `research/official-and-projects.md`.

T01-T40 are marked completed while G0-G7 are unpassed. The unit wrapper fails phase order; 34 desktop structural checks passed this turn. See `research/baseline-and-evidence.md`.

## Requirements

- R1: Freeze profile-specific query contracts, measurement protocol and honest validation status. Owner: C01.
- R2: Connect desktop reads to an official engine session in an isolated owned fixture with accurate lifecycle/capability state. Owner: C02.
- R3: Preserve upstream meaning, ranking and pagination; bound read work and avoid host-wide lock contention. Owner: C03.
- R4: Latest user intent wins; requests/navigation cannot silently discard unsaved local text. Owner: C04.
- R5: Make search, selection and reading primary, with readable typography and accessible responsive controls. Owner: C05.
- R6: Make hits actionable, expose only supported modes/filters, and defer hidden diagnostics and expensive rendering. Owner: C06.
- R7: Verify functional, Chinese-retrieval and performance results per profile; distinguish native and release evidence. Owner: C07.

## Scope

Proposed implementation includes read-first engine integration in an isolated fixture, editor/request correctness, layout, query presentation and measured efficiency. Editor changes protect the existing in-memory session; they do not enable new engine writes or promise restart persistence.

Excluded: real-vault onboarding/writes, Cloud/provider enablement, global configuration, a second index/direct BM database access, docking platforms, rich Markdown dependencies, full graph redesign, installer/signing/release and completing all original G0/G7 obligations. Plain-text/source reading remains safe and gains better typography; rich Markdown is deferred.

## Acceptance criteria

- [ ] AC1 (R1): C01 provides profile-local contract evidence, deterministic fixture recipe, raw measurements and passed/failed/unverified checks without hiding inherited phase-order failure.
- [x] AC2 (R2): Actual desktop reads return physically seeded fixture notes through each pinned engine; unavailable, empty and error remain distinct and non-owned routes are rejected.
- [x] AC3 (R3): Identifiers, ordering, pagination, exactness and modes agree with direct MCP results; a delayed read does not hold the host-wide state lock.
- [x] AC4 (R4): Deferred-response/editor tests prove obsolete completions cannot replace the active note/query/newer typing; failures retain text.
- [ ] AC5 (R5): At 1200x800, 720x480 and 200% text zoom, search, selection and content remain accessible without scrolling past diagnostics. Native claims require native evidence.
- [ ] AC6 (R6): Ordinary opening emits zero diagnostic/catalog/inspector calls; one query emits one search and zero inspector calls; hit activation opens the reader and rendered rows stay bounded.
- [ ] AC7 (R7): C07 publishes separate profile results under frozen measurement rules and satisfies its regression/overhead budgets or leaves the criterion incomplete; mocks do not close native/product gates.

AC2–AC3 pass at generated-fixture typed/headless plus specified unit-test scope;
AC4 passes production-function behavior tests. AC1/5/6/7 remain partial for the
full matrix, native/DOM evidence and clock/complete-budget requirements.
The exact evidence map and per-profile numbers are in
`../09-19-c07-integration-acceptance/research/integration-acceptance.md`.

## Authority and delivery state

Parent and seven linked children contain PRD, design, implementation plan and real context manifests. Task creation and subsequent implementation were explicitly approved on 2026-09-19. Active child statuses and evidence are tracked separately; incomplete performance/native criteria are not closed by code completion.

The approved read-first scope, UI arrangement and frozen performance budgets govern implementation. New dependencies, real-vault access, commit/push/archive and native UI operation are outside that approval unless separately requested. The full 428-cell matrix remains a pending acceptance task under the user's latest instruction.
