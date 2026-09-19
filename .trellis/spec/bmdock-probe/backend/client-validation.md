# Client contract and behavior validation

## 1. Scope / trigger

C01 introduces a direct-engine baseline harness and an executable frontend
snapshot seam. These are developer checks, not native UI or product-gate evidence.
Do not compare the production `EmptyLibrary` response time with a connected engine.
The approved task and its frozen measurement manifest own performance scope.

## 2. Signatures

`python -m scripts.client_baseline --help` exposes fixture generation, capture,
measurement, scenario listing and manifest freezing. Its outputs stay within
`artifacts/client-baseline/` or `execution/evidence/`; generated engine sandboxes
remain in `.work/g0/client-baseline/`. Use the current CLI help for exact arguments.

From `apps/bmdock-desktop`, `npm run test:behavior` compiles the actual TypeScript
snapshot producer with the installed compiler and executes Node's built-in tests.
The production seam is:

```ts
readShellSnapshot(
  invoke: (command: IpcCommand) => Promise<IpcResponse> = invokeTyped,
): Promise<Exclude<ShellLoadState, { phase: "loading" }>>
```

The developer probe additionally permits `list_directory`, `build_context`, and
`recent_activity`. This does not expose a raw tool interface to the renderer.

## 3. Contracts

- Preserve release and main-preview SHA/tool inventory independently. Existing
  sandbox marker, verified fixture config and scrubbed environment remain mandatory.
- Store exact requests and raw responses. A structured envelope may contain an
  application error: note content and identity must be checked before timing a
  successful read. Search uses `current_page`, `total_is_exact`, and `has_more`.
- Full-source reads explicitly request `include_frontmatter=true` and omit line
  ranges. Compare returned UTF-8/YAML/CRLF text with the known fixture; report
  observed upstream changes separately from client preservation.
- A bound frozen manifest identifies seed, corpora, query labels, scenarios,
  counts and budgets. A stale manifest is not proof that the running code obeyed it.
- Five process-cold and thirty warm samples use nearest-rank p95 (rank 29 for 30).
  Distinguish startup, direct MCP roundtrip, adapter overhead and native presentation.
- Snapshot tests inject only typed transport. Normal production callers retain
  the default transport. A deferred promise is evidence about state production,
  not React rendering or rejection of stale selections.

## 4. Validation and error matrix

| Condition | Required result |
|---|---|
| Wrong fixture ownership/profile, external output or manifest drift | Reject before measurement |
| Protocol check fails after child creation | Close/abort the owned process, then propagate failure |
| `isError`, guidance text or structured missing-note error | Record outcome; do not time it as a successful read |
| Optional semantic provider unavailable | Explicit unavailable result, never zero latency/recall |
| Delayed shell response | Promise remains pending, then returns ready or typed error |
| Wrong shell response variant | Schema error; no invented ready data |
| Rejected invoke | Existing invoke error state |
| Native trace lacks changed-frame presentation join | Unverified; rAF is not a substitute |

## 5. Good / base / bad cases

Good: an unchanged generated CRLF note is retrieved through the pinned engine,
with its full source, exact request, payload outcome and process receipt recorded.
Base: the typed shell fixture resolves capabilities/runtime/projects and produces
the ready snapshot used by App. Bad: `NOTE_NOT_FOUND` is a structured dictionary
and is mistakenly counted as a fast successful full-note read.

## 6. Required tests and assertion points

Use focused corpus/statistics/manifest/measurement tests, the probe allow/deny
tests, and `npm run test:behavior`. Assertions cover deterministic physical hashes,
independent expected identities, manifest drift rejection, valid response shapes,
sample counts, p95 rank, delayed/reversed completion and typed error preservation.
The behavior runner uses only installed TypeScript and Node; no dependency is added.

`python -m scripts.tasks unit` and `gate` remain separate. The inherited later-task
before-G0 failure must be reported and must not be removed to make this check green.
Native frame evidence follows the task's frozen protocol, not these unit checks.

## 7. Wrong vs correct

Wrong: hash an old manifest while using changed live scenario constants, report a
null-content read as successful, or claim a Node snapshot test fixed a React race.
Correct: bind or validate the exact frozen workload, classify operation outcomes,
and state what the executed producer tests actually prove.
