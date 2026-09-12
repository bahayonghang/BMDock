# Typed IPC and Fixture Policy

This contract describes the T06 Tauri command boundary, the T07
`get_runtime_state` snapshot projection, the T09 read-only
`run_preflight` / `discover_config` commands, the T10 explicit
project/workspace route, the T11 paginated `list_tree` /
`read_note` commands, the T12 `list_backups` /
`restore_fixture` baseline, the T13
`inspect_windows_runtime` Windows host prototype, the T14
`save_draft` / `load_draft` editor session, the T15
typed `write_note` / `edit_note` / `move_note` /
`delete_note` fixture note CRUD, and the T16 in-process
same-target conflict coordinator plus unknown-result no-retry
gate. It applies to
`apps/bmdock-desktop/src-tauri/src/ipc.rs`,
`apps/bmdock-desktop/src-tauri/src/library.rs`,
`apps/bmdock-desktop/src-tauri/src/conflict.rs`,
`apps/bmdock-desktop/src-tauri/src/backups.rs`,
`apps/bmdock-desktop/src-tauri/src/drafts.rs`,
`apps/bmdock-desktop/src-tauri/src/preflight.rs`,
`apps/bmdock-desktop/src-tauri/src/routing.rs`,
`apps/bmdock-desktop/src-tauri/src/windows_runtime.rs`, and
`apps/bmdock-desktop/src/ipc.ts`; the P0 `bmdock-probe` and `just contract*`
interfaces remain separate. Lifecycle ownership lives in
[supervisor-state.md](./supervisor-state.md).

## 1. Scope / Trigger

- Trigger: a renderer needs to invoke Rust or listen for an application event.
- Scope: typed command and event DTOs, capability discovery, the
  fixture-only project policy, read-only projection of Supervisor
  runtime state, side-effect-free preflight / config discovery,
  BMDock-owned project/workspace listing with explicit routing,
  T11 paginated fixture-backed tree listing plus note read,
  T12 BMDock-owned generated backup listing plus fixture restore,
  T13 Windows host runtime prototype observation, T14
  BMDock-owned draft persistence plus editor session, T15
  fixture-backed typed note write/edit/move/delete, and T16
  in-process same-identifier inflight conflict coordination
  plus timeout_unknown no-retry.
- The boundary does not start or stop the Supervisor, call the official
  engine over rmcp, access a user vault, or expose raw `callTool`. T14
  drafts are BMDock-owned session artifacts, not a second note index and
  not official engine persist. T15 typed `write_note` / `edit_note` /
  `move_note` / `delete_note` are host commands on `NoteLibrary`, not
  raw MCP `callTool`. Production `EmptyLibrary` CRUD is unsupported
  (`engine/library unavailable`). Tests inject `FixtureLibrary` over
  generated owned temp `{temp}/bmdock-t15-*`. T16 wraps those same typed
  CRUD commands with `ConflictCoordinator`; it does not add a new IPC
  command or rmcp.
- T09 preflight and discovery inspect BMDock-owned in-repo or explicitly
  generated fixture paths only. T10 lists only generated BMDock-owned
  workspace/project records. T07 remains the owner of start/stop.
- T11 `list_tree` and `read_note`, T12 `list_backups` /
  `restore_fixture`, T14 `save_draft` / `load_draft`, and T15
  `write_note` / `edit_note` / `move_note` / `delete_note` must carry an
  explicit `ExplicitRouteArgs` (`workspace` + `project`) on every call and
  must not inherit an implicit current project. T16 coordinates overlapping
  same-identifier inflight writes on those CRUD commands: the second
  overlapping same-target call returns `classified_as: conflict` (not
  `disk_verified`, not `timeout_unknown`, not policy-for-path). Sequential
  dest-exists remains `unsupported` and is not T16 atomicity. True
  concurrent OS-thread filesystem races remain UNVERIFIED; in-process
  inflight is host coordination, not an OS file lock. T12 restores generated markdown into a generated owned target
  only; it is not user-vault restore and not T17/T37/T38 recovery.
  T14 writes generated draft bytes into a generated owned temp root
  `{temp}/bmdock-t14-*/` in tests only. T15 writes generated note markdown
  into `{temp}/bmdock-t15-*/` in tests only.
- T13 `inspect_windows_runtime` takes `EmptyArgs`. It observes host OS,
  well-known WebView2 install-dir/loader files, Job Object API documentation,
  and `tauri.conf.json` `bundle.active`. It does not launch a WebView2/Tauri
  window, create or assign a Job Object, enable installer bundling, spawn
  an engine, or scan user Obsidian / Basic Memory home. Compiled exe / npm
  build / cargo test are not native GUI. WebView2 files present is not a
  WebView2 session. Job Object API/docs is not Job Object assignment.
  `just contract` is not Windows runtime evidence. T12 fixture restore is
  not Windows recovery. Signing remains T37.

## 2. Signatures

Rust exposes one Tauri command:

```rust
#[tauri::command]
fn ipc_invoke(command: ipc::IpcCommand) -> ipc::IpcResponse
```

`IpcCommand` is a tagged DTO with `command` and `args` fields:

```text
get_capabilities: {}
get_runtime_state: {}
select_project: { project: "bmdock-fixture" }
list_projects: {}
run_preflight: {}
discover_config: {}
list_tree: { workspace, project, cursor?, page_size? }
read_note: { workspace, project, identifier }
list_backups: { workspace, project }
restore_fixture: { workspace, project, backup_id }
inspect_windows_runtime: {}
save_draft: { workspace, project, identifier, body }
load_draft: { workspace, project, identifier }
write_note: { workspace, project, identifier, title, body }
edit_note: { workspace, project, identifier, body }
move_note: { workspace, project, identifier, destination }
delete_note: { workspace, project, identifier }
```

