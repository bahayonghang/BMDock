# Engine Supervisor and Runtime State

This contract records the T07 lifecycle boundary in
`apps/bmdock-desktop/src-tauri/src/supervisor.rs` and its typed projection in
`src-tauri/src/ipc.rs` / `apps/bmdock-desktop/src/ipc.ts`. The P0 probe remains
a separate developer tool.

## 1. Scope / Trigger

- Trigger: Tauri needs to own an engine child process and report its lifecycle
  without exposing a raw MCP control channel.
- Scope: profile identity, lifecycle transitions, failure classification,
  injected transport/child seams, bounded two-stage shutdown, and the
  `get_runtime_state` DTO projection.
- This module does not register start/stop IPC, raw `callTool`, vault
  routing, or note CRUD. `just contract` / `just contract-main` and
  `bmdock-probe` stay independent.
- Real Basic Memory engine handshake, native GUI behavior, and cross-platform
  job-object / process-tree evidence remain `UNVERIFIED` until directly
  exercised. `Drop` kill is host-handle cleanup, not that evidence.

## 2. Signatures

```rust
pub enum EngineProfile { Release, MainPreview }
pub enum ConnectionState {
    NotStarted, Starting, Connected, Stopping, Stopped, Failed,
}
pub enum FailureKind { Policy, Transport, TimeoutUnknown, Process, Unverified }

pub struct ShutdownReceipt {
    pub transport_cancelled: bool,
    pub child_exited: bool,
    pub forced: bool,
    pub timeout_unknown: bool,
    pub exit_code: Option<i32>,
}

pub struct RuntimeSnapshot {
    pub state: ConnectionState,
    pub profile: Option<EngineProfile>,
    pub child_pid: Option<u32>,
    pub failure: Option<FailureKind>,
    pub shutdown: Option<ShutdownReceipt>,
}

impl Supervisor {
    pub fn snapshot(&self) -> RuntimeSnapshot;
    pub fn start(&mut self, spec: EngineLaunchSpec, child: Box<dyn ChildHandle>)
        -> Result<(), FailureKind>;
    pub fn spawn(&mut self, spec: EngineLaunchSpec) -> Result<(), FailureKind>;
    pub fn mark_connected(&mut self) -> Result<(), FailureKind>;
    pub fn mark_failed(&mut self, kind: FailureKind) -> Result<(), FailureKind>;
    pub fn shutdown<T: Transport>(&mut self, transport: &mut T, timeout: Duration)
        -> Result<ShutdownReceipt, FailureKind>;
}
```

`EngineProfile::Release` and `MainPreview` retain independent fixed commits
and expected tool counts (`c0bd87c6…` / 21 and `3452c821…` / 27). They must
never be merged into one runtime capability set.

`FailureKind` is a runtime-snapshot category. It is not an IPC
`ErrorCategory`. The IPC error union stays `policy` / `schema` /
`unsupported`.

## 3. Contracts

- Valid start states are `NotStarted` and `Stopped`; start enters `Starting`.
- `spawn` and `start` must reject empty program and illegal states **before**
  creating or replacing a child. `spawn` may call `OwnedChild::spawn` only
  after that policy check. OS spawn I/O failure then maps to `Process`.
- Only `Starting` may become `Connected`; invalid transitions return `policy`.
- `mark_failed` is valid only while starting or connected and preserves the
  failure category and last profile.
- Shutdown is valid from starting, connected, or failed. It enters
  `Stopping`, calls transport cancellation first, waits for the child
  within the supplied budget, then kills only after a wait timeout/unknown
  result, and ends in `Stopped` with a `ShutdownReceipt`.
- After `Failed`, the next profile may start only once shutdown has reached
  `Stopped`. Sequential profiles are not a merged capability set.
- `get_runtime_state` projects `RuntimeSnapshot` into typed DTO fields:
  `status`, `profile` (`EngineProfile::id()`, kebab-case), `failure`, and
  `shutdown`. `project` stays `null` until a later routing task. `child_pid`
  stays internal. Process exit does not claim disk materialization.
- The Tauri managed state owns `Supervisor` behind `Mutex`; a poisoned lock
  returns an `unsupported` IPC error instead of panicking.
