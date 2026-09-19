# Ordered implementation plan

Implementation was approved on 2026-09-19 with `设计没有问题，继续`.
The subsequent representative-first decision bounds this delivery to code and
representative measurements; full-matrix and native acceptance stay open.
See `research/implementation-progress.md` and C07's integration report for results.

## Task/dependency map

| Task | Deliverable | Prerequisite |
|---|---|---|
| C01 | Query contracts, baseline and desktop check route | none |
| C02 | Official engine read session | C01 |
| C03 | Bounded query execution and paging | C02 |
| C04 | Request/editor correctness | C01 |
| C05 | Workspace layout and reading | C04 |
| C06 | Search UX and rendering efficiency | C03, C04, C05 |
| C07 | Integrated functional/performance/native evidence | C01-C06 |

C02/C03 may run beside C04 only after shared identities are frozen and files have exclusive owners. Shared ipc.ts changes have one owner per batch. C04 -> C05 -> C06 is serial because they share the workbench. Tree links encode ownership, not execution dependencies.

C05 layout and C06 typing acceptance remain incomplete without their native
evidence. Under the user's later representative-first instruction, C07 integrates
the available code and receipts while preserving those missing prerequisites;
its activation does not declare C01-C06 fully accepted. Reuse applicable receipts
and rerun only scenarios invalidated by later changes.

## Execution

1. Apply the approved plan. C01 freezes pinned schemas, measurement manifest and evidence limits.
2. Connect isolated official reads (C02/C03) and repair request/editor correctness (C04). Regression tests must reproduce relevant failures, not merely find strings.
3. Complete C05/C06 with retained session ownership; measure before considering new caching, editor or virtualization dependencies.
4. Run C07 separately for both profiles. Keep missing native measurements incomplete and update specs/docs only for verified behavior.
5. Review parent AC1-AC7 and report inherited gate failures. Commit, push, archive, release and real-vault onboarding remain separate authority stages.

## Checks

Planning: `python .trellis/scripts/task.py validate <task>`, tree/manifests checks and `git diff --check`.

Desktop checks: `python -m unittest tests.test_desktop_shell -q` (34 source checks), `cargo fmt --all -- --check`, `cargo test -p bmdock-app --locked --offline` and relevant probe tests. Frontend: `npm --prefix apps/bmdock-desktop run build` and `npm --prefix apps/bmdock-desktop run test:behavior`, using installed dependencies. C07 records the final counts and evidence boundaries.

Integration uses a named desktop read/benchmark entry; `just contract` and `just contract-main` remain separate probe evidence. `python -m scripts.tasks unit` presently fails phase order and `python -m scripts.tasks gate` remains nonpassing. A subset must not be labeled full CI.

## Rollback and final review

C02/C03 rollback reports disconnected/unavailable. C04-C06 rollback preserves dirty content and never reverses disk data. C01/C07 do not rewrite past evidence. Source anchors must be refreshed after code moves.

Separate planning validation, source inspection, unit/contract checks, isolated live-engine results, mock renderer results, native visual/IME/accessibility/timing evidence and release gates. None substitutes for another.
