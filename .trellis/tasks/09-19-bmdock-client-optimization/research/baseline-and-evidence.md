# Current baseline and evidence audit

Date: 2026-09-19. Source revision: `251c74b` on `main`. Product inspection was read-only.

## Verified present state

| Finding | Evidence | Consequence |
|---|---|---|
| Desktop code exists despite older P0-only guidance | `apps/bmdock-desktop/package.json:1`, `apps/bmdock-desktop/src-tauri/src/main.rs:70` | Route work by live code; document the current desktop scope. |
| Desktop wires empty providers and an unstarted supervisor | `apps/bmdock-desktop/src-tauri/src/main.rs:72` | Connect a real read provider before making production query-performance claims. |
| Task completion does not establish product acceptance | `execution/status.json:5`, `execution/status.json:11`, `execution/evidence/t40-release-gate-decision.json` | G0-G7 remain unpassed; do not infer release readiness. |
| Unit/CI wrapper rejects the present status | `scripts/tasks.py:79`, `scripts/tasks.py:86`, `scripts/tasks.py:124` | Phase-order inconsistency is reproduced. |
| Desktop tests mainly inspect source strings | `tests/test_desktop_shell.py:105`, `tests/test_desktop_shell.py:865`, `apps/bmdock-desktop/package.json:6` | They cannot prove delayed-response, editing, visual or timing behavior. |
| Hosted workflow remains G0 oriented | `.github/workflows/ci.yml:18`, `.github/workflows/ci.yml:44` | No current hosted frontend or native performance result was obtained. |

App.tsx has 9,593 lines, library.rs 10,286, and test_desktop_shell.py 2,470. These counts indicate concentrated responsibilities, not performance measurements or a standalone reason to refactor.

## Checks performed

| Command | Result | Interpretation |
|---|---|---|
| `python -m scripts.tasks gate` | Reports all G0-G7 unpassed; command contract returns 2 | Expected release boundary. |
| `python -m scripts.tasks unit` | Exit 1: `A later task was completed before G0` | Fails before unittest discovery. |
| `python -m unittest tests.test_desktop_shell -q` | 34 tests passed | Structural checks only. |
| `python -m unittest discover -s tests -v`, Python 3.14 | 94 tests ran, 41 error records, including phase order and Windows temporary-directory PermissionError | Environment-affected baseline; records are not necessarily distinct failing tests. |
| `py -V:Astral/CPython3.12.12 -m unittest discover -s tests -q` | No completed summary; stopped after prolonged lack of output | Incomplete diagnostic attempt, not a pass. |

Planning checks will be recorded in `planning-validation.md`. No native app, UI automation, installer, real user vault, live-engine benchmark or hosted CI was run. No latency, RSS, recall improvement or native visual acceptance is claimed.

C01 adds a documented desktop-focused check route while continuing to disclose the phase-order failure. It must not weaken the guard, mass-change historical T01-T40 state or claim full CI from a subset run. Reconciliation of historical stage policy and completion of all G0/G7 obligations remains outside this optimization plan.

The six existing untracked files in `.trellis/tasks/09-12-bmdock-product-completion/` were read but not edited. No commit, push, archive, global configuration or real-vault mutation was performed.
