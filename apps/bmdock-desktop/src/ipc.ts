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

export type ListRelationsArgs = ExplicitRouteArgs & {
  identifier: string;
};

export type ExpandGraphArgs = ExplicitRouteArgs & {
  identifier: string;
  cursor?: string;
  page_size?: number;
};

export type SearchNotesArgs = ExplicitRouteArgs & {
  query: string;
  cursor?: string;
  page_size?: number;
};

export type InspectSearchArgs = ExplicitRouteArgs & {
  query: string;
  identifier?: string;
};

export type RecallBenchmarkArgs = ExplicitRouteArgs & {
  k?: number;
};

export type SchemaValidateArgs = ExplicitRouteArgs & {
  identifier: string;
  schema_id?: string;
};

export type ListResourcesArgs = ExplicitRouteArgs & {
  cursor?: string;
  page_size?: number;
};

export type ListPromptsArgs = ExplicitRouteArgs & {
  cursor?: string;
  page_size?: number;
};

export type InspectToolsArgs = ExplicitRouteArgs & {
  profile_id: EngineProfile;
};

export type ListCliInventoryArgs = ExplicitRouteArgs & {
  profile_id: EngineProfile;
  cursor?: string;
  page_size?: number;
};

export type ImportNotesArgs = ExplicitRouteArgs & {
  source_id: string;
};

export type InspectApiAuditArgs = ExplicitRouteArgs & {
  profile_id: EngineProfile;
};

export type PreviewContextArgs = ExplicitRouteArgs & {
  identifier: string;
  query?: string;
};

export type ListActivityArgs = ExplicitRouteArgs & {
  cursor?: string;
  page_size?: number;
};

export type RestoreFixtureArgs = ExplicitRouteArgs & {
  backup_id: string;
};

export type SaveDraftArgs = ExplicitRouteArgs & {
  identifier: string;
  body: string;
};

export type LoadDraftArgs = ExplicitRouteArgs & {
  identifier: string;
};

export type WriteNoteArgs = ExplicitRouteArgs & {
  identifier: string;
  title: string;
  body: string;
};

export type EditNoteArgs = ExplicitRouteArgs & {
  identifier: string;
  body: string;
};

export type MoveNoteArgs = ExplicitRouteArgs & {
  identifier: string;
  destination: string;
};

export type DeleteNoteArgs = ExplicitRouteArgs & {
  identifier: string;
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
  | { command: "list_relations"; args: ListRelationsArgs }
  | { command: "expand_graph"; args: ExpandGraphArgs }
  | { command: "search_notes"; args: SearchNotesArgs }
  | { command: "inspect_search"; args: InspectSearchArgs }
  | { command: "run_recall_benchmark"; args: RecallBenchmarkArgs }
  | { command: "schema_validate"; args: SchemaValidateArgs }
  | { command: "list_resources"; args: ListResourcesArgs }
  | { command: "list_prompts"; args: ListPromptsArgs }
  | { command: "inspect_tools"; args: InspectToolsArgs }
  | { command: "list_cli_inventory"; args: ListCliInventoryArgs }
  | { command: "import_notes"; args: ImportNotesArgs }
  | { command: "inspect_api_audit"; args: InspectApiAuditArgs }
  | { command: "preview_context"; args: PreviewContextArgs }
  | { command: "list_activity"; args: ListActivityArgs }
  | { command: "list_backups"; args: ExplicitRouteArgs }
  | { command: "restore_fixture"; args: RestoreFixtureArgs }
  | { command: "inspect_windows_runtime"; args: Record<string, never> }
  | { command: "save_draft"; args: SaveDraftArgs }
  | { command: "load_draft"; args: LoadDraftArgs }
  | { command: "write_note"; args: WriteNoteArgs }
  | { command: "edit_note"; args: EditNoteArgs }
  | { command: "move_note"; args: MoveNoteArgs }
  | { command: "delete_note"; args: DeleteNoteArgs }
  | { command: "begin_shutdown"; args: Record<string, never> };

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
  host_drain: DrainPhase;
  semantic_model_loaded: false;
}

export type DrainPhase = "idle" | "draining" | "drained";
export type DrainClass = "idle_not_started" | "inflight_unknown";

