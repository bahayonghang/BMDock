import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export const FIXTURE_PROJECT = "bmdock-fixture" as const;
export const OWNED_WORKSPACE = "bmdock-workspace" as const;
export const OWNED_KIND = "bmdock_owned" as const;
export const TREE_PAGE_SIZE = 20 as const;
export const QUERY_PAGE_SIZE = 50 as const;

export type ExplicitRouteArgs = {
  workspace: typeof OWNED_WORKSPACE;
  project: typeof FIXTURE_PROJECT;
};

export function copyFixtureRoute(): ExplicitRouteArgs {
  return { workspace: OWNED_WORKSPACE, project: FIXTURE_PROJECT };
}

export type ListTreeArgs = ExplicitRouteArgs & {
  directory?: string;
  expected_session?: SessionIdentity;
  cursor?: string;
  page_size?: number;
};

export type ReadNoteArgs = ExplicitRouteArgs & {
  identifier: string;
  expected_session?: SessionIdentity;
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
  expected_session?: SessionIdentity;
  request_generation?: number;
  options?: SearchOptions;
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

export type InspectExtrasArgs = ExplicitRouteArgs & {
  extra_id?: string;
};

export type IngestDocumentArgs = ExplicitRouteArgs & {
  source_id: string;
};

export type InspectCloudArgs = ExplicitRouteArgs;
export type InspectSyncArgs = ExplicitRouteArgs;
export type ListSharesArgs = ExplicitRouteArgs;
export type InspectHooksArgs = ExplicitRouteArgs;
export type InspectProvidersArgs = ExplicitRouteArgs;
export type InspectRoutesArgs = ExplicitRouteArgs;
export type InspectPrivacyArgs = ExplicitRouteArgs;
export type InspectInstallArgs = ExplicitRouteArgs;
export type InspectBundleArgs = ExplicitRouteArgs;
export type InspectHelpArgs = ExplicitRouteArgs;
export type InspectReleaseArgs = ExplicitRouteArgs;

export type PreviewContextArgs = ExplicitRouteArgs & {
  expected_session?: SessionIdentity;
  request_generation?: number;
  options?: ContextOptions;
  cursor?: string;
  page_size?: number;
  identifier: string;
  query?: string;
};

export type ListActivityArgs = ExplicitRouteArgs & {
  expected_session?: SessionIdentity;
  request_generation?: number;
  options?: ActivityOptions;
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
  | { command: "inspect_extras"; args: InspectExtrasArgs }
  | { command: "ingest_document"; args: IngestDocumentArgs }
  | { command: "inspect_cloud"; args: InspectCloudArgs }
  | { command: "inspect_sync"; args: InspectSyncArgs }
  | { command: "list_shares"; args: ListSharesArgs }
  | { command: "inspect_hooks"; args: InspectHooksArgs }
  | { command: "inspect_providers"; args: InspectProvidersArgs }
  | { command: "inspect_routes"; args: InspectRoutesArgs }
  | { command: "inspect_privacy"; args: InspectPrivacyArgs }
  | { command: "inspect_install"; args: InspectInstallArgs }
  | { command: "inspect_bundle"; args: InspectBundleArgs }
  | { command: "inspect_help"; args: InspectHelpArgs }
  | { command: "inspect_release"; args: InspectReleaseArgs }
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
  cloud_allowed: false;
  provider_enabled: false;
}

export interface RuntimeStateDto {
  status: RuntimeStatus;
  project: typeof FIXTURE_PROJECT | null;
  profile: EngineProfile | null;
  session_generation: number | null;
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

export interface SessionIdentity {
  profile: EngineProfile;
  generation: number;
}

export interface TreeEntryDto {
  note_identifier?: string;
  identifier: string;
  title: string;
  kind: TreeEntryKind;
}

export interface TreePageDto {
  session?: SessionIdentity;
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
  session?: SessionIdentity;
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

export type SearchMode = "text" | "title" | "permalink";
export interface SearchOptions {
  mode?: SearchMode;
  entity_types?: string[];
  note_types?: string[];
  categories?: string[];
  tags?: string[];
  metadata_filters?: Record<string, unknown>;
  status?: string;
  after_date?: string;
  min_similarity?: number;
  compact?: boolean;
  valid_at?: string;
  valid_overlaps?: string;
  time_kind?: string;
}
export interface ContextOptions {
  depth?: number;
  max_related?: number;
  timeframe?: string;
  compact?: boolean;
}
export interface ActivityOptions {
  types?: string[];
  depth?: number;
  timeframe?: string;
}
export interface QueryIdentity {
  session: SessionIdentity;
  workspace: typeof OWNED_WORKSPACE;
  project: typeof FIXTURE_PROJECT;
  operation: "search_notes" | "preview_context" | "list_activity";
  arguments: Record<string, unknown>;
  request_generation: number;
}
export interface EngineHitDto {
  result_kind: string;
  identifier: string;
  note_identifier: string | null;
  title: string | null;
  excerpt: string | null;
  score: number | null;
  file_path: string | null;
  category: string | null;
  relation_type: string | null;
  from_entity: string | null;
  to_entity: string | null;
  to_name: string | null;
  created_at: string | null;
}
export interface EngineSearchPageDto {
  engine_search: true;
  session: SessionIdentity;
  request: QueryIdentity;
  query: string;
  hits: EngineHitDto[];
  page: number;
  page_size: number;
  next_cursor: string | null;
  has_more: boolean;
  total: number;
  total_is_exact: boolean;
}
export interface EngineContextDto {
  engine_context: true;
  session: SessionIdentity;
  request: QueryIdentity;
  identifier: string;
  results: { primary_result: EngineHitDto; observations: EngineHitDto[]; related_results: EngineHitDto[] }[];
  metadata: Record<string, unknown>;
  page: number;
  page_size: number;
  next_cursor: string | null;
  has_more: boolean;
}
export interface EngineActivityDto {
  engine_activity: true;
  session: SessionIdentity;
  request: QueryIdentity;
  entries: EngineHitDto[];
  page: number;
  page_size: number;
  next_cursor: null;
  has_more: null;
  total: null;
  total_is_exact: null;
}
export type SearchPageDto = FixtureSearchPageDto | EngineSearchPageDto;
export type ContextPreviewDto = FixtureContextPreviewDto | EngineContextDto;
export type ActivityPageDto = FixtureActivityPageDto | EngineActivityDto;

export interface FixtureSearchPageDto {
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

export interface FixtureContextPreviewDto {
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

export interface FixtureActivityPageDto {
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

export interface ExtraEntryDto {
  extra_id: string;
  kind: string;
  body: string;
}

export interface ExtrasCatalogDto {
  extra_id: string | null;
  extras: ExtraEntryDto[];
  extras_enabled: boolean;
  semantic_enabled: false;
  model_loaded: false;
  observation: NoteCrudObservationDto;
  engine_extras: false;
  official_pdf_office: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface IngestResultDto {
  source_id: string;
  extra_id: string;
  files: ImportedFileDto[];
  files_written: boolean;
  observation: ImportObservationDto;
  extras_enabled: boolean;
  engine_extras: false;
  official_pdf_office: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface CloudInspectionDto {
  cloud_enabled: false;
  remote_auth: false;
  credentials_present: false;
  cloud_allowed: false;
  connected: false;
  authenticated: false;
  cloud_claimed: false;
  local_offline: true;
  live_official_cloud_session: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_cloud: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface SyncInspectionDto {
  sync_enabled: false;
  sharing_enabled: false;
  remote_restore: false;
  last_sync: "none";
  synced: false;
  shared: false;
  sync_claimed: false;
  local_offline: true;
  live_official_cloud_session: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_sync: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface ShareRecordDto {
  identifier: string;
}

export interface ShareCatalogDto {
  shares: ShareRecordDto[];
  sharing_enabled: false;
  share_claimed: false;
  live_shared_remote: false;
  remote_restore: false;
  local_offline: true;
  live_official_cloud_session: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_share: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  files_written: false;
}

export interface HookRecordDto {
  identifier: string;
}

export interface HookInspectionDto {
  hooks: HookRecordDto[];
  hooks_enabled: false;
  agent_connected: false;
  files_written: false;
  installed: false;
  hook_claimed: false;
  local_offline: true;
  live_official_agent_session: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_hooks: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
  scanned_cursor_rules: false;
  scanned_user_agent_config: false;
}

export interface ProviderRecordDto {
  identifier: string;
  status: "unavailable";
  verified: false;
}

export interface ProviderInspectionDto {
  providers: ProviderRecordDto[];
  unavailable: ProviderRecordDto[];
  provider_enabled: false;
  semantic_enabled: false;
  model_loaded: false;
  embedding_backend: "none";
  backend_tier: "disabled";
  files_written: false;
  connected: false;
  provider_claimed: false;
  local_offline: true;
  live_provider_session: false;
  official_semantic: false;
  official_search: false;
  official_fetch: false;
  search_fetch_distinct: true;
  cloud_credential_route: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_providers: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface RouteRecordDto {
  workspace: string;
  project: string;
}

export interface RouteInspectionDto {
  routes: RouteRecordDto[];
  present_commands: string[];
  absent_commands: string[];
  cloud_allowed: false;
  sync_enabled: false;
  sharing_enabled: false;
  hooks_enabled: false;
  provider_enabled: false;
  semantic_enabled: false;
  cross_project_search_allowed: false;
  full_api_coverage: false;
  connected: false;
  synced: false;
  installed: false;
  files_written: false;
  route_claimed: false;
  local_offline: true;
  live_official_cloud_session: false;
  live_official_sync_session: false;
  live_official_agent_session: false;
  live_provider_session: false;
  remote_hosts_contacted: false;
  secrets_stored: false;
  env_tokens_read: false;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_routes: false;
  official_mcp_inferred: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface PrivacyRecordDto {
  identifier: string;
}

export interface PrivacyInspectionDto {
  catalog: PrivacyRecordDto[];
  license_path: "LICENSE";
  notice_path: "NOTICE";
  sbom_path: "docs/sbom/lockfile-inventory.json";
  license_present: true;
  notice_present: true;
  sbom_present: true;
  vulnerability_scan: "UNVERIFIED";
  human_legal_review: "UNVERIFIED";
  secrets_stored: false;
  env_tokens_read: false;
  remote_hosts_contacted: false;
  telemetry: false;
  cloud_allowed: false;
  provider_enabled: false;
  html_executed: false;
  executed: false;
  files_written: false;
  privacy_claimed: false;
  sbom_cleared: false;
  g7_passed: false;
  local_offline: true;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_privacy: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface InstallRecordDto {
  identifier: string;
}

export interface InstallInspectionDto {
  catalog: InstallRecordDto[];
  installer_bundle_active: false;
  signed: false;
  signing: "UNVERIFIED";
  upgrade_channel: false;
  native_gui: false;
  native_gui_status: "UNVERIFIED";
  installer_rollback: false;
  recovery_command: "restore_fixture";
  restore_sync_present: false;
  files_written: false;
  install_claimed: false;
  signed_upgrade: false;
  kill_recovery: "UNVERIFIED";
  job_object: "UNVERIFIED";
  sleep_resume: "UNVERIFIED";
  disk_failure: "UNVERIFIED";
  secrets_stored: false;
  env_tokens_read: false;
  remote_hosts_contacted: false;
  local_offline: true;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_install: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface BundleRecordDto {
  identifier: string;
}

export interface BundleInspectionDto {
  catalog: BundleRecordDto[];
  installer_artifact_present: false;
  installer_bundle_active: false;
  signed: false;
  signing: "UNVERIFIED";
  native_gui: false;
  native_gui_status: "UNVERIFIED";
  installer_rollback: false;
  recovery_command: "restore_fixture";
  restore_sync_present: false;
  files_written: false;
  bundle_claimed: false;
  installer_present: false;
  kill_recovery: "UNVERIFIED";
  job_object: "UNVERIFIED";
  sleep_resume: "UNVERIFIED";
  disk_failure: "UNVERIFIED";
  search_elapsed_ms: 0;
  search_elapsed_host_side: true;
  secrets_stored: false;
  env_tokens_read: false;
  remote_hosts_contacted: false;
  local_offline: true;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_bundle: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface HelpRecordDto {
  identifier: string;
}

export interface HelpInspectionDto {
  catalog: HelpRecordDto[];
  files_written: false;
  help_claimed: false;
  a11y_cleared: false;
  skip_link: true;
  nav_landmark: true;
  main_landmark: true;
  labelled_panels: true;
  focus_visible: true;
  keyboard_focusable: true;
  native_gui: false;
  native_gui_status: "UNVERIFIED";
  screen_reader: "UNVERIFIED";
  ime: "UNVERIFIED";
  preview_context_owned: true;
  list_activity_owned: true;
  engine_activity: false;
  recent_activity_mcp: "UNVERIFIED";
  build_context_mcp: "UNVERIFIED";
  recovery_inventory: ["list_backups", "restore_fixture"];
  recovery_command: "restore_fixture";
  restore_sync_present: false;
  installer_rollback: false;
  license_present: true;
  notice_present: true;
  sbom_present: true;
  g0_passed: false;
  g7_passed: false;
  secrets_stored: false;
  env_tokens_read: false;
  remote_hosts_contacted: false;
  local_offline: true;
  mixed_profiles: false;
  observation: NoteCrudObservationDto;
  engine_help: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
}

export interface ReleaseRecordDto {
  identifier: string;
}

export interface ReleaseInspectionDto {
  catalog: ReleaseRecordDto[];
  files_written: false;
  release_claimed: false;
  gate_passed: false;
  release_allowed: false;
  g0_passed: false;
  g7_passed: false;
  g0_status: "in_progress";
  g7_status: "not_started";
  decision: "do_not_release";
  mixed_profiles: false;
  release_commit: "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048";
  release_tool_count: 21;
  main_preview_commit: "3452c821d76c083823d020984d71e06904a1ff1e";
  main_preview_tool_count: 27;
  search_identity: "search";
  fetch_identity: "fetch";
  search_fetch_distinct: true;
  unknown_tools_auto_admitted: false;
  full_api_coverage: false;
  named_gaps: string[];
  prefix_buckets_hide_leaves: false;
  just_build_is_g0_probe: true;
  just_tauri_dev_is_desktop: true;
  just_tauri_build_is_desktop: true;
  just_contract_is_probe: true;
  call_tool_present: false;
  restore_sync_present: false;
  enable_provider_present: false;
  typed_command_count: 45;
  inspect_release_present: true;
  official_mcp_inferred_from_allowlist: false;
  license_present: true;
  notice_present: true;
  sbom_present: true;
  help_doc_consistent: true;
  verification_doc_consistent: true;
  secrets_stored: false;
  env_tokens_read: false;
  remote_hosts_contacted: false;
  local_offline: true;
  observation: NoteCrudObservationDto;
  engine_release: false;
  scanned_user_obsidian_vault: false;
  scanned_user_basic_memory_home: false;
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
  | { kind: "capabilities" } & CapabilitiesDto
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
  | { kind: "extras_catalog" } & ExtrasCatalogDto
  | { kind: "document_ingested" } & IngestResultDto
  | { kind: "cloud_inspection" } & CloudInspectionDto
  | { kind: "sync_inspection" } & SyncInspectionDto
  | { kind: "share_catalog" } & ShareCatalogDto
  | { kind: "hook_inspection" } & HookInspectionDto
  | { kind: "provider_inspection" } & ProviderInspectionDto
  | { kind: "route_inspection" } & RouteInspectionDto
  | { kind: "privacy_inspection" } & PrivacyInspectionDto
  | { kind: "install_inspection" } & InstallInspectionDto
  | { kind: "bundle_inspection" } & BundleInspectionDto
  | { kind: "help_inspection" } & HelpInspectionDto
  | { kind: "release_inspection" } & ReleaseInspectionDto
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
  session_generation: number | null;
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
    case "inspect_extras":
    case "ingest_document":
    case "inspect_cloud":
    case "inspect_sync":
    case "list_shares":
    case "inspect_hooks":
    case "inspect_providers":
    case "inspect_routes":
    case "inspect_privacy":
    case "inspect_install":
    case "inspect_bundle":
    case "inspect_help":
    case "inspect_release":
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

export const listTree = (args: { directory?: string; cursor?: string; page_size?: number; expected_session?: SessionIdentity } = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "tree_page" } & TreePageDto>({
    command: "list_tree",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(args.directory !== undefined ? { directory: args.directory } : {}),
      ...(args.expected_session ? { expected_session: args.expected_session } : {}),
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? QUERY_PAGE_SIZE,
    },
  });
};

export const readNote = (identifier: string, expected_session?: SessionIdentity) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "note_read" } & NoteReadDto>({
    command: "read_note",
    args: {
      workspace: route.workspace,
      project: route.project,
      identifier,
      ...(expected_session ? { expected_session } : {}),
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

export const searchNotes = (args: Omit<SearchNotesArgs, keyof ExplicitRouteArgs>) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "search_page" } & SearchPageDto>({
    command: "search_notes",
    args: {
      ...args,
      workspace: route.workspace,
      project: route.project,
      query: args.query,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? QUERY_PAGE_SIZE,
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

export const previewContext = (args: Omit<PreviewContextArgs, keyof ExplicitRouteArgs>) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "context_preview" } & ContextPreviewDto>({
    command: "preview_context",
    args: {
      ...args,
      workspace: route.workspace,
      project: route.project,
      identifier: args.identifier,
      page_size: args.page_size ?? QUERY_PAGE_SIZE,
      ...(args.query ? { query: args.query } : {}),
    },
  });
};

export const listActivity = (args: Omit<ListActivityArgs, keyof ExplicitRouteArgs> = {}) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "activity_page" } & ActivityPageDto>({
    command: "list_activity",
    args: {
      ...args,
      workspace: route.workspace,
      project: route.project,
      ...(args.cursor ? { cursor: args.cursor } : {}),
      page_size: args.page_size ?? QUERY_PAGE_SIZE,
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

export const inspectExtras = (extra_id?: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "extras_catalog" } & ExtrasCatalogDto>({
    command: "inspect_extras",
    args: {
      workspace: route.workspace,
      project: route.project,
      ...(extra_id ? { extra_id } : {}),
    },
  });
};

export const ingestDocument = (source_id: string) => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "document_ingested" } & IngestResultDto>({
    command: "ingest_document",
    args: {
      workspace: route.workspace,
      project: route.project,
      source_id,
    },
  });
};

export const inspectCloud = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "cloud_inspection" } & CloudInspectionDto>({
    command: "inspect_cloud",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectSync = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "sync_inspection" } & SyncInspectionDto>({
    command: "inspect_sync",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const listShares = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "share_catalog" } & ShareCatalogDto>({
    command: "list_shares",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectHooks = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "hook_inspection" } & HookInspectionDto>({
    command: "inspect_hooks",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectProviders = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "provider_inspection" } & ProviderInspectionDto>({
    command: "inspect_providers",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectRoutes = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "route_inspection" } & RouteInspectionDto>({
    command: "inspect_routes",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectPrivacy = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "privacy_inspection" } & PrivacyInspectionDto>({
    command: "inspect_privacy",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectInstall = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "install_inspection" } & InstallInspectionDto>({
    command: "inspect_install",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectBundle = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "bundle_inspection" } & BundleInspectionDto>({
    command: "inspect_bundle",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectHelp = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "help_inspection" } & HelpInspectionDto>({
    command: "inspect_help",
    args: {
      workspace: route.workspace,
      project: route.project,
    },
  });
};

export const inspectRelease = () => {
  const route = copyFixtureRoute();
  return invokeTyped<{ kind: "release_inspection" } & ReleaseInspectionDto>({
    command: "inspect_release",
    args: {
      workspace: route.workspace,
      project: route.project,
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
