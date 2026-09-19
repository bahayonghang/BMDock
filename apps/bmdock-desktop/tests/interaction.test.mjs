import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { join } from "node:path";
import test from "node:test";
const require = createRequire(import.meta.url);
const {
  WorkbenchRequests, EditSessions, openNote, runSearch, loadMoreSearch, loadTools,
  loadMoreTree, runEditNote, loadContextPreview, loadMoreCli, cli, note, page, saved, unsupported, transport, deferredResponse, loadGraph, loadMoreGraph, graph, renderNoteSource, renderAppShell, renderSearchSelection, loadTree,
} = require(join(process.env.BMDOCK_BEHAVIOR_OUTPUT, "tests", "interaction-fixtures.js"));
const noop = () => {};
const { SearchWindow, restoreSearchFocus, enginePage, renderEngineSearch, renderClosedDiagnostics, renderClosedDemand,
  captureContentDiagnostics, diagnosticIsCurrent, deliverPrimaryNote } = require(join(process.env.BMDOCK_BEHAVIOR_OUTPUT, "tests", "interaction-fixtures.js"));

test("primary result marker commits with delivered body and survives a failed newer selection", async () => {
  const requests = new WorkbenchRequests(); let visible = note("A"); let marker = "result-A";
  const start = (id, wire) => requests.run("openNote", id, ({ guard, current }) => openNote(id,
    guard((value) => deliverPrimaryNote(value, `result-${id}`, (next) => { visible = next; }, (next) => { marker = next; })),
    noop, noop, noop, noop, noop, noop, noop, noop, current, wire.invoke, current, false));
  const failed = transport(); const failure = start("B", failed); failed.pending.resolve(unsupported); await failure;
  assert.equal(visible.identifier, "A"); assert.equal(marker, "result-A");
  const stale = transport(); const fresh = transport(); const a = start("stale", stale); const b = start("fresh", fresh);
  fresh.pending.resolve(note("actual/permalink")); await b; stale.pending.resolve(note("stale")); await a;
  assert.equal(visible.identifier, "actual/permalink"); assert.equal(marker, "result-fresh");
});

test("same-owner observation rows keep latest row intent while duplicate same-hit activation deduplicates", async () => {
  const requests = new WorkbenchRequests(); let marker = null;
  const start = (hit, wire) => requests.run("openNote", JSON.stringify(["shared-owner", hit]), ({ guard, current }) => openNote("shared-owner",
    guard((value) => deliverPrimaryNote(value, hit, noop, (next) => { marker = next; })),
    noop, noop, noop, noop, noop, noop, noop, noop, current, wire.invoke, current, false));
  const a = transport(); const b = transport(); const duplicate = transport();
  const pa = start("observation-A", a); const pb = start("observation-B", b); await start("observation-B", duplicate);
  assert.equal(duplicate.calls.length, 0);
  b.pending.resolve(note("owner/permalink")); await pb; a.pending.resolve(note("owner/permalink")); await pa;
  assert.equal(marker, "observation-B"); assert.equal(a.calls[0].args.identifier, b.calls[0].args.identifier);
});

test("official query uses one search command, exact filters/text and 50 rows without an inspector", async () => {
  const wire = transport(); let result;
  const options = { mode: "title", note_types: ["meeting"], categories: ["fact"], tags: ["project"] };
  const pending = runSearch("  exact text  ", (value) => { result = value; }, noop, wire.invoke, options);
  wire.pending.resolve({ kind: "search_page", ...enginePage(1) }); await pending;
  assert.equal(wire.calls.length, 1); assert.equal(wire.calls[0].command, "search_notes");
  assert.equal(wire.calls[0].args.query, "  exact text  "); assert.equal(wire.calls[0].args.page_size, 50);
  assert.deepEqual(wire.calls[0].args.options, options); assert.equal(result.engine_search, true);
  assert.equal(result.hits[0].score, -0.5); assert.equal("semantic_enabled" in result, false);
});

