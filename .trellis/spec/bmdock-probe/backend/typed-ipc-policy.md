# Typed IPC and Fixture Policy

This contract describes the T06 Tauri command boundary and the T07
`get_runtime_state` snapshot projection. It applies to
`apps/bmdock-desktop/src-tauri/src/ipc.rs` and
`apps/bmdock-desktop/src/ipc.ts`; the P0 `bmdock-probe` and `just contract*`
interfaces remain separate. Lifecycle ownership lives in
[supervisor-state.md](./supervisor-state.md).

## 1. Scope / Trigger

- Trigger: a renderer needs to invoke Rust or listen for an application event.
- Scope: typed command and event DTOs, capability discovery, the
  fixture-only project policy, and read-only projection of Supervisor
  runtime state.
- The boundary does not start or stop the Supervisor, call the official
  engine, access a vault, or expose note CRUD.

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

- `get_capabilities` returns `kind: "capabilities"`, the three command names,
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
There is no path field and no raw `callTool` DTO or handler. The renderer must
not send arbitrary project paths or forward a tool name and arguments through
this boundary.

### Boundary ownership

The Rust side is the trusted policy boundary. TypeScript checks the fixture
constant before invoking, while Rust remains authoritative and returns a
`policy` error for any non-fixture project. The official Basic Memory engine
remains the future data owner; this command surface does not read or write
user data. T07 may expose Supervisor snapshot fields, but it still does not
start the engine from the renderer.

## 4. Validation & Error Matrix

| Input or condition | Boundary behavior | Category |
| --- | --- | --- |
| Known command with its exact DTO | Dispatch the typed response | — |
| Unknown `command`, including `call_tool` | Serde deserialization fails closed | `schema` at the boundary |
| Extra field in `args` | `deny_unknown_fields` rejects the DTO | `schema` |
| `select_project` for any value other than `bmdock-fixture` | Dispatcher rejects without filesystem access | `policy` |
| Arbitrary path or raw `callTool` payload | No DTO/handler exists; never forward it | `schema` |
| Capability outside the T06 allowlist | Keep it absent and do not infer support | `unsupported` |
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
- Bad: send `{"command":"select_project","args":{"project":"bmdock-fixture","path":"C:\\vault"}}`; deserialization fails because the extra path is denied.
- Bad: send `{"command":"call_tool","args":{"name":"read_note"}}`; no
  raw tool route is accepted or advertised.

## 6. Tests Required

- Rust unit test: capability response lists exactly three commands and two
  events, and both arbitrary-path and raw-callTool policy flags are false.
- Rust unit test: a non-fixture project returns `ErrorCategory::Policy`.
- Rust unit test: unknown command, extra project path, and extra runtime-state
  path all fail `serde_json::from_str::<IpcCommand>`.
- Rust unit test: a connected snapshot projects `status`/`profile` and keeps
  `project` null; a stopped snapshot serializes `failure: "timeout_unknown"`,
  `profile: "main-preview"`, nested shutdown fields, and omits `child_pid`.
- TypeScript `RuntimeStateDto` / `FailureKind` / `ShutdownReceipt` stay aligned
  with that JSON shape through `npm run build`.
- Validation checks: `task.py validate`, `cargo fmt --all -- --check`,
  `cargo test --workspace --locked --offline`,
  `cargo check --workspace --locked --offline`, and `git diff --check`.

## 7. Wrong vs Correct

### Wrong

```ts
invoke("callTool", { name: "read_note", arguments: { path } });
invoke("select_project", { project: userSuppliedPath });
```

This bypasses the command union, exposes a raw MCP route, and permits a path
outside the generated fixture.

### Correct

```ts
await selectFixtureProject();
await getRuntimeState();
await listenTyped("runtime_state", (state) => renderState(state));
```

These calls use the shared DTOs and the explicit fixture/event allowlist.

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
fields. `getRuntimeState()` is how the renderer reads the latter.
