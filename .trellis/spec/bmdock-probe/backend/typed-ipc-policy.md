# Typed IPC and Fixture Policy

This contract describes the T06 Tauri command boundary, the T07
`get_runtime_state` snapshot projection, and the T09 read-only
`run_preflight` / `discover_config` commands. It applies to
`apps/bmdock-desktop/src-tauri/src/ipc.rs`,
`apps/bmdock-desktop/src-tauri/src/preflight.rs`, and
`apps/bmdock-desktop/src/ipc.ts`; the P0 `bmdock-probe` and `just contract*`
interfaces remain separate. Lifecycle ownership lives in
[supervisor-state.md](./supervisor-state.md).

## 1. Scope / Trigger

- Trigger: a renderer needs to invoke Rust or listen for an application event.
- Scope: typed command and event DTOs, capability discovery, the
  fixture-only project policy, read-only projection of Supervisor
  runtime state, and side-effect-free preflight / config discovery.
- The boundary does not start or stop the Supervisor, call the official
  engine, access a user vault, expose note CRUD, or expose raw `callTool`.
- T09 preflight and discovery inspect BMDock-owned in-repo or explicitly
  generated fixture paths only. T07 remains the owner of start/stop.

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
run_preflight: {}
discover_config: {}
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

## 3. Contracts

### Request and response fields

- `get_capabilities` returns `kind: "capabilities"`, the five command names,
  the two event names, and a policy DTO.
- `get_runtime_state` returns `kind: "runtime_state"` projected from the
  managed `Supervisor` snapshot:
  - `status`: `not_started` / `starting` / `connected` / `stopping` /
    `stopped` / `failed`
  - `project`: always `null` until a later routing task
  - `profile`: `release` / `main-preview` from `EngineProfile::id()`, or
    `null`
  - `failure`: `FailureKind` as snake_case (`policy`, `transport`,
    `timeout_unknown`, `process`, `unverified`) or `null`
  - `shutdown`: `ShutdownReceipt` or `null`
  - `child_pid` is not part of the DTO
- `select_project` returns `kind: "project_selected"` only for the exact
  project `bmdock-fixture`.
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

`SelectProjectArgs` and `EmptyArgs` use `#[serde(deny_unknown_fields)]`.
There is no path field on `run_preflight` / `discover_config` and no raw
`callTool` DTO or handler. The renderer must not send arbitrary project
paths or forward a tool name and arguments through this boundary.

Inspecting an arbitrary user path or real vault is `policy` and must not
open the path. Extra path/root fields on empty-args commands fail closed as
`schema` via `deny_unknown_fields`.

### Boundary ownership

The Rust side is the trusted policy boundary. TypeScript checks the fixture
constant before invoking, while Rust remains authoritative and returns a
`policy` error for any non-fixture project. The official Basic Memory engine
remains the future data owner; this command surface does not read or write
user data. T07 may expose Supervisor snapshot fields, but it still does not
start the engine from the renderer. T09 reports readiness from that snapshot
without taking lifecycle ownership.

## 4. Validation & Error Matrix

| Input or condition | Boundary behavior | Category |
| --- | --- | --- |
| Known command with its exact DTO | Dispatch the typed response | — |
| Unknown `command`, including `call_tool` | Serde deserialization fails closed | `schema` at the boundary |
| Extra field in `args` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra `path` / `root` on `run_preflight` or `discover_config` | `deny_unknown_fields` rejects the DTO | `schema` |
| Extra top-level field such as `path` beside `command`/`args` | `deny_unknown_fields` on `IpcCommand` | `schema` |
| `select_project` for any value other than `bmdock-fixture` | Dispatcher rejects without filesystem access | `policy` |
| Inspect an arbitrary user path or real vault | Reject without opening the path | `policy` |
| Arbitrary path or raw `callTool` payload | No DTO/handler exists; never forward it | `schema` |
| Capability outside the current allowlist | Keep it absent and do not infer support | `unsupported` |
| Supervisor mutex is poisoned | Return an error response; do not panic | `unsupported` |

## 5. Good / Base / Bad Cases

