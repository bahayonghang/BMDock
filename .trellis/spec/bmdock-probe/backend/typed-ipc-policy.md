# Typed IPC and Fixture Policy

This contract describes the T06 Tauri command boundary, the T07
`get_runtime_state` snapshot projection, the T09 read-only
`run_preflight` / `discover_config` commands, the T10 explicit
project/workspace route, the T11 paginated `list_tree` /
`read_note` commands, and the T12 `list_backups` /
`restore_fixture` baseline. It applies to
`apps/bmdock-desktop/src-tauri/src/ipc.rs`,
`apps/bmdock-desktop/src-tauri/src/library.rs`,
`apps/bmdock-desktop/src-tauri/src/backups.rs`,
`apps/bmdock-desktop/src-tauri/src/preflight.rs`,
`apps/bmdock-desktop/src-tauri/src/routing.rs`, and
`apps/bmdock-desktop/src/ipc.ts`; the P0 `bmdock-probe` and `just contract*`
interfaces remain separate. Lifecycle ownership lives in
[supervisor-state.md](./supervisor-state.md).

## 1. Scope / Trigger

- Trigger: a renderer needs to invoke Rust or listen for an application event.
- Scope: typed command and event DTOs, capability discovery, the
  fixture-only project policy, read-only projection of Supervisor
  runtime state, side-effect-free preflight / config discovery,
  BMDock-owned project/workspace listing with explicit routing,
  T11 paginated fixture-backed tree listing plus note read, and
  T12 BMDock-owned generated backup listing plus fixture restore.
- The boundary does not start or stop the Supervisor, call the official
  engine over rmcp, access a user vault, expose note write/edit/move/delete,
  or expose raw `callTool`.
- T09 preflight and discovery inspect BMDock-owned in-repo or explicitly
  generated fixture paths only. T10 lists only generated BMDock-owned
  workspace/project records. T07 remains the owner of start/stop.
- T11 `list_tree` and `read_note` and T12 `list_backups` /
  `restore_fixture` must carry an explicit `ExplicitRouteArgs`
  (`workspace` + `project`) on every call and must not inherit an implicit
  current project. T15 CRUD remains out of scope. T12 restores generated
  markdown into a generated owned target only; it is not user-vault
  restore and not T17/T37/T38 recovery.

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
`root`). Extra `path` / `root` fields fail closed as `schema`.

## 3. Contracts

### Request and response fields

- `get_capabilities` returns `kind: "capabilities"`, the ten command names,
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
  backup store returns `unsupported`. Forced-kill, Job Object,
  sleep-resume, and disk-failure remain UNVERIFIED; a successful fixture
  restore is not T17/T37/T38 evidence.
- Error responses use `kind: "error"` and `category` in `policy`, `schema`, or
  `unsupported`, plus a human-readable `message`. Do not add `timeout_unknown`,
  `transport`, or `process` to this IPC error union; those belong on the
  runtime-state DTO.

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
`RestoreFixtureArgs`, and `EmptyArgs` use `#[serde(deny_unknown_fields)]`.
There is no path field on `list_projects` / `run_preflight` /
`discover_config` / `list_tree` / `read_note` / `list_backups` /
`restore_fixture` and no raw `callTool`, search, or note write DTO or
handler. The renderer must not send arbitrary project paths or forward a
tool name and arguments through this boundary.

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
`EmptyBackupStore`. T07 may expose Supervisor
snapshot fields, but it still does not start the engine from the renderer.
T09 reports readiness from that snapshot without taking lifecycle ownership.
T10 `RouteState` records the explicit fixture selection for projection only;
T11 reads and T12 backup/restore still send `ExplicitRouteArgs` and must
not use the stored route as an implicit target.

## 4. Validation & Error Matrix

