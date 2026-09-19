import assert from "node:assert/strict";
import { createRequire } from "node:module";
import { join } from "node:path";
import test from "node:test";

const require = createRequire(import.meta.url);
const {
  readShellSnapshot, shellTransport, capabilities, runtime, catalog, denied,
} = require(join(process.env.BMDOCK_BEHAVIOR_OUTPUT, "tests", "shell-fixtures.js"));
const tick = () => new Promise((resolve) => setImmediate(resolve));

test("delayed snapshot stays pending and then supplies the ready state consumed by App", async () => {
  const transport = shellTransport();
  let settled = false;
  const snapshot = readShellSnapshot(transport.invoke).then((state) => {
    settled = true;
    return state;
  });
  await tick();
  assert.equal(settled, false);
  assert.deepEqual(transport.calls, [{ command: "get_capabilities", args: {} }]);
  transport.pending.resolve(capabilities);
  const state = await snapshot;
  assert.equal(state.phase, "ready");
  assert.equal(state.runtime.profile, "release");
  assert.equal(state.runtime.status, "connected");
  assert.deepEqual(state.catalog.projects, catalog.projects);
  assert.deepEqual(transport.calls.map((call) => call.command), [
    "get_capabilities", "get_runtime_state", "list_projects",
  ]);
});

test("two snapshots can complete in reverse order without mixing their profile data", async () => {
  const first = shellTransport("release");
  const second = shellTransport("main-preview");
  const completed = [];
  const a = readShellSnapshot(first.invoke).then((state) => { completed.push(state); return state; });
  const b = readShellSnapshot(second.invoke).then((state) => { completed.push(state); return state; });
  second.pending.resolve(capabilities);
  assert.equal((await b).runtime.profile, "main-preview");
  assert.equal(completed.length, 1);
  first.pending.resolve(capabilities);
  assert.equal((await a).runtime.profile, "release");
  assert.deepEqual(completed.map((state) => state.runtime.profile), ["main-preview", "release"]);
  // This checks the real snapshot producer, not React's obsolete-result guard.
});

test("delayed typed error preserves its category/message and stops follow-on reads", async () => {
  const transport = shellTransport();
  const snapshot = readShellSnapshot(transport.invoke);
  await tick();
  transport.pending.resolve(denied);
  assert.deepEqual(await snapshot, { phase: "error", category: "policy", message: denied.message });
  assert.equal(transport.calls.length, 1);
});

test("rejected invoke produces the existing invoke error state", async () => {
  const transport = shellTransport();
  const snapshot = readShellSnapshot(transport.invoke);
  transport.pending.reject(new Error("Disconnected during shell load"));
  assert.deepEqual(await snapshot, {
    phase: "error", category: "invoke", message: "Disconnected during shell load",
  });
  assert.equal(transport.calls.length, 1);
});

test("unexpected typed variant produces a schema error rather than a ready state", async () => {
  const transport = shellTransport();
  const snapshot = readShellSnapshot(transport.invoke);
  transport.pending.resolve(runtime("release"));
  const state = await snapshot;
  assert.equal(state.phase, "error");
  assert.equal(state.category, "schema");
  assert.ok(state.message.length > 0);
  assert.equal(transport.calls.length, 1);
});
