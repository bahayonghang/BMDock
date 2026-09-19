# C01 independent implementation review

Date: 2026-09-19. Reviewer: `trellis-check`, dispatched independently from
the C01 implementation owner. Scope excludes concurrent C02/C04 changes.

## Verdict

No remaining concrete C01 code finding after recheck. **Partial acceptance**:
AC1, AC3 and AC4 pass at their stated boundaries; AC2 remains incomplete while
the frozen 428-phase-cell timing matrix is largely unmeasured. Keep the task
`in_progress`. Selected samples and corpus reproducibility are not full coverage.

## Corrected findings and evidence

| Finding | Verified correction |
|---|---|
| Structured missing-note response counted as fast success | Operation-specific validators reject null/error identity, missing/full-source mismatch and truncated content |
| Protocol failure after Probe creation leaked ownership | Abort on failed post-construction validation; focused regression verifies cleanup |
| Manifest hash did not bind actual workload | Load/validate the frozen protocol, counts/scenarios and hash; no silent live-definition drift |
| Logged discovery args and search page differed from wire | Exact sent arguments retained; search uses `current_page` |
| Invalidated run could contribute coverage | Reject incomplete/failed, changed-corpus and wrong-profile-SHA reports |
| Native typing coordinates used raw CRLF string | Versioned v3 manifest uses normalized textarea API UTF-16 coordinates and retains raw/API lengths separately |

Both final profile-local contract records have no unresolved required records,
an unchanged physical corpus and normal close receipts. Complete YAML/Chinese/
CRLF source is physically equal for the observed 4 KiB fixture; default reads
strip frontmatter. The ordering probe returns the strongest last-alphabetical
identity before the weaker earlier identity. No universal byte-fidelity claim follows.

All ten v3 profile/scale corpus pairs were physically recreated twice with matching
hashes/labels/counts. v2-to-v3 changes are confined to typing coordinates and frozen
timestamp; backend workload is unchanged, but historical v2 samples are excluded
from authoritative v3 coverage. The actual native trace correlation is unverified.

## Executed verification

The final focused Python harness suite passes **17/17**. Python AST, whitespace
checks and C01 context validation pass. Earlier independent C01 checks passed
the five shell-producer behavior tests, six probe tests, TypeScript and probe
Clippy. The shell tests execute the real producer and prove delayed typed-state
outcomes; they do not establish C04 React race correctness or native performance.

The historical full-unit guard and G0–G7 failures remain unchanged. No long timing
matrix was rerun by the reviewer. Refresh only the artifact/coverage portion after
the implementation owner finishes its selected live measurements.