The renderer uses the matching `IpcCommand` union through:

```ts
invokeTyped<T extends IpcResponse>(command: IpcCommand): Promise<T>
listenTyped<K extends IpcEventName>(
  event: K,
  handler: (payload: IpcEventPayload[K]) => void,
): Promise<UnlistenFn>
```

The only event names are `runtime_state` and `policy`; `listenTyped` prefixes
them with `bmdock://` before calling Tauri `listen`.

Future read/write DTOs use:

```rust
#[serde(deny_unknown_fields)]
struct ExplicitRouteArgs { workspace: String, project: String }
```

T11 `list_tree` and `read_note` consume this struct on every call. `read_note`
uses a note identifier/permalink/title field, not a user-vault filesystem
`path`. T12 `list_backups` consumes this struct on every call.
`restore_fixture` adds a generated `backup_id` (not a filesystem `path` or
`root`). T14 `save_draft` and `load_draft` consume this struct on every call
plus a draft `identifier` (permalink, not a filesystem `path`) and, for
save, a `body`. T15 `write_note` / `edit_note` / `move_note` /
`delete_note` consume this struct on every call plus a note `identifier`
(permalink, not a filesystem `path`). `write_note` also takes `title` and
`body`. `edit_note` takes `body`. `move_note` takes `destination` as a
permalink/identifier, not a filesystem path. Extra `path` / `root` fields
fail closed as `schema`.

## 3. Contracts

### Request and response fields

- `get_capabilities` returns `kind: "capabilities"`, the seventeen command names,
  the two event names, and a policy DTO.
- `get_runtime_state` returns `kind: "runtime_state"` projected from the
  managed `Supervisor` snapshot plus T10 `RouteState`:
  - `status`: `not_started` / `starting` / `connected` / `stopping` /
    `stopped` / `failed`
  - `project`: `null` until `select_project` accepts `bmdock-fixture`, then
    `"bmdock-fixture"`. This projection is not an implicit write target.
  - `profile`: `release` / `main-preview` from `EngineProfile::id()`, or
    `null`
  - `failure`: `FailureKind` as snake_case (`policy`, `transport`,
    `timeout_unknown`, `process`, `unverified`) or `null`
  - `shutdown`: `ShutdownReceipt` or `null`
  - `child_pid` is not part of the DTO
- `select_project` returns `kind: "project_selected"` only for the exact
  project `bmdock-fixture`. It updates `RouteState` to workspace
  `bmdock-workspace` and project `bmdock-fixture`. It does not write files
  or start Supervisor.
- `list_projects` returns `kind: "project_catalog"` with BMDock-owned
  records only: workspace `bmdock-workspace` and project `bmdock-fixture`.
  Default listing is this generated catalog. It does not scan or open a
  user Obsidian vault or global Basic Memory home. Flags:
  `scanned_user_obsidian_vault=false`,
  `scanned_user_basic_memory_home=false`,
  `cross_project_search_allowed=false`,
  `implicit_current_project_writes=false`,
  `cloud_or_credential_required=false`, `local_offline=true`,
  `files_written=false`.
- `run_preflight` returns `kind: "preflight"` with two isolated profile
  records and host checks. It does not spawn an engine, start or stop
  Supervisor, or write files. `engine_spawned` is projected from the
  existing Supervisor lifecycle (`not_started` is false; starting /
  connected / stopping / stopped / failed are true). `files_written`
  stays false because T09 writes nothing. Profiles remain `release`
  (`c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048`, 21 tools) and `main-preview`
  (`3452c821d76c083823d020984d71e06904a1ff1e`, 27 tools). Never mix, merge,
  or average the two records.
- `discover_config` returns `kind: "config_discovery"`. The default root is
  `"none"` with an empty candidate list. Empty discovery is the empty state,
  not success-with-user-vault. The command does not scan the user's real
  Basic Memory home, copy a production `config.json`, or rewrite one.
- `list_tree` returns `kind: "tree_page"` with `entries[]`, `next_cursor`
  (null on the last page), `page`, and `truncated=false`. Args are
  `ExplicitRouteArgs` plus optional `cursor` and bounded `page_size`
  (default 20, reject 0 / greater than 64 as `schema`). Invalid cursor,
  repeated next-cursor loops, and truncated/partial inventory fail closed
  as `schema` or `unsupported`; never return a silent partial success or
  an unbounded tree dump. Missing library and disconnected Supervisor yield
  empty `entries` (empty state), not a user-vault success. The command does
  not scan `%APPDATA%`, user Obsidian vaults, or global Basic Memory home.
  Official `list_directory` MCP remains UNVERIFIED.
- `read_note` returns `kind: "note_read"` with `title`, `identifier`,
  markdown `body`, and an observation DTO. The identifier is a permalink or
  title, not a filesystem `path` field. Observation never treats a success
  envelope as disk proof (`envelope_is_not_disk_proof=true`). Fixture tests
  write BMDock-owned markdown (Chinese text and wiki links allowed) and
  assert the returned body matches the physical file (`classified_as:
  body_matches_disk`, `disk_verified=true`). Production default without an
  installed fixture library returns `unsupported` (`engine/library
  unavailable`). Official engine `read_note` MCP remains UNVERIFIED.
- `list_backups` returns `kind: "backup_catalog"` with BMDock-owned
  generated fixture backup records only. Args are `ExplicitRouteArgs`.
  Flags: `scanned_user_obsidian_vault=false`,
  `scanned_user_basic_memory_home=false`, `local_offline=true`,
  `cloud_or_credential_required=false`. `files_written` is true only after
  a restore that actually wrote owned files; listing itself does not write.
  An empty `backups[]` is empty state, not a user-vault success. The
  command does not scan `%APPDATA%`, user Obsidian vaults, or global Basic
  Memory home.