test("ordinary primary read emits no hidden relations, graph or context requests", async () => {
  const wire = transport(); let visible;
  const pending = openNote("owner-uuid", (value) => { visible = value; }, noop, noop, noop, noop, noop, noop, noop, noop,
    () => true, wire.invoke, () => true, false);
  wire.pending.resolve(note("returned/permalink")); await pending;
  assert.equal(visible.identifier, "returned/permalink");
  assert.deepEqual(wire.calls.map((call) => call.command), ["read_note"]);
  assert.equal(wire.calls[0].args.identifier, "owner-uuid");
});

test("initial tree path requests 50 and has no diagnostic or catalog follow-on", async () => {
  const wire = transport();
  const pending = loadTree(noop, noop, noop, noop, undefined, noop, () => true, () => true, wire.invoke);
  wire.pending.resolve({ kind: "error", category: "unsupported", message: "fixture unavailable" });
  await pending;
  assert.deepEqual(wire.calls.map((call) => call.command), ["list_tree"]);
  assert.equal(wire.calls[0].args.page_size, 50);
});

test("search window retains at most three pages and preserves the active result page", () => {
  const window = new SearchWindow(); window.accept(enginePage(1));
  window.activeResult = "result-1-0";
  let visible;
  for (let page = 2; page <= 8; page++) {
    visible = window.accept(enginePage(page), String(page));
    assert.ok(window.pages.length <= 3); assert.ok(visible.hits.length <= 150);
    assert.ok(visible.hits.some((hit) => hit.identifier === "result-1-0"));
  }
  assert.deepEqual(window.pages.map((page) => page.value.page), [1, 7, 8]);
  assert.equal(visible.hits.filter((hit) => hit.note_identifier === "shared-owner-uuid").length, 150);
  assert.equal(window.history.length, 8); assert.deepEqual(window.previous(), { available: true, cursor: "7" });
  window.accept(enginePage(7), "7"); assert.deepEqual(window.previous(), { available: true, cursor: "6" });
  const revisited = window.accept(enginePage(7), "7");
  assert.deepEqual([revisited.hits[0].identifier, revisited.hits[50].identifier, revisited.hits[100].identifier], ["result-1-0", "result-7-0", "result-8-0"]);
  window.reset(); assert.equal(window.pages.length, 0); assert.equal(window.history.length, 0);
});

test("previous navigation sends observed cursor unchanged, and errors preserve the current window", async () => {
  const window = new SearchWindow(); window.accept(enginePage(1)); window.accept(enginePage(2), "2"); window.accept(enginePage(3), "3");
  const previous = window.previous(); const wire = transport(); let error;
  const pending = runSearch("test", (page) => window.accept(page, previous.cursor), (value) => { error = value; }, wire.invoke, { categories: ["fact"] }, previous.cursor);
  wire.pending.resolve(unsupported); await pending;
  assert.equal(wire.calls[0].args.cursor, "2"); assert.equal(error.category, "unsupported");
  assert.deepEqual(window.pages.map((page) => page.value.page), [1, 2, 3]);
});

test("official result rows retain observation identities and render title/snippet safely", () => {
  const html = renderEngineSearch(enginePage(1, 2));
  assert.equal((html.match(/aria-current="true"/g) ?? []).length, 1);
  assert.match(html, /Title 0/); assert.match(html, /result-1-0/); assert.match(html, /result-1-1/);
  assert.match(html, /&lt;script&gt;escaped snippet&lt;\/script&gt;/); assert.doesNotMatch(html, /<script>/);
  assert.match(html, /<dd>-0.5<\/dd>/); assert.match(html, /<dd>observation<\/dd>/);
});

test("removed pagination control restores focus to persistent summary; existing control stays put", () => {
  let focused = 0; const fallback = { focus: () => { focused++; } };
  restoreSearchFocus({ isConnected: true }, fallback, true); assert.equal(focused, 0);
  restoreSearchFocus({ isConnected: false }, fallback, false); assert.equal(focused, 0);
  restoreSearchFocus({ isConnected: false }, fallback, true); assert.equal(focused, 1);
  restoreSearchFocus(null, fallback, true); assert.equal(focused, 1);
});

