import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const FIXTURE_PROJECT = "bmdock-fixture" as const;

export type IpcCommand =
  | { command: "get_capabilities"; args: Record<string, never> }
  | { command: "get_runtime_state"; args: Record<string, never> }
  | { command: "select_project"; args: { project: typeof FIXTURE_PROJECT } };

export type IpcCommandName = IpcCommand["command"];
export type IpcEventName = "runtime_state" | "policy";
export type ErrorCategory = "policy" | "schema" | "unsupported";

export interface IpcError {
  category: ErrorCategory;
  message: string;
}

export interface PolicyDto {
  project: typeof FIXTURE_PROJECT;
  arbitrary_paths_allowed: false;
  raw_call_tool_allowed: false;
}

export interface CapabilitiesDto {
  commands: IpcCommandName[];
  events: IpcEventName[];
  policy: PolicyDto;
}

export interface RuntimeStateDto {
  status: RuntimeStatus;
  project: null;
  profile: EngineProfile | null;
  failure: FailureKind | null;
  shutdown: ShutdownReceipt | null;
}

export type EngineProfile = "release" | "main-preview";
export type RuntimeStatus = "not_started" | "starting" | "connected" | "stopping" | "stopped" | "failed";
export type FailureKind = "policy" | "transport" | "timeout_unknown" | "process" | "unverified";
export interface ShutdownReceipt {
  transport_cancelled: boolean;
  child_exited: boolean;
  forced: boolean;
  timeout_unknown: boolean;
  exit_code: number | null;
}

export type IpcResponse =
  | { kind: "capabilities"; commands: IpcCommandName[]; events: IpcEventName[]; policy: PolicyDto }
  | { kind: "runtime_state" } & RuntimeStateDto
  | { kind: "project_selected"; project: typeof FIXTURE_PROJECT }
  | { kind: "error"; category: ErrorCategory; message: string };

export interface RuntimeStateEvent {
  status: RuntimeStatus;
  project: string | null;
  profile: EngineProfile | null;
  failure: FailureKind | null;
  shutdown: ShutdownReceipt | null;
}

export interface PolicyEvent {
  category: ErrorCategory;
  message: string;
}

export type IpcEventPayload = {
  runtime_state: RuntimeStateEvent;
  policy: PolicyEvent;
};

function assertFixtureCommand(command: IpcCommand): void {
  if (command.command === "select_project" && command.args.project !== FIXTURE_PROJECT) {
    throw new Error("Only the generated fixture project is allowed");
  }
}

export async function invokeTyped<T extends IpcResponse>(command: IpcCommand): Promise<T> {
  assertFixtureCommand(command);
  return invoke<T>("ipc_invoke", { command });
}

export function listenTyped<K extends IpcEventName>(
  event: K,
  handler: (payload: IpcEventPayload[K]) => void,
): Promise<UnlistenFn> {
  return listen<IpcEventPayload[K]>(`bmdock://${event}`, (received) => handler(received.payload));
}

export const getCapabilities = () =>
  invokeTyped<{ kind: "capabilities" } & CapabilitiesDto>({ command: "get_capabilities", args: {} });

export const getRuntimeState = () =>
  invokeTyped<{ kind: "runtime_state" } & RuntimeStateDto>({ command: "get_runtime_state", args: {} });

export const selectFixtureProject = () =>
  invokeTyped<{ kind: "project_selected"; project: typeof FIXTURE_PROJECT }>({
    command: "select_project",
    args: { project: FIXTURE_PROJECT },
  });