- `restore_fixture` returns `kind: "fixture_restored"` with `backup_id`,
  restored file identifiers, `files_written`, and an observation DTO.
  Args are `ExplicitRouteArgs` plus a generated `backup_id`. The command
  copies generated markdown from a named backup snapshot into a generated
  BMDock-owned target directory. Tests must observe physical files after
  restore (body/path exist). Text `"restored"` is not disk proof
  (`envelope_is_not_disk_proof=true`). Classify `disk_verified` vs
  `accepted_unverified`. Do not restore into `%APPDATA%`, user Obsidian,
  or global Basic Memory config. Production default without an installed
  backup store returns `unsupported`.   Forced-kill, Job Object,
  sleep-resume, and disk-failure remain UNVERIFIED; a successful fixture
  restore is not T17/T37/T38 evidence.
- `inspect_windows_runtime` returns `kind: "windows_runtime"` with a DTO
  that separates observed facts from UNVERIFIED claims. Args are
  `EmptyArgs`. Extra `path` / `root` fail closed as `schema`. Required
  snake_case fields, mirrored in `ipc.ts`:
  - `host_os`: `windows` / `other`
  - `webview2_files_present`: Evergreen/loader file or well-known
    EdgeWebView install-dir observation only
  - `webview2_session_verified`: false unless a WebView2/Tauri window was
    actually launched and interacted with
  - `job_object_assigned`: false unless a real Job Object was created AND
    a child assigned
  - `job_object_api_documented`: Windows Job Object is the intended
    process-tree mechanism; API presence/docs is not assignment proof
  - `installer_bundle_active`: must match `tauri.conf.json` `bundle.active`
    (currently false). T13 does not enable bundling or signing
  - `files_written`: false
  - `scanned_user_obsidian_vault`: false
  - `scanned_user_basic_memory_home`: false
  The command does not spawn engines, mix release/main-preview tool
  counts, open user vaults, start or stop Supervisor, or produce an
  installer. Evidence taxonomy that must stay encoded in tests and
  evidence JSON:
  - compiled exe / npm build / cargo test ≠ native GUI
  - WebView2 files present ≠ WebView2 session
  - Job Object API/docs ≠ Job Object assigned
  - `just contract` ≠ Windows runtime
  - T12 fixture restore ≠ Windows recovery
- `save_draft` returns `kind: "draft_saved"` with `identifier`, `body`,
  `files_written`, `engine_persisted=false`, vault-scan flags false, and an
  observation DTO. Args are `ExplicitRouteArgs` plus `identifier` and
  `body`. Drafts are BMDock-owned session artifacts, not official engine
  writes and not a second note index. Tests must observe physical draft
  files after save (exact body roundtrip, including Chinese text and
  wiki-link `[[欢迎]]`). Envelope `"saved"` is not disk proof
  (`envelope_is_not_disk_proof=true`). Classify `disk_verified` vs
  `accepted_unverified`. `files_written` is true only after owned draft
  bytes exist. Do not write into `%APPDATA%`, user Obsidian, or global
  Basic Memory config. Do not call MCP `write_note`. Production default
  `EmptyDraftStore` save is `unsupported` with
  `"engine/draft store unavailable"` (not `"engine/library unavailable"`).
  Tests inject `FixtureDraftStore` over a generated owned temp root
  `{temp}/bmdock-t14-*/`.
- `load_draft` returns `kind: "draft_loaded"` with the same DTO. Args are
  `ExplicitRouteArgs` plus `identifier`. An empty session (`body=""`,
  `classified_as: empty`, `files_written=false`, `engine_persisted=false`)
  is empty state, not a user-vault success. The editor session can save and
  reload the same identifier. Renderer dirty/in-memory is distinct from
  `disk_verified` and from `engine_persisted` (which must stay false).
  Drafts remain distinct from T15 note CRUD.
- `write_note` returns `kind: "note_written"` with `identifier`, `title`,
  `body`, `files_written`, `engine_persisted=false`, vault-scan flags false,
  and a CRUD observation DTO. Args are `ExplicitRouteArgs` plus
  `identifier`, `title`, and `body`. Tests must observe the physical
  markdown file after write (exact body, including Chinese text and
  wiki-link `[[欢迎]]`). Envelope `"saved"` is not disk proof
  (`envelope_is_not_disk_proof=true`). Classify `disk_verified` vs
  `accepted_unverified`. Do not write into `%APPDATA%`, user Obsidian, or
  global Basic Memory config. Do not add rmcp/live MCP. Production default
  `EmptyLibrary` write is `unsupported` with
  `"engine/library unavailable"`. Tests inject `FixtureLibrary` over
  `{temp}/bmdock-t15-*`.
- `edit_note` returns `kind: "note_edited"` with `identifier`, `body`,
  `files_written`, `engine_persisted=false`, and observation. It
  overwrite/replaces the body of an existing identifier. Tests observe the
  physical file. Missing identifier is `unsupported`.
- `move_note` returns `kind: "note_moved"` with source `identifier`,
  `destination`, `body`, `files_written`, `engine_persisted=false`, and
  observation. Destination is a permalink/identifier, not a filesystem
  path. After a fixture move, the old path is gone and the new path exists
  with   the same body. Filesystem destination is `policy` and does not open
  the library. Sequential move onto an existing destination is
  `unsupported` and is not T16 same-target conflict or atomic
  concurrent overwrite. Overlapping same-target inflight (including a
  move whose source or destination is already inflight) is
  `classified_as: conflict` on the CRUD DTO.