test("closed diagnostics perform zero full-body scans/previews through 30 edits of 1 and 5 MiB", () => {
  let scans = 0;
  const inspect = () => { scans++; throw new Error("closed diagnostics scanned"); };
  for (const size of [1, 5]) {
    let body = "x".repeat(size * 1024 * 1024);
    for (let revision = 0; revision < 30; revision++) {
      body += "中";
      const html = renderClosedDiagnostics(body, String(revision), inspect);
      assert.doesNotMatch(html, /<pre/); assert.ok(html.length < 1000);
    }
  }
  assert.equal(scans, 0);
  let loads = 0; const html = renderClosedDemand(() => { loads++; });
  assert.equal(loads, 0); assert.doesNotMatch(html, /unmounted details/);
});

test("explicit diagnostic snapshot stays bound to submitted revision across subsequent edits", () => {
  let scans = 0;
  const snapshot = captureContentDiagnostics("source A", "note-A:1", () => { scans++; return { unsafe_html_present: false, executed: false, line_endings: "none" }; });
  assert.equal(diagnosticIsCurrent(snapshot, "note-A:1"), true);
  for (let revision = 2; revision < 32; revision++) assert.equal(diagnosticIsCurrent(snapshot, `note-A:${revision}`), false);
  assert.equal(diagnosticIsCurrent(snapshot, "note-B:1"), false); assert.equal(scans, 1); assert.equal(snapshot.body, "source A");
});

for (const order of ["A-first", "B-first"]) {
  test(`actual note chain suppresses obsolete state and downstream commands (${order})`, async () => {
    const requests = new WorkbenchRequests();
    const a = transport(); const b = transport();
    let visible = null; let error = null;
    const start = (id, wire) => requests.run("openNote", id, ({ guard, current }) => openNote(
      id, guard((value) => { visible = value; }), guard(noop), guard(noop), guard(noop), guard(noop),
      guard(noop), guard(noop), guard((value) => { error = value; }), guard(noop), current, wire.invoke,
    ));
    const pa = start("A", a); const pb = start("B", b);
    if (order === "A-first") { a.pending.resolve(note("A")); await pa; b.pending.resolve(note("B")); }
    else { b.pending.resolve(note("B")); await pb; a.pending.resolve(note("A")); }
    await Promise.all([pa, pb]);
    assert.equal(visible.identifier, "B"); assert.equal(error, null);
    assert.equal(a.calls.length, 1); assert.equal(b.calls.length, 4);
    assert.deepEqual(requests.snapshot(), []);
  });
}

test("obsolete note error and cleanup cannot replace current error or pending slot", async () => {
  const requests = new WorkbenchRequests(); const a = transport(); const b = transport();
  let error = null;
  const start = (id, wire) => requests.run("openNote", id, ({ guard, current }) => openNote(
    id, guard(noop), guard(noop), guard(noop), guard(noop), guard(noop), guard(noop), guard(noop),
    guard((value) => { error = value; }), guard(noop), current, wire.invoke,
  ));
  const pa = start("A", a); const pb = start("B", b);
  a.pending.reject(new Error("obsolete failure")); await pa;
  assert.equal(error, null); assert.deepEqual(requests.snapshot(), ["openNote"]);
  b.pending.resolve(unsupported); await pb;
  assert.equal(error.category, "unsupported");
});

for (const order of ["A-first", "B-first"]) {
  test(`actual search keeps latest query (${order})`, async () => {
    const requests = new WorkbenchRequests(); const a = transport(); const b = transport();
    let result = null;
    const start = (query, wire) => requests.run("runSearch", query, ({ guard }) => runSearch(query, guard((value) => { result = value; }), guard(noop), wire.invoke));
    const pa = start("A", a); const pb = start("B", b);
    if (order === "A-first") { a.pending.resolve(page("A")); await pa; b.pending.resolve(page("B")); }
    else { b.pending.resolve(page("B")); await pb; a.pending.resolve(page("A")); }
    await Promise.all([pa, pb]); assert.equal(result.query, "B");
  });
}