| Input or condition | Boundary behavior | Category |
| --- | --- | --- |
| Known command with its exact DTO | Dispatch the typed response | — |
| Unknown `command`, including `call_tool`, `search_notes`, `write_note` | Serde deserialization fails closed | `schema` at the boundary |
| Extra field in `args` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra `path` / `root` on `list_projects`, `run_preflight`, `discover_config`, `list_tree`, `read_note`, `list_backups`, or `restore_fixture` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra top-level field such as `path` beside `command`/`args` | `deny_unknown_fields` on `IpcCommand` | `schema` |
| `select_project` for any value other than `bmdock-fixture` | Dispatcher rejects without filesystem access | `policy` |
| `ExplicitRouteArgs` missing `project`/`workspace` or carrying an extra `path` | `deny_unknown_fields` rejects the DTO | `schema` |
| `ExplicitRouteArgs` with a non-fixture project or non-owned workspace | Helper rejects without filesystem access | `policy` |
| `list_tree` `page_size` 0 or greater than 64 | Reject without listing | `schema` |
| `list_tree` invalid cursor, empty cursor, or next-cursor loop | Reject; do not return a partial page | `schema` |
| Truncated or partial tree inventory | Reject; `truncated=true` is not a success | `unsupported` |
| `read_note` identifier that looks like a user vault filesystem path | Reject without opening the path | `policy` |
| `restore_fixture` `backup_id` that looks like a user vault / `%APPDATA%` / `.basic-memory` path | Reject without opening the backup store | `policy` |
| Inspect an arbitrary user path or real vault | Reject without opening the path | `policy` |
| Empty library `list_tree` | Empty `entries`, `next_cursor=null`, `truncated=false` | empty state |
| Empty library `read_note` | Engine/library unavailable | `unsupported` |
| Empty backup store `list_backups` | Empty `backups[]`, `files_written=false` | empty state |
| Empty backup store `restore_fixture` | Engine/backup store unavailable | `unsupported` |
| Restore into `%APPDATA%`, user Obsidian, or global Basic Memory config | Reject without writing | `policy` |
| Arbitrary path or raw `callTool` payload | No DTO/handler exists; never forward it | `schema` |
| Cross-project search or implicit current-project write | Keep it absent; catalog flags stay false | `unsupported` |
| Capability outside the current allowlist | Keep it absent and do not infer support | `unsupported` |
| Supervisor mutex is poisoned | Return an error response; do not panic | `unsupported` |

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
- Bad: send `{"command":"call_tool","args":{"name":"read_note"}}` or
  `{"command":"search_notes","args":{"query":"..."}}` or
  `{"command":"write_note","args":{"project":"bmdock-fixture"}}`;
  no raw tool, search, or write route is accepted or advertised.

## 6. Tests Required

- Rust unit test: capability response lists exactly ten commands and two
  events, and both arbitrary-path and raw-callTool policy flags are false.
  `list_tree`, `read_note`, `list_backups`, and `restore_fixture` are
  present; `call_tool`, `search_notes`, and `write_note` are absent.
- Rust unit test: a non-fixture project returns `ErrorCategory::Policy`.
- Rust unit test: unknown command including `call_tool`, extra project path,
  extra runtime-state path, extra `list_projects` path/root, extra preflight
  path, extra discovery path/root,   extra `list_tree` path, extra `read_note` path, extra `list_backups`
  path/root, and extra `restore_fixture` path all fail
  `serde_json::from_str::<IpcCommand>`. Incomplete
  `read_note` args (missing workspace/identifier) and incomplete
  `restore_fixture` args (missing backup_id) also fail closed.
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
  `restore_fixture` must not open the library or backup store.
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
- TypeScript `RuntimeStateDto` / `FailureKind` / `ShutdownReceipt` /
  `PreflightDto` / `ConfigDiscoveryDto` / `ProjectCatalogDto` /
  `TreePageDto` / `NoteReadDto` / `BackupCatalogDto` / `RestoreResultDto`
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
await listenTyped("runtime_state", (state) => renderState(state));
```

These calls use the shared DTOs and the explicit fixture/event allowlist.
`list_projects`, `run_preflight`, and `discover_config` take empty args.
`select_project` remains fixture-only. `list_tree`, `read_note`,
`list_backups`, and `restore_fixture` copy `ExplicitRouteArgs` on every
call and must not treat `runtime.project` as an implicit target.
`restore_fixture` restores generated markdown into a generated owned
target; it does not restore a user vault. Preflight reports
`supervisor_status` and `engine_spawned` from the snapshot without taking
start/stop ownership; listing backups does not set `files_written`.

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
