# BMDock implementation constraints

Read README.md, docs/IMPLEMENTATION.md, docs/VERIFICATION.md and execution/status.json before making changes.

- Preserve P0–P7 / G0–G7 order from the 2026-09-08 implementation plan. P0 probes are developer tooling, not T05/product completion.
- Do not connect real vaults or user/Agent config for tests. Generate fresh sandbox/config and strip inherited provider credentials/routing.
- Reuse the official Basic Memory engine. Do not write its private database or build a second authoritative knowledge index.
- Do not conflate release/main schemas, RPC success, business success, file materialization and search readiness.
- Do not blindly retry unknown writes. File hash comparison is not atomic CAS with Obsidian.
- Every completion claim needs actual command, exit status, engine/SDK ref, OS and evidence. Unit mocks are not real MCP or native desktop proof.
- CI must fail when prerequisites are missing; do not suppress errors, skip required suites or mark blocked capabilities passed.
- Only explicit setup/lock commands may fetch or change dependency resolutions; ci/dev/build must not upgrade the engine.
- Never force-push, overwrite concurrent user changes, publish a release, or alter repository settings without explicit authorization.