- Good: invoke `select_project` with `{ project: "bmdock-fixture" }` and receive
  `project_selected`.
- Base: invoke `get_runtime_state` before start and receive `not_started`
  with `project: null`, `profile: null`, `failure: null`, and
  `shutdown: null`.
- Good: after Supervisor is connected on the `release` profile, the same
  command returns `status: "connected"` and `profile: "release"` without a
  `child_pid` field.
- Good: invoke `run_preflight` with empty args against a `not_started`
  snapshot and receive two isolated profile records plus
  `engine_spawned: false` / `files_written: false`.
- Good: after Supervisor is connected, the same command reports
  `engine_spawned: true` from lifecycle state and `files_written: false`
  because T09 writes nothing. It does not start or stop Supervisor.
- Base: invoke `discover_config` with empty args and receive `root: "none"`
  and an empty candidate list. That empty listing is not a user-vault
  success.
- Bad: send `{"command":"select_project","args":{"project":"bmdock-fixture","path":"C:\\vault"}}`; deserialization fails because the extra path is denied.
- Bad: send `{"command":"run_preflight","args":{"path":"C:\\vault"}}` or
  `{"command":"discover_config","args":{"root":"/home/user/.basic-memory"}}`;
  extra fields fail closed.
- Bad: send `{"command":"call_tool","args":{"name":"read_note"}}`; no
  raw tool route is accepted or advertised.

## 6. Tests Required

- Rust unit test: capability response lists exactly five commands and two
  events, and both arbitrary-path and raw-callTool policy flags are false.
- Rust unit test: a non-fixture project returns `ErrorCategory::Policy`.
- Rust unit test: unknown command, extra project path, extra runtime-state
  path, extra preflight path, and extra discovery path/root all fail
  `serde_json::from_str::<IpcCommand>`.
- Rust unit test: a connected snapshot projects `status`/`profile` and keeps
  `project` null; a stopped snapshot serializes `failure: "timeout_unknown"`,
  `profile: "main-preview"`, nested shutdown fields, and omits `child_pid`.
- Rust unit test: `run_preflight` does not spawn, does not write, and leaves
  a `not_started` snapshot unchanged. A connected or stopped snapshot reports
  `engine_spawned` from lifecycle and `files_written: false` without
  start/stop. Profiles stay isolated (21 vs 27,
  distinct commits). A non-owned path is `policy` and is not opened. Default
  discovery is empty, not success-with-user-vault.
- TypeScript `RuntimeStateDto` / `FailureKind` / `ShutdownReceipt` /
  `PreflightDto` / `ConfigDiscoveryDto` stay aligned with that JSON shape
  through `npm run build`.
- Validation checks: `task.py validate`, `cargo fmt --all -- --check`,
  `cargo test --workspace --locked --offline`,
  `cargo check --workspace --locked --offline`, and `git diff --check`.

## 7. Wrong vs Correct

### Wrong

```ts
invoke("callTool", { name: "read_note", arguments: { path } });
invoke("select_project", { project: userSuppliedPath });
invoke("discover_config", { root: userHomeBasicMemory });
```

This bypasses the command union, exposes a raw MCP route, and permits a path
outside the generated fixture.

### Correct

```ts
await selectFixtureProject();
await getRuntimeState();
await runPreflight();
await discoverConfig();
await listenTyped("runtime_state", (state) => renderState(state));
```

These calls use the shared DTOs and the explicit fixture/event allowlist.
`run_preflight` and `discover_config` take empty args. Preflight reports
`supervisor_status` and `engine_spawned` from the snapshot without taking
start/stop ownership; `files_written` stays false.

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
  project: null;
  profile: EngineProfile | null;
  failure: FailureKind | null;
  shutdown: ShutdownReceipt | null;
};
```

Keep IPC command errors and runtime failure/shutdown receipts on separate
fields. `getRuntimeState()` is how the renderer reads the latter. Preflight
reports `supervisor_status` and `engine_spawned` from the snapshot without
taking start/stop ownership. `files_written` stays false because T09
writes nothing.
