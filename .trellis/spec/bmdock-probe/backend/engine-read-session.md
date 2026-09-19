# Official engine read session

C03 extends this owner as described in [Official engine queries](./engine-queries.md):
search/context/activity projections, exact-target read protection, stable tree
note UUIDs and owner-retained permits after consumer cancellation. C02 v2 records
below remain historical evidence for the original source/binary fingerprints.

## 1. Scope / trigger

C02 production reads use `engine_session::EngineSession` through
`main.rs::dispatch_host`. The historical T07 Supervisor process/transport seams
are test-only; its default snapshot and policy types remain shared. T17
`begin_shutdown` remains a host-drain receipt, not the live engine close owner.
This contract overrides older unavailable/fixture-only descriptions only for
the connected generated-fixture tree and note reads described here.

## 2. Signatures

```rust
SessionIdentity { profile: EngineProfile, generation: u32 }
FixtureLaunch::verified(profile: EngineProfile, sandbox: &Path)
    -> Result<FixtureLaunch, IpcError>
EngineSession::start(launch: FixtureLaunch) -> Result<EngineSession, IpcError>
EngineSession::read(&self, read: PreparedRead) -> Result<IpcResponse, IpcError>
EngineSession::close(&self) -> ShutdownReceipt
```

The last two methods are async. Renderer `ListTreeArgs` and `ReadNoteArgs` carry
optional `expected_session: SessionIdentity`; connected production reads require
it. `ListTreeArgs.directory` is a validated project-relative logical directory,
not a host path; page size is bounded separately.
Returned `TreePageDto` / `NoteReadDto` carry `session`. Runtime DTO/event carries
`session_generation: number | null` with the actual runtime profile; diagnostic
profile selection is a separate UI value. Existing command count remains 45.

Host-only launch entries are `--fixture-session <profile> <sandbox>` (native UI)
and `--session-check <profile> <sandbox>` (headless acceptance). Neither is
renderer IPC. The latter uses the same `dispatch_host`, not a parallel read client.

## 3. Contracts

- Host verifies a canonical owned `.work/g0` descendant, matching sandbox marker,
  single local `bmdock-fixture` config, fixed checkout SHA and clean tracked engine
  sources. Executable/worker/environment are host-derived. Preserve the venv
  executable spelling: resolving a Unix venv symlink can lose its `pyvenv.cfg`.
- Child launch clears inherited environment before applying the scrubbed map.
  Auto-update, semantic search and cloud routing stay disabled. No global config,
  credentials or user-vault root is taken from the renderer.
- Connected requires MCP handshake and profile-specific tool discovery (21/27).
  Each new session receives the next checked generation, even for the same profile.
  Explicit route and expected session/profile validation precede read admission;
  connection state and owner identity are checked again when a result completes.
  No cache or automatic retry exists in production read dispatch.
- `dispatch_host` clones the session under the short app-state lock and releases
  it before I/O. The owner limits retained reads to eight with recoverable
  overflow. Connect/read budgets are 120/90 seconds; SDK close, normal child wait
  and forced reap each have 30-second budgets. A read timeout retires the session.
- The owner waits for SDK transport completion, explicit stop, and owned-child
  exit. Transport EOF while a child is alive must fail and retire without waiting
  for a new read. Runtime changes emit the Tauri event `runtime_state`.
- Request `read_note` with JSON, `include_frontmatter=true`, no line slicing.
  Decode `structuredContent.result` once. Missing/null content is not success.
  Preserve the delivered string, distinguish folders from note identities, and
  fetch only the requested directory page. C03 owns later query projection.
- EmptyLibrary/draft/backup mutation boundaries remain unchanged. A connected
  read session does not enable writes, imports, restore, providers or cloud.

## 4. Validation and error matrix

| Condition | Required behavior |
|---|---|
| Invalid launch/config/launch profile or explicit route | Policy rejection before unauthorized I/O |
| No connected session or unavailable note | Explicit unavailable; no fake empty success |
| Missing/wrong expected session while connected | Policy/stale rejection, never attributed to new session |
| Malformed structured result | Schema error |
| MCP `isError` or non-null structured `error` | Typed `unsupported`, not successful data |
| Guidance text without the required structured object | Typed `schema` error; no prose fallback |
| RPC/transport EOF | Runtime `transport` failure and owned retirement; pending reads return `unsupported` |
| Read deadline exceeded | Runtime `timeout_unknown` and typed `unsupported`; retire, no automatic retry |
| Eight retained reads occupied | Recoverable `unsupported` capacity error, no unbounded queue |
| T17 drain succeeds | Drain receipt only; not process termination evidence |

## 5. Good / base / bad cases

Good: two known nested Chinese/YAML/CRLF reads through production dispatch reuse
one child and generation, then record an actual normal close. Base: launch without
the fixture entry reports unavailable. Bad: the child still lives after transport
EOF and runtime remains connected until another read notices the break.

## 6. Required tests and assertion points

Run focused `engine_session` tests plus `cargo test -p bmdock-app --locked --offline`.
Assert profile inventory, complete body, error decoding, stale identity, bounded
capacity, lock-free runtime access, close outcomes and the real SDK EOF regression.
`python -m unittest tests.test_client_session tests.test_desktop_shell` validates
receipt interpretation and continued policy/source boundaries.

`python -m scripts.client_session <profile> --output execution/evidence/<fresh>.json`
records actual production dispatch, OS/SHA, generated sandbox, binary/source
fingerprints, file equality and close receipt. C02 final records are
`c02-release-session-v2.json` and `c02-main-preview-session-v2.json`. New source
changes invalidate same-revision claims; retain these as historical evidence and
rerun affected integration at C07. Native UI/window close, Job Objects, process-tree
containment, official forced-kill and sleep/resume remain UNVERIFIED.

## 7. Wrong vs correct

Wrong: claim a successful G0 probe or receipt-only drain proves desktop lifecycle,
hold AppState while awaiting MCP, or apply `.envs()` without clearing ambient env.
Correct: exercise the shared production dispatcher and actual owner, preserve
short locks and scrubbed launch, and record exactly the shutdown that happened.
