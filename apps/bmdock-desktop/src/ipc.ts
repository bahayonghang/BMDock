import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const FIXTURE_PROJECT = "bmdock-fixture" as const;

export type IpcCommand =
  | { command: "get_capabilities"; args: Record<string, never> }
  | { command: "get_runtime_state"; args: Record<string, never> }
  | { command: "select_project"; args: { project: typeof FIXTURE_PROJECT } }
  | { command: "list_projects"; args: Record<string, never> }
  | { command: "run_preflight"; args: Record<string, never> }
  | { command: "discover_config"; args: Record<string, never> };

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
  project: typeof FIXTURE_PROJECT | null;
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

export interface ProfileRecordDto {
  id: EngineProfile;
  commit: string;
  expected_tools: number;
}

export interface PreflightHostDto {
  profiles_metadata_present: boolean;
  arbitrary_paths_allowed: false;
  raw_call_tool_allowed: false;
  supervisor_status: RuntimeStatus;
  supervisor_idle: boolean;
  cloud_or_credential_required: false;
  engine_spawned: boolean;
  files_written: boolean;
  local_offline: true;
}

export interface PreflightDto {
  profiles: ProfileRecordDto[];
  host: PreflightHostDto;
}

export interface ConfigCandidateDto {
  path: string;
  kind: string;
}

export interface ConfigDiscoveryDto {
  root: string;
  candidates: ConfigCandidateDto[];
  scanned_user_basic_memory_home: boolean;
  copied_or_rewrote_production_config: boolean;
}

export const OWNED_WORKSPACE = "bmdock-workspace" as const;
export const OWNED_KIND = "bmdock_owned" as const;

export interface WorkspaceRecordDto {
  id: typeof OWNED_WORKSPACE;
  kind: typeof OWNED_KIND;
}

export interface ProjectRecordDto {
  id: typeof FIXTURE_PROJECT;
  workspace: typeof OWNED_WORKSPACE;
  kind: typeof OWNED_KIND;
}

export interface ProjectCatalogDto {
  workspaces: WorkspaceRecordDto[];
  projects: ProjectRecordDto[];
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  cross_project_search_allowed: false;
  implicit_current_project_writes: false;
  cloud_or_credential_required: false;
  local_offline: true;
  files_written: false;
}

export type IpcResponse =
  | { kind: "capabilities"; commands: IpcCommandName[]; events: IpcEventName[]; policy: PolicyDto }
  | { kind: "runtime_state" } & RuntimeStateDto
  | { kind: "project_selected"; project: typeof FIXTURE_PROJECT }
  | { kind: "project_catalog" } & ProjectCatalogDto
  | { kind: "preflight" } & PreflightDto
  | { kind: "config_discovery" } & ConfigDiscoveryDto
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

export const listProjects = () =>
  invokeTyped<{ kind: "project_catalog" } & ProjectCatalogDto>({
    command: "list_projects",
    args: {},
  });

export const runPreflight = () =>
  invokeTyped<{ kind: "preflight" } & PreflightDto>({ command: "run_preflight", args: {} });

export const discoverConfig = () =>
  invokeTyped<{ kind: "config_discovery" } & ConfigDiscoveryDto>({
    command: "discover_config",
    args: {},
  });