export interface DrainResultDto {
  host_drain: DrainPhase;
  supervisor_status: RuntimeStatus;
  engine_spawned: boolean;
  child_killed: boolean;
  files_written: boolean;
  classified_as: DrainClass;
  inflight_unknown: string[];
  shutdown: ShutdownReceipt;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
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

export type RelationTargetClass = "present" | "empty" | "unsupported";

export interface RelationDto {
  identifier: string;
  classified_as: RelationTargetClass;
}

export interface RelationListDto {
  identifier: string;
  relations: RelationDto[];
  observation: NoteCrudObservationDto;
  engine_graph: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export type GraphNodeClass = "present" | "empty";

export interface GraphNodeDto {
  identifier: string;
  classified_as: GraphNodeClass;
}

export interface GraphEdgeDto {
  source: string;
  target: string;
}

export interface GraphPageDto {
  identifier: string;
  nodes: GraphNodeDto[];
  edges: GraphEdgeDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  engine_graph: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
  depth: 1;
}

export interface SearchHitDto {
  identifier: string;
  lexical_score: number;
  semantic_score: number;
}

export interface SearchPageDto {
  query: string;
  hits: SearchHitDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  semantic_enabled: false;
  engine_search: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export type EmbeddingBackend = "none";
export type ModelClass = "unclassified";

export interface SearchInspectorDto {
  query: string;
  identifier: string | null;
  hits: SearchHitDto[];
  observation: NoteCrudObservationDto;
  semantic_enabled: false;
  model_id: null;
  model_loaded: false;
  embedding_backend: EmbeddingBackend;
  model_class: ModelClass;
  engine_search: false;
  files_written: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  semantic_disabled_reason: string;
}

export interface RecallQueryDto {
  query: string;
  relevant: string[];
  hits: string[];
  retrieved_relevant: number;
  relevant_count: number;
}

export interface RecallBenchmarkDto {
  k: number;
  query_count: number;
  recall_hits: number;
  recall_relevant: number;
  queries: RecallQueryDto[];
  chinese_permalinks: string[];
  search_elapsed_ms: number;
  expand_elapsed_ms: number;
  observation: NoteCrudObservationDto;
  semantic_enabled: false;
  engine_search: false;
  native_gui: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export type SchemaVerdict = "valid" | "invalid" | "empty" | "unsupported";

export interface SchemaValidateDto {
  identifier: string;
  schema_id: string;
  verdict: SchemaVerdict;
  required_fields: string[];
  missing_fields: string[];
  observed_title: boolean;
  observed_body: boolean;
  observation: NoteCrudObservationDto;
  engine_schema: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface ContextPreviewDto {
  identifier: string;
  query: string | null;
  snippet: string;
  executed: false;
  unsafe_html_present: boolean;
  observation: NoteCrudObservationDto;
  engine_context: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export interface ActivityEntryDto {
  identifier: string;
  observed_mtime: number;
}

export interface ActivityPageDto {
  entries: ActivityEntryDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  engine_activity: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export interface ResourceEntryDto {
  identifier: string;
  title: string;
}

export interface ResourcePageDto {
  entries: ResourceEntryDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  engine_resources: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export interface PromptEntryDto {
  identifier: string;
  title: string;
}

export interface PromptPageDto {
  entries: PromptEntryDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  engine_prompts: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
}

export type ToolAdmission = "allowlisted" | "denied";

export interface InspectedToolDto {
  name: string;
  identity: string;
  admission: ToolAdmission;
  live_execution: false;
}

export interface ToolInspectionDto {
  profile_id: EngineProfile;
  expected_tool_count: number;
  tools: InspectedToolDto[];
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_tools: false;
  call_tool_allowed: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface CliLeafDto {
  path: string[];
  executed: false;
}

export interface CliInventoryDto {
  profile_id: EngineProfile;
  leaves: CliLeafDto[];
  next_cursor: string | null;
  page: number;
  truncated: boolean;
  observation: NoteCrudObservationDto;
  engine_cli: false;
  executed: false;
  mixed_profiles: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: boolean;
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

export type ImportClass = "empty" | "disk_verified" | "accepted_unverified" | "unclassified";

export interface ImportObservationDto {
  classified_as: ImportClass;
  disk_verified: boolean;
  envelope_is_not_disk_proof: true;
}

export interface ImportedFileDto {
  identifier: string;
  kind: typeof OWNED_KIND;
}

export interface ImportResultDto {
  source_id: string;
  files: ImportedFileDto[];
  files_written: boolean;
  observation: ImportObservationDto;
  engine_import: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export type AuditCoverage = "present" | "missing" | "unverified";
export type CapabilityStatus = "unavailable" | "unverified";

export interface AuditedApiLeafDto {
  name: string;
  identity: string;
  coverage: AuditCoverage;
  admission: ToolAdmission;
  live_execution: false;
}

export interface AuditedCliLeafDto {
  path: string[];
  coverage: AuditCoverage;
  executed: false;
}

export interface AuditedIpcCommandDto {
  name: string;
  coverage: AuditCoverage;
}

export interface UnavailableCapabilityDto {
  name: string;
  status: CapabilityStatus;
}

export interface ApiAuditDto {
  profile_id: EngineProfile;
  expected_tool_count: number;
  api_leaves: AuditedApiLeafDto[];
  cli_leaves: AuditedCliLeafDto[];
  ipc_commands: AuditedIpcCommandDto[];
  uncovered: string[];
  unavailable: UnavailableCapabilityDto[];
  mixed_profiles: false;
  full_api_coverage: false;
  semantic_enabled: false;
  model_loaded: false;
  observation: NoteCrudObservationDto;
  engine_tools: false;
  engine_cli: false;
  engine_schema: false;
  live_mcp: false;
  live_cli: false;
  call_tool_allowed: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export type DraftClass = "empty" | "disk_verified" | "accepted_unverified" | "unclassified";

export interface DraftObservationDto {
  classified_as: DraftClass;
  disk_verified: boolean;
  envelope_is_not_disk_proof: true;
}

export interface DraftResultDto {
  identifier: string;
  body: string;
  files_written: boolean;
  engine_persisted: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  observation: DraftObservationDto;
}

export type NoteCrudClass =
  | "empty"
  | "disk_verified"
  | "accepted_unverified"
  | "conflict"
  | "unclassified";

export interface NoteCrudObservationDto {
  classified_as: NoteCrudClass;
  disk_verified: boolean;
  envelope_is_not_disk_proof: true;
}

export interface NoteWriteDto {
  identifier: string;
  title: string;
  body: string;
  files_written: boolean;
  engine_persisted: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  observation: NoteCrudObservationDto;
}

export interface NoteEditDto {
  identifier: string;
  body: string;
  files_written: boolean;
  engine_persisted: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  observation: NoteCrudObservationDto;
}

export interface NoteMoveDto {
  identifier: string;
  destination: string;
  body: string;
  files_written: boolean;
  engine_persisted: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  observation: NoteCrudObservationDto;
}

export interface NoteDeleteDto {
  identifier: string;
  files_written: boolean;
  engine_persisted: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  observation: NoteCrudObservationDto;
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
  | { kind: "relation_list" } & RelationListDto
  | { kind: "graph_page" } & GraphPageDto
  | { kind: "search_page" } & SearchPageDto
  | { kind: "search_inspector" } & SearchInspectorDto
  | { kind: "recall_benchmark" } & RecallBenchmarkDto
  | { kind: "schema_validated" } & SchemaValidateDto
  | { kind: "resource_page" } & ResourcePageDto
  | { kind: "prompt_page" } & PromptPageDto
  | { kind: "tool_inspection" } & ToolInspectionDto
  | { kind: "cli_inventory" } & CliInventoryDto
  | { kind: "notes_imported" } & ImportResultDto
  | { kind: "api_audit" } & ApiAuditDto
  | { kind: "context_preview" } & ContextPreviewDto
  | { kind: "activity_page" } & ActivityPageDto
  | { kind: "backup_catalog" } & BackupCatalogDto
  | { kind: "fixture_restored" } & RestoreResultDto
  | { kind: "windows_runtime" } & WindowsRuntimeDto
  | { kind: "draft_saved" } & DraftResultDto
  | { kind: "draft_loaded" } & DraftResultDto
  | { kind: "note_written" } & NoteWriteDto
  | { kind: "note_edited" } & NoteEditDto
  | { kind: "note_moved" } & NoteMoveDto
  | { kind: "note_deleted" } & NoteDeleteDto
  | { kind: "shutdown_begun" } & DrainResultDto
  | { kind: "error"; category: ErrorCategory; message: string };

export interface RuntimeStateEvent {
  status: RuntimeStatus;
  project: string | null;
  profile: EngineProfile | null;
  failure: FailureKind | null;
  shutdown: ShutdownReceipt | null;
  host_drain: DrainPhase;
  semantic_model_loaded: false;
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
    case "list_relations":
    case "expand_graph":
    case "search_notes":
    case "inspect_search":
    case "run_recall_benchmark":
    case "schema_validate":
    case "list_resources":
    case "list_prompts":
    case "inspect_tools":
    case "list_cli_inventory":
    case "import_notes":
    case "inspect_api_audit":
    case "preview_context":
    case "list_activity":
    case "list_backups":
    case "restore_fixture":
    case "save_draft":
    case "load_draft":
    case "write_note":
    case "edit_note":
    case "move_note":
    case "delete_note":
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
    case "begin_shutdown":
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

export const listRelations = (identifier: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "relation_list" } & RelationListDto>({
    command: "list_relations",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
    },
  });
};

export const expandGraph = (args: { identifier: string; cursor?: string; page_size?: number }) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "graph_page" } & GraphPageDto>({
    command: "expand_graph",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier: args.identifier,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const searchNotes = (args: { query: string; cursor?: string; page_size?: number }) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "search_page" } & SearchPageDto>({
    command: "search_notes",
    args: {
      workspace: route.workspace,
      project: route.project,
      query: args.query,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const inspectSearch = (args: { query: string; identifier?: string }) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "search_inspector" } & SearchInspectorDto>({
    command: "inspect_search",
    args: {
      workspace: route.workspace,
      project: route.project,
      query: args.query,
      ...(args.identifier ? { identifier: args.identifier } : {}),
    },
  });
};

export const runRecallBenchmark = (args: { k?: number } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "recall_benchmark" } & RecallBenchmarkDto>({
    command: "run_recall_benchmark",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.k !== undefined ? { k: args.k } : {}),
    },
  });
};

export const schemaValidate = (args: { identifier: string; schema_id?: string }) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "schema_validated" } & SchemaValidateDto>({
    command: "schema_validate",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier: args.identifier,
      ...(args.schema_id ? { schema_id: args.schema_id } : {}),
    },
  });
};

export const listResources = (args: { cursor?: string; page_size?: number } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "resource_page" } & ResourcePageDto>({
    command: "list_resources",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const listPrompts = (args: { cursor?: string; page_size?: number } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "prompt_page" } & PromptPageDto>({
    command: "list_prompts",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const inspectTools = (profile_id: EngineProfile) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "tool_inspection" } & ToolInspectionDto>({
    command: "inspect_tools",
    args: {
      workspace: route.workspace,
      project: route.project,
      profile_id,
    },
  });
};

export const listCliInventory = (
  args: { profile_id: EngineProfile; cursor?: string; page_size?: number },
) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "cli_inventory" } & CliInventoryDto>({
    command: "list_cli_inventory",
    args: {
      workspace: route.workspace,
      project: route.project,
      profile_id: args.profile_id,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
    },
  });
};

export const previewContext = (args: { identifier: string; query?: string }) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "context_preview" } & ContextPreviewDto>({
    command: "preview_context",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier: args.identifier,
      ...(args.query ? { query: args.query } : {}),
    },
  });
};

export const listActivity = (args: { cursor?: string; page_size?: number } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "activity_page" } & ActivityPageDto>({
    command: "list_activity",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? TREE_PAGE_SIZE,
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

export const importNotes = (source_id: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "notes_imported" } & ImportResultDto>({
    command: "import_notes",
    args: {
      workspace: route.workspace,
      project: route.project,
      source_id,
    },
  });
};

export const inspectApiAudit = (profile_id: EngineProfile) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "api_audit" } & ApiAuditDto>({
    command: "inspect_api_audit",
    args: {
      workspace: route.workspace,
      project: route.project,
      profile_id,
    },
  });
};

export const inspectWindowsRuntime = () =>
  invokeTyped<{ kind: "windows_runtime" } & WindowsRuntimeDto>({
    command: "inspect_windows_runtime",
    args: {},
  });

export const saveDraft = (identifier: string, body: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "draft_saved" } & DraftResultDto>({
    command: "save_draft",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
      body,
    },
  });
};

export const loadDraft = (identifier: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "draft_loaded" } & DraftResultDto>({
    command: "load_draft",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
    },
  });
};

export const writeNote = (identifier: string, title: string, body: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_written" } & NoteWriteDto>({
    command: "write_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
      title,
      body,
    },
  });
};

export const editNote = (identifier: string, body: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_edited" } & NoteEditDto>({
    command: "edit_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
      body,
    },
  });
};

export const moveNote = (identifier: string, destination: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_moved" } & NoteMoveDto>({
    command: "move_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
      destination,
    },
  });
};

export const deleteNote = (identifier: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_deleted" } & NoteDeleteDto>({
    command: "delete_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
    },
  });
};

export const beginShutdown = () =>
  invokeTyped<{ kind: "shutdown_begun" } & DrainResultDto>({
    command: "begin_shutdown",
    args: {},
  });