test("new query invalidates an old page; duplicate pending page emits one request", async () => {
  const requests = new WorkbenchRequests(); const older = transport(); const newer = transport();
  let result = page("A", "next"); let error = null;
  const next = () => requests.run("loadMoreSearch", "A:next", ({ guard }) => loadMoreSearch(result, guard((v) => { result = v; }), guard((v) => { error = v; }), older.invoke));
  const p = next(); await next(); assert.equal(older.calls.length, 1);
  const q = requests.run("runSearch", "B", ({ guard }) => runSearch("B", guard((v) => { result = v; }), guard((v) => { error = v; }), newer.invoke));
  newer.pending.resolve(page("B")); await q;
  older.pending.resolve(page("A")); await p;
  assert.equal(result.query, "B"); assert.equal(error, null);
});

test("search and page errors preserve loaded results", async () => {
  let result = page("A", "next"); const original = result;
  for (const execute of [runSearch.bind(null, "B"), loadMoreSearch.bind(null, result)]) {
    const wire = transport(); let error = null;
    const pending = execute((v) => { result = v; }, (v) => { error = v; }, wire.invoke);
    wire.pending.resolve(unsupported); await pending;
    assert.equal(result, original); assert.equal(error.category, "unsupported");
  }
});

test("diagnostic profile and runtime disposal ignore pending old results", async () => {
  const requests = new WorkbenchRequests(); const a = transport(); const b = transport();
  let error = null;
  const pa = requests.run("loadTools", "release", ({ guard }) => loadTools("release", guard(noop), guard((v) => { error = v; }), a.invoke));
  requests.changeDiagnosticProfile();
  const pb = requests.run("loadTools", "main-preview", ({ guard }) => loadTools("main-preview", guard(noop), guard((v) => { error = v; }), b.invoke));
  b.pending.resolve({ ...unsupported, message: "current" }); await pb;
  a.pending.resolve({ ...unsupported, message: "old" }); await pa;
  assert.equal(error.message, "current");
  const c = transport();
  const pc = requests.run("loadTools", "release", ({ guard }) => loadTools("release", guard(noop), guard((v) => { error = v; }), c.invoke));
  requests.invalidate(); c.pending.resolve(unsupported); await pc;
  assert.equal(error.message, "current");
});

test("save A, type B, complete A preserves B and acknowledges only A", async () => {
  const sessions = new EditSessions(); const seed = { identifier: "notes/a", body: "A\r\n中文" };
  sessions.get("a", seed); const wire = transport();
  const saving = sessions.draft("a", "save_draft", wire.invoke);
  await sessions.draft("a", "save_draft", wire.invoke);
  assert.equal(wire.calls.length, 1); assert.equal(sessions.get("a", seed).pending, true);
  sessions.update("a", { body: "B\r\n中文" });
  wire.pending.resolve(saved(seed.identifier, seed.body)); await saving;
  const state = sessions.get("a", seed);
  assert.equal(state.body, "B\r\n中文"); assert.equal(state.diskBody, seed.body);
  assert.notEqual(state.body, state.diskBody); assert.equal(state.pending, false);
});

test("returning to a retained editor does not apply a new seed; purpose/profile keys stay independent", () => {
  const sessions = new EditSessions(); const seed = { identifier: "a", body: "original\r\n" };
  sessions.get("release/a/draft", seed); sessions.update("release/a/draft", { body: "edited\r\n" });
  sessions.get("release/b/draft", { identifier: "b", body: "other" });
  assert.equal(sessions.get("release/a/draft", { ...seed, body: "new read" }).body, "edited\r\n");
  assert.equal(sessions.get("preview/a/draft", seed).body, seed.body);
  assert.equal(sessions.get("release/a/crud", seed).body, seed.body);
});

test("unsupported and rejected saves preserve text and never claim persistence or retry", async () => {
  for (const rejects of [false, true]) {
    const sessions = new EditSessions(); const seed = { identifier: "a", body: "unsaved" };
    sessions.get("a", seed); const wire = transport(); const p = sessions.draft("a", "save_draft", wire.invoke);
    if (rejects) wire.pending.reject(new Error("timeout_unknown")); else wire.pending.resolve(unsupported);
    await p; const state = sessions.get("a", seed);
    assert.equal(state.body, seed.body); assert.equal(state.diskBody, null);
    assert.equal(state.draftResult, null); assert.equal(wire.calls.length, 1);
    assert.equal(state.error.category, rejects ? "invoke" : "unsupported");
  }
});