- `delete_note` returns `kind: "note_deleted"` with `identifier`,
  `files_written`, `engine_persisted=false`, and observation. Missing after
  delete is success-with-observation (`disk_verified` when the file is
  confirmed absent), not user-vault success. Envelope-only delete is
  `accepted_unverified`.
- Overlapping same-target `write_note` / `edit_note` / `move_note` /
  `delete_note` returns the matching CRUD DTO with
  `observation.classified_as: conflict`, `disk_verified=false`,
  `files_written=false`. Conflict is not an IPC error category, not
  `timeout_unknown`, and not policy-for-path. Distinct-target sequential
  success stays `disk_verified` and is not same-target atomicity.
- After `timeout_unknown` on `RuntimeStateDto` / `ShutdownReceipt`, do
  not auto-retry non-idempotent writes. `ConflictCoordinator` /
  `auto_retry_non_idempotent_write` must not invoke the retry helper.
  `timeout_unknown` stays on the runtime snapshot; the IPC error union
  stays `policy` / `schema` / `unsupported`. T16 records conflict and
  unknown results; it is not T17/T37 recovery. Forced-kill, Job Object,
  and disk-failure remain UNVERIFIED.
- Error responses use `kind: "error"` and `category` in `policy`, `schema`, or
  `unsupported`, plus a human-readable `message`. Do not add `timeout_unknown`,
  `transport`, `process`, or `conflict` to this IPC error union; runtime
  failure kinds belong on the runtime-state DTO, and same-target conflict
  belongs on the CRUD observation class.

### Policy fields

The capability policy must report:

```json
{
  "project": "bmdock-fixture",
  "arbitrary_paths_allowed": false,
  "raw_call_tool_allowed": false
}
```

`SelectProjectArgs`, `ExplicitRouteArgs`, `ListTreeArgs`, `ReadNoteArgs`,
`RestoreFixtureArgs`, `SaveDraftArgs`, `LoadDraftArgs`, `WriteNoteArgs`,
`EditNoteArgs`, `MoveNoteArgs`, `DeleteNoteArgs`, and `EmptyArgs`
use `#[serde(deny_unknown_fields)]`.
There is no path field on `list_projects` / `run_preflight` /
`discover_config` / `list_tree` / `read_note` / `list_backups` /
`restore_fixture` / `inspect_windows_runtime` / `save_draft` /
`load_draft` / `write_note` / `edit_note` / `move_note` /
`delete_note` and no raw `callTool` or search DTO or handler. Typed
`write_note` is a host command, not raw MCP. The renderer must not send
arbitrary project paths or forward a tool name and arguments through this
boundary.

Inspecting an arbitrary user path or real vault is `policy` and must not
open the path. Extra path/root fields on empty-args commands fail closed as
`schema` via `deny_unknown_fields`.

### Boundary ownership

The Rust side is the trusted policy boundary. TypeScript checks the fixture
constant before invoking, while Rust remains authoritative and returns a
`policy` error for any non-fixture project. The official Basic Memory engine
remains the future data owner; T11 injects a `NoteLibrary` trait so tests
can install a fixture-backed store without starting rmcp or mixing engine
profiles. Production default is `EmptyLibrary`. T12 injects a
`BackupStore` trait so tests can install a fixture-backed backup snapshot
and owned restore target without starting rmcp. Production default is
`EmptyBackupStore`. T14 injects a `DraftStore` trait so tests can install
a fixture-backed owned draft root without starting rmcp or calling
`write_note`. Production default is `EmptyDraftStore`. T07 may expose
Supervisor snapshot fields, but it still does not start the engine from
the renderer. T09 reports readiness from that snapshot without taking
lifecycle ownership. T10 `RouteState` records the explicit fixture selection
for projection only; T11 reads, T12 backup/restore, and T14 drafts still
send `ExplicitRouteArgs` and must not use the stored route as an implicit
target. T13 inspects the Windows host prototype without taking Supervisor
start/stop ownership. T14 editor session does not start or stop Supervisor.
T15 CRUD also sends `ExplicitRouteArgs` on every call; `RouteState.project`
is not an implicit write target. T15 does not start Supervisor or add rmcp.
T16 wraps those CRUD commands with `ConflictCoordinator` and refuses
auto-retry after `timeout_unknown`. It does not add a command, start
Supervisor, or add rmcp.

## 4. Validation & Error Matrix

