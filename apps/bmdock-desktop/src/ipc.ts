import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const FIXTURE_PROJECT = "bmdock-fixture" as const;
export const OWNED_WORKSPACE = "bmdock-workspace" as const;
export const OWNED_KIND = "bmdock_owned" as const;
export const TREE_PAGE_SIZE = 20 as const;

export type ExplicitRouteArgs = {
  workspace: typeof OWNED_WORKSPACE;
  project: typeof FIXTURE_PROJECT;
};

export function copyFixtureRoute(): ExplicitRouteArgs {
  return { workspace: OWNED_WORKSPACE, project: FIXTURE_PROJECT };
}

export type ListTreeArgs = ExplicitRouteArgs & {
  cursor?: string;
  page_size?: number;
};

export type ReadNoteArgs = ExplicitRouteArgs & {
  identifier: string;
};

export type RestoreFixtureArgs = ExplicitRouteArgs & {
  backup_id: string;
};

export type IpcCommand =
  | { command: "get_capabilities"; args: Record<string, never> }
  | { command: "get_runtime_state"; args: Record<string, never> }
  | { command: "select_project"; args: { project: typeof FIXTURE_PROJECT } }
  | { command: "list_projects"; args: Record<string, never> }
  | { command: "run_preflight"; args: Record<string, never> }
  | { command: "discover_config"; args: Record<string, never> }
  | { command: "list_tree"; args: ListTreeArgs }
  | { command: "read_note"; args: ReadNoteArgs }
  | { command: "list_backups"; args: ExplicitRouteArgs }
  | { command: "restore_fixture"; args: RestoreFixtureArgs }
  | { command: "inspect_windows_runtime"; args: Record<string, never> };

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

export type TreeEntryKind = "note" | "directory";

export interface TreeEntryDto {
  identifier: string;
  title: string;
  kind: TreeEntryKind;
}

export interface TreePageDto {
  entries: TreeEntryDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
}

export type ObservationClass = "empty" | "body_matches_disk" | "accepted_unverified" | "unclassified";

export interface NoteObservationDto {
  classified_as: ObservationClass;
  disk_verified: boolean;
  envelope_is_not_disk_proof: true;
}

export interface NoteReadDto {
  title: string;
  identifier: string;
  body: string;
  observation: NoteObservationDto;
}

export interface BackupRecordDto {
  id: string;
  kind: typeof OWNED_KIND;
}

export interface BackupCatalogDto {
  backups: BackupRecordDto[];
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  cloud_or_credential_required: false;
  local_offline: true;
  files_written: boolean;
}

export type RestoreClass = "empty" | "disk_verified" | "accepted_unverified" | "unclassified";

export interface RestoreObservationDto {
  classified_as: RestoreClass;
  disk_verified: boolean;
  envelope_is_not_disk_proof: true;
}

export interface RestoredFileDto {
  identifier: string;
  kind: typeof OWNED_KIND;
}

export interface RestoreResultDto {
  backup_id: string;
  files: RestoredFileDto[];
  files_written: boolean;
  observation: RestoreObservationDto;
}

export type HostOs = "windows" | "other";

export interface WindowsRuntimeDto {
  host_os: HostOs;
  webview2_files_present: boolean;
  webview2_session_verified: boolean;
  job_object_assigned: boolean;
  job_object_api_documented: boolean;
  installer_bundle_active: boolean;
  files_written: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export type IpcResponse =
  | { kind: "capabilities"; commands: IpcCommandName[]; events: IpcEventName[]; policy: PolicyDto }
  | { kind: "runtime_state" } & RuntimeStateDto
  | { kind: "project_selected"; project: typeof FIXTURE_PROJECT }
  | { kind: "project_catalog" } & ProjectCatalogDto
  | { kind: "preflight" } & PreflightDto
  | { kind: "config_discovery" } & ConfigDiscoveryDto
  | { kind: "tree_page" } & TreePageDto
  | { kind: "note_read" } & NoteReadDto
  | { kind: "backup_catalog" } & BackupCatalogDto
  | { kind: "fixture_restored" } & RestoreResultDto
  | { kind: "windows_runtime" } & WindowsRuntimeDto
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
  switch (command.command) {
    case "select_project":
      if (command.args.project !== FIXTURE_PROJECT) {
        throw new Error("Only the generated fixture project is allowed");
      }
      return;
    case "list_tree":
    case "read_note":
    case "list_backups":
    case "restore_fixture":
      if (command.args.project !== FIXTURE_PROJECT || command.args.workspace !== OWNED_WORKSPACE) {
        throw new Error("Only the generated fixture project is allowed");
      }
      return;
    case "get_capabilities":
    case "get_runtime_state":
    case "list_projects":
    case "run_preflight":
    case "discover_config":
    case "inspect_windows_runtime":
      return;
    default: {
      const exhaustive: never = command;
      return exhaustive;
    }
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

export const listTree = (args: { cursor?: string; page_size?: number } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "tree_page" } & TreePageDto>({
    command: "list_tree",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const readNote = (identifier: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_read" } & NoteReadDto>({
    command: "read_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
    },
  });
};

export const listBackups = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "backup_catalog" } & BackupCatalogDto>({
    command: "list_backups",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const restoreFixture = (backup_id: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "fixture_restored" } & RestoreResultDto>({
    command: "restore_fixture",
    args: {
      workspace: route.workspace,
      project: route.project,
      backup_id,
    },
  });
};

export const inspectWindowsRuntime = () =>
  invokeTyped<{ kind: "windows_runtime" } & WindowsRuntimeDto>({
    command: "inspect_windows_runtime",
    args: {},
  });