test("retained editor operation slot deduplicates CRUD and survives navigation", async () => {
  const sessions = new EditSessions(); const seed = { identifier: "a", body: "text" };
  sessions.get("a", seed); const deferred = deferredResponse(); let count = 0;
  const mutate = async () => { count++; await deferred.promise; };
  const operation = sessions.run("a", mutate); await sessions.run("a", mutate);
  sessions.get("b", { identifier: "b", body: "other" });
  assert.equal(count, 1); assert.equal(sessions.get("a", seed).pending, true);
  deferred.resolve(unsupported); await operation;
  assert.equal(sessions.get("a", seed).body, "text"); assert.equal(count, 1);
});

test("actual edit command is sent once while pending and does not retry unknown outcome", async () => {
  const sessions = new EditSessions(); const seed = { identifier: "fixture/notes/a.md", body: "text" };
  sessions.get("a", seed); const wire = transport(); let error = null;
  const edit = () => sessions.run("a", () => runEditNote(seed.identifier, seed.body, noop, (v) => { error = v; }, noop, wire.invoke));
  const first = edit(); await edit();
  assert.equal(wire.calls.length, 1); assert.equal(wire.calls[0].command, "edit_note");
  wire.pending.reject(new Error("timeout_unknown")); await first;
  assert.equal(error.category, "invoke"); assert.equal(wire.calls.length, 1);
  assert.equal(sessions.get("a", seed).body, seed.body);
});

test("draft reload cannot overwrite text typed after its request; discard is explicit", async () => {
  const sessions = new EditSessions(); const seed = { identifier: "a", body: "seed" };
  sessions.get("a", seed); const wire = transport(); const request = sessions.draft("a", "load_draft", wire.invoke);
  sessions.update("a", { body: "typed later" });
  sessions.discard("a", seed); assert.equal(sessions.get("a", seed).body, "typed later");
  wire.pending.resolve({ ...saved("a", "stored"), kind: "draft_loaded" }); await request;
  assert.equal(sessions.get("a", seed).body, "typed later");
  sessions.discard("a", seed); assert.equal(sessions.get("a", seed).body, "seed");
});

test("retained large editor strings remain available without eviction", (context) => {
  const sessions = new EditSessions();
  const before = process.memoryUsage().heapUsed;
  let payloadBytes = 0;
  for (const mib of [1, 5]) {
    const key = `large-${mib}`;
    const body = "a".repeat(mib * 1024 * 1024 - 1) + "\n";
    sessions.get(key, { identifier: key, body });
    sessions.update(key, { body: "b" + body.slice(1) });
    payloadBytes += Buffer.byteLength(sessions.get(key, { identifier: key, body }).body, "utf8");
  }
  assert.equal(sessions.get("large-1", { identifier: "ignored", body: "" }).body.length, 1024 * 1024);
  assert.equal(sessions.get("large-5", { identifier: "ignored", body: "" }).body.length, 5 * 1024 * 1024);
  context.diagnostic(JSON.stringify({ retainedBodyUtf8Bytes: payloadBytes, heapDeltaBytes: process.memoryUsage().heapUsed - before, scope: "Node host observation; GC uncontrolled; not native typing or RSS budget" }));
});

