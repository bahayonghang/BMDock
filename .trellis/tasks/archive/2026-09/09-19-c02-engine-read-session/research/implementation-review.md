# C02 implementation review

Independent reviewer: `c01_check`, 2026-09-19. Scope is the production headless
generated-fixture read owner, not native GUI or private vault onboarding.

The reviewer found SDK transport EOF could leave a live child and a connected
runtime until another read. The owner now selects SDK `service.waiting()` against
stop and child exit, then retires under bounded budgets. The reviewer independently
ran the real SDK EOF regression successfully. No remaining C02 code finding.

Both `execution/evidence/c02-{release,main-preview}-session-v2.json` passed and the
reviewer matched every recorded source SHA and desktop binary SHA to the final
C02 files. Actual dispatch is `main.rs::dispatch_host`; repeated reads reuse child
and generation, return known full source, reject stale/wrong routes, leave the
physical corpus unchanged and close with cancelled transport, child exit 0,
no forced kill and no unknown timeout.

Owner validation: 226 Rust tests passed, one subprocess fixture ignored by the
ordinary runner but explicitly invoked by its EOF regression; 37 Python checks,
fmt and diff checks passed. Reviewer independently ran Python 3 checks and the
10 targeted session/EOF checks, and verified both live evidence artifacts.

Full app Clippy fails on 103 emitted diagnostics at 102 unique inherited IPC
spans. Reviewer checked all primary spans exist verbatim at baseline `251c74b`;
no suppression or unrelated repair was applied. Full unit wrapper still has the
historical later-task-before-G0 failure. These failures are not passing gates.

Root synchronized `engine-read-session.md`, the supervisor/typed-IPC scope
amendments and C01 connected capability truth for AC5. AC1–AC5 are supported
within the stated fixture/headless scope. Native GUI/window close, Windows Job
Object, process-tree containment, official forced-kill and sleep/resume remain
UNVERIFIED. C03 changes require new matching-revision integration receipts.
