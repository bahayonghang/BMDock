# Workbench Request and Editor Ownership

## 1. Scope / Trigger

This contract records C04 interaction correctness in
`apps/bmdock-desktop/src/workbenchRequests.ts`, `src/editSessions.ts`,
`src/App.tsx` and `src/shell.ts`. Apply it when changing note selection,
details, paging, diagnostic profile selection, navigation or editor responses.
Stable anchors are the named functions below; layout tasks may move their lines.

C06 now uses [demand-driven details and bounded search](client-search-experience.md).
Ordinary App reads pass `loadDetails=false`; the automatic chain below remains
a guarded callable path and regression surface, not an ordinary-read requirement.

`App` owns the request and editor stores above transient panels. This is
process-lifetime state, not durable drafts, a search cache or physical request
cancellation. Fixture routing and the backend's error/persistence boundaries
remain authoritative. C04 does not enable engine writes or install a draft store.

## 2. Signatures

The owning public seams are:

```ts
WorkbenchRequests.run(
  operation: string,
  identity: string,
  work: (scope: {
    current: () => boolean;
    detailsCurrent: () => boolean;
    guard: <T extends unknown[]>(callback: (...args: T) => void)
      => (...args: T) => void;
  }) => Promise<void>,
): Promise<void>;
WorkbenchRequests.invalidate(...operations: string[]): void;
WorkbenchRequests.changeDiagnosticProfile(): void;
WorkbenchRequests.generation(): number;
WorkbenchRequests.snapshot(): readonly string[];

EditSessions.get(key: string, seed: {
  identifier: string; title?: string; body: string;
}): EditorState;
EditSessions.update(key: string, patch: Partial<EditorState>): void;
EditSessions.run(key: string, work: (submitted: EditorState) => Promise<void>): Promise<void>;
EditSessions.draft(key: string, operation: "save_draft" | "load_draft",
  invoke?: (command: IpcCommand) => Promise<IpcResponse>): Promise<void>;
EditSessions.discard(key: string, seed: {
  identifier: string; title?: string; body: string;
}): void;
```

Both stores expose `subscribe` for `useSyncExternalStore`. `EditorState` owns
the current identifier/title/body, revision, pending/error state, draft baseline
`diskBody`, draft/CRUD receipts, destination and delete confirmation.
`App.useEditorSession` supplies the key and production invoke path. Named async
operations such as `openNote`, `runSearch`, `loadMoreSearch`, `loadMoreCli` and
`runEditNote` accept a typed invoke seam; tests call those production functions.

## 3. Contracts

Request identity contains actual runtime profile, backend session generation,
explicit fixture route, page size and applicable query/identifier/page/cursor
inputs. The operation slot distinguishes operation kind. Diagnostic `toolProfile`
is not engine session identity. `readShellSnapshot` preserves
`RuntimeStateDto.session_generation`; connected tree/read calls pass
`expected_session: { profile, generation }`. Future supported mode/filter fields
must join the owning identity when introduced; C04 invents no unsupported fields.

`run` admits a slot synchronously, deduplicates an equivalent pending operation,
and gives replacements a new token. Only current success/error/cleanup callbacks
may publish. Navigation, shell refresh, runtime events and workbench disposal
invalidate outstanding work. Initial `loadTree` also checks the captured lifetime
after its await. Invalidation ignores completion; it does not stop backend work.

When enabled, automatic `openNote` relations/graph/context share a captured detail intent.
`openNote`, `runSearch`, explicit context, graph replacement and graph paging
advance that intent. Check `detailsCurrent` before clearing detail state, each
detail commit/error and each subsequent detail request. Keep the independent
note token: a newer preview need not invalidate an otherwise current note read.
The ordinary App instead admits related work when its disclosure is open and
invalidates that work on closure/selection. C06 search selection identity also
includes the result row distinct from its owning note; successful delivery
publishes body and marker together without weakening these request guards.

Page replacement invalidates the corresponding pending page. Page admission is
also rejected synchronously while its replacement is pending: tree, search,
graph, CLI, activity, resources and prompts. Graph paging additionally waits for
`openNote`. Button state alone cannot prevent a stale rendered callback from
running. Diagnostic profile changes invalidate and clear tools/CLI/audit data;
`loadMoreCli` checks the captured page's profile before IPC and response profile
before merging. Never combine profiles while reporting `mixed_profiles: false`.

