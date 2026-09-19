# C04 implementation preflight

Read-only review by `frontend_ui_engineer`, 2026-09-19. C01 supplies the typed
deferred test entry; this document does not claim C04 implementation or acceptance.

1. Keep retained editor sessions above `SectionBody` in `App`: changing sections
   or shell load phase currently unmounts the workbench. Sessions distinguish
   profile/workspace/project/note/editor purpose; changing request generations
   must not erase text. Initialize a session once rather than resetting on seed
   effects. Keep the new-note editor in one stable slot while typing its identifier.
2. A narrow production request module should own the actual note/detail/tree/search
   and diagnostic-profile operations, with injectable typed invoke. Guard success,
   error, cleanup and follow-on calls by identity. In particular, stale note loads
   must not launch relations/graph/context; pages retain their parent query generation.
3. Capture submitted body/revision for saves. Acknowledgement advances that baseline
   without replacing newer text. Occupy per-operation pending slots synchronously,
   preserve loaded content on local failures, and keep unsupported saves visibly
   unpersisted. No automatic mutation retries on unknown outcomes.
4. `toolProfile` selects diagnostic baselines, not the active engine. C02 must expose
   the actual profile and a session generation that changes on replacement, including
   same-profile replacement. C03 owns complete query/filter/mode/page identities.
   Do not invent a frontend session generation as proof of backend ownership.
5. Extend the existing C01 behavior command with actual extracted operations used by
   App. Reproduce A/B ordering, stale pages, save-A/type-B/ack-A, double activation,
   session navigation, local failures and obsolete follow-on counts before fixes.
   DOM, focus, native IME and presented-frame performance remain separate evidence.

Source anchors at base `251c74b`: `App.tsx:211,269,301,346,674,718,1618,1709,4456,4927,5014,5050`;
`ipc.ts:218,1210`. Full paths are under `apps/bmdock-desktop/src/`.

The inherited full-unit failure remains `A later task was completed before G0`;
all G0–G7 gates remain unpassed. This preflight does not change those records.