test("new explicit preview supersedes delayed automatic note details and their follow-ons", async () => {
  const requests = new WorkbenchRequests(); const relations = deferredResponse(); const reached = deferredResponse();
  const calls = []; let previewError = null;
  const wire = async (command) => {
    calls.push(command.command);
    if (command.command === "read_note") return note("A");
    if (command.command === "list_relations") { reached.resolve(unsupported); return relations.promise; }
    return { ...unsupported, message: "obsolete automatic A" };
  };
  const automatic = requests.run("openNote", "A", ({ guard, current, detailsCurrent }) => openNote(
    "A", guard(noop), guard(noop), guard(noop), guard(noop), guard(noop), guard(noop),
    guard((v) => { previewError = v; }), guard(noop), guard(noop), current, wire, detailsCurrent,
  ));
  await reached.promise;
  await requests.run("loadContextPreview", "B", ({ guard }) => loadContextPreview(
    "B", "query B", guard(noop), guard((v) => { previewError = v; }), async () => ({ ...unsupported, message: "current explicit B" }),
  ));
  relations.resolve(unsupported); await automatic;
  assert.equal(previewError.message, "current explicit B");
  assert.deepEqual(calls, ["read_note", "list_relations"]);
});

test("an unfinished note read cannot clear a newer explicit preview", async () => {
  const requests = new WorkbenchRequests(); const wire = transport(); let previewError = null;
  const automatic = requests.run("openNote", "A", ({ guard, current, detailsCurrent }) => openNote(
    "A", guard(noop), guard(noop), guard(noop), guard(noop), guard(noop), guard(noop),
    guard((v) => { previewError = v; }), guard(noop), guard(noop), current, wire.invoke, detailsCurrent,
  ));
  await requests.run("loadContextPreview", "B", ({ guard }) => loadContextPreview(
    "B", "B", guard(noop), guard((v) => { previewError = v; }), async () => unsupported,
  ));
  wire.pending.resolve(note("A")); await automatic;
  assert.equal(previewError.category, "unsupported"); assert.equal(wire.calls.length, 1);
});

test("CLI pagination refuses a previous profile cursor before invoking and a mismatched response before merging", async () => {
  const old = cli("release"); let current = old; let error = null;
  const wire = transport();
  await loadMoreCli("main-preview", old, (v) => { current = v; }, (v) => { error = v; }, wire.invoke);
  assert.equal(wire.calls.length, 0); assert.equal(current, old); assert.equal(error.category, "schema");
  const next = loadMoreCli("release", old, (v) => { current = v; }, (v) => { error = v; }, wire.invoke);
  wire.pending.resolve(cli("main-preview")); await next;
  assert.equal(current, old); assert.equal(error.category, "schema");
});

test("successful tree page retry clears the prior local error", async () => {
  let error = { category: "unsupported", message: "previous page failed" };
  const wire = transport();
  const next = loadMoreTree("next", [], noop, noop, (v) => { error = v; }, noop, wire.invoke);
  wire.pending.resolve({ kind: "tree_page", entries: [], next_cursor: null, page: 2, truncated: false });
  await next; assert.equal(error, null);
});

test("response-driven editor target changes invalidate delete confirmation", () => {
  const sessions = new EditSessions(); const seed = { identifier: "A", body: "text" };
  sessions.get("a", seed); sessions.update("a", { confirmDelete: true });
  sessions.update("a", { identifier: "B" });
  assert.equal(sessions.get("a", seed).confirmDelete, false);
});

test("stale graph page controls cannot admit work while replacement graph is pending", async () => {
  const requests = new WorkbenchRequests(); const replace = transport(); const more = transport();
  let current = graph("A");
  const nextGraph = requests.run("loadGraph", "B", ({ guard }) => loadGraph("B", guard((v) => { current = v; }), guard(noop), replace.invoke));
  await requests.run("loadMoreGraph", "A:next", ({ guard }) => loadMoreGraph(current, guard((v) => { current = v; }), guard(noop), more.invoke));
  assert.equal(more.calls.length, 0);
  replace.pending.resolve(graph("B")); await nextGraph;
  assert.equal(current.identifier, "B");
  const nextPage = () => requests.run("loadMoreGraph", "B:next", ({ guard }) => loadMoreGraph(current, guard((v) => { current = v; }), guard(noop), more.invoke));
  const first = nextPage(); await nextPage(); assert.equal(more.calls.length, 1);
  more.pending.resolve({ ...graph("B"), page: 2, next_cursor: null }); await first;
  assert.equal(current.identifier, "B");
});