Editor keys contain actual profile, explicit route, selected note identifier and
draft/CRUD purpose; a new note has a stable slot. `get` seeds only once. Rereads,
selection changes and transient unmounts must not reseed edited text. Retain the
last delivered selected note while rereading so a local read failure leaves its
editor available. There is no silent eviction, filesystem or localStorage write.

Editing body/identifier/title advances revision. `run` deduplicates mutation
activation while pending, including before React renders the disabled state.
A verified draft-save acknowledgement advances the submitted baseline without
replacing the current body. Draft reload replaces the body only if the submitted
revision is still current. Failed/unsupported saves retain text and do not infer
persistence. Dirty reload confirmation is bound to editor key and revision.
Identifier changes clear delete confirmation centrally, including changes from
a move response. Ordinary identifier-input changes already cleared it before
this central safeguard. Explicit discard cannot remove a pending session.

Read/page failures remain local and preserve unrelated loaded content and edits.
A successful tree-page retry clears its old error. Pending status is announced;
typed `policy`/`schema`/`unsupported` errors retain their category, while rejected
invokes remain `invoke`. Unknown outcomes never cause automatic mutation retry.

## 4. Validation & Error Matrix

| Condition | Required behavior |
|---|---|
| A completes after replacement B | Ignore A's success, error and cleanup; B retains its slot/state |
| New explicit preview while automatic details await | Preserve the new preview; suppress old clears, commits and follow-on calls |
| Old page control fires during replacement | Admit no page IPC; allow deliberate current-page loading after replacement |
| CLI page belongs to another profile | Reject before IPC; mismatched response is rejected before merge |
| Save A, type B, acknowledge A | Keep B visible; baseline may advance to verified A; B remains dirty |
| Unsupported save or rejected invoke | Preserve text and error category; no new persistence claim or retry |
| Reload response follows further typing | Preserve the later revision; retain receipt/baseline separately |
| Confirmation target/revision changes | Prior reload confirmation no longer applies; identifier changes clear delete confirmation |
| Page retry succeeds after error | Keep loaded content and clear the prior local alert |
| App exits | In-memory state has no durability guarantee |

## 5. Good / Base / Bad Cases

Good: read A waits on relations, the user explicitly previews B, then A finishes;
B's preview/error stays current and A launches no later graph/context work.
Good: a save of A finishes after typing B; B remains visible and dirty.
Base: the default draft store rejects saving as unsupported and the editor keeps
its text. Bad: clearing pending state from A removes B's current slot, an old
profile page merges into the new profile, or a save receipt replaces newer text.

## 6. Tests Required

Run `npm run test:behavior` and `npm run build` in `apps/bmdock-desktop`, plus
`python -m unittest tests.test_desktop_shell -q` from the repository root.
Use deferred typed responses and the actual production operations/stores.

Assertion points: A/B note and query orders; obsolete error/cleanup; no stale
detail follow-ons; parent replacement versus page admission; profile-bound page
invocation/merge; synchronous page and mutation deduplication; local-error content
retention; save-A/type-B/ack-A; retained session keys; rejected saves without
retry; revision-safe reload/discard; and response-driven confirmation invalidation.
Root wiring and reload-confirmation identity require source review as well.

C04 review verified 27 behavior tests, 34 structural/boundary tests and the
TypeScript/Vite build. These are production-function/state-owner tests plus
source inspection, not mounted DOM or native interaction tests. Negative controls
restoring stale commits and response-body overwrite produced 8 expected failures
in the earlier 19-test suite. CRLF assertions prove JavaScript string retention,
not textarea normalization or physical bytes. Native IME, focus, screen reader,
app-exit behavior and presented-frame timing remain UNVERIFIED. Full performance
matrix and historical G0 failures are separate; no gate is weakened here.

## 7. Wrong vs Correct

Wrong: guard only the original note request and let its automatic details publish
after the user has requested another preview; replace the editor body from a
save response.

```ts
await loadRelations(identifier, setRelations, setRelationsError, invoke);
await loadContextPreview(identifier, null, setPreview, setPreviewError, invoke);
sessions.update(key, { body: response.body });
```

Correct: check detail intent before both commits and subsequent calls; a save
acknowledgement changes the verified baseline while preserving current typing.

```ts
if (!detailsCurrent()) return;
await loadRelations(identifier,
  value => { if (detailsCurrent()) setRelations(value); },
  error => { if (detailsCurrent()) setRelationsError(error); }, invoke);
if (!detailsCurrent()) return;
// Apply the same guard to the next detail operation.
sessions.update(key, { diskBody: response.body, draftResult: response });
// The baseline update above requires the verified draft-result branch.
```