| Input or condition | Boundary behavior | Category |
| --- | --- | --- |
| Known command with its exact DTO | Dispatch the typed response | — |
| Unknown `command`, including `call_tool` and `search_notes` | Serde deserialization fails closed | `schema` at the boundary |
| Incomplete `write_note` args (for example only `project`) | `deny_unknown_fields` / missing fields | `schema` |
| Extra field in `args` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra `path` / `root` on `list_projects`, `run_preflight`, `discover_config`, `list_tree`, `read_note`, `list_backups`, `restore_fixture`, `inspect_windows_runtime`, `save_draft`, `load_draft`, `write_note`, `edit_note`, `move_note`, or `delete_note` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra top-level field such as `path` beside `command`/`args` | `deny_unknown_fields` on `IpcCommand` | `schema` |
| `select_project` for any value other than `bmdock-fixture` | Dispatcher rejects without filesystem access | `policy` |
| `ExplicitRouteArgs` missing `project`/`workspace` or carrying an extra `path` | `deny_unknown_fields` rejects the DTO | `schema` |
| `ExplicitRouteArgs` with a non-fixture project or non-owned workspace | Helper rejects without filesystem access | `policy` |
| `list_tree` `page_size` 0 or greater than 64 | Reject without listing | `schema` |
| `list_tree` invalid cursor, empty cursor, or next-cursor loop | Reject; do not return a partial page | `schema` |
| Truncated or partial tree inventory | Reject; `truncated=true` is not a success | `unsupported` |
| `read_note` identifier that looks like a user vault filesystem path | Reject without opening the path | `policy` |
| `save_draft` / `load_draft` identifier that looks like a user vault filesystem path | Reject without opening the draft store | `policy` |
| `write_note` / `edit_note` / `delete_note` identifier that looks like a user vault filesystem path | Reject without opening the library | `policy` |
| `move_note` destination that looks like a filesystem path | Reject without opening the library | `policy` |
| Non-fixture `write_note` / `edit_note` / `move_note` / `delete_note` | Reject without opening the library | `policy` |
| Empty library `write_note` / `edit_note` / `move_note` / `delete_note` | Engine/library unavailable | `unsupported` |
| Envelope-only `"saved"` write/edit/move/delete | Classify `accepted_unverified`; not disk proof | — |
| Sequential move onto an existing destination | Reject; not T16 same-target conflict or atomic concurrent overwrite | `unsupported` |
| Overlapping same-target inflight write/edit/move/delete | Return CRUD DTO `classified_as: conflict`; do not write; not policy-for-path | — |
| Distinct-target sequential writes | Each may be `disk_verified`; this is not same-target atomicity | — |
| `timeout_unknown` on Supervisor snapshot / shutdown receipt | Stay on `RuntimeStateDto`; do not auto-retry non-idempotent writes; do not add to IPC error union | — |
| `restore_fixture` `backup_id` that looks like a user vault / `%APPDATA%` / `.basic-memory` path | Reject without opening the backup store | `policy` |
| Inspect an arbitrary user path or real vault | Reject without opening the path | `policy` |
| Empty library `list_tree` | Empty `entries`, `next_cursor=null`, `truncated=false` | empty state |
| Empty library `read_note` | Engine/library unavailable | `unsupported` |
| Empty backup store `list_backups` | Empty `backups[]`, `files_written=false` | empty state |
| Empty backup store `restore_fixture` | Engine/backup store unavailable | `unsupported` |
| Empty draft store `load_draft` | Empty session, `files_written=false`, `engine_persisted=false` | empty state |
| Empty draft store `save_draft` | Engine/draft store unavailable | `unsupported` |
| Save draft into `%APPDATA%`, user Obsidian, or global Basic Memory config | Reject without writing | `policy` |
| Restore into `%APPDATA%`, user Obsidian, or global Basic Memory config | Reject without writing | `policy` |
| Arbitrary path or raw `callTool` payload | No DTO/handler exists; never forward it | `schema` |
| Cross-project search or implicit current-project write | Keep it absent; catalog flags stay false | `unsupported` |
| Capability outside the current allowlist | Keep it absent and do not infer support | `unsupported` |
| Supervisor mutex is poisoned | Return an error response; do not panic | `unsupported` |
| `inspect_windows_runtime` on this host | Return observed host OS / WebView2 files / Job Object API docs / `bundle.active`; keep session and Job Object assignment false unless proven | — |

## 5. Good / Base / Bad Cases

- Good: invoke `select_project` with `{ project: "bmdock-fixture" }` and receive
  `project_selected`. Subsequent `get_runtime_state` projects
  `project: "bmdock-fixture"`.
- Base: invoke `get_runtime_state` before start and before select and receive
  `not_started` with `project: null`, `profile: null`, `failure: null`, and
  `shutdown: null`.
- Good: invoke `list_projects` with empty args and receive workspace
  `bmdock-workspace` plus project `bmdock-fixture`, with
  `cross_project_search_allowed=false` and
  `implicit_current_project_writes=false`.
- Good: after Supervisor is connected on the `release` profile, the same
  command returns `status: "connected"` and `profile: "release"` without a
  `child_pid` field. `project` stays null until an explicit fixture select.
- Good: invoke `run_preflight` with empty args against a `not_started`
  snapshot and receive two isolated profile records plus
  `engine_spawned: false` / `files_written: false`.
- Good: after Supervisor is connected, the same command reports
  `engine_spawned: true` from lifecycle state and `files_written: false`
  because T09 writes nothing. It does not start or stop Supervisor.
- Base: invoke `discover_config` with empty args and receive `root: "none"`
  and an empty candidate list. That empty listing is not a user-vault
  success.
- Base: invoke `list_tree` with explicit fixture route and no installed
  library; receive empty `entries`, `next_cursor: null`, `truncated: false`.
  That empty listing is not a user-vault success.
- Good: install a fixture library of BMDock-owned markdown, page with
  `page_size=2`, follow `next_cursor` until null, and read a note whose
  body matches the physical file including Chinese text and a wiki link.
- Base: invoke `list_backups` with explicit fixture route and no installed
  backup store; receive empty `backups[]` and `files_written=false`. That
  empty listing is not a user-vault success.
- Good: install a generated fixture backup snapshot, restore it into a
  generated owned target, and observe the physical markdown file (Chinese
  body and wiki link). Classify `disk_verified` with `files_written=true`.
  Envelope-only `"restored"` is `accepted_unverified` and not disk proof.
- Good: invoke `inspect_windows_runtime` with empty args and receive
  `host_os` plus observed WebView2 files / Job Object API docs /
  `installer_bundle_active` matching `tauri.conf.json`. `webview2_session_verified`
  and `job_object_assigned` stay false unless a native window was actually
  interacted with and a Job Object was created and assigned. Compiled exe,
  npm build, cargo test, `just contract`, and T12 fixture restore are not
  those proofs.
- Base: invoke `load_draft` with explicit fixture route and no installed
  draft store; receive an empty session (`body=""`, `files_written=false`,
  `engine_persisted=false`). That empty session is not a user-vault success.
