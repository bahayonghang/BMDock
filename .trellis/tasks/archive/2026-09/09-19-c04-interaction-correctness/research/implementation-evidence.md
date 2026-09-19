# C04 implementation evidence

Date: 2026-09-19. Scope: production frontend state ownership and deterministic
typed-IPC behavior. No native/browser UI operation, dependency installation,
durable storage, real-vault access or backend change by this worker.

## Production ownership

- `App.tsx` owns one app-lifetime `EditSessions` and `WorkbenchRequests` instance.
  Editor keys contain actual runtime profile, explicit fixture route, note seed
  identifier and draft/CRUD purpose. The new-note editor uses one stable slot.
  Selection is retained per profile; returning to the workbench reads that note
  again but does not reseed retained text. Discard is an explicit action.
- `workbenchRequests.ts` synchronously admits pending work and ignores obsolete
  completion/error/cleanup callbacks. Search replacement invalidates its page;
  note replacement invalidates its related work; diagnostic profile changes
  invalidate tools/CLI/audit independently of the actual engine profile.
  Navigation, shell refresh and runtime events invalidate work synchronously.
  Initial tree loading checks owner lifetime in addition to effect cleanup.
- Actual asynchronous operations remain named functions in `App.tsx`; tests
  import those same functions with typed invoke injection. No duplicate request
  implementation exists only in tests. `openNote` checks currency before each
  downstream relation/graph/context request. Note/page errors remain local.
- Connected tree/read calls include C02 `expected_session`; shell projection
  includes backend `session_generation`. Request identity contains actual
  profile/generation, route, page size and existing operation inputs. Cursor
  identities use query/page/cursor scalars, not serialized result bodies.
  No unsupported query filters/modes were invented ahead of C03.
- `editSessions.ts` retains pending/dirty text across transient panel unmounts.
  Save acknowledgements advance verified baseline without replacing current
  text. Unsupported/rejected saves preserve text. Draft reload cannot replace
  typing made after the request; replacing existing dirty text requires an
  explicit second confirmation. Pending mutation slots do not retry.

## Checks performed

| Check | Result | Evidence boundary |
|---|---|---|
| `npm run test:behavior` | **27/27 PASS** after review fixes | Node 26.7.0; actual typed production operations and retained editor owner |
| Controlled pre-fix behavior restoration | **8 expected failures / 19 tests** | Disabled currency guard and restored response-body overwrite temporarily; stale note/query/page/profile, obsolete errors, save/reload overwrites reproduced |
| Restored fixed behavior | **19/19 PASS**, then **20/20 PASS** after retention observation was added | No mutation code remains |
| `npm run build` | **PASS** | TypeScript 5.9.2 + Vite 7.1.7, 38 modules; approved local execution for installed esbuild spawn |
| `python -m unittest tests.test_desktop_shell -q` | **34/34 PASS** | Root adjusted two source-location checks to include extracted editor implementation; safety assertions preserved |
| `git diff --check` | **PASS** | Workspace check at execution time |

Behavior scenarios cover A/B note and query completion in both orders; old note
error and cleanup while B remains pending; no obsolete note follow-on calls;
new query vs old page; duplicate page/mutation activation; diagnostic profile
supersession and owner disposal; preserved results on query/page failure;
save-A/type-B/ack-A; retained editor/purpose/profile separation; unsupported and
rejected save without retry; actual `edit_note` deduplication; delayed draft
reload and explicit discard.

The negative-control run restored the **relevant former behavior** in the
current production seams. It was not a checkout/run of the entire historical
commit. Its failures establish regression-test sensitivity without claiming
an observed native incident.

## Retention and evidence limits

The large-editor retention scenario kept independent 1 MiB and 5 MiB edited
strings available without eviction: total retained current-body UTF-8 payload
6,291,456 bytes. One Node-host run observed heapUsed delta 11,602,848 bytes.
Garbage collection was uncontrolled; this includes transient/string allocation
  effects and is **not** a native memory limit, steady-state RSS measurement or
typing-performance result. No silent eviction or memory threshold was added.

CRLF/Chinese assertions establish exact JavaScript string retention through
these state operations. They do not prove textarea API line-ending behavior,
IME composition, filesystem roundtrip or native presented-frame timing.
Native layout, focus traversal, screen reader and IME remain **UNVERIFIED**;
this worker did not operate a browser or desktop UI.

The inherited full-unit failure is `A later task was completed before G0`;
G0-G7 remain unpassed according to root's independent check. Narrow frontend
checks do not weaken those guards or establish release/native acceptance.
Independent C04 review is pending at the time this evidence file was written.

## Independent-review fixes and final rerun

The reviewer found cross-operation detail ownership and previous-profile CLI
paging defects not covered by the original 20 tests. The owner fixed them:
automatic note details now capture a shared detail-intent generation, including
their initial clearing and subsequent requests; a newer explicit preview/query
or graph intent prevents old automatic detail work from committing or launching.
Diagnostic profile changes clear old tools/CLI/audit data, and CLI paging checks
profile identity before invocation and before merge.

Reload confirmation is keyed by editor session and revision rather than a
component boolean. Response-driven editor target changes clear delete
confirmation in the retained store. A successful tree-page retry clears its
previous local error. Returning to a selected note retains the last delivered
note snapshot while rereading, so a local reread failure cannot hide its editor.

Five additional executable regressions cover delayed automatic details versus
explicit preview, delayed note-read clearing versus explicit preview, wrong
profile cursor/response, tree retry alert and response-driven delete target.
At that review-fix checkpoint: **25/25 behavior tests**, **34/34 structural checks**, full
TypeScript/Vite build (**38 modules**) and `git diff --check` all passed.
The reload-confirmation session/revision expression was source-reviewed;
no renderer/DOM test is claimed. Independent reviewer closeout remains the
separate authoritative review artifact.

The review then identified the later-admission counterpart: an old graph page
could be requested after replacement graph loading had begun. The concrete
request owner now refuses page admission synchronously while the corresponding
parent replacement is pending. The same scoped check was applied once to every
workbench paging owner: graph, search, CLI, tree, activity, resources and prompts.
Initial tree loading now uses the named `loadTree` operation as well as its
lifetime guard. Same-parent pending pages remain deduplicated.

The actual graph-operation interleaving and the complete page-owner admission
matrix brought the owner result to **27/27 behavior tests PASS**. The independent
reviewer performs the final build/source-check rerun after these last changes;
its closeout report supplies those final results. No unrelated full-engine or
native verification was repeated.