- `Drop` for `Supervisor` / `OwnedChild` kills a remaining handle. That is not a
  `ShutdownReceipt`, not a retry, and not Job Object / process-tree proof.

## 4. Validation & Error Matrix

| Condition | Result |
| --- | --- |
| Start/spawn while starting, connected, stopping, or failed | `FailureKind::Policy`; existing child is not replaced; no new OS process |
| Empty program | `FailureKind::Policy`; no OS process is created |
| Eligible start, then OS spawn I/O failure | `FailureKind::Process` |
| Connected before starting | `FailureKind::Policy` |
| Transport cancel returns timed out/failed | receipt marks `timeout_unknown`; no retry is issued |
| Child wait exits normally | `child_exited=true`, `forced=false`, exit code preserved |
| Child wait times out or is unknown | bounded kill attempted; `timeout_unknown=true` and `forced` records kill result |
| Supervisor lock is poisoned at IPC boundary | typed `unsupported` response |
| Drop without shutdown | remaining handle is killed; no receipt; not Job Object evidence |

## 5. Good / Base / Bad Cases

- Good: select one profile, spawn one owned child, mark it connected, cancel
  transport, and observe an exit receipt.
- Good: fail, shutdown to `Stopped`, then start the other profile; the new
  snapshot contains only the new profile.
- Base: request runtime state before start and receive `not_started` with null
  profile, null project, and no shutdown receipt.
- Bad: start a second profile while connected; reject it instead of mixing
  release and main-preview capabilities.
- Bad: call `spawn` before the eligibility check; a missing program then looks
  like `Process`, and a successful extra spawn leaks a child.
- Bad: retry an operation after `timeout_unknown`; the receipt is the durable
  indication that the result is unresolved.

## 6. Tests Required

- Profile constants remain distinct and map to the fixed commit/tool
  baselines for **both** profiles.
- State transitions reject connected-before-start, duplicate start, spawn
  while connected (must be `Policy`, not `Process`), and shutdown-before-start.
- Empty program is `Policy` and leaves `NotStarted`.
- Failed → shutdown → start of the other profile is sequential and clears the
  previous failure.
- Runtime IPC projects a connected snapshot and profile into the DTO; JSON
  uses `timeout_unknown`, `main-preview`, nested shutdown fields, and omits
  `child_pid`.
- Normal shutdown records cancel-before-wait, exit code, and no force.
- `CancelOutcome::Failed` and `WaitOutcome::Unknown` record `timeout_unknown`
  without claiming success.
- A cross-platform shell child can be spawned through `OwnedChild` and
  stopped under a bounded budget. That test does not verify Job Objects.

## 7. Wrong vs Correct

### Wrong

```rust
let _ = supervisor.shutdown(&mut transport, timeout);
supervisor.start(main_preview, child)?;
```

This discards the shutdown receipt and can replace a live profile without
proving the previous child and transport were drained.

### Correct

```rust
let receipt = supervisor.shutdown(&mut transport, timeout)?;
if receipt.timeout_unknown {
    return Err(FailureKind::TimeoutUnknown);
}
supervisor.start(next_profile, child)?;
```

The caller preserves the unknown-result boundary and starts a new profile
only after the prior lifecycle has reached `Stopped`.

### Wrong

```rust
pub fn spawn(&mut self, spec: EngineLaunchSpec) -> Result<(), FailureKind> {
    let child = OwnedChild::spawn(&spec).map_err(|_| FailureKind::Process)?;
    self.start(spec, Box::new(child))
}
```

A second start can create a process before `start` returns `Policy`. If the
program is missing, the duplicate start is misclassified as `Process`.

### Correct

```rust
pub fn spawn(&mut self, spec: EngineLaunchSpec) -> Result<(), FailureKind> {
    if !self.can_start() || spec.program.as_os_str().is_empty() {
        return Err(FailureKind::Policy);
    }
    let child = OwnedChild::spawn(&spec).map_err(|_| FailureKind::Process)?;
    self.start(spec, Box::new(child))
}
```

Policy is decided before any OS process exists. `Process` is reserved for an
eligible spawn that then fails at the OS boundary.