- Good: install a generated owned draft root `{temp}/bmdock-t14-*`, save
  a draft whose body includes Chinese text and `[[欢迎]]`, observe the
  physical file, then load the same identifier and round-trip the body.
  Classify `disk_verified` with `files_written=true`. Envelope-only
  `"saved"` is `accepted_unverified` and not disk proof. `engine_persisted`
  stays false.
- Good: install a generated owned fixture library `{temp}/bmdock-t15-*`,
  `write_note` a body including Chinese text and `[[欢迎]]`, observe the
  physical file, then `edit_note` overwrite, `move_note` within the owned
  library, and `delete_note` until the file is missing. Classify
  `disk_verified`. Envelope-only `"saved"` is `accepted_unverified`.
  `engine_persisted` stays false. Overlapping same-target inflight is
  `conflict`. Sequential dest-exists stays `unsupported`. Dual
  profiles stay isolated.
- Good: two overlapping same-identifier typed writes; the second
  returns `classified_as: conflict` with `files_written=false`. A
  sequential write to a distinct identifier remains `disk_verified`
  and is not same-target atomicity. After a `timeout_unknown`
  runtime snapshot, `auto_retry_non_idempotent_write` does not
  invoke the retry helper. Forced-kill / Job Object / disk-failure
  stay UNVERIFIED and are not T17/T37 recovery.
- Bad: send `{"command":"select_project","args":{"project":"bmdock-fixture","path":"C:\\vault"}}`; deserialization fails because the extra path is denied.
- Bad: send `{"command":"list_projects","args":{"path":"C:\\vault"}}` or
  `{"command":"list_projects","args":{"root":"/home/user/.basic-memory"}}`;
  extra fields fail closed.
- Bad: send `{"command":"run_preflight","args":{"path":"C:\\vault"}}` or
  `{"command":"discover_config","args":{"root":"/home/user/.basic-memory"}}`;
  extra fields fail closed.
- Bad: send `{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}`
  or `read_note` with an extra `path`; extra fields fail closed.
- Bad: send `{"command":"read_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"C:\\\\Users\\\\someone\\\\vault\\\\note.md"}}`;
  policy rejects the filesystem identifier without opening it.
- Bad: send `{"command":"list_backups","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}`
  or `restore_fixture` with an extra `path` / `root`; extra fields fail closed.
- Bad: send `{"command":"restore_fixture","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","backup_id":"%APPDATA%\\\\Obsidian"}}`;
  policy rejects the filesystem backup id without opening the store.
- Bad: send `{"command":"inspect_windows_runtime","args":{"path":"C:\\vault"}}` or
  `{"command":"inspect_windows_runtime","args":{"root":"/home/user/.basic-memory"}}`;
  extra fields fail closed.
- Bad: send `{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"x","path":"C:\\vault"}}`
  or `load_draft` with an extra `path` / `root`; extra fields fail closed.
- Bad: send `{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"%APPDATA%\\\\Obsidian\\\\note.md","body":"x"}}`;
  policy rejects the filesystem identifier without opening the store.
- Bad: send `{"command":"call_tool","args":{"name":"read_note"}}` or
  `{"command":"search_notes","args":{"query":"..."}}` or
  `{"command":"write_note","args":{"project":"bmdock-fixture"}}`;
  raw tool/search stay absent; incomplete typed `write_note` is schema.
- Bad: send `write_note` / `edit_note` / `move_note` / `delete_note` with
  extra `path` / `root`; extra fields fail closed as schema.
- Bad: send `write_note` with a non-fixture project or a filesystem
  identifier; policy rejects without opening the library. `RouteState.project`
  is not an implicit write target.
- Bad: send `move_note` with destination `C:\\Users\\someone\\vault\\note.md`;
  filesystem destination is policy.

## 6. Tests Required

- Rust unit test: capability response lists exactly seventeen commands and two
  events, and both arbitrary-path and raw-callTool policy flags are false.
  `list_tree`, `read_note`, `list_backups`, `restore_fixture`,
  `inspect_windows_runtime`, `save_draft`, `load_draft`, `write_note`,
  `edit_note`, `move_note`, and `delete_note` are present; `call_tool` and
  `search_notes` are absent. Incomplete `write_note` args remain schema.
- Rust unit test: a non-fixture project returns `ErrorCategory::Policy`.
- Rust unit test: unknown command including `call_tool`, extra project path,
  extra runtime-state path, extra `list_projects` path/root, extra preflight
  path, extra discovery path/root,   extra `list_tree` path, extra `read_note` path, extra `list_backups`
  path/root, extra `restore_fixture` path, extra
  `inspect_windows_runtime` path/root, extra `save_draft` path/root, and extra
  `load_draft` path/root all fail
  `serde_json::from_str::<IpcCommand>`. Incomplete
  `read_note` args (missing workspace/identifier), incomplete
  `restore_fixture` args (missing backup_id), incomplete `save_draft`
  args (missing body), and incomplete `load_draft` args (missing
  identifier) also fail closed.
- Rust unit test: a connected snapshot projects `status`/`profile` and keeps
  `project` null until fixture select; after select, `project` is
  `bmdock-fixture` without writing files or starting Supervisor. A stopped
  snapshot serializes `failure: "timeout_unknown"`, `profile: "main-preview"`,
  nested shutdown fields, and omits `child_pid`.
- Rust unit test: `list_projects` returns only BMDock-owned records, does not
  scan user vaults, and keeps `cross_project_search_allowed` and
  `implicit_current_project_writes` false.   `ExplicitRouteArgs` requires both
  fields, rejects extra paths as schema, and rejects non-fixture routes as
  policy. Non-fixture `list_tree` / `read_note` / `list_backups` /
  `restore_fixture` / `save_draft` / `load_draft` / `write_note` /
  `edit_note` / `move_note` / `delete_note` must not open the library,
  backup store, or draft store.
