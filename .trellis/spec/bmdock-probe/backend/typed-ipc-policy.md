# Typed IPC and Fixture Policy

This contract describes the T06 Tauri command boundary. It applies to
`apps/bmdock-desktop/src-tauri/src/ipc.rs` and
`apps/bmdock-desktop/src/ipc.ts`; the P0 `bmdock-probe` and `just contract*`
interfaces remain separate.

## 1. Scope / Trigger

- Trigger: a renderer needs to invoke Rust or listen for an application event.
- Scope: typed command and event DTOs, capability discovery, and the
  fixture-only project policy implemented by T06.
- The boundary does not start the Supervisor, call the official engine, access
  a vault, or expose note CRUD.

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
- `get_runtime_state` returns `kind: "runtime_state"`, `status: "not_started"`,
  and `project: null`, `profile: null` until a later task owns the Supervisor.
- `select_project` returns `kind: "project_selected"` only for the exact
  project `bmdock-fixture`.
- Error responses use `kind: "error"` and `category` in `policy`, `schema`, or
  `unsupported`, plus a human-readable `message`.

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
remains the future data owner; T06 does not read or write user data.

## 4. Validation & Error Matrix

| Input or condition | Boundary behavior | Category |
| --- | --- | --- |
| Known command with its exact DTO | Dispatch the typed response | — |
| Unknown `command`, including `call_tool` | Serde deserialization fails closed | `schema` at the boundary |
| Extra field in `args` | `deny_unknown_fields` rejects the DTO | `schema` |
| `select_project` for any value other than `bmdock-fixture` | Dispatcher rejects without filesystem access | `policy` |
| Arbitrary path or raw `callTool` payload | No DTO/handler exists; never forward it | `schema` |
| Capability outside the T06 allowlist | Keep it absent and do not infer support | `unsupported` |

## 5. Good / Base / Bad Cases

- Good: invoke `select_project` with `{ project: "bmdock-fixture" }` and receive
  `project_selected`.
- Base: invoke `get_runtime_state` before T07 and receive the explicit
  `not_started` state with null project/profile.
- Bad: send `{"command":"select_project","args":{"project":"bmdock-fixture","path":"C:\\vault"}}`; deserialization fails because the extra path is denied.
- Bad: send `{"command":"call_tool","args":{"name":"read_note"}}`; no
  raw tool route is accepted or advertised.

## 6. Tests Required

- Rust unit test: capability response lists exactly three commands and two
  events, and both arbitrary-path and raw-callTool policy flags are false.
- Rust unit test: a non-fixture project returns `ErrorCategory::Policy`.
- Rust unit test: unknown command, extra project path, and extra runtime-state
  path all fail `serde_json::from_str::<IpcCommand>`.
- Renderer build check: TypeScript command and event unions compile through
  `npm run build`.
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
