import type { IpcCommand, IpcResponse, EngineProfile } from "../src/ipc";
export { readShellSnapshot } from "../src/shell";

export function deferredResponse() {
  let resolve!: (value: IpcResponse) => void;
  let reject!: (reason: unknown) => void;
  const promise = new Promise<IpcResponse>((accept, fail) => {
    resolve = accept;
    reject = fail;
  });
  return { promise, resolve, reject };
}

// Every response is checked against the production IPC union by TypeScript.
export const capabilities = {
  kind: "capabilities",
  commands: ["get_capabilities", "get_runtime_state", "list_projects"],
  events: [],
  policy: { project: "bmdock-fixture", arbitrary_paths_allowed: false, raw_call_tool_allowed: false },
  cloud_allowed: false,
  provider_enabled: false,
} satisfies IpcResponse;

export function runtime(profile: EngineProfile): IpcResponse {
  return {
    kind: "runtime_state", status: "connected", project: "bmdock-fixture", profile,
    session_generation: 1, failure: null, shutdown: null, host_drain: "idle", semantic_model_loaded: false,
  };
}

export const catalog = {
  kind: "project_catalog",
  workspaces: [{ id: "bmdock-workspace", kind: "bmdock_owned" }],
  projects: [{ id: "bmdock-fixture", workspace: "bmdock-workspace", kind: "bmdock_owned" }],
  scanned_user_obsidian_vault: false, scanned_user_basic_memory_home: false,
  cross_project_search_allowed: false, implicit_current_project_writes: false,
  cloud_or_credential_required: false, local_offline: true, files_written: false,
} satisfies IpcResponse;

export const denied = {
  kind: "error", category: "policy", message: "Fixture selection denied",
} satisfies IpcResponse;

export function shellTransport(profile: EngineProfile = "release") {
  const pending = deferredResponse();
  const calls: IpcCommand[] = [];
  const invoke = async (command: IpcCommand): Promise<IpcResponse> => {
    calls.push(command);
    switch (command.command) {
      case "get_capabilities": return pending.promise;
      case "get_runtime_state": return runtime(profile);
      case "list_projects": return catalog;
      default: throw new Error(`Unexpected shell command: ${command.command}`);
    }
  };
  return { pending, calls, invoke };
}