- Rust unit test: `run_preflight` does not spawn, does not write, and leaves
  a `not_started` snapshot unchanged. A connected or stopped snapshot reports
  `engine_spawned` from lifecycle and `files_written: false` without
  start/stop. Profiles stay isolated (21 vs 27,
  distinct commits). A non-owned path is `policy` and is not opened. Default
  discovery is empty, not success-with-user-vault.
- Rust unit test: `list_tree` paginates a fixture library (`entries`,
  `next_cursor`, `page`, `truncated=false`). Invalid cursor, `page_size` 0 or
  huge, and truncated inventory fail closed. `read_note` body matches the
  physical fixture file; envelope-only success is not `disk_verified`.
  Empty library listing is empty, not a user-vault success; empty-library
  `read_note` is `unsupported`.
- Rust unit test: `list_backups` returns only BMDock-owned generated backup
  ids, does not scan user vaults, and keeps `files_written` false until a
  restore actually writes owned files. Empty catalog is empty state, not a
  user-vault success. `restore_fixture` copies generated markdown into a
  generated owned target; tests observe the physical file body/path.
  Envelope-only `"restored"` is `accepted_unverified` and not
  `disk_verified`. Filesystem backup ids and forbidden restore roots
  (`%APPDATA%`, Obsidian, `.basic-memory`) are `policy` and do not open the
  store. Empty-store restore is `unsupported`. Forced-kill, Job Object,
  sleep-resume, and disk-failure stay UNVERIFIED.
- Rust unit test: `inspect_windows_runtime` uses `EmptyArgs`, does not spawn,
  does not write, does not open the library, backup store, or draft store, and does not
  start Supervisor. The DTO separates observed facts from UNVERIFIED
  claims. `webview2_session_verified` and `job_object_assigned` stay false
  unless proven. `installer_bundle_active` matches committed
  `tauri.conf.json` `bundle.active` (false). Taxonomy encoded in tests:
  compiled exe / npm build / cargo test ≠ native GUI; WebView2 files ≠
  session; Job Object API/docs ≠ assigned; `just contract` ≠ Windows
  runtime (no `expected_tools`); T12 fixture restore ≠ Windows recovery.
  Dual profiles stay isolated (21 vs 27, distinct commits). On Windows,
  `webview2_files_present` may follow a real well-known install-dir
  observation; that still does not verify a session.
- Rust unit test: `save_draft` / `load_draft` require `ExplicitRouteArgs`.
  Extra `path` / `root` fail closed as `schema`. Non-fixture routes and
  filesystem identifiers are `policy` and do not open the store. Empty
  store load is empty session, not a user-vault success; empty-store save
  is `unsupported` with `"engine/draft store unavailable"`. Fixture store
  save writes `{temp}/bmdock-t14-*/welcome.md`; tests observe the physical
  body including Chinese text and `[[欢迎]]`, then reload the same
  identifier. `classified_as=disk_verified`, `files_written=true`,
  `engine_persisted=false`. Envelope-only `"saved"` is
  `accepted_unverified`. Dual profiles stay isolated. T15 CRUD is a
  separate typed host path; `save_draft` is not `write_note`.
- Rust unit test: `write_note` / `edit_note` / `move_note` /
  `delete_note` require `ExplicitRouteArgs`. Extra `path` / `root` fail closed as
  `schema`. Non-fixture routes and filesystem identifiers/destinations are
  `policy` and do not open the library. Empty library CRUD is
  `unsupported` with `"engine/library unavailable"`. Fixture library write
  writes `{temp}/bmdock-t15-*/welcome.md`; tests observe the physical body
  including Chinese text and `[[欢迎]]`. Edit overwrites that identifier.
  Move leaves the old path gone and the new path present with the same
  body. Delete observes the file missing. `classified_as=disk_verified`,
  `engine_persisted=false`. Envelope-only `"saved"` is
  `accepted_unverified`. Sequential dest-exists stays `unsupported` and
  is not T16. Dual
  profiles stay isolated.
- Rust unit test: `ConflictCoordinator` rejects a second overlapping
  same-identifier inflight write/edit/move/delete as
  `classified_as: conflict` (not `disk_verified`, not
  `timeout_unknown`, not `policy`). Distinct-target sequential writes
  succeed and must not be recorded as same-target atomicity. Two
  overlapping OS threads against the inflight guard observe a conflict
  class; that is host coordination, not an OS file lock. True
  concurrent OS filesystem races stay UNVERIFIED.
- Rust unit test: `timeout_unknown` remains on `RuntimeStateDto` /
  `ShutdownReceipt`. The IPC error union stays `policy` / `schema` /
  `unsupported`. After `timeout_unknown`, `auto_retry_non_idempotent_write`
  does not invoke the retry helper. A user-initiated typed write after
  that snapshot is not an auto-retry and must not add `timeout_unknown`
  to the CRUD error category. Forced-kill, Job Object, and disk-failure
  stay UNVERIFIED; T16 is not T17/T37 recovery.
- TypeScript `RuntimeStateDto` / `FailureKind` / `ShutdownReceipt` /
  `PreflightDto` / `ConfigDiscoveryDto` / `ProjectCatalogDto` /
  `TreePageDto` / `NoteReadDto` / `BackupCatalogDto` / `RestoreResultDto` /
  `WindowsRuntimeDto` / `DraftResultDto` / `NoteWriteDto` / `NoteEditDto` /
  `NoteMoveDto` / `NoteDeleteDto` / `NoteCrudClass` (`conflict` included)
  stay aligned with that JSON shape through
  `npm run build`.