test("every workbench page owner rejects stale controls after its replacement begins", async () => {
  for (const [parent, child] of [
    ["loadTree", "loadMoreTree"], ["runSearch", "loadMoreSearch"], ["loadCli", "loadMoreCli"],
    ["loadActivity", "loadMoreActivity"], ["loadResources", "loadMoreResources"], ["loadPrompts", "loadMorePrompts"],
  ]) {
    const owner = new WorkbenchRequests(); const replacement = deferredResponse(); let pageInvocations = 0;
    const pending = owner.run(parent, "replacement", async () => { await replacement.promise; });
    await owner.run(child, "obsolete-page", async () => { pageInvocations++; });
    assert.equal(pageInvocations, 0, child);
    replacement.resolve(unsupported); await pending;
    await owner.run(child, "current-page", async () => { pageInvocations++; });
    assert.equal(pageInvocations, 1, child);
  }
});

test("source reader renders hostile content as text and preserves delivered YAML/newlines", () => {
  const body = '---\r\ntitle: 中文\r\n---\r\n<script>alert(1)</script>\r\n<img src="https://invalid.test/tracker" onerror="alert(2)">';
  const markup = renderNoteSource(body, "a".repeat(200));
  assert.ok(markup.includes('---\r\ntitle: 中文\r\n---\r\n'));
  assert.ok(markup.includes('&lt;script&gt;alert(1)&lt;/script&gt;'));
  assert.ok(markup.includes('&lt;img src=&quot;https://invalid.test/tracker&quot;'));
  assert.ok(!markup.includes('<script')); assert.ok(!markup.includes('<img'));
  assert.ok(markup.includes('data-executed="false"'));
  assert.ok(markup.includes('tabindex="0"'));
  assert.ok(markup.includes('a'.repeat(200)));
});

test("reading original source does not mutate retained draft content", () => {
  const sessions = new EditSessions(); const seed = { identifier: "a", body: "---\r\ntitle: 原文\r\n---\r\n" };
  sessions.get("release/a/draft", seed); sessions.update("release/a/draft", { body: seed.body + "未保存编辑" });
  const markup = renderNoteSource(seed.body);
  assert.ok(markup.includes(seed.body));
  assert.equal(sessions.get("release/a/draft", seed).body, seed.body + "未保存编辑");
});

test("grouped shell retains all destinations as semantic buttons and disclosures", () => {
  const markup = renderAppShell();
  assert.equal((markup.match(/class="nav-button"/g) ?? []).length, 18);
  assert.equal((markup.match(/class="nav-group"/g) ?? []).length, 2);
  assert.ok(markup.includes('<summary>诊断与设置'));
  assert.ok(markup.includes('<summary>维护与集成'));
  assert.ok(markup.includes('aria-current="page"'));
  assert.ok(markup.includes('href="#main"'));
});

test("selected search result exposes the same programmatic current state as the tree", () => {
  assert.ok(renderSearchSelection("result-note").includes('class="tree-item" aria-current="true"'));
  assert.ok(!renderSearchSelection("different-note").includes('aria-current="true"'));
});

test("initial tree completion cannot restore retained A over user-selected B", async () => {
  const requests = new WorkbenchRequests(); const tree = transport(); const read = transport();
  let selected = "A"; let restored = 0;
  const listing = requests.run("loadTree", "directory", ({ current, detailsCurrent }) => loadTree(
    noop, noop, noop, noop, "A", () => { restored++; }, current, detailsCurrent, tree.invoke,
  ));
  const selection = requests.run("openNote", "B", ({ guard, current, detailsCurrent }) => openNote(
    "B", guard((value) => { selected = value.identifier; }), guard(noop), guard(noop), guard(noop), guard(noop),
    guard(noop), guard(noop), guard(noop), guard(noop), current, read.invoke, detailsCurrent,
  ));
  tree.pending.resolve({ kind: "tree_page", entries: [], next_cursor: null, page: 1, truncated: false }); await listing;
  assert.equal(restored, 0);
  read.pending.resolve(note("B")); await selection;
  assert.equal(selected, "B");
});
