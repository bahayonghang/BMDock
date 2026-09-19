# C01: Query contracts, performance baseline and validation entry

## Goal and dependency

Provide reproducible evidence and a frozen shared contract before implementation optimizes any path. No prerequisite child. Parent requirement R1.

## Confirmed facts

Desktop runtime wires empty stores (`apps/bmdock-desktop/src-tauri/src/main.rs:72`). The current unit wrapper stops on historical phase order (`scripts/tasks.py:79`), while frontend tests inspect strings (`tests/test_desktop_shell.py:105`). Parent research distinguishes fixture behavior and pinned engines.

## Requirements

- R1: Capture profile-local read/search/context/activity schemas and representative success/error envelopes, including count exactness, full-note reads and unsupported semantic/compact features.
- R2: Freeze repeatable fixtures, independent relevance labels and measurement rules before implementation claims a gain.
- R3: Define a narrow executable desktop check entry and dependency decision for delayed-response tests without concealing inherited CI/gate failures.

## Acceptance criteria

- [x] AC1 (R1): Separate release/main-preview contract records contain SHA, observed request/response examples, route identity, query filter/mode semantics, pagination and error mapping. Full-source requests explicitly include frontmatter; known UTF-8/YAML/CRLF fixture content is compared with the delivered string and upstream normalization is recorded separately. Unobserved transport shapes remain explicitly unresolved and block the adapter that depends on them.
- [ ] AC2 (R2): A rerun reproduces note count, fixture hashes, query labels and scenario configuration for 100/1,000/10,000 notes and large-note cases. Five cold/30 warm raw samples per supported measured scenario follow parent D4; no speedup is computed from EmptyLibrary.
- [x] AC3 (R2): A dated manifest freezes parent D4 budgets, hardware, estimator and scope before C03/C06 optimization, including the 30-edit input-to-presented-frame protocol and native p95 limits of 50 ms for 1 MiB and 100 ms for 5 MiB. Missing native or semantic baseline is labeled unavailable, with a reason; it is not represented as a zero score or passing timing.
- [x] AC4 (R3): The chosen desktop behavior-check command exercises an observable user-state transition with a delayed typed response and produces pass/fail output. It is distinct from full CI. Its documentation reports the existing unit/gate failure; no guard or historical gate status is weakened.

## Scope and exclusions

Own contract captures, measurement recipe/harness, scoped check documentation and one minimal executable test seam. No adapter/UI optimization, package installation without explicit authorization, global configuration, real vault or gate-state edits. Current planning only creates these artifacts.