- Validation checks: `task.py validate`, `cargo fmt --all -- --check`,
  `cargo test --workspace --locked --offline`,
  `cargo check --workspace --locked --offline`, and `git diff --check`.

## 7. Wrong vs Correct

### Wrong

```ts
invoke("callTool", { name: "read_note", arguments: { path } });
invoke("select_project", { project: userSuppliedPath });
invoke("list_projects", { root: userHomeBasicMemory });
invoke("search_notes", { query: "all projects" });
invoke("write_note", { title: "x" }); // implicit current project
invoke("callTool", { name: "write_note", arguments: { title: "x" } });
invoke("save_draft", { path: userVaultFile });
invoke("read_note", { path: userVaultFile });
invoke("restore_fixture", { path: userAppData });
```

This bypasses the command union, exposes a raw MCP route, permits a path
outside the generated fixture, or invents cross-project search / implicit
writes.

### Correct

```ts
await listProjects();
await selectFixtureProject();
await getRuntimeState();
await runPreflight();
await discoverConfig();
const route = copyFixtureRoute();
await invokeTyped({
  command: "list_tree",
  args: { workspace: route.workspace, project: route.project, page_size: 20 },
});
await invokeTyped({
  command: "read_note",
  args: { workspace: route.workspace, project: route.project, identifier },
});
await invokeTyped({
  command: "list_backups",
  args: { workspace: route.workspace, project: route.project },
});
await invokeTyped({
  command: "restore_fixture",
  args: {
    workspace: route.workspace,
    project: route.project,
    backup_id: "fixture-welcome",
  },
});
await inspectWindowsRuntime();
await invokeTyped({
  command: "save_draft",
  args: {
    workspace: route.workspace,
    project: route.project,
    identifier,
    body,
  },
});
await invokeTyped({
  command: "load_draft",
  args: { workspace: route.workspace, project: route.project, identifier },
});
await invokeTyped({
  command: "write_note",
  args: {
    workspace: route.workspace,
    project: route.project,
    identifier,
    title,
    body,
  },
});
await invokeTyped({
  command: "edit_note",
  args: { workspace: route.workspace, project: route.project, identifier, body },
});
await invokeTyped({
  command: "move_note",
  args: {
    workspace: route.workspace,
    project: route.project,
    identifier,
    destination,
  },
});
await invokeTyped({
  command: "delete_note",
  args: { workspace: route.workspace, project: route.project, identifier },
});
await listenTyped("runtime_state", (state) => renderState(state));
```

These calls use the shared DTOs and the explicit fixture/event allowlist.
`list_projects`, `run_preflight`, and `discover_config` take empty args.
`select_project` remains fixture-only. `list_tree`, `read_note`,
`list_backups`, `restore_fixture`, `save_draft`, `load_draft`,
`write_note`, `edit_note`, `move_note`, and `delete_note` copy
`ExplicitRouteArgs` on every call and must not treat `runtime.project` as
an implicit target.
`restore_fixture` restores generated markdown into a generated owned
target; it does not restore a user vault. `save_draft` / `load_draft`
persist BMDock-owned session drafts, not official engine notes. Typed
`write_note` is a host command on `NoteLibrary`, not raw `callTool`.
Preflight reports
`supervisor_status` and `engine_spawned` from the snapshot without taking
start/stop ownership; listing backups does not set `files_written`.
`inspect_windows_runtime` takes empty args, does not start Supervisor, and
must not treat compiled binaries, WebView2 files, Job Object docs,
`just contract`, or T12 fixture restore as native GUI / session /
assignment / installer / recovery proof.
T15 does not start Supervisor and does not add rmcp. `save_draft` stays
distinct from `write_note`. T16 same-target overlapping inflight is
`classified_as: conflict` on the CRUD DTO. `timeout_unknown` stays on
`RuntimeStateDto`. Sequential dest-exists stays `unsupported`. True OS
filesystem races, forced-kill, Job Object, and disk-failure stay
UNVERIFIED and are not T17/T37 recovery.

### Wrong

```ts
type ErrorCategory = "policy" | "schema" | "unsupported" | "timeout_unknown";
```

This collapses a Supervisor lifecycle outcome into the IPC transport error
union. Renderer code can no longer tell a bad command from an unresolved
engine shutdown.

### Correct

```ts
type ErrorCategory = "policy" | "schema" | "unsupported";
type FailureKind = "policy" | "transport" | "timeout_unknown" | "process" | "unverified";

type RuntimeStateDto = {
  status: RuntimeStatus;
  project: typeof FIXTURE_PROJECT | null;
  profile: EngineProfile | null;
  failure: FailureKind | null;
  shutdown: ShutdownReceipt | null;
};
```

Keep IPC command errors and runtime failure/shutdown receipts on separate
fields. `getRuntimeState()` is how the renderer reads the latter. Selected
`project` is a routing projection, not an implicit write target. Preflight
reports `supervisor_status` and `engine_spawned` from the snapshot without
taking start/stop ownership. `files_written` stays false because T09
writes nothing.

### Wrong

```ts
type NoteCrudClass = "disk_verified" | "timeout_unknown";
autoRetryNonIdempotentWrite();
```

This collapses `timeout_unknown` into the CRUD observation class or retries
a non-idempotent write after an unknown result.

### Correct

```ts
type ErrorCategory = "policy" | "schema" | "unsupported";
type NoteCrudClass = "empty" | "disk_verified" | "accepted_unverified" | "conflict" | "unclassified";
type FailureKind = "policy" | "transport" | "timeout_unknown" | "process" | "unverified";
```

Keep `conflict` on the CRUD observation, `timeout_unknown` on
`RuntimeStateDto`, and never auto-retry the unresolved write.
