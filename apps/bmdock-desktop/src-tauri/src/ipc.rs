use serde::{Deserialize, Serialize};

use crate::backups::{self, BackupCatalogDto, BackupStore, RestoreResultDto};
use crate::conflict::{self, ConflictCoordinator};
use crate::drafts::{self, DraftResultDto, DraftStore};
use crate::drain::{self, DrainPhase, DrainResultDto, HostDrain};
use crate::library::{
    self, ActivityPageDto, ContextPreviewDto, GraphPageDto, NoteDeleteDto, NoteEditDto,
    NoteLibrary, NoteMoveDto, NoteReadDto, NoteWriteDto, RecallBenchmarkDto, RelationListDto,
    SchemaValidateDto, SearchInspectorDto, SearchPageDto, TreePageDto,
};
use crate::preflight::{self, ConfigDiscoveryDto, PreflightDto};
use crate::routing::{self, ExplicitRouteArgs, ProjectCatalogDto, RouteState};
use crate::supervisor::{FailureKind, RuntimeSnapshot, ShutdownReceipt};
use crate::windows_runtime::{self, WindowsRuntimeDto};

pub const FIXTURE_PROJECT: &str = "bmdock-fixture";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpcCommandName {
    GetCapabilities,
    GetRuntimeState,
    SelectProject,
    ListProjects,
    RunPreflight,
    DiscoverConfig,
    ListTree,
    ReadNote,
    ListRelations,
    ExpandGraph,
    SearchNotes,
    InspectSearch,
    RunRecallBenchmark,
    SchemaValidate,
    PreviewContext,
    ListActivity,
    ListBackups,
    RestoreFixture,
    InspectWindowsRuntime,
    SaveDraft,
    LoadDraft,
    WriteNote,
    EditNote,
    MoveNote,
    DeleteNote,
    BeginShutdown,
}

pub fn allowed_commands() -> Vec<IpcCommandName> {
    vec![
        IpcCommandName::GetCapabilities,
        IpcCommandName::GetRuntimeState,
        IpcCommandName::SelectProject,
        IpcCommandName::ListProjects,
        IpcCommandName::RunPreflight,
        IpcCommandName::DiscoverConfig,
        IpcCommandName::ListTree,
        IpcCommandName::ReadNote,
        IpcCommandName::ListRelations,
        IpcCommandName::ExpandGraph,
        IpcCommandName::SearchNotes,
        IpcCommandName::InspectSearch,
        IpcCommandName::RunRecallBenchmark,
        IpcCommandName::SchemaValidate,
        IpcCommandName::PreviewContext,
        IpcCommandName::ListActivity,
        IpcCommandName::ListBackups,
        IpcCommandName::RestoreFixture,
        IpcCommandName::InspectWindowsRuntime,
        IpcCommandName::SaveDraft,
        IpcCommandName::LoadDraft,
        IpcCommandName::WriteNote,
        IpcCommandName::EditNote,
        IpcCommandName::MoveNote,
        IpcCommandName::DeleteNote,
        IpcCommandName::BeginShutdown,
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpcEventName {
    RuntimeState,
    Policy,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EmptyArgs {}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SelectProjectArgs {
    pub project: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ListTreeArgs {
    pub workspace: String,
    pub project: String,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

impl ListTreeArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ReadNoteArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
}

impl ReadNoteArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ListRelationsArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
}

impl ListRelationsArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ExpandGraphArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

impl ExpandGraphArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SearchNotesArgs {
    pub workspace: String,
    pub project: String,
    pub query: String,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

impl SearchNotesArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InspectSearchArgs {
    pub workspace: String,
    pub project: String,
    pub query: String,
    #[serde(default)]
    pub identifier: Option<String>,
}

impl InspectSearchArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RecallBenchmarkArgs {
    pub workspace: String,
    pub project: String,
    #[serde(default)]
    pub k: Option<u32>,
}

impl RecallBenchmarkArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SchemaValidateArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    #[serde(default)]
    pub schema_id: Option<String>,
}

impl SchemaValidateArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct PreviewContextArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    #[serde(default)]
    pub query: Option<String>,
}

impl PreviewContextArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ListActivityArgs {
    pub workspace: String,
    pub project: String,
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub page_size: Option<u32>,
}

impl ListActivityArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RestoreFixtureArgs {
    pub workspace: String,
    pub project: String,
    pub backup_id: String,
}

impl RestoreFixtureArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SaveDraftArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    pub body: String,
}

impl SaveDraftArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LoadDraftArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
}

impl LoadDraftArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WriteNoteArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    pub title: String,
    pub body: String,
}

impl WriteNoteArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct EditNoteArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    pub body: String,
}

impl EditNoteArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct MoveNoteArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
    pub destination: String,
}

impl MoveNoteArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct DeleteNoteArgs {
    pub workspace: String,
    pub project: String,
    pub identifier: String,
}

impl DeleteNoteArgs {
    fn route(&self) -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: self.workspace.clone(),
            project: self.project.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    tag = "command",
    content = "args",
    rename_all = "snake_case",
    deny_unknown_fields
)]
pub enum IpcCommand {
    GetCapabilities(EmptyArgs),
    GetRuntimeState(EmptyArgs),
    SelectProject(SelectProjectArgs),
    ListProjects(EmptyArgs),
    RunPreflight(EmptyArgs),
    DiscoverConfig(EmptyArgs),
    ListTree(ListTreeArgs),
    ReadNote(ReadNoteArgs),
    ListRelations(ListRelationsArgs),
    ExpandGraph(ExpandGraphArgs),
    SearchNotes(SearchNotesArgs),
    InspectSearch(InspectSearchArgs),
    RunRecallBenchmark(RecallBenchmarkArgs),
    SchemaValidate(SchemaValidateArgs),
    PreviewContext(PreviewContextArgs),
    ListActivity(ListActivityArgs),
    ListBackups(ExplicitRouteArgs),
    RestoreFixture(RestoreFixtureArgs),
    InspectWindowsRuntime(EmptyArgs),
    SaveDraft(SaveDraftArgs),
    LoadDraft(LoadDraftArgs),
    WriteNote(WriteNoteArgs),
    EditNote(EditNoteArgs),
    MoveNote(MoveNoteArgs),
    DeleteNote(DeleteNoteArgs),
    BeginShutdown(EmptyArgs),
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCategory {
    Policy,
    Schema,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct IpcError {
    pub category: ErrorCategory,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct PolicyDto {
    pub project: &'static str,
    pub arbitrary_paths_allowed: bool,
    pub raw_call_tool_allowed: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct CapabilitiesDto {
    pub commands: Vec<IpcCommandName>,
    pub events: Vec<IpcEventName>,
    pub policy: PolicyDto,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct RuntimeStateDto {
    pub status: String,
    pub project: Option<String>,
    pub profile: Option<String>,
    pub failure: Option<FailureKind>,
    pub shutdown: Option<ShutdownReceipt>,
    pub host_drain: DrainPhase,
    pub semantic_model_loaded: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum IpcResponse {
    Capabilities(CapabilitiesDto),
    RuntimeState(RuntimeStateDto),
    ProjectSelected { project: &'static str },
    ProjectCatalog(ProjectCatalogDto),
    Preflight(PreflightDto),
    ConfigDiscovery(ConfigDiscoveryDto),
    TreePage(TreePageDto),
    NoteRead(NoteReadDto),
    RelationList(RelationListDto),
    GraphPage(GraphPageDto),
    SearchPage(SearchPageDto),
    SearchInspector(SearchInspectorDto),
    RecallBenchmark(RecallBenchmarkDto),
    SchemaValidated(SchemaValidateDto),
    ContextPreview(ContextPreviewDto),
    ActivityPage(ActivityPageDto),
    BackupCatalog(BackupCatalogDto),
    FixtureRestored(RestoreResultDto),
    WindowsRuntime(WindowsRuntimeDto),
    DraftSaved(DraftResultDto),
    DraftLoaded(DraftResultDto),
    NoteWritten(NoteWriteDto),
    NoteEdited(NoteEditDto),
    NoteMoved(NoteMoveDto),
    NoteDeleted(NoteDeleteDto),
    ShutdownBegun(DrainResultDto),
    Error(IpcError),
}

impl From<library::LibraryError> for IpcError {
    fn from(error: library::LibraryError) -> Self {
        match error {
            library::LibraryError::Schema(message) => Self {
                category: ErrorCategory::Schema,
                message,
            },
            library::LibraryError::Policy(message) => Self {
                category: ErrorCategory::Policy,
                message,
            },
            library::LibraryError::Unsupported(message) => Self {
                category: ErrorCategory::Unsupported,
                message,
            },
        }
    }
}

#[cfg(test)]
pub fn dispatch(command: IpcCommand) -> Result<IpcResponse, IpcError> {
    dispatch_with_snapshot(
        command,
        RuntimeSnapshot {
            state: crate::supervisor::ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        },
    )
}

#[cfg(test)]
pub fn dispatch_with_snapshot(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
) -> Result<IpcResponse, IpcError> {
    let mut route = RouteState::default();
    dispatch_with_route(command, snapshot, &mut route)
}

#[cfg(test)]
pub fn dispatch_with_route(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
) -> Result<IpcResponse, IpcError> {
    dispatch_with_library(
        command,
        snapshot,
        route,
        &library::EmptyLibrary,
        &backups::EmptyBackupStore,
    )
}

#[cfg(test)]
pub fn dispatch_with_library(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
    library: &dyn NoteLibrary,
    backups: &dyn BackupStore,
) -> Result<IpcResponse, IpcError> {
    dispatch_with_stores(
        command,
        snapshot,
        route,
        library,
        backups,
        &drafts::EmptyDraftStore,
    )
}

#[cfg(test)]
pub fn dispatch_with_stores(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
    library: &dyn NoteLibrary,
    backups: &dyn BackupStore,
    drafts: &dyn DraftStore,
) -> Result<IpcResponse, IpcError> {
    let conflicts = ConflictCoordinator::default();
    let drain = HostDrain::default();
    dispatch_with_drain(
        command, snapshot, route, library, backups, drafts, &conflicts, &drain,
    )
}

#[cfg(test)]
pub fn dispatch_with_conflicts(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
    library: &dyn NoteLibrary,
    backups: &dyn BackupStore,
    drafts: &dyn DraftStore,
    conflicts: &ConflictCoordinator,
) -> Result<IpcResponse, IpcError> {
    let drain = HostDrain::default();
    dispatch_with_drain(
        command, snapshot, route, library, backups, drafts, conflicts, &drain,
    )
}

pub fn dispatch_with_drain(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
    library: &dyn NoteLibrary,
    backups: &dyn BackupStore,
    drafts: &dyn DraftStore,
    conflicts: &ConflictCoordinator,
    drain: &HostDrain,
) -> Result<IpcResponse, IpcError> {
    match command {
        IpcCommand::GetCapabilities(_) => Ok(IpcResponse::Capabilities(CapabilitiesDto {
            commands: allowed_commands(),
            events: vec![IpcEventName::RuntimeState, IpcEventName::Policy],
            policy: PolicyDto {
                project: FIXTURE_PROJECT,
                arbitrary_paths_allowed: false,
                raw_call_tool_allowed: false,
            },
        })),
        IpcCommand::GetRuntimeState(_) => Ok(IpcResponse::RuntimeState(runtime_state(
            snapshot, route, drain,
        ))),
        IpcCommand::SelectProject(args) if args.project == FIXTURE_PROJECT => {
            route.select_fixture();
            Ok(IpcResponse::ProjectSelected {
                project: FIXTURE_PROJECT,
            })
        }
        IpcCommand::SelectProject(_) => Err(IpcError {
            category: ErrorCategory::Policy,
            message: routing::POLICY_NON_OWNED_PROJECT.to_owned(),
        }),
        IpcCommand::ListProjects(_) => Ok(IpcResponse::ProjectCatalog(routing::owned_catalog())),
        IpcCommand::RunPreflight(_) => {
            Ok(IpcResponse::Preflight(preflight::run_preflight(&snapshot)))
        }
        IpcCommand::DiscoverConfig(_) => {
            Ok(IpcResponse::ConfigDiscovery(preflight::empty_discovery()))
        }
        IpcCommand::ListTree(args) => {
            require_explicit_fixture_route(&args.route())?;
            let page_size = library::bound_page_size(args.page_size)?;
            let cursor = library::validate_request_cursor(args.cursor.as_deref())?;
            let page = library.list_tree(cursor, page_size)?;
            let page = library::accept_tree_page(cursor, page)?;
            Ok(IpcResponse::TreePage(page))
        }
        IpcCommand::ReadNote(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_filesystem_identifier(&args.identifier)?;
            Ok(IpcResponse::NoteRead(library.read_note(&args.identifier)?))
        }
        IpcCommand::ListRelations(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_filesystem_identifier(&args.identifier)?;
            Ok(IpcResponse::RelationList(
                library.list_relations(&args.identifier)?,
            ))
        }
        IpcCommand::ExpandGraph(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            let page_size = library::bound_page_size(args.page_size)?;
            let cursor = library::validate_request_cursor(args.cursor.as_deref())?;
            let page = library.expand_graph(&args.identifier, cursor, page_size)?;
            let page = library::accept_graph_page(cursor, page)?;
            Ok(IpcResponse::GraphPage(page))
        }
        IpcCommand::SearchNotes(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_search_query(&args.query)?;
            let page_size = library::bound_page_size(args.page_size)?;
            let cursor = library::validate_request_cursor(args.cursor.as_deref())?;
            let page = library.search_notes(&args.query, cursor, page_size)?;
            let page = library::accept_search_page(cursor, page)?;
            Ok(IpcResponse::SearchPage(page))
        }
        IpcCommand::InspectSearch(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_search_query(&args.query)?;
            let identifier = library::reject_inspect_identifier(args.identifier.as_deref())?;
            let inspector = library.inspect_search(&args.query, identifier)?;
            Ok(IpcResponse::SearchInspector(
                library::accept_search_inspector(inspector)?,
            ))
        }
        IpcCommand::RunRecallBenchmark(args) => {
            require_explicit_fixture_route(&args.route())?;
            let k = library::bound_recall_k(args.k)?;
            let report = library.run_recall_benchmark(k)?;
            Ok(IpcResponse::RecallBenchmark(
                library::accept_recall_benchmark(report)?,
            ))
        }
        IpcCommand::SchemaValidate(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            let schema_id = library::resolve_schema_id(args.schema_id.as_deref())?;
            let report = library.schema_validate(&args.identifier, Some(schema_id))?;
            Ok(IpcResponse::SchemaValidated(library::accept_schema_report(
                report,
            )?))
        }
        IpcCommand::PreviewContext(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            let query = library::reject_preview_query(args.query.as_deref())?;
            Ok(IpcResponse::ContextPreview(
                library.preview_context(&args.identifier, query)?,
            ))
        }
        IpcCommand::ListActivity(args) => {
            require_explicit_fixture_route(&args.route())?;
            let page_size = library::bound_page_size(args.page_size)?;
            let cursor = library::validate_request_cursor(args.cursor.as_deref())?;
            let page = library.list_activity(cursor, page_size)?;
            let page = library::accept_activity_page(cursor, page)?;
            Ok(IpcResponse::ActivityPage(page))
        }
        IpcCommand::ListBackups(route_args) => {
            require_explicit_fixture_route(&route_args)?;
            Ok(IpcResponse::BackupCatalog(backups.list_backups()?))
        }
        IpcCommand::RestoreFixture(args) => {
            require_explicit_fixture_route(&args.route())?;
            backups::reject_backup_id(&args.backup_id).map_err(IpcError::from)?;
            Ok(IpcResponse::FixtureRestored(
                backups.restore_fixture(&args.backup_id)?,
            ))
        }
        IpcCommand::InspectWindowsRuntime(_) => Ok(IpcResponse::WindowsRuntime(
            windows_runtime::inspect_windows_runtime(),
        )),
        IpcCommand::SaveDraft(args) => {
            require_explicit_fixture_route(&args.route())?;
            drafts::reject_draft_identifier(&args.identifier).map_err(IpcError::from)?;
            Ok(IpcResponse::DraftSaved(
                drafts.save_draft(&args.identifier, &args.body)?,
            ))
        }
        IpcCommand::LoadDraft(args) => {
            require_explicit_fixture_route(&args.route())?;
            drafts::reject_draft_identifier(&args.identifier).map_err(IpcError::from)?;
            Ok(IpcResponse::DraftLoaded(
                drafts.load_draft(&args.identifier)?,
            ))
        }
        IpcCommand::WriteNote(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            library::reject_empty_title(&args.title)?;
            refuse_if_draining(drain)?;
            conflict::refuse_auto_retry(&snapshot);
            match conflicts.try_acquire(conflict::identifier_keys(&args.identifier)) {
                Err(_) => Ok(IpcResponse::NoteWritten(conflict::write_conflict(
                    &args.identifier,
                    &args.title,
                    &args.body,
                ))),
                Ok(guard) => {
                    let written = library.write_note(&args.identifier, &args.title, &args.body)?;
                    drop(guard);
                    Ok(IpcResponse::NoteWritten(written))
                }
            }
        }
        IpcCommand::EditNote(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            refuse_if_draining(drain)?;
            conflict::refuse_auto_retry(&snapshot);
            match conflicts.try_acquire(conflict::identifier_keys(&args.identifier)) {
                Err(_) => Ok(IpcResponse::NoteEdited(conflict::edit_conflict(
                    &args.identifier,
                    &args.body,
                ))),
                Ok(guard) => {
                    let edited = library.edit_note(&args.identifier, &args.body)?;
                    drop(guard);
                    Ok(IpcResponse::NoteEdited(edited))
                }
            }
        }
        IpcCommand::MoveNote(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            library::reject_filesystem_destination(&args.destination)?;
            refuse_if_draining(drain)?;
            conflict::refuse_auto_retry(&snapshot);
            match conflicts.try_acquire(conflict::move_keys(&args.identifier, &args.destination)) {
                Err(_) => Ok(IpcResponse::NoteMoved(conflict::move_conflict(
                    &args.identifier,
                    &args.destination,
                    "",
                ))),
                Ok(guard) => {
                    let moved = library.move_note(&args.identifier, &args.destination)?;
                    drop(guard);
                    Ok(IpcResponse::NoteMoved(moved))
                }
            }
        }
        IpcCommand::DeleteNote(args) => {
            require_explicit_fixture_route(&args.route())?;
            library::reject_note_identifier(&args.identifier)?;
            refuse_if_draining(drain)?;
            conflict::refuse_auto_retry(&snapshot);
            match conflicts.try_acquire(conflict::identifier_keys(&args.identifier)) {
                Err(_) => Ok(IpcResponse::NoteDeleted(conflict::delete_conflict(
                    &args.identifier,
                ))),
                Ok(guard) => {
                    let deleted = library.delete_note(&args.identifier)?;
                    drop(guard);
                    Ok(IpcResponse::NoteDeleted(deleted))
                }
            }
        }
        IpcCommand::BeginShutdown(_) => Ok(IpcResponse::ShutdownBegun(
            drain.begin(&snapshot, conflicts),
        )),
    }
}

fn require_explicit_fixture_route(route: &ExplicitRouteArgs) -> Result<(), IpcError> {
    routing::require_explicit_fixture_route(route).map_err(|message| IpcError {
        category: ErrorCategory::Policy,
        message: message.to_owned(),
    })
}

fn refuse_if_draining(drain: &HostDrain) -> Result<(), IpcError> {
    if drain.is_closed() {
        return Err(IpcError {
            category: ErrorCategory::Unsupported,
            message: drain::UNSUPPORTED_HOST_DRAINING.to_owned(),
        });
    }
    Ok(())
}

fn runtime_state(
    snapshot: RuntimeSnapshot,
    route: &RouteState,
    drain: &HostDrain,
) -> RuntimeStateDto {
    RuntimeStateDto {
        status: snapshot.state.as_str().to_owned(),
        project: route.project.clone(),
        profile: snapshot.profile.map(|profile| profile.id().to_owned()),
        failure: snapshot.failure,
        shutdown: snapshot.shutdown.or_else(|| drain.last_shutdown()),
        host_drain: drain.phase(),
        semantic_model_loaded: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preflight::POLICY_NON_OWNED_PATH;
    use crate::supervisor::ConnectionState;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};

    #[test]
    fn capabilities_are_explicit_and_fail_closed() {
        let IpcResponse::Capabilities(capabilities) =
            dispatch(IpcCommand::GetCapabilities(EmptyArgs {})).unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(FIXTURE_PROJECT, "bmdock-fixture");
        assert_eq!(capabilities.policy.project, "bmdock-fixture");
        assert!(!capabilities.policy.arbitrary_paths_allowed);
        assert!(!capabilities.policy.raw_call_tool_allowed);
        assert_eq!(capabilities.commands, allowed_commands());
        assert_eq!(
            capabilities.commands,
            vec![
                IpcCommandName::GetCapabilities,
                IpcCommandName::GetRuntimeState,
                IpcCommandName::SelectProject,
                IpcCommandName::ListProjects,
                IpcCommandName::RunPreflight,
                IpcCommandName::DiscoverConfig,
                IpcCommandName::ListTree,
                IpcCommandName::ReadNote,
                IpcCommandName::ListRelations,
                IpcCommandName::ExpandGraph,
                IpcCommandName::SearchNotes,
                IpcCommandName::InspectSearch,
                IpcCommandName::RunRecallBenchmark,
                IpcCommandName::SchemaValidate,
                IpcCommandName::PreviewContext,
                IpcCommandName::ListActivity,
                IpcCommandName::ListBackups,
                IpcCommandName::RestoreFixture,
                IpcCommandName::InspectWindowsRuntime,
                IpcCommandName::SaveDraft,
                IpcCommandName::LoadDraft,
                IpcCommandName::WriteNote,
                IpcCommandName::EditNote,
                IpcCommandName::MoveNote,
                IpcCommandName::DeleteNote,
                IpcCommandName::BeginShutdown,
            ]
        );
        assert_eq!(capabilities.commands.len(), 26);
        assert_eq!(
            capabilities.events,
            vec![IpcEventName::RuntimeState, IpcEventName::Policy]
        );
        let json = serde_json::to_value(&IpcResponse::Capabilities(capabilities)).unwrap();
        let commands = json["commands"].as_array().unwrap();
        assert_eq!(commands.len(), 26);
        assert!(commands.iter().any(|command| command == "list_projects"));
        assert!(commands.iter().any(|command| command == "select_project"));
        assert!(commands.iter().any(|command| command == "list_tree"));
        assert!(commands.iter().any(|command| command == "read_note"));
        assert!(commands.iter().any(|command| command == "list_relations"));
        assert!(commands.iter().any(|command| command == "expand_graph"));
        assert!(commands.iter().any(|command| command == "search_notes"));
        assert!(commands.iter().any(|command| command == "inspect_search"));
        assert!(commands
            .iter()
            .any(|command| command == "run_recall_benchmark"));
        assert!(commands.iter().any(|command| command == "schema_validate"));
        assert!(commands.iter().any(|command| command == "preview_context"));
        assert!(commands.iter().any(|command| command == "list_activity"));
        assert!(commands.iter().any(|command| command == "list_backups"));
        assert!(commands.iter().any(|command| command == "restore_fixture"));
        assert!(commands
            .iter()
            .any(|command| command == "inspect_windows_runtime"));
        assert!(commands.iter().any(|command| command == "save_draft"));
        assert!(commands.iter().any(|command| command == "load_draft"));
        assert!(commands.iter().any(|command| command == "write_note"));
        assert!(commands.iter().any(|command| command == "edit_note"));
        assert!(commands.iter().any(|command| command == "move_note"));
        assert!(commands.iter().any(|command| command == "delete_note"));
        assert!(commands.iter().any(|command| command == "begin_shutdown"));
        assert!(commands.iter().any(|command| command == "preview_context"));
        assert!(commands.iter().any(|command| command == "list_activity"));
        assert!(!commands.iter().any(|command| command == "call_tool"));
        assert!(!commands.iter().any(|command| command == "search"));
        assert!(!commands.iter().any(|command| command == "schema_infer"));
        assert!(!commands.iter().any(|command| command == "schema_diff"));
        assert!(!commands.iter().any(|command| command == "recent_activity"));
        assert!(!commands.iter().any(|command| command == "build_context"));
    }

    #[test]
    fn select_project_accepts_fixture() {
        let mut route = RouteState::default();
        let IpcResponse::ProjectSelected { project } = dispatch_with_route(
            IpcCommand::SelectProject(SelectProjectArgs {
                project: FIXTURE_PROJECT.to_owned(),
            }),
            RuntimeSnapshot {
                state: crate::supervisor::ConnectionState::NotStarted,
                profile: None,
                child_pid: None,
                failure: None,
                shutdown: None,
            },
            &mut route,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(project, "bmdock-fixture");
        assert_eq!(route.project.as_deref(), Some(FIXTURE_PROJECT));
        assert_eq!(
            route.workspace.as_deref(),
            Some(crate::routing::OWNED_WORKSPACE_ID)
        );
    }

    #[test]
    fn project_policy_rejects_non_fixture() {
        let snapshot = RuntimeSnapshot {
            state: crate::supervisor::ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        };
        let mut route = RouteState::default();
        route.select_fixture();
        for project in [
            "C:\\Users\\someone\\vault",
            "C:\\Users\\someone\\.basic-memory",
            "/home/someone/.basic-memory",
            "",
        ] {
            let result = dispatch_with_route(
                IpcCommand::SelectProject(SelectProjectArgs {
                    project: project.to_owned(),
                }),
                snapshot.clone(),
                &mut route,
            );
            assert_eq!(result.unwrap_err().category, ErrorCategory::Policy);
            assert_eq!(route.project.as_deref(), Some(FIXTURE_PROJECT));
            assert_eq!(
                route.workspace.as_deref(),
                Some(crate::routing::OWNED_WORKSPACE_ID)
            );
        }
        let mut empty_route = RouteState::default();
        let result = dispatch_with_route(
            IpcCommand::SelectProject(SelectProjectArgs {
                project: "C:\\Users\\someone\\Documents\\Obsidian".to_owned(),
            }),
            snapshot,
            &mut empty_route,
        );
        assert_eq!(result.unwrap_err().category, ErrorCategory::Policy);
        assert_eq!(empty_route.project, None);
        assert_eq!(empty_route.workspace, None);
    }

    #[test]
    fn runtime_state_before_start_is_not_started() {
        let IpcResponse::RuntimeState(state) =
            dispatch(IpcCommand::GetRuntimeState(EmptyArgs {})).unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(state.status, "not_started");
        assert_eq!(state.project, None);
        assert_eq!(state.profile, None);
        assert_eq!(state.failure, None);
        assert_eq!(state.shutdown, None);
        assert_eq!(state.host_drain, DrainPhase::Idle);
        assert!(!state.semantic_model_loaded);
        let json = serde_json::to_value(&IpcResponse::RuntimeState(state)).unwrap();
        assert!(json.get("child_pid").is_none());
        assert_eq!(json["host_drain"], "idle");
        assert_eq!(json["semantic_model_loaded"], false);
    }

    #[test]
    fn runtime_state_projects_supervisor_snapshot() {
        let response = dispatch_with_snapshot(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(crate::supervisor::EngineProfile::Release),
                child_pid: Some(42),
                failure: None,
                shutdown: None,
            },
        )
        .unwrap();
        let IpcResponse::RuntimeState(state) = response else {
            panic!("wrong response variant")
        };
        assert_eq!(state.status, "connected");
        assert_eq!(state.profile.as_deref(), Some("release"));
        assert_eq!(state.project, None);
        assert_eq!(state.failure, None);
        assert_eq!(state.shutdown, None);
        assert_eq!(state.host_drain, DrainPhase::Idle);
        assert!(!state.semantic_model_loaded);
    }

    #[test]
    fn runtime_state_dto_matches_typescript_field_names() {
        let response = dispatch_with_snapshot(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            RuntimeSnapshot {
                state: ConnectionState::Stopped,
                profile: Some(crate::supervisor::EngineProfile::MainPreview),
                child_pid: Some(9),
                failure: Some(FailureKind::TimeoutUnknown),
                shutdown: Some(ShutdownReceipt {
                    transport_cancelled: false,
                    child_exited: false,
                    forced: true,
                    timeout_unknown: true,
                    exit_code: None,
                }),
            },
        )
        .unwrap();
        let json = serde_json::to_value(&response).unwrap();
        assert_eq!(json["kind"], "runtime_state");
        assert_eq!(json["status"], "stopped");
        assert_eq!(json["profile"], "main-preview");
        assert_eq!(json["failure"], "timeout_unknown");
        assert!(json["project"].is_null());
        assert!(json.get("child_pid").is_none());
        assert_eq!(json["shutdown"]["transport_cancelled"], false);
        assert_eq!(json["shutdown"]["child_exited"], false);
        assert_eq!(json["shutdown"]["forced"], true);
        assert_eq!(json["shutdown"]["timeout_unknown"], true);
        assert!(json["shutdown"]["exit_code"].is_null());
        assert_eq!(json["host_drain"], "idle");
        assert_eq!(json["semantic_model_loaded"], false);
    }

    #[test]
    fn unknown_command_and_arbitrary_path_are_rejected_by_dto() {
        let unknown = serde_json::from_str::<IpcCommand>(
            r#"{"command":"call_tool","args":{"name":"read_note"}}"#,
        );
        assert!(unknown.is_err());
        let path = serde_json::from_str::<IpcCommand>(
            r#"{"command":"select_project","args":{"project":"bmdock-fixture","path":"C:\\vault"}}"#,
        );
        assert!(path.is_err());
        let path_on_runtime = serde_json::from_str::<IpcCommand>(
            r#"{"command":"get_runtime_state","args":{"path":"C:\\vault"}}"#,
        );
        assert!(path_on_runtime.is_err());
        let path_on_preflight = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_preflight","args":{"path":"C:\\vault"}}"#,
        );
        assert!(path_on_preflight.is_err());
        let path_on_discovery = serde_json::from_str::<IpcCommand>(
            r#"{"command":"discover_config","args":{"path":"C:\\Users\\someone\\.basic-memory"}}"#,
        );
        assert!(path_on_discovery.is_err());
        let root_on_discovery = serde_json::from_str::<IpcCommand>(
            r#"{"command":"discover_config","args":{"root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(root_on_discovery.is_err());
        let top_level_path = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_preflight","args":{},"path":"C:\\vault"}"#,
        );
        assert!(top_level_path.is_err());
        let path_on_list = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_projects","args":{"path":"C:\\Users\\someone\\vault"}}"#,
        );
        assert!(path_on_list.is_err());
        let root_on_list = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_projects","args":{"root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(root_on_list.is_err());
        let search_without_route = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"query":"cross-project"}}"#,
        );
        assert!(search_without_route.is_err());
        let mcp_search_identity = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search","args":{"query":"cross-project"}}"#,
        );
        assert!(mcp_search_identity.is_err());
        let extra_id_on_search = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","id":"welcome"}}"#,
        );
        assert!(extra_id_on_search.is_err());
        let extra_path_on_search = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_search.is_err());
        let extra_root_on_search = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_search.is_err());
        let missing_query = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(missing_query.is_err());
        let well_formed_search = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","page_size":2}}"#,
        );
        assert!(well_formed_search.is_ok());
        let extra_id_on_inspect = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","id":"welcome"}}"#,
        );
        assert!(extra_id_on_inspect.is_err());
        let extra_path_on_inspect = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_inspect.is_err());
        let extra_root_on_inspect = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_inspect.is_err());
        let missing_query_inspect = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(missing_query_inspect.is_err());
        let inspect_without_route = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"query":"欢迎"}}"#,
        );
        assert!(inspect_without_route.is_err());
        let well_formed_inspect = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_search","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","query":"欢迎","identifier":"welcome"}}"#,
        );
        assert!(well_formed_inspect.is_ok());
        let extra_path_on_recall = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_recall_benchmark","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","k":5,"path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_recall.is_err());
        let extra_root_on_recall = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_recall_benchmark","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_recall.is_err());
        let recall_without_route = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_recall_benchmark","args":{"k":5}}"#,
        );
        assert!(recall_without_route.is_err());
        let well_formed_recall = serde_json::from_str::<IpcCommand>(
            r#"{"command":"run_recall_benchmark","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","k":5}}"#,
        );
        assert!(well_formed_recall.is_ok());
        let extra_path_on_schema = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_validate","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_schema.is_err());
        let extra_root_on_schema = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_validate","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_schema.is_err());
        let missing_identifier_schema = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_validate","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(missing_identifier_schema.is_err());
        let schema_without_route = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_validate","args":{"identifier":"welcome"}}"#,
        );
        assert!(schema_without_route.is_err());
        let mcp_schema_infer = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_infer","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(mcp_schema_infer.is_err());
        let mcp_schema_diff = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_diff","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(mcp_schema_diff.is_err());
        let well_formed_schema = serde_json::from_str::<IpcCommand>(
            r#"{"command":"schema_validate","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","schema_id":"note"}}"#,
        );
        assert!(well_formed_schema.is_ok());
        let extra_path_on_preview = serde_json::from_str::<IpcCommand>(
            r#"{"command":"preview_context","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_preview.is_err());
        let extra_root_on_preview = serde_json::from_str::<IpcCommand>(
            r#"{"command":"preview_context","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_preview.is_err());
        let missing_identifier_preview = serde_json::from_str::<IpcCommand>(
            r#"{"command":"preview_context","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(missing_identifier_preview.is_err());
        let well_formed_preview = serde_json::from_str::<IpcCommand>(
            r#"{"command":"preview_context","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","query":"欢迎"}}"#,
        );
        assert!(well_formed_preview.is_ok());
        let extra_path_on_activity = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_activity","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_activity.is_err());
        let extra_root_on_activity = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_activity","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_activity.is_err());
        let well_formed_activity = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_activity","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","page_size":2}}"#,
        );
        assert!(well_formed_activity.is_ok());
        let mcp_recent_activity = serde_json::from_str::<IpcCommand>(
            r#"{"command":"recent_activity","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(mcp_recent_activity.is_err());
        let mcp_build_context = serde_json::from_str::<IpcCommand>(
            r#"{"command":"build_context","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(mcp_build_context.is_err());
        let incomplete_write_note = serde_json::from_str::<IpcCommand>(
            r#"{"command":"write_note","args":{"project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_write_note.is_err());
        let raw_call_tool = serde_json::from_str::<IpcCommand>(
            r#"{"command":"call_tool","args":{"name":"write_note","arguments":{"title":"x"}}}"#,
        );
        assert!(raw_call_tool.is_err());
        let incomplete_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_read.is_err());
        let extra_path_on_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_read.is_err());
        let extra_path_on_relations = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_relations","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_relations.is_err());
        let extra_root_on_relations = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_relations","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_relations.is_err());
        let incomplete_relations = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_relations","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_relations.is_err());
        let extra_path_on_graph = serde_json::from_str::<IpcCommand>(
            r#"{"command":"expand_graph","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_graph.is_err());
        let extra_root_on_graph = serde_json::from_str::<IpcCommand>(
            r#"{"command":"expand_graph","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_graph.is_err());
        let incomplete_graph = serde_json::from_str::<IpcCommand>(
            r#"{"command":"expand_graph","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_graph.is_err());
        let extra_path_on_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_tree.is_err());
        let extra_fs_path_on_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","filesystem_path":"%APPDATA%\\\\Obsidian"}}"#,
        );
        assert!(extra_fs_path_on_tree.is_err());
        let extra_path_on_backups = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_backups","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_backups.is_err());
        let extra_root_on_backups = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_backups","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_backups.is_err());
        let extra_path_on_restore = serde_json::from_str::<IpcCommand>(
            r#"{"command":"restore_fixture","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","backup_id":"fixture-welcome","path":"%APPDATA%\\\\Obsidian"}}"#,
        );
        assert!(extra_path_on_restore.is_err());
        let extra_root_on_restore = serde_json::from_str::<IpcCommand>(
            r#"{"command":"restore_fixture","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","backup_id":"fixture-welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_restore.is_err());
        let incomplete_restore = serde_json::from_str::<IpcCommand>(
            r#"{"command":"restore_fixture","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_restore.is_err());
        let well_formed_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(well_formed_read.is_ok());
        let well_formed_relations = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_relations","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(well_formed_relations.is_ok());
        let well_formed_graph = serde_json::from_str::<IpcCommand>(
            r#"{"command":"expand_graph","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","page_size":2}}"#,
        );
        assert!(well_formed_graph.is_ok());
        let well_formed_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","page_size":2}}"#,
        );
        assert!(well_formed_tree.is_ok());
        let well_formed_backups = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_backups","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(well_formed_backups.is_ok());
        let well_formed_restore = serde_json::from_str::<IpcCommand>(
            r#"{"command":"restore_fixture","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","backup_id":"fixture-welcome"}}"#,
        );
        assert!(well_formed_restore.is_ok());
        let path_on_windows_runtime = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_windows_runtime","args":{"path":"C:\\vault"}}"#,
        );
        assert!(path_on_windows_runtime.is_err());
        let root_on_windows_runtime = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_windows_runtime","args":{"root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(root_on_windows_runtime.is_err());
        let well_formed_windows_runtime = serde_json::from_str::<IpcCommand>(
            r#"{"command":"inspect_windows_runtime","args":{}}"#,
        );
        assert!(well_formed_windows_runtime.is_ok());
        let extra_path_on_save_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"x","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_save_draft.is_err());
        let extra_root_on_save_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"x","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_save_draft.is_err());
        let extra_path_on_load_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"load_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"%APPDATA%\\\\Obsidian"}}"#,
        );
        assert!(extra_path_on_load_draft.is_err());
        let extra_root_on_load_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"load_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_load_draft.is_err());
        let incomplete_save_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(incomplete_save_draft.is_err());
        let incomplete_load_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"load_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_load_draft.is_err());
        let well_formed_save_draft = serde_json::from_str::<IpcCommand>(
            r##"{"command":"save_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"# 中文草稿\n\n参见 [[欢迎]]。\n"}}"##,
        );
        assert!(well_formed_save_draft.is_ok());
        let well_formed_load_draft = serde_json::from_str::<IpcCommand>(
            r#"{"command":"load_draft","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(well_formed_load_draft.is_ok());
        let extra_path_on_write = serde_json::from_str::<IpcCommand>(
            r#"{"command":"write_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","title":"中文夹具笔记","body":"x","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_write.is_err());
        let extra_root_on_write = serde_json::from_str::<IpcCommand>(
            r#"{"command":"write_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","title":"中文夹具笔记","body":"x","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_write.is_err());
        let extra_path_on_edit = serde_json::from_str::<IpcCommand>(
            r#"{"command":"edit_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"x","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_edit.is_err());
        let extra_root_on_edit = serde_json::from_str::<IpcCommand>(
            r#"{"command":"edit_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"x","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_edit.is_err());
        let extra_path_on_move = serde_json::from_str::<IpcCommand>(
            r#"{"command":"move_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","destination":"renamed","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_move.is_err());
        let extra_root_on_move = serde_json::from_str::<IpcCommand>(
            r#"{"command":"move_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","destination":"renamed","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_move.is_err());
        let extra_path_on_delete = serde_json::from_str::<IpcCommand>(
            r#"{"command":"delete_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"%APPDATA%\\\\Obsidian"}}"#,
        );
        assert!(extra_path_on_delete.is_err());
        let extra_root_on_delete = serde_json::from_str::<IpcCommand>(
            r#"{"command":"delete_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(extra_root_on_delete.is_err());
        let incomplete_edit = serde_json::from_str::<IpcCommand>(
            r#"{"command":"edit_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(incomplete_edit.is_err());
        let incomplete_move = serde_json::from_str::<IpcCommand>(
            r#"{"command":"move_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(incomplete_move.is_err());
        let incomplete_delete = serde_json::from_str::<IpcCommand>(
            r#"{"command":"delete_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_delete.is_err());
        let well_formed_write = serde_json::from_str::<IpcCommand>(
            r##"{"command":"write_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","title":"中文夹具笔记","body":"# 中文夹具笔记\n\n参见 [[欢迎]]。\n"}}"##,
        );
        assert!(well_formed_write.is_ok());
        let well_formed_edit = serde_json::from_str::<IpcCommand>(
            r##"{"command":"edit_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","body":"# 覆盖后的笔记\n\n[[欢迎]]\n"}}"##,
        );
        assert!(well_formed_edit.is_ok());
        let well_formed_move = serde_json::from_str::<IpcCommand>(
            r#"{"command":"move_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","destination":"renamed"}}"#,
        );
        assert!(well_formed_move.is_ok());
        let well_formed_delete = serde_json::from_str::<IpcCommand>(
            r#"{"command":"delete_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(well_formed_delete.is_ok());
        let path_on_begin_shutdown = serde_json::from_str::<IpcCommand>(
            r#"{"command":"begin_shutdown","args":{"path":"C:\\vault"}}"#,
        );
        assert!(path_on_begin_shutdown.is_err());
        let root_on_begin_shutdown = serde_json::from_str::<IpcCommand>(
            r#"{"command":"begin_shutdown","args":{"root":"/home/someone/.basic-memory"}}"#,
        );
        assert!(root_on_begin_shutdown.is_err());
        let well_formed_begin_shutdown =
            serde_json::from_str::<IpcCommand>(r#"{"command":"begin_shutdown","args":{}}"#);
        assert!(well_formed_begin_shutdown.is_ok());
    }

    #[test]
    fn list_projects_returns_owned_catalog_without_user_vaults() {
        let snapshot = RuntimeSnapshot {
            state: crate::supervisor::ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        };
        let before = snapshot.clone();
        let mut route = RouteState::default();
        let IpcResponse::ProjectCatalog(catalog) = dispatch_with_route(
            IpcCommand::ListProjects(EmptyArgs {}),
            snapshot.clone(),
            &mut route,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert_eq!(route.project, None);
        assert_eq!(route.workspace, None);
        assert_eq!(catalog.workspaces.len(), 1);
        assert_eq!(catalog.workspaces[0].id, crate::routing::OWNED_WORKSPACE_ID);
        assert_eq!(catalog.workspaces[0].kind, crate::routing::OWNED_KIND);
        assert_eq!(catalog.projects.len(), 1);
        assert_eq!(catalog.projects[0].id, FIXTURE_PROJECT);
        assert_eq!(
            catalog.projects[0].workspace,
            crate::routing::OWNED_WORKSPACE_ID
        );
        assert!(!catalog.scanned_user_obsidian_vault);
        assert!(!catalog.scanned_user_basic_memory_home);
        assert!(!catalog.cross_project_search_allowed);
        assert!(!catalog.implicit_current_project_writes);
        assert!(!catalog.cloud_or_credential_required);
        assert!(catalog.local_offline);
        assert!(!catalog.files_written);
        let json = serde_json::to_value(&IpcResponse::ProjectCatalog(catalog)).unwrap();
        assert_eq!(json["kind"], "project_catalog");
        assert_eq!(json["workspaces"][0]["id"], "bmdock-workspace");
        assert_eq!(json["projects"][0]["id"], "bmdock-fixture");
        assert_eq!(json["scanned_user_obsidian_vault"], false);
        assert_eq!(json["scanned_user_basic_memory_home"], false);
        assert_eq!(json["cross_project_search_allowed"], false);
        assert_eq!(json["implicit_current_project_writes"], false);
        assert_eq!(json["cloud_or_credential_required"], false);
        assert_eq!(json["local_offline"], true);
        assert_eq!(json["files_written"], false);
        assert!(json.get("child_pid").is_none());
        assert!(json.get("search").is_none());
        assert!(json.get("query").is_none());
    }

    #[test]
    fn selected_fixture_is_projected_without_implicit_writes() {
        let snapshot = RuntimeSnapshot {
            state: ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        };
        let mut route = RouteState::default();
        let IpcResponse::RuntimeState(before) = dispatch_with_route(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            snapshot.clone(),
            &mut route,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(before.project, None);
        dispatch_with_route(
            IpcCommand::SelectProject(SelectProjectArgs {
                project: FIXTURE_PROJECT.to_owned(),
            }),
            snapshot.clone(),
            &mut route,
        )
        .unwrap();
        let IpcResponse::RuntimeState(after) = dispatch_with_route(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            snapshot.clone(),
            &mut route,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(after.project.as_deref(), Some(FIXTURE_PROJECT));
        assert_eq!(snapshot.state, ConnectionState::NotStarted);
        assert!(!routing::owned_catalog().implicit_current_project_writes);
        assert!(!routing::owned_catalog().files_written);
        assert!(!routing::owned_catalog().cross_project_search_allowed);
    }

    #[test]
    fn run_preflight_is_read_only_and_does_not_spawn() {
        let snapshot = RuntimeSnapshot {
            state: crate::supervisor::ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        };
        let before = snapshot.clone();
        let IpcResponse::Preflight(report) =
            dispatch_with_snapshot(IpcCommand::RunPreflight(EmptyArgs {}), snapshot.clone())
                .unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert!(!report.host.engine_spawned);
        assert!(!report.host.files_written);
        assert!(report.host.supervisor_idle);
        assert_eq!(report.host.supervisor_status, "not_started");
        assert_eq!(report.profiles[0].expected_tools, 21);
        assert_eq!(report.profiles[1].expected_tools, 27);
        let json = serde_json::to_value(&IpcResponse::Preflight(report)).unwrap();
        assert_eq!(json["kind"], "preflight");
        assert!(json.get("child_pid").is_none());
        assert_eq!(json["host"]["engine_spawned"], false);
        assert_eq!(json["host"]["files_written"], false);
    }

    #[test]
    fn run_preflight_reports_connected_lifecycle_without_start_or_stop() {
        let snapshot = RuntimeSnapshot {
            state: ConnectionState::Connected,
            profile: Some(crate::supervisor::EngineProfile::Release),
            child_pid: Some(42),
            failure: None,
            shutdown: None,
        };
        let before = snapshot.clone();
        let IpcResponse::Preflight(report) =
            dispatch_with_snapshot(IpcCommand::RunPreflight(EmptyArgs {}), snapshot.clone())
                .unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert!(report.host.engine_spawned);
        assert!(!report.host.files_written);
        assert!(!report.host.supervisor_idle);
        assert_eq!(report.host.supervisor_status, "connected");
        assert_eq!(report.host.supervisor_status, snapshot.state.as_str());
        let json = serde_json::to_value(&IpcResponse::Preflight(report)).unwrap();
        assert_eq!(json["host"]["engine_spawned"], true);
        assert_eq!(json["host"]["files_written"], false);
        assert!(json.get("child_pid").is_none());
    }

    #[test]
    fn discover_config_default_is_empty_not_user_vault() {
        let IpcResponse::ConfigDiscovery(discovery) =
            dispatch(IpcCommand::DiscoverConfig(EmptyArgs {})).unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(discovery.root, "none");
        assert!(discovery.candidates.is_empty());
        assert!(!discovery.scanned_user_basic_memory_home);
        assert!(!discovery.copied_or_rewrote_production_config);
        let json = serde_json::to_value(&IpcResponse::ConfigDiscovery(discovery)).unwrap();
        assert_eq!(json["kind"], "config_discovery");
        assert_eq!(json["root"], "none");
        assert_eq!(json["candidates"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn non_owned_path_inspect_is_policy_not_schema() {
        let error = preflight::discover_config(Some(r"C:\Users\someone\vault")).unwrap_err();
        assert_eq!(error, POLICY_NON_OWNED_PATH);
        let mapped = IpcError {
            category: ErrorCategory::Policy,
            message: error.to_owned(),
        };
        assert_eq!(mapped.category, ErrorCategory::Policy);
        let project_error =
            routing::owned_project_or_policy(r"C:\Users\someone\Documents\Obsidian").unwrap_err();
        assert_eq!(project_error, routing::POLICY_NON_OWNED_PROJECT);
    }

    struct PanicLibrary;

    impl NoteLibrary for PanicLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn list_relations(
            &self,
            _identifier: &str,
        ) -> Result<library::RelationListDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn expand_graph(
            &self,
            _identifier: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::GraphPageDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn search_notes(
            &self,
            _query: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::SearchPageDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn inspect_search(
            &self,
            _query: &str,
            _identifier: Option<&str>,
        ) -> Result<library::SearchInspectorDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn run_recall_benchmark(
            &self,
            _k: u32,
        ) -> Result<library::RecallBenchmarkDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn schema_validate(
            &self,
            _identifier: &str,
            _schema_id: Option<&str>,
        ) -> Result<library::SchemaValidateDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn preview_context(
            &self,
            _identifier: &str,
            _query: Option<&str>,
        ) -> Result<library::ContextPreviewDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn list_activity(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::ActivityPageDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn write_note(
            &self,
            _identifier: &str,
            _title: &str,
            _body: &str,
        ) -> Result<library::NoteWriteDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn edit_note(
            &self,
            _identifier: &str,
            _body: &str,
        ) -> Result<library::NoteEditDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn move_note(
            &self,
            _identifier: &str,
            _destination: &str,
        ) -> Result<library::NoteMoveDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }

        fn delete_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteDeleteDto, library::LibraryError> {
            panic!("policy rejection must not open the library")
        }
    }

    struct TruncatingLibrary;

    impl NoteLibrary for TruncatingLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            Ok(library::TreePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
            })
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            Err(library::LibraryError::unsupported(
                library::UNSUPPORTED_LIBRARY_UNAVAILABLE,
            ))
        }

        fn expand_graph(
            &self,
            identifier: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::GraphPageDto, library::LibraryError> {
            Ok(library::GraphPageDto {
                identifier: identifier.to_owned(),
                nodes: Vec::new(),
                edges: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::Unclassified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                engine_graph: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
                depth: library::GRAPH_DEPTH,
            })
        }

        fn search_notes(
            &self,
            query: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::SearchPageDto, library::LibraryError> {
            Ok(library::SearchPageDto {
                query: query.to_owned(),
                hits: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::Unclassified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                semantic_enabled: false,
                engine_search: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_activity(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::ActivityPageDto, library::LibraryError> {
            Ok(library::ActivityPageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::Unclassified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                engine_activity: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }
    }

    fn idle_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            state: crate::supervisor::ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        }
    }

    fn fixture_tree_args(cursor: Option<&str>, page_size: Option<u32>) -> ListTreeArgs {
        ListTreeArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            cursor: cursor.map(ToOwned::to_owned),
            page_size,
        }
    }

    fn fixture_read_args(identifier: &str) -> ReadNoteArgs {
        ReadNoteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    fn fixture_relations_args(identifier: &str) -> ListRelationsArgs {
        ListRelationsArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    fn fixture_graph_args(
        identifier: &str,
        cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> ExpandGraphArgs {
        ExpandGraphArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            cursor: cursor.map(ToOwned::to_owned),
            page_size,
        }
    }

    fn fixture_search_args(
        query: &str,
        cursor: Option<&str>,
        page_size: Option<u32>,
    ) -> SearchNotesArgs {
        SearchNotesArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            query: query.to_owned(),
            cursor: cursor.map(ToOwned::to_owned),
            page_size,
        }
    }

    fn fixture_inspect_args(query: &str, identifier: Option<&str>) -> InspectSearchArgs {
        InspectSearchArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            query: query.to_owned(),
            identifier: identifier.map(ToOwned::to_owned),
        }
    }

    fn fixture_recall_args(k: Option<u32>) -> RecallBenchmarkArgs {
        RecallBenchmarkArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            k,
        }
    }

    fn fixture_schema_args(identifier: &str, schema_id: Option<&str>) -> SchemaValidateArgs {
        SchemaValidateArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            schema_id: schema_id.map(ToOwned::to_owned),
        }
    }

    fn fixture_preview_args(identifier: &str, query: Option<&str>) -> PreviewContextArgs {
        PreviewContextArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            query: query.map(ToOwned::to_owned),
        }
    }

    fn fixture_activity_args(cursor: Option<&str>, page_size: Option<u32>) -> ListActivityArgs {
        ListActivityArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            cursor: cursor.map(ToOwned::to_owned),
            page_size,
        }
    }

    #[test]
    fn list_tree_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::TreePage(page) = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(library::DEFAULT_PAGE_SIZE))),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(page.entries.is_empty());
        assert_eq!(page.next_cursor, None);
        assert_eq!(page.page, 1);
        assert!(!page.truncated);
        let json = serde_json::to_value(&IpcResponse::TreePage(page)).unwrap();
        assert_eq!(json["kind"], "tree_page");
        assert_eq!(json["truncated"], false);
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("path").is_none());
    }

    #[test]
    fn read_note_without_library_is_unsupported() {
        let mut route = RouteState::default();
        let error = dispatch_with_library(
            IpcCommand::ReadNote(fixture_read_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(error.category, ErrorCategory::Unsupported);
        assert_eq!(error.message, library::UNSUPPORTED_LIBRARY_UNAVAILABLE);
    }

    #[test]
    fn non_fixture_tree_and_read_are_policy_and_do_not_open() {
        let snapshot = idle_snapshot();
        let mut route = RouteState::default();
        let tree = dispatch_with_library(
            IpcCommand::ListTree(ListTreeArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(tree.category, ErrorCategory::Policy);
        let workspace = dispatch_with_library(
            IpcCommand::ListTree(ListTreeArgs {
                workspace: "user-home".to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(workspace.category, ErrorCategory::Policy);
        let identifier = dispatch_with_library(
            IpcCommand::ReadNote(ReadNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(identifier.category, ErrorCategory::Policy);
        let relations_route = dispatch_with_library(
            IpcCommand::ListRelations(ListRelationsArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(relations_route.category, ErrorCategory::Policy);
        let relations_path = dispatch_with_library(
            IpcCommand::ListRelations(ListRelationsArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(relations_path.category, ErrorCategory::Policy);
        let graph_route = dispatch_with_library(
            IpcCommand::ExpandGraph(ExpandGraphArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_route.category, ErrorCategory::Policy);
        let graph_path = dispatch_with_library(
            IpcCommand::ExpandGraph(ExpandGraphArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_path.category, ErrorCategory::Policy);
        let search_route = dispatch_with_library(
            IpcCommand::SearchNotes(SearchNotesArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                query: "欢迎".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_route.category, ErrorCategory::Policy);
        let search_query_path = dispatch_with_library(
            IpcCommand::SearchNotes(SearchNotesArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                query: r"C:\Users\someone\vault\note.md".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_query_path.category, ErrorCategory::Policy);
        let inspect_route = dispatch_with_library(
            IpcCommand::InspectSearch(InspectSearchArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                query: "欢迎".to_owned(),
                identifier: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(inspect_route.category, ErrorCategory::Policy);
        let inspect_query_path = dispatch_with_library(
            IpcCommand::InspectSearch(InspectSearchArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                query: r"C:\Users\someone\vault\note.md".to_owned(),
                identifier: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(inspect_query_path.category, ErrorCategory::Policy);
        let inspect_identifier_path = dispatch_with_library(
            IpcCommand::InspectSearch(InspectSearchArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                query: "欢迎".to_owned(),
                identifier: Some(r"C:\Users\someone\vault\note.md".to_owned()),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(inspect_identifier_path.category, ErrorCategory::Policy);
        let recall_route = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(RecallBenchmarkArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                k: Some(5),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(recall_route.category, ErrorCategory::Policy);
        let schema_route = dispatch_with_library(
            IpcCommand::SchemaValidate(SchemaValidateArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
                schema_id: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(schema_route.category, ErrorCategory::Policy);
        let schema_path = dispatch_with_library(
            IpcCommand::SchemaValidate(SchemaValidateArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
                schema_id: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(schema_path.category, ErrorCategory::Policy);
        let schema_id_path = dispatch_with_library(
            IpcCommand::SchemaValidate(SchemaValidateArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "welcome".to_owned(),
                schema_id: Some(r"C:\Users\someone\vault\note.md".to_owned()),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(schema_id_path.category, ErrorCategory::Policy);
        let preview_route = dispatch_with_library(
            IpcCommand::PreviewContext(PreviewContextArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
                query: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(preview_route.category, ErrorCategory::Policy);
        let preview_path = dispatch_with_library(
            IpcCommand::PreviewContext(PreviewContextArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
                query: None,
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(preview_path.category, ErrorCategory::Policy);
        let preview_query_path = dispatch_with_library(
            IpcCommand::PreviewContext(PreviewContextArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "welcome".to_owned(),
                query: Some(r"C:\Users\someone\vault\note.md".to_owned()),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(preview_query_path.category, ErrorCategory::Policy);
        let activity_route = dispatch_with_library(
            IpcCommand::ListActivity(ListActivityArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                cursor: None,
                page_size: Some(2),
            }),
            snapshot,
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(activity_route.category, ErrorCategory::Policy);
        assert_eq!(route.project, None);
    }

    #[test]
    fn page_size_and_cursor_fail_closed_as_schema() {
        let mut route = RouteState::default();
        let zero = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(0))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(zero.category, ErrorCategory::Schema);
        let huge = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(library::MAX_PAGE_SIZE + 1))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(huge.category, ErrorCategory::Schema);
        let empty_cursor = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(Some(""), Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(empty_cursor.category, ErrorCategory::Schema);
        let truncated = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(2))),
            idle_snapshot(),
            &mut route,
            &TruncatingLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(truncated.category, ErrorCategory::Unsupported);
        let graph_zero = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("welcome", None, Some(0))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_zero.category, ErrorCategory::Schema);
        let graph_huge = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args(
                "welcome",
                None,
                Some(library::MAX_PAGE_SIZE + 1),
            )),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_huge.category, ErrorCategory::Schema);
        let graph_empty_cursor = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("welcome", Some(""), Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_empty_cursor.category, ErrorCategory::Schema);
        let graph_truncated = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("welcome", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &TruncatingLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(graph_truncated.category, ErrorCategory::Unsupported);
        let search_zero = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", None, Some(0))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_zero.category, ErrorCategory::Schema);
        let search_huge = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args(
                "欢迎",
                None,
                Some(library::MAX_PAGE_SIZE + 1),
            )),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_huge.category, ErrorCategory::Schema);
        let search_empty_cursor = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", Some(""), Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_empty_cursor.category, ErrorCategory::Schema);
        let search_empty_query = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_empty_query.category, ErrorCategory::Schema);
        let inspect_empty_query = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("", None)),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(inspect_empty_query.category, ErrorCategory::Schema);
        let inspect_empty_identifier = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", Some(""))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(inspect_empty_identifier.category, ErrorCategory::Schema);
        let recall_zero = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(Some(0))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(recall_zero.category, ErrorCategory::Schema);
        let recall_huge = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(Some(library::MAX_PAGE_SIZE + 1))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(recall_huge.category, ErrorCategory::Schema);
        let schema_empty_identifier = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("", None)),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(schema_empty_identifier.category, ErrorCategory::Schema);
        let schema_empty_schema_id = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("welcome", Some(""))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(schema_empty_schema_id.category, ErrorCategory::Schema);
        let search_truncated = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &TruncatingLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(search_truncated.category, ErrorCategory::Unsupported);
        let activity_zero = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(None, Some(0))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(activity_zero.category, ErrorCategory::Schema);
        let activity_huge = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(
                None,
                Some(library::MAX_PAGE_SIZE + 1),
            )),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(activity_huge.category, ErrorCategory::Schema);
        let activity_empty_cursor = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(Some(""), Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(activity_empty_cursor.category, ErrorCategory::Schema);
        let activity_truncated = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(None, Some(2))),
            idle_snapshot(),
            &mut route,
            &TruncatingLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(activity_truncated.category, ErrorCategory::Unsupported);
    }

    #[test]
    fn fixture_library_read_matches_physical_markdown() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t11-ipc-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let body = "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n";
        for index in 1..=5 {
            std::fs::write(
                dir.join(format!("note-{index:02}.md")),
                format!("# 笔记 {index}\n\n夹具正文 [[欢迎]]\n"),
            )
            .unwrap();
        }
        std::fs::write(dir.join("welcome.md"), body).unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::TreePage(first) = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(first.entries.len(), 2);
        assert_eq!(first.next_cursor.as_deref(), Some("2"));
        assert!(!first.truncated);
        assert_eq!(first.page, 1);
        let second = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(first.next_cursor.as_deref(), Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap();
        let IpcResponse::TreePage(second) = second else {
            panic!("wrong response variant")
        };
        assert_eq!(second.page, 2);
        assert_eq!(second.next_cursor.as_deref(), Some("4"));
        let repeated_follow = library::follow_tree_pages(&library, 2).unwrap();
        assert_eq!(repeated_follow.0.len(), 6);
        assert_eq!(repeated_follow.1, 3);
        let IpcResponse::NoteRead(note) = dispatch_with_library(
            IpcCommand::ReadNote(fixture_read_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let disk = std::fs::read_to_string(dir.join("welcome.md")).unwrap();
        assert_eq!(note.body, disk);
        assert_eq!(note.body, body);
        assert!(note.body.contains("[[欢迎]]"));
        assert_eq!(note.title, "中文夹具笔记");
        assert!(note.observation.disk_verified);
        assert!(note.observation.envelope_is_not_disk_proof);
        assert_eq!(
            note.observation.classified_as,
            library::ObservationClass::BodyMatchesDisk
        );
        let json = serde_json::to_value(&IpcResponse::NoteRead(note)).unwrap();
        assert_eq!(json["kind"], "note_read");
        assert_eq!(json["observation"]["disk_verified"], true);
        assert!(json.get("path").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn tree_read_does_not_merge_engine_profiles() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::TreePage(page) = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(2))),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::TreePage(page)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["entries"].as_array().unwrap().len(), 0);
    }

    struct PanicBackupStore;

    impl BackupStore for PanicBackupStore {
        fn list_backups(&self) -> Result<backups::BackupCatalogDto, library::LibraryError> {
            panic!("policy rejection must not open the backup store")
        }

        fn restore_fixture(
            &self,
            _backup_id: &str,
        ) -> Result<backups::RestoreResultDto, library::LibraryError> {
            panic!("policy rejection must not open the backup store")
        }
    }

    fn fixture_backup_args() -> ExplicitRouteArgs {
        ExplicitRouteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
        }
    }

    fn fixture_restore_args(backup_id: &str) -> RestoreFixtureArgs {
        RestoreFixtureArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            backup_id: backup_id.to_owned(),
        }
    }

    #[test]
    fn list_backups_empty_store_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::BackupCatalog(catalog) = dispatch_with_library(
            IpcCommand::ListBackups(fixture_backup_args()),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(catalog.backups.is_empty());
        assert!(!catalog.scanned_user_obsidian_vault);
        assert!(!catalog.scanned_user_basic_memory_home);
        assert!(!catalog.files_written);
        assert!(catalog.local_offline);
        let json = serde_json::to_value(&IpcResponse::BackupCatalog(catalog)).unwrap();
        assert_eq!(json["kind"], "backup_catalog");
        assert_eq!(json["scanned_user_obsidian_vault"], false);
        assert_eq!(json["files_written"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
    }

    #[test]
    fn restore_without_store_is_unsupported() {
        let mut route = RouteState::default();
        let error = dispatch_with_library(
            IpcCommand::RestoreFixture(fixture_restore_args("fixture-welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(error.category, ErrorCategory::Unsupported);
        assert_eq!(error.message, backups::UNSUPPORTED_BACKUP_UNAVAILABLE);
    }

    #[test]
    fn non_fixture_backup_and_restore_are_policy_and_do_not_open() {
        let snapshot = idle_snapshot();
        let mut route = RouteState::default();
        let list = dispatch_with_library(
            IpcCommand::ListBackups(ExplicitRouteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &PanicBackupStore,
        )
        .unwrap_err();
        assert_eq!(list.category, ErrorCategory::Policy);
        let workspace = dispatch_with_library(
            IpcCommand::ListBackups(ExplicitRouteArgs {
                workspace: "user-home".to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &PanicBackupStore,
        )
        .unwrap_err();
        assert_eq!(workspace.category, ErrorCategory::Policy);
        let restore = dispatch_with_library(
            IpcCommand::RestoreFixture(RestoreFixtureArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                backup_id: r"%APPDATA%\Obsidian\vault".to_owned(),
            }),
            snapshot,
            &mut route,
            &library::EmptyLibrary,
            &PanicBackupStore,
        )
        .unwrap_err();
        assert_eq!(restore.category, ErrorCategory::Policy);
        assert_eq!(route.project, None);
    }

    #[test]
    fn fixture_restore_observes_physical_files_not_restored_text() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t12-ipc-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let snapshots = dir.join("snapshots");
        let target = dir.join("target");
        let store = backups::FixtureBackupStore::new(snapshots, target.clone()).unwrap();
        let body = "# 中文夹具备份\n\n这是 BMDock 自有恢复正文。参见 [[欢迎]]。\n";
        store
            .seed_backup("fixture-welcome", &[("welcome", body)])
            .unwrap();
        let mut route = RouteState::default();
        let snapshot = idle_snapshot();
        let before = snapshot.clone();
        let IpcResponse::BackupCatalog(catalog) = dispatch_with_library(
            IpcCommand::ListBackups(fixture_backup_args()),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert_eq!(catalog.backups.len(), 1);
        assert_eq!(catalog.backups[0].id, "fixture-welcome");
        assert!(!catalog.files_written);
        assert!(!catalog.scanned_user_obsidian_vault);
        assert!(!catalog.scanned_user_basic_memory_home);

        let IpcResponse::FixtureRestored(result) = dispatch_with_library(
            IpcCommand::RestoreFixture(fixture_restore_args("fixture-welcome")),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        let dest = target.join("welcome.md");
        assert!(dest.is_file(), "restore must observe a physical owned file");
        let disk = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert!(result.files_written);
        assert!(result.observation.disk_verified);
        assert!(result.observation.envelope_is_not_disk_proof);
        assert_eq!(
            result.observation.classified_as,
            backups::RestoreClass::DiskVerified
        );
        let json = serde_json::to_value(&IpcResponse::FixtureRestored(result)).unwrap();
        assert_eq!(json["kind"], "fixture_restored");
        assert_eq!(json["files_written"], true);
        assert_eq!(json["observation"]["disk_verified"], true);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());

        let IpcResponse::BackupCatalog(after) = dispatch_with_library(
            IpcCommand::ListBackups(fixture_backup_args()),
            snapshot,
            &mut route,
            &library::EmptyLibrary,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(after.files_written);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn envelope_restore_is_accepted_unverified_and_does_not_write() {
        let store = backups::EnvelopeBackupStore {
            backup_id: "fixture-welcome".to_owned(),
        };
        let mut route = RouteState::default();
        let IpcResponse::FixtureRestored(result) = dispatch_with_library(
            IpcCommand::RestoreFixture(fixture_restore_args("fixture-welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!result.files_written);
        assert!(!result.observation.disk_verified);
        assert_eq!(
            result.observation.classified_as,
            backups::RestoreClass::AcceptedUnverified
        );
        let json = serde_json::to_value(&IpcResponse::FixtureRestored(result)).unwrap();
        assert_eq!(json["kind"], "fixture_restored");
        assert_eq!(json["files_written"], false);
        assert_ne!(json["observation"]["classified_as"], "restored");
    }

    #[test]
    fn backup_restore_does_not_merge_engine_profiles() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::BackupCatalog(catalog) = dispatch_with_library(
            IpcCommand::ListBackups(fixture_backup_args()),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::BackupCatalog(catalog)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["backups"].as_array().unwrap().len(), 0);
    }

    struct PanicDraftStore;

    impl DraftStore for PanicDraftStore {
        fn save_draft(
            &self,
            _identifier: &str,
            _body: &str,
        ) -> Result<drafts::DraftResultDto, library::LibraryError> {
            panic!("policy rejection must not open the draft store")
        }

        fn load_draft(
            &self,
            _identifier: &str,
        ) -> Result<drafts::DraftResultDto, library::LibraryError> {
            panic!("policy rejection must not open the draft store")
        }
    }

    fn fixture_save_draft_args(identifier: &str, body: &str) -> SaveDraftArgs {
        SaveDraftArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            body: body.to_owned(),
        }
    }

    fn fixture_load_draft_args(identifier: &str) -> LoadDraftArgs {
        LoadDraftArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    #[test]
    fn inspect_windows_runtime_does_not_spawn_write_or_open_stores() {
        let snapshot = idle_snapshot();
        let before = snapshot.clone();
        let mut route = RouteState::default();
        let IpcResponse::WindowsRuntime(dto) = dispatch_with_stores(
            IpcCommand::InspectWindowsRuntime(EmptyArgs {}),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &PanicBackupStore,
            &PanicDraftStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert_eq!(route.project, None);
        assert!(!dto.files_written);
        assert!(!dto.webview2_session_verified);
        assert!(!dto.job_object_assigned);
        assert!(!dto.scanned_user_obsidian_vault);
        assert!(!dto.scanned_user_basic_memory_home);
        assert!(!dto.installer_bundle_active);
        let json = serde_json::to_value(&IpcResponse::WindowsRuntime(dto)).unwrap();
        assert_eq!(json["kind"], "windows_runtime");
        assert_eq!(json["webview2_session_verified"], false);
        assert_eq!(json["job_object_assigned"], false);
        assert_eq!(json["installer_bundle_active"], false);
        assert_eq!(json["files_written"], false);
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("child_pid").is_none());
        assert!(json.get("path").is_none());
        assert!(json.get("backup_id").is_none());
    }

    #[test]
    fn inspect_windows_runtime_taxonomy_stays_unverified() {
        let mut route = RouteState::default();
        let IpcResponse::WindowsRuntime(dto) = dispatch_with_stores(
            IpcCommand::InspectWindowsRuntime(EmptyArgs {}),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &PanicBackupStore,
            &PanicDraftStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(
            !dto.webview2_session_verified,
            "compiled exe / npm build / cargo test are not native GUI"
        );
        if dto.webview2_files_present {
            assert!(
                !dto.webview2_session_verified,
                "WebView2 files present is not a WebView2 session"
            );
        }
        if dto.job_object_api_documented {
            assert!(
                !dto.job_object_assigned,
                "Job Object API/docs is not Job Object assignment"
            );
        }
        assert!(
            !dto.job_object_assigned,
            "T12 fixture restore is not Windows recovery"
        );
        let json = serde_json::to_value(&IpcResponse::WindowsRuntime(dto)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("protocolVersion").is_none());
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        assert_ne!(release.expected_tools(), preview.expected_tools());
    }

    #[test]
    fn load_draft_empty_store_is_empty_session_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::DraftLoaded(loaded) = dispatch_with_library(
            IpcCommand::LoadDraft(fixture_load_draft_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(loaded.body.is_empty());
        assert!(!loaded.files_written);
        assert!(!loaded.engine_persisted);
        assert!(!loaded.scanned_user_obsidian_vault);
        assert!(!loaded.scanned_user_basic_memory_home);
        assert_eq!(loaded.observation.classified_as, drafts::DraftClass::Empty);
        let json = serde_json::to_value(&IpcResponse::DraftLoaded(loaded)).unwrap();
        assert_eq!(json["kind"], "draft_loaded");
        assert_eq!(json["engine_persisted"], false);
        assert_eq!(json["files_written"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
    }

    #[test]
    fn save_draft_without_store_is_unsupported() {
        let mut route = RouteState::default();
        let error = dispatch_with_library(
            IpcCommand::SaveDraft(fixture_save_draft_args(
                "welcome",
                "# 中文草稿\n\n参见 [[欢迎]]。\n",
            )),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(error.category, ErrorCategory::Unsupported);
        assert_eq!(error.message, drafts::UNSUPPORTED_DRAFT_UNAVAILABLE);
        assert_ne!(error.message, library::UNSUPPORTED_LIBRARY_UNAVAILABLE);
        assert_ne!(error.message, backups::UNSUPPORTED_BACKUP_UNAVAILABLE);
    }

    #[test]
    fn non_fixture_draft_is_policy_and_does_not_open() {
        let snapshot = idle_snapshot();
        let mut route = RouteState::default();
        let save = dispatch_with_stores(
            IpcCommand::SaveDraft(SaveDraftArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
                body: "x".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &PanicDraftStore,
        )
        .unwrap_err();
        assert_eq!(save.category, ErrorCategory::Policy);
        let workspace = dispatch_with_stores(
            IpcCommand::LoadDraft(LoadDraftArgs {
                workspace: "user-home".to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "welcome".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &PanicDraftStore,
        )
        .unwrap_err();
        assert_eq!(workspace.category, ErrorCategory::Policy);
        let identifier = dispatch_with_stores(
            IpcCommand::SaveDraft(SaveDraftArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"%APPDATA%\Obsidian\welcome.md".to_owned(),
                body: "x".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &PanicDraftStore,
        )
        .unwrap_err();
        assert_eq!(identifier.category, ErrorCategory::Policy);
        let empty_identifier = dispatch_with_stores(
            IpcCommand::LoadDraft(LoadDraftArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "".to_owned(),
            }),
            snapshot,
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &PanicDraftStore,
        )
        .unwrap_err();
        assert_eq!(empty_identifier.category, ErrorCategory::Schema);
        assert_eq!(route.project, None);
    }

    #[test]
    fn fixture_draft_save_reload_observes_physical_files_not_saved_text() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t14-ipc-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let store = drafts::FixtureDraftStore::new(dir.clone()).unwrap();
        let body = "# 中文草稿\n\n这是 BMDock 自有草稿正文。参见 [[欢迎]]。\n";
        let mut route = RouteState::default();
        let snapshot = idle_snapshot();
        let before = snapshot.clone();
        let IpcResponse::DraftSaved(saved) = dispatch_with_stores(
            IpcCommand::SaveDraft(fixture_save_draft_args("welcome", body)),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        let dest = dir.join("welcome.md");
        assert!(
            dest.is_file(),
            "save must observe a physical owned draft file"
        );
        let disk = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert_eq!(saved.body, body);
        assert!(saved.files_written);
        assert!(!saved.engine_persisted);
        assert!(saved.observation.disk_verified);
        assert!(saved.observation.envelope_is_not_disk_proof);
        assert_eq!(
            saved.observation.classified_as,
            drafts::DraftClass::DiskVerified
        );
        let json = serde_json::to_value(&IpcResponse::DraftSaved(saved)).unwrap();
        assert_eq!(json["kind"], "draft_saved");
        assert_eq!(json["files_written"], true);
        assert_eq!(json["engine_persisted"], false);
        assert_eq!(json["observation"]["disk_verified"], true);
        assert_ne!(json["observation"]["classified_as"], "saved");
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());

        let IpcResponse::DraftLoaded(loaded) = dispatch_with_stores(
            IpcCommand::LoadDraft(fixture_load_draft_args("welcome")),
            snapshot,
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(loaded.body, disk);
        assert_eq!(loaded.body, body);
        assert!(!loaded.engine_persisted);
        assert!(loaded.observation.disk_verified);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn envelope_draft_is_accepted_unverified_and_does_not_write() {
        let store = drafts::EnvelopeDraftStore {
            body: "saved".to_owned(),
        };
        let mut route = RouteState::default();
        let IpcResponse::DraftSaved(saved) = dispatch_with_stores(
            IpcCommand::SaveDraft(fixture_save_draft_args("welcome", "ignored")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!saved.files_written);
        assert!(!saved.engine_persisted);
        assert!(!saved.observation.disk_verified);
        assert_eq!(
            saved.observation.classified_as,
            drafts::DraftClass::AcceptedUnverified
        );
        let json = serde_json::to_value(&IpcResponse::DraftSaved(saved)).unwrap();
        assert_eq!(json["kind"], "draft_saved");
        assert_eq!(json["files_written"], false);
        assert_ne!(json["observation"]["classified_as"], "saved");
    }

    #[test]
    fn draft_commands_do_not_merge_engine_profiles() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::DraftLoaded(loaded) = dispatch_with_library(
            IpcCommand::LoadDraft(fixture_load_draft_args("welcome")),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::DraftLoaded(loaded)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["engine_persisted"], false);
    }

    fn fixture_write_args(identifier: &str, title: &str, body: &str) -> WriteNoteArgs {
        WriteNoteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
        }
    }

    fn fixture_edit_args(identifier: &str, body: &str) -> EditNoteArgs {
        EditNoteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            body: body.to_owned(),
        }
    }

    fn fixture_move_args(identifier: &str, destination: &str) -> MoveNoteArgs {
        MoveNoteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
            destination: destination.to_owned(),
        }
    }

    fn fixture_delete_args(identifier: &str) -> DeleteNoteArgs {
        DeleteNoteArgs {
            workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
            project: FIXTURE_PROJECT.to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    fn chinese_note_body() -> &'static str {
        "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n"
    }

    #[test]
    fn empty_library_crud_is_unsupported() {
        let mut route = RouteState::default();
        let body = chinese_note_body();
        let write = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args("welcome", "中文夹具笔记", body)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(write.category, ErrorCategory::Unsupported);
        assert_eq!(write.message, library::UNSUPPORTED_LIBRARY_UNAVAILABLE);
        let edit = dispatch_with_library(
            IpcCommand::EditNote(fixture_edit_args("welcome", body)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(edit.category, ErrorCategory::Unsupported);
        let moved = dispatch_with_library(
            IpcCommand::MoveNote(fixture_move_args("welcome", "renamed")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(moved.category, ErrorCategory::Unsupported);
        let deleted = dispatch_with_library(
            IpcCommand::DeleteNote(fixture_delete_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(deleted.category, ErrorCategory::Unsupported);
        assert_eq!(route.project, None);
    }

    #[test]
    fn non_fixture_crud_is_policy_and_does_not_open() {
        let snapshot = idle_snapshot();
        let mut route = RouteState::default();
        route.select_fixture();
        let body = chinese_note_body();
        let write = dispatch_with_library(
            IpcCommand::WriteNote(WriteNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: r"C:\Users\someone\Documents\Obsidian".to_owned(),
                identifier: "welcome".to_owned(),
                title: "中文夹具笔记".to_owned(),
                body: body.to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(write.category, ErrorCategory::Policy);
        let workspace = dispatch_with_library(
            IpcCommand::EditNote(EditNoteArgs {
                workspace: "user-home".to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "welcome".to_owned(),
                body: body.to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(workspace.category, ErrorCategory::Policy);
        let identifier = dispatch_with_library(
            IpcCommand::WriteNote(WriteNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"%APPDATA%\Obsidian\welcome.md".to_owned(),
                title: "中文夹具笔记".to_owned(),
                body: body.to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(identifier.category, ErrorCategory::Policy);
        let destination = dispatch_with_library(
            IpcCommand::MoveNote(MoveNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: "welcome".to_owned(),
                destination: r"C:\Users\someone\vault\note.md".to_owned(),
            }),
            snapshot.clone(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(destination.category, ErrorCategory::Policy);
        let deleted = dispatch_with_library(
            IpcCommand::DeleteNote(DeleteNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"%APPDATA%\Obsidian\welcome.md".to_owned(),
            }),
            snapshot,
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(deleted.category, ErrorCategory::Policy);
        assert_eq!(route.project.as_deref(), Some(FIXTURE_PROJECT));
        assert!(!routing::owned_catalog().implicit_current_project_writes);
    }

    #[test]
    fn fixture_crud_observes_physical_files_not_saved_text() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t15-ipc-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let body = chinese_note_body();
        let mut route = RouteState::default();
        let snapshot = idle_snapshot();
        let before = snapshot.clone();
        let IpcResponse::NoteWritten(written) = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args("welcome", "中文夹具笔记", body)),
            snapshot.clone(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(snapshot, before);
        assert_eq!(route.project, None);
        let dest = dir.join("welcome.md");
        assert!(dest.is_file(), "write must observe a physical owned file");
        let disk = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert_eq!(written.body, body);
        assert!(written.files_written);
        assert!(!written.engine_persisted);
        assert!(written.observation.disk_verified);
        assert!(written.observation.envelope_is_not_disk_proof);
        assert_eq!(
            written.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        let json = serde_json::to_value(&IpcResponse::NoteWritten(written)).unwrap();
        assert_eq!(json["kind"], "note_written");
        assert_eq!(json["engine_persisted"], false);
        assert_ne!(json["observation"]["classified_as"], "saved");
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());

        let replacement = "# 覆盖后的笔记\n\n替换后的中文正文 [[欢迎]]。\n";
        let IpcResponse::NoteEdited(edited) = dispatch_with_library(
            IpcCommand::EditNote(fixture_edit_args("welcome", replacement)),
            snapshot.clone(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let edited_disk = std::fs::read_to_string(&dest).unwrap();
        assert_eq!(edited_disk, replacement);
        assert_eq!(edited.body, replacement);
        assert!(edited.observation.disk_verified);
        assert!(!edited.engine_persisted);

        let IpcResponse::NoteMoved(moved) = dispatch_with_library(
            IpcCommand::MoveNote(fixture_move_args("welcome", "renamed")),
            snapshot.clone(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!dest.exists());
        let renamed = dir.join("renamed.md");
        assert!(renamed.is_file());
        assert_eq!(std::fs::read_to_string(&renamed).unwrap(), replacement);
        assert_eq!(moved.body, replacement);
        assert!(moved.observation.disk_verified);
        assert!(!moved.engine_persisted);

        let IpcResponse::NoteDeleted(deleted) = dispatch_with_library(
            IpcCommand::DeleteNote(fixture_delete_args("renamed")),
            snapshot,
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!renamed.exists());
        assert!(deleted.files_written);
        assert!(!deleted.engine_persisted);
        assert!(deleted.observation.disk_verified);
        assert_eq!(
            deleted.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        let json = serde_json::to_value(&IpcResponse::NoteDeleted(deleted)).unwrap();
        assert_eq!(json["kind"], "note_deleted");
        assert!(json.get("path").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn sequential_move_onto_existing_destination_is_unsupported_not_t16() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t15-ipc-conflict-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let body = chinese_note_body();
        library.write_note("welcome", "中文夹具笔记", body).unwrap();
        library
            .write_note("renamed", "已有目标", "other body\n")
            .unwrap();
        let mut route = RouteState::default();
        let error = dispatch_with_library(
            IpcCommand::MoveNote(fixture_move_args("welcome", "renamed")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(error.category, ErrorCategory::Unsupported);
        assert_eq!(error.message, library::UNSUPPORTED_MOVE_DESTINATION_EXISTS);
        assert!(
            error.message.contains("not T16"),
            "sequential dest-exists must not claim T16 same-target conflict"
        );
        assert_ne!(error.category, ErrorCategory::Policy);
        assert!(dir.join("welcome.md").is_file());
        assert!(dir.join("renamed.md").is_file());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn envelope_crud_is_accepted_unverified_and_does_not_write() {
        let library = library::EnvelopeCrudLibrary;
        let mut route = RouteState::default();
        let IpcResponse::NoteWritten(written) = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                chinese_note_body(),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!written.files_written);
        assert!(!written.engine_persisted);
        assert!(!written.observation.disk_verified);
        assert_eq!(
            written.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        let json = serde_json::to_value(&IpcResponse::NoteWritten(written)).unwrap();
        assert_eq!(json["kind"], "note_written");
        assert_ne!(json["observation"]["classified_as"], "saved");
        let IpcResponse::NoteDeleted(deleted) = dispatch_with_library(
            IpcCommand::DeleteNote(fixture_delete_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!deleted.files_written);
        assert!(!deleted.observation.disk_verified);
        assert_eq!(
            deleted.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
    }

    #[test]
    fn crud_commands_do_not_merge_engine_profiles() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        assert_ne!(release.expected_tools(), preview.expected_tools());
        let mut route = RouteState::default();
        let error = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                chinese_note_body(),
            )),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(error.category, ErrorCategory::Unsupported);
        assert!(serde_json::to_value(&IpcResponse::Error(error))
            .unwrap()
            .get("expected_tools")
            .is_none());
    }

    struct CountingCrudLibrary {
        writes: AtomicUsize,
    }

    impl NoteLibrary for CountingCrudLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            Ok(library::TreePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: false,
            })
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            Err(library::LibraryError::unsupported(
                library::UNSUPPORTED_LIBRARY_UNAVAILABLE,
            ))
        }

        fn write_note(
            &self,
            identifier: &str,
            title: &str,
            body: &str,
        ) -> Result<library::NoteWriteDto, library::LibraryError> {
            self.writes.fetch_add(1, Ordering::SeqCst);
            Ok(library::NoteWriteDto {
                identifier: identifier.to_owned(),
                title: title.to_owned(),
                body: body.to_owned(),
                files_written: false,
                engine_persisted: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::AcceptedUnverified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
            })
        }
    }

    struct BlockingWriteLibrary {
        inner: library::FixtureLibrary,
        entered: Arc<Barrier>,
        release: Arc<Barrier>,
        writes: Arc<AtomicUsize>,
    }

    impl NoteLibrary for BlockingWriteLibrary {
        fn list_tree(
            &self,
            cursor: Option<&str>,
            page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            self.inner.list_tree(cursor, page_size)
        }

        fn read_note(
            &self,
            identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            self.inner.read_note(identifier)
        }

        fn write_note(
            &self,
            identifier: &str,
            title: &str,
            body: &str,
        ) -> Result<library::NoteWriteDto, library::LibraryError> {
            self.entered.wait();
            self.release.wait();
            self.writes.fetch_add(1, Ordering::SeqCst);
            self.inner.write_note(identifier, title, body)
        }
    }

    fn timeout_unknown_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            state: ConnectionState::Stopped,
            profile: Some(crate::supervisor::EngineProfile::MainPreview),
            child_pid: Some(9),
            failure: Some(FailureKind::TimeoutUnknown),
            shutdown: Some(ShutdownReceipt {
                transport_cancelled: false,
                child_exited: false,
                forced: true,
                timeout_unknown: true,
                exit_code: None,
            }),
        }
    }

    fn assert_ipc_error_excludes_timeout_unknown(category: ErrorCategory) {
        match category {
            ErrorCategory::Policy | ErrorCategory::Schema | ErrorCategory::Unsupported => {}
        }
    }

    #[test]
    fn ipc_error_union_excludes_timeout_unknown() {
        for category in [
            ErrorCategory::Policy,
            ErrorCategory::Schema,
            ErrorCategory::Unsupported,
        ] {
            assert_ipc_error_excludes_timeout_unknown(category);
            let json = serde_json::to_value(&IpcError {
                category,
                message: "x".to_owned(),
            })
            .unwrap();
            assert_ne!(json["category"], "timeout_unknown");
            assert_ne!(json["category"], "transport");
            assert_ne!(json["category"], "process");
            assert_ne!(json["category"], "conflict");
        }
    }

    #[test]
    fn overlapping_same_target_write_is_conflict_not_disk_or_timeout() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t15-t16-ipc-overlap-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let entered = Arc::new(Barrier::new(2));
        let release = Arc::new(Barrier::new(2));
        let writes = Arc::new(AtomicUsize::new(0));
        let library = BlockingWriteLibrary {
            inner: library::FixtureLibrary::new(dir.clone()),
            entered: Arc::clone(&entered),
            release: Arc::clone(&release),
            writes: Arc::clone(&writes),
        };
        let conflicts = ConflictCoordinator::default();
        let body = chinese_note_body();
        let (first, second) = std::thread::scope(|scope| {
            let first = scope.spawn(|| {
                let mut route = RouteState::default();
                dispatch_with_conflicts(
                    IpcCommand::WriteNote(fixture_write_args("welcome", "中文夹具笔记", body)),
                    idle_snapshot(),
                    &mut route,
                    &library,
                    &backups::EmptyBackupStore,
                    &drafts::EmptyDraftStore,
                    &conflicts,
                )
            });
            entered.wait();
            let mut route = RouteState::default();
            let second = dispatch_with_conflicts(
                IpcCommand::WriteNote(fixture_write_args(
                    "welcome",
                    "中文夹具笔记",
                    "overlapping body\n",
                )),
                idle_snapshot(),
                &mut route,
                &library,
                &backups::EmptyBackupStore,
                &drafts::EmptyDraftStore,
                &conflicts,
            )
            .unwrap();
            release.wait();
            (first.join().unwrap().unwrap(), second)
        });
        let IpcResponse::NoteWritten(first) = first else {
            panic!("first overlapping writer must complete as note_written")
        };
        let IpcResponse::NoteWritten(second) = second else {
            panic!("second overlapping writer must complete as note_written conflict")
        };
        assert_eq!(writes.load(Ordering::SeqCst), 1);
        assert_eq!(
            first.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert_eq!(
            second.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        assert!(!second.observation.disk_verified);
        assert!(!second.files_written);
        assert!(!second.engine_persisted);
        let json = serde_json::to_value(&IpcResponse::NoteWritten(second)).unwrap();
        assert_eq!(json["kind"], "note_written");
        assert_eq!(json["observation"]["classified_as"], "conflict");
        assert_ne!(json["observation"]["classified_as"], "disk_verified");
        assert_ne!(json["observation"]["classified_as"], "timeout_unknown");
        assert_ne!(json["kind"], "error");
        assert!(json.get("category").is_none());
        assert_eq!(
            std::fs::read_to_string(dir.join("welcome.md")).unwrap(),
            body
        );
        assert!(conflict::OS_FILESYSTEM_RACE_UNVERIFIED.contains("UNVERIFIED"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn preheld_identifier_conflicts_across_crud_ops() {
        let library = library::EnvelopeCrudLibrary;
        let conflicts = ConflictCoordinator::default();
        let guard = conflicts
            .try_acquire(conflict::identifier_keys("welcome"))
            .unwrap();
        let mut route = RouteState::default();
        let IpcResponse::NoteEdited(edited) = dispatch_with_conflicts(
            IpcCommand::EditNote(fixture_edit_args("welcome", "body")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            edited.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        let IpcResponse::NoteDeleted(deleted) = dispatch_with_conflicts(
            IpcCommand::DeleteNote(fixture_delete_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            deleted.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        let IpcResponse::NoteMoved(moved) = dispatch_with_conflicts(
            IpcCommand::MoveNote(fixture_move_args("other", "welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            moved.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        drop(guard);
        let IpcResponse::NoteEdited(released) = dispatch_with_conflicts(
            IpcCommand::EditNote(fixture_edit_args("welcome", "body")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            released.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
    }

    #[test]
    fn distinct_target_sequential_writes_are_not_same_target_atomicity() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t15-t16-distinct-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::NoteWritten(first) = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                chinese_note_body(),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let IpcResponse::NoteWritten(second) = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args(
                "other",
                "另一篇",
                "# 另一篇\n\n[[欢迎]]\n",
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            first.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert_eq!(
            second.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert_ne!(
            first.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        assert!(dir.join("welcome.md").is_file());
        assert!(dir.join("other.md").is_file());
        assert!(conflict::OS_FILESYSTEM_RACE_UNVERIFIED.contains("not an OS file lock"));
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn timeout_unknown_write_does_not_auto_retry() {
        let library = CountingCrudLibrary {
            writes: AtomicUsize::new(0),
        };
        let snapshot = timeout_unknown_snapshot();
        let retried = std::sync::atomic::AtomicBool::new(false);
        let invoked = conflict::auto_retry_non_idempotent_write(&snapshot, || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        let mut route = RouteState::default();
        let IpcResponse::NoteWritten(written) = dispatch_with_conflicts(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                chinese_note_body(),
            )),
            snapshot.clone(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &ConflictCoordinator::default(),
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(library.writes.load(Ordering::SeqCst), 1);
        assert_eq!(
            written.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        assert_ne!(
            written.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        let write_json = serde_json::to_value(&IpcResponse::NoteWritten(written)).unwrap();
        assert_ne!(write_json["kind"], "error");
        assert_ne!(
            write_json["observation"]["classified_as"],
            "timeout_unknown"
        );
        let IpcResponse::RuntimeState(state) =
            dispatch_with_snapshot(IpcCommand::GetRuntimeState(EmptyArgs {}), snapshot).unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(state.failure, Some(FailureKind::TimeoutUnknown));
        assert_eq!(
            state
                .shutdown
                .as_ref()
                .map(|receipt| receipt.timeout_unknown),
            Some(true)
        );
        let json = serde_json::to_value(&IpcResponse::RuntimeState(state)).unwrap();
        assert_eq!(json["failure"], "timeout_unknown");
        assert_eq!(json["kind"], "runtime_state");
        assert!(conflict::RECOVERY_NOT_T16.contains("UNVERIFIED"));
    }

    #[test]
    fn begin_shutdown_idle_not_started_does_not_start_or_kill() {
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let snapshot = idle_snapshot();
        let mut route = RouteState::default();
        let IpcResponse::ShutdownBegun(begun) = dispatch_with_drain(
            IpcCommand::BeginShutdown(EmptyArgs {}),
            snapshot.clone(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(begun.host_drain, DrainPhase::Drained);
        assert_eq!(
            begun.classified_as,
            crate::drain::DrainClass::IdleNotStarted
        );
        assert_eq!(begun.supervisor_status, "not_started");
        assert!(!begun.engine_spawned);
        assert!(!begun.child_killed);
        assert!(!begun.files_written);
        assert!(!begun.shutdown.forced);
        assert!(!begun.shutdown.timeout_unknown);
        assert!(!begun.shutdown.child_exited);
        assert!(!begun.shutdown.transport_cancelled);
        assert!(begun.inflight_unknown.is_empty());
        assert!(!begun.scanned_user_obsidian_vault);
        assert!(!begun.scanned_user_basic_memory_home);
        let json = serde_json::to_value(&IpcResponse::ShutdownBegun(begun)).unwrap();
        assert_eq!(json["kind"], "shutdown_begun");
        assert_ne!(json["kind"], "note_written");
        assert_ne!(json["kind"], "fixture_restored");
        assert_ne!(json["classified_as"], "conflict");
        assert_ne!(json["classified_as"], "disk_verified");
        assert_eq!(json["shutdown"]["forced"], false);
        assert_eq!(snapshot, idle_snapshot());
        let IpcResponse::RuntimeState(state) = dispatch_with_drain(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            snapshot,
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(state.status, "not_started");
        assert_eq!(state.host_drain, DrainPhase::Drained);
        assert_eq!(
            state.shutdown.as_ref().map(|receipt| receipt.forced),
            Some(false)
        );
        assert!(drain::T07_FAKES_ARE_NOT_NATIVE_PROCESS_TREE.contains("UNVERIFIED"));
        assert!(drain::T17_IS_NOT_T37_T38.contains("not T37"));
    }

    #[test]
    fn drain_refuses_new_crud_and_does_not_retry() {
        let library = CountingCrudLibrary {
            writes: AtomicUsize::new(0),
        };
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let mut route = RouteState::default();
        let IpcResponse::NoteWritten(written) = dispatch_with_drain(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                chinese_note_body(),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            written.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        assert_eq!(library.writes.load(Ordering::SeqCst), 1);
        dispatch_with_drain(
            IpcCommand::BeginShutdown(EmptyArgs {}),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap();
        let write = dispatch_with_drain(
            IpcCommand::WriteNote(fixture_write_args(
                "welcome",
                "中文夹具笔记",
                "after drain\n",
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(write.category, ErrorCategory::Unsupported);
        assert_eq!(write.message, drain::UNSUPPORTED_HOST_DRAINING);
        let edit = dispatch_with_drain(
            IpcCommand::EditNote(fixture_edit_args("welcome", "after drain\n")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(edit.category, ErrorCategory::Unsupported);
        let moved = dispatch_with_drain(
            IpcCommand::MoveNote(fixture_move_args("welcome", "renamed")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(moved.category, ErrorCategory::Unsupported);
        let deleted = dispatch_with_drain(
            IpcCommand::DeleteNote(fixture_delete_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(deleted.category, ErrorCategory::Unsupported);
        assert_eq!(library.writes.load(Ordering::SeqCst), 1);
        let retried = std::sync::atomic::AtomicBool::new(false);
        let invoked = conflict::auto_retry_non_idempotent_write(&idle_snapshot(), || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        let restore = dispatch_with_drain(
            IpcCommand::RestoreFixture(RestoreFixtureArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                backup_id: "fixture-welcome".to_owned(),
            }),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(restore.category, ErrorCategory::Unsupported);
        assert_ne!(restore.message, drain::UNSUPPORTED_HOST_DRAINING);
        assert!(drain::T12_RESTORE_IS_NOT_T17_DRAIN.contains("not T17 drain"));
        assert!(drain::T16_CONFLICT_IS_NOT_T17_DRAIN.contains("not T17 host drain"));
        assert!(drain::FORCED_KILL_UNVERIFIED.contains("UNVERIFIED"));
        assert!(drain::JOB_OBJECT_UNVERIFIED.contains("UNVERIFIED"));
        assert!(drain::SLEEP_RESUME_UNVERIFIED.contains("UNVERIFIED"));
        assert!(drain::DISK_FAILURE_UNVERIFIED.contains("UNVERIFIED"));
    }

    #[test]
    fn inflight_keys_are_recorded_unknown_on_begin_shutdown() {
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let guard = conflicts
            .try_acquire(conflict::identifier_keys("welcome"))
            .unwrap();
        let mut route = RouteState::default();
        let IpcResponse::ShutdownBegun(begun) = dispatch_with_drain(
            IpcCommand::BeginShutdown(EmptyArgs {}),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(begun.host_drain, DrainPhase::Draining);
        assert_eq!(
            begun.classified_as,
            crate::drain::DrainClass::InflightUnknown
        );
        assert_eq!(begun.inflight_unknown, vec!["welcome".to_owned()]);
        assert!(begun.shutdown.timeout_unknown);
        assert!(!begun.shutdown.forced);
        assert!(!begun.child_killed);
        let write = dispatch_with_drain(
            IpcCommand::WriteNote(fixture_write_args("other", "title", "body")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap_err();
        assert_eq!(write.category, ErrorCategory::Unsupported);
        assert_eq!(write.message, drain::UNSUPPORTED_HOST_DRAINING);
        let IpcResponse::RuntimeState(state) = dispatch_with_drain(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(state.host_drain, DrainPhase::Draining);
        assert_eq!(
            state
                .shutdown
                .as_ref()
                .map(|receipt| receipt.timeout_unknown),
            Some(true)
        );
        assert_eq!(
            state.shutdown.as_ref().map(|receipt| receipt.forced),
            Some(false)
        );
        let runtime_json = serde_json::to_value(&IpcResponse::RuntimeState(state)).unwrap();
        assert_eq!(runtime_json["kind"], "runtime_state");
        assert_eq!(runtime_json["host_drain"], "draining");
        assert_eq!(runtime_json["shutdown"]["timeout_unknown"], true);
        assert_eq!(runtime_json["shutdown"]["forced"], false);
        assert_ne!(runtime_json["kind"], "error");
        drop(guard);
        let retried = std::sync::atomic::AtomicBool::new(false);
        let invoked = conflict::auto_retry_non_idempotent_write(&idle_snapshot(), || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        assert!(drain::T07_FAKES_ARE_NOT_NATIVE_PROCESS_TREE.contains("not a native process-tree"));
    }

    #[test]
    fn drain_does_not_mix_engine_profiles() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        assert_ne!(release.commit(), preview.commit());
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let mut route = RouteState::default();
        let IpcResponse::ShutdownBegun(begun) = dispatch_with_drain(
            IpcCommand::BeginShutdown(EmptyArgs {}),
            RuntimeSnapshot {
                state: ConnectionState::NotStarted,
                profile: Some(preview),
                child_pid: None,
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &drafts::EmptyDraftStore,
            &conflicts,
            &drain,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!begun.engine_spawned);
        assert!(!begun.child_killed);
        assert_eq!(begun.supervisor_status, "not_started");
    }

    #[test]
    fn unsafe_html_crlf_wiki_roundtrips_on_draft_and_note_paths() {
        let body = crate::content_safety::UNSAFE_HTML_WIKI_CRLF_BODY;
        let class = crate::content_safety::classify_body(body);
        assert!(class.unsafe_html_present);
        assert!(!class.executed);
        assert_eq!(
            class.line_endings,
            crate::content_safety::LineEndingClass::Crlf
        );

        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let draft_dir = std::env::temp_dir().join(format!("bmdock-t14-t18-{nanos}"));
        std::fs::create_dir_all(&draft_dir).unwrap();
        let store = drafts::FixtureDraftStore::new(draft_dir.clone()).unwrap();
        let mut route = RouteState::default();
        let IpcResponse::DraftSaved(saved) = dispatch_with_stores(
            IpcCommand::SaveDraft(fixture_save_draft_args("welcome", body)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let draft_dest = draft_dir.join("welcome.md");
        let draft_disk = crate::content_safety::read_exact_bytes(&draft_dest).unwrap();
        assert_eq!(draft_disk, body.as_bytes());
        assert!(saved.observation.disk_verified);
        assert_eq!(saved.body, body);
        let IpcResponse::DraftLoaded(loaded) = dispatch_with_stores(
            IpcCommand::LoadDraft(fixture_load_draft_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
            &store,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(loaded.body, body);
        let _ = std::fs::remove_dir_all(&draft_dir);

        let note_dir = std::env::temp_dir().join(format!("bmdock-t15-t18-{nanos}"));
        std::fs::create_dir_all(&note_dir).unwrap();
        let note_library = library::FixtureLibrary::new(note_dir.clone());
        let IpcResponse::NoteWritten(written) = dispatch_with_library(
            IpcCommand::WriteNote(fixture_write_args("welcome", "中文夹具笔记", body)),
            idle_snapshot(),
            &mut route,
            &note_library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let note_dest = note_dir.join("welcome.md");
        let note_disk = crate::content_safety::read_exact_bytes(&note_dest).unwrap();
        assert_eq!(note_disk, body.as_bytes());
        assert!(note_disk.windows(2).any(|pair| pair == b"\r\n"));
        assert!(written.observation.disk_verified);
        assert_eq!(written.body, body);
        let IpcResponse::NoteRead(read) = dispatch_with_library(
            IpcCommand::ReadNote(fixture_read_args("welcome")),
            idle_snapshot(),
            &mut route,
            &note_library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(read.body, body);
        assert!(read.body.contains("<script>"));
        assert!(read.body.contains("onerror="));
        assert!(read.body.contains("[[欢迎]]"));
        crate::content_safety::persist_exact_utf8(&note_dest, &body.replace("\r\n", "\n")).unwrap();
        let lost = crate::content_safety::read_exact_bytes(&note_dest).unwrap();
        assert!(crate::content_safety::crlf_normalized_to_lf(body, &lost));
        assert!(!crate::content_safety::disk_verified_from_bytes(
            body, &lost
        ));
        let slash = dispatch_with_library(
            IpcCommand::WriteNote(WriteNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
                title: "中文夹具笔记".to_owned(),
                body: body.to_owned(),
            }),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(slash.category, ErrorCategory::Policy);
        let _ = std::fs::remove_dir_all(&note_dir);
    }

    #[test]
    fn list_relations_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::RelationList(listed) = dispatch_with_library(
            IpcCommand::ListRelations(fixture_relations_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(listed.relations.is_empty());
        assert_eq!(listed.identifier, "welcome");
        assert!(!listed.engine_graph);
        assert!(!listed.files_written);
        assert!(!listed.scanned_user_obsidian_vault);
        assert!(!listed.scanned_user_basic_memory_home);
        assert_eq!(
            listed.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!listed.observation.disk_verified);
        let json = serde_json::to_value(&IpcResponse::RelationList(listed)).unwrap();
        assert_eq!(json["kind"], "relation_list");
        assert_eq!(json["engine_graph"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "empty");
        let _ = (
            library::ENGINE_GRAPH_NOT_OWNED,
            library::RECENT_ACTIVITY_MCP_UNVERIFIED,
            library::BUILD_CONTEXT_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn list_relations_fixture_matches_physical_wiki_links() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t19-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let body =
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]] 与 [[missing-target]]。\n";
        std::fs::write(dir.join("welcome.md"), body).unwrap();
        std::fs::write(dir.join("欢迎.md"), "# 欢迎\n\n目标正文\n").unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::RelationList(listed) = dispatch_with_library(
            IpcCommand::ListRelations(fixture_relations_args("welcome")),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let disk = std::fs::read_to_string(dir.join("welcome.md")).unwrap();
        assert_eq!(disk, body);
        assert_eq!(
            listed
                .relations
                .iter()
                .map(|item| item.identifier.as_str())
                .collect::<Vec<_>>(),
            library::extract_wiki_link_identifiers(&disk)
        );
        assert_eq!(listed.relations[0].identifier, "欢迎");
        assert_eq!(
            listed.relations[0].classified_as,
            library::RelationTargetClass::Present
        );
        assert_eq!(listed.relations[1].identifier, "missing-target");
        assert_eq!(
            listed.relations[1].classified_as,
            library::RelationTargetClass::Empty
        );
        assert!(!listed.relations.iter().any(|item| {
            item.identifier.contains('\\')
                || item.identifier.contains(':')
                || item.identifier.contains('/')
        }));
        assert_eq!(
            listed.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert_ne!(
            listed.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        assert!(listed.observation.disk_verified);
        assert!(!listed.engine_graph);
        let json = serde_json::to_value(&IpcResponse::RelationList(listed)).unwrap();
        assert_eq!(json["kind"], "relation_list");
        assert_eq!(json["relations"][0]["identifier"], "欢迎");
        assert_eq!(
            json["relations"][0]["identifier"]
                .as_str()
                .unwrap()
                .contains('\\'),
            false
        );
        assert_eq!(json["observation"]["classified_as"], "disk_verified");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_relations_does_not_merge_engine_profiles_or_call_graph_mcp() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::RelationList(listed) = dispatch_with_library(
            IpcCommand::ListRelations(fixture_relations_args("welcome")),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::RelationList(listed)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["engine_graph"], false);
        assert_eq!(json["kind"], "relation_list");
        let _ = (
            library::RECENT_ACTIVITY_MCP_UNVERIFIED,
            library::BUILD_CONTEXT_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn expand_graph_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::GraphPage(page) = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args(
                "welcome",
                None,
                Some(library::DEFAULT_PAGE_SIZE),
            )),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(page.nodes.is_empty());
        assert!(page.edges.is_empty());
        assert_eq!(page.identifier, "welcome");
        assert_eq!(page.next_cursor, None);
        assert!(!page.truncated);
        assert_eq!(page.depth, library::GRAPH_DEPTH);
        assert!(!page.engine_graph);
        assert!(!page.files_written);
        assert!(!page.scanned_user_obsidian_vault);
        assert!(!page.scanned_user_basic_memory_home);
        assert_eq!(
            page.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!page.observation.disk_verified);
        let json = serde_json::to_value(&IpcResponse::GraphPage(page)).unwrap();
        assert_eq!(json["kind"], "graph_page");
        assert_eq!(json["engine_graph"], false);
        assert_eq!(json["depth"], 1);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "empty");
        let _ = (
            library::ENGINE_GRAPH_NOT_OWNED,
            library::NATIVE_GUI_UNVERIFIED,
            library::BOUNDED_HOST_EXPANSION,
        );
    }

    #[test]
    fn expand_graph_fixture_is_one_hop_wiki_links_with_chinese_permalink() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t20-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("welcome.md"),
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]] 与 [[alpha]] [[beta]] [[gamma]] [[delta]] 与 [[missing-target]]。\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("欢迎.md"),
            "# 欢迎\n\n第二跳正文。参见 [[second-hop]] 与 [[missing-second]]。\n",
        )
        .unwrap();
        std::fs::write(dir.join("second-hop.md"), "# second-hop\n\n第三层正文\n").unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::GraphPage(first) = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("welcome", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let disk = std::fs::read_to_string(dir.join("welcome.md")).unwrap();
        let expected = library::extract_wiki_link_identifiers(&disk);
        assert_eq!(
            first
                .edges
                .iter()
                .map(|edge| edge.target.clone())
                .collect::<Vec<_>>(),
            expected[..2]
        );
        assert_eq!(first.edges.len(), 2);
        assert_eq!(first.next_cursor.as_deref(), Some("2"));
        assert!(!first.truncated);
        assert_eq!(first.depth, 1);
        assert_eq!(first.nodes[1].identifier, "欢迎");
        assert_eq!(
            first.nodes[1].classified_as,
            library::GraphNodeClass::Present
        );
        assert_eq!(first.nodes[2].classified_as, library::GraphNodeClass::Empty);
        assert!(!first
            .nodes
            .iter()
            .any(|node| node.identifier == "second-hop"));
        assert_eq!(
            first.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert_ne!(
            first.observation.classified_as,
            library::NoteCrudClass::Conflict
        );
        assert!(first.observation.disk_verified);
        assert!(!first.engine_graph);
        let json = serde_json::to_value(&IpcResponse::GraphPage(first.clone())).unwrap();
        assert_eq!(json["kind"], "graph_page");
        assert_eq!(json["nodes"][1]["identifier"], "欢迎");
        assert_eq!(json["observation"]["classified_as"], "disk_verified");
        assert!(json.get("path").is_none());
        let IpcResponse::GraphPage(second) = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args(
                "welcome",
                first.next_cursor.as_deref(),
                Some(2),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(second.page, 2);
        assert!(!second
            .nodes
            .iter()
            .any(|node| node.identifier == "second-hop"));
        let repeated = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args(
                "welcome",
                first.next_cursor.as_deref(),
                Some(2),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap();
        let IpcResponse::GraphPage(repeated) = repeated else {
            panic!("wrong response variant")
        };
        let looped = library::accept_graph_page(
            first.next_cursor.as_deref(),
            library::GraphPageDto {
                next_cursor: first.next_cursor.clone(),
                ..repeated
            },
        );
        assert_eq!(
            looped.unwrap_err(),
            library::LibraryError::schema(library::SCHEMA_INVALID_CURSOR)
        );
        let IpcResponse::GraphPage(hop) = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("欢迎", None, Some(20))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let neighbor_disk = std::fs::read_to_string(dir.join("欢迎.md")).unwrap();
        assert_eq!(
            hop.edges
                .iter()
                .map(|edge| edge.target.clone())
                .collect::<Vec<_>>(),
            library::extract_wiki_link_identifiers(&neighbor_disk)
        );
        assert!(hop.nodes.iter().any(|node| node.identifier == "second-hop"));
        assert!(!hop.nodes.iter().any(|node| node.identifier == "alpha"));
        assert_eq!(
            hop.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn expand_graph_does_not_merge_engine_profiles_or_claim_native_gui() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::GraphPage(page) = dispatch_with_library(
            IpcCommand::ExpandGraph(fixture_graph_args("welcome", None, Some(2))),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::GraphPage(page)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["engine_graph"], false);
        assert_eq!(json["kind"], "graph_page");
        let _ = (
            library::RECENT_ACTIVITY_MCP_UNVERIFIED,
            library::BUILD_CONTEXT_MCP_UNVERIFIED,
            library::NATIVE_GUI_UNVERIFIED,
            library::BOUNDED_HOST_EXPANSION,
        );
    }

    struct EnvelopeSearchLibrary;

    impl NoteLibrary for EnvelopeSearchLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            Ok(library::TreePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: false,
            })
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            Err(library::LibraryError::unsupported(
                library::UNSUPPORTED_LIBRARY_UNAVAILABLE,
            ))
        }

        fn search_notes(
            &self,
            query: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::SearchPageDto, library::LibraryError> {
            Ok(library::SearchPageDto {
                query: query.to_owned(),
                hits: vec![library::SearchHitDto {
                    identifier: "welcome".to_owned(),
                    lexical_score: library::LEXICAL_SCORE_BODY,
                    semantic_score: library::SEMANTIC_SCORE_DISABLED,
                }],
                next_cursor: None,
                page: 1,
                truncated: false,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::AcceptedUnverified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                semantic_enabled: false,
                engine_search: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }
    }

    #[test]
    fn search_notes_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::SearchPage(page) = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args(
                "欢迎",
                None,
                Some(library::DEFAULT_PAGE_SIZE),
            )),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(page.hits.is_empty());
        assert_eq!(page.query, "欢迎");
        assert_eq!(page.next_cursor, None);
        assert!(!page.truncated);
        assert!(!page.semantic_enabled);
        assert!(!page.engine_search);
        assert!(!page.files_written);
        assert!(!page.scanned_user_obsidian_vault);
        assert!(!page.scanned_user_basic_memory_home);
        assert_eq!(
            page.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!page.observation.disk_verified);
        let json = serde_json::to_value(&IpcResponse::SearchPage(page)).unwrap();
        assert_eq!(json["kind"], "search_page");
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["engine_search"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "empty");
        let _ = (
            library::ENGINE_SEARCH_NOT_OWNED,
            library::SEMANTIC_SEARCH_UNVERIFIED,
            library::OFFICIAL_SEARCH_MCP_UNVERIFIED,
            library::OFFICIAL_FETCH_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn search_notes_fixture_lexical_hits_match_physical_utf8() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t21-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("welcome.md"),
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n",
        )
        .unwrap();
        std::fs::write(dir.join("欢迎.md"), "# 欢迎\n\n第二篇夹具正文。\n").unwrap();
        std::fs::write(
            dir.join("alpha.md"),
            "# alpha\n\nEnglish body without CJK.\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("beta.md"),
            "# beta\n\nAnother 欢迎 hit for paging.\n",
        )
        .unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::SearchPage(first) = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(first.hits.len(), 2);
        assert_eq!(first.next_cursor.as_deref(), Some("2"));
        assert!(!first.truncated);
        assert!(!first.semantic_enabled);
        assert!(!first.engine_search);
        assert_eq!(
            first.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert!(first.observation.disk_verified);
        assert!(first.observation.envelope_is_not_disk_proof);
        for hit in &first.hits {
            assert_ne!(hit.lexical_score, hit.semantic_score);
            assert_eq!(hit.semantic_score, library::SEMANTIC_SCORE_DISABLED);
            assert!(hit.lexical_score > library::SEMANTIC_SCORE_DISABLED);
            assert!(!hit.identifier.contains('\\'));
            assert!(!hit.identifier.contains(':'));
            let path = dir.join(format!("{}.md", hit.identifier));
            let disk = std::fs::read_to_string(&path).unwrap();
            assert!(disk.contains("欢迎"));
        }
        let json = serde_json::to_value(&IpcResponse::SearchPage(first.clone())).unwrap();
        assert_eq!(json["kind"], "search_page");
        assert_eq!(json["semantic_enabled"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "disk_verified");
        let IpcResponse::SearchPage(second) = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args(
                "欢迎",
                first.next_cursor.as_deref(),
                Some(2),
            )),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(second.page, 2);
        assert!(second.hits.iter().any(|hit| hit.identifier == "欢迎"));
        let looped = library::accept_search_page(
            first.next_cursor.as_deref(),
            library::SearchPageDto {
                next_cursor: first.next_cursor.clone(),
                ..second
            },
        );
        assert_eq!(
            looped.unwrap_err(),
            library::LibraryError::schema(library::SCHEMA_INVALID_CURSOR)
        );
        let IpcResponse::SearchPage(envelope) = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", None, Some(2))),
            idle_snapshot(),
            &mut route,
            &EnvelopeSearchLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            envelope.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        assert!(!envelope.observation.disk_verified);
        assert!(envelope.observation.envelope_is_not_disk_proof);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn search_notes_does_not_merge_engine_profiles_or_enable_semantic() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::SearchPage(page) = dispatch_with_library(
            IpcCommand::SearchNotes(fixture_search_args("欢迎", None, Some(2))),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::SearchPage(page)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["engine_search"], false);
        assert_eq!(json["kind"], "search_page");
        let _ = (
            library::ENGINE_SEARCH_NOT_OWNED,
            library::SEMANTIC_SEARCH_UNVERIFIED,
            library::OFFICIAL_SEARCH_MCP_UNVERIFIED,
            library::OFFICIAL_FETCH_MCP_UNVERIFIED,
        );
    }

    struct EnvelopeInspectorLibrary;

    impl NoteLibrary for EnvelopeInspectorLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            Ok(library::TreePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: false,
            })
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            Err(library::LibraryError::unsupported(
                library::UNSUPPORTED_LIBRARY_UNAVAILABLE,
            ))
        }

        fn inspect_search(
            &self,
            query: &str,
            identifier: Option<&str>,
        ) -> Result<library::SearchInspectorDto, library::LibraryError> {
            Ok(library::SearchInspectorDto {
                query: query.to_owned(),
                identifier: identifier.map(ToOwned::to_owned),
                hits: vec![library::SearchHitDto {
                    identifier: "welcome".to_owned(),
                    lexical_score: library::LEXICAL_SCORE_BODY,
                    semantic_score: library::SEMANTIC_SCORE_DISABLED,
                }],
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::AcceptedUnverified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                semantic_enabled: false,
                model_id: None,
                model_loaded: false,
                embedding_backend: library::EmbeddingBackend::None,
                model_class: library::ModelClass::Unclassified,
                engine_search: false,
                files_written: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                semantic_disabled_reason: library::SEMANTIC_DISABLED_REASON.to_owned(),
            })
        }
    }

    #[test]
    fn inspect_search_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::SearchInspector(inspector) = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", None)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(inspector.hits.is_empty());
        assert_eq!(inspector.query, "欢迎");
        assert!(inspector.identifier.is_none());
        assert!(!inspector.semantic_enabled);
        assert!(!inspector.model_loaded);
        assert!(inspector.model_id.is_none());
        assert_eq!(inspector.embedding_backend, library::EmbeddingBackend::None);
        assert_eq!(inspector.model_class, library::ModelClass::Unclassified);
        assert!(!inspector.engine_search);
        assert!(!inspector.files_written);
        assert!(!inspector.scanned_user_obsidian_vault);
        assert!(!inspector.scanned_user_basic_memory_home);
        assert_eq!(
            inspector.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!inspector.observation.disk_verified);
        assert_eq!(
            inspector.semantic_disabled_reason,
            library::SEMANTIC_DISABLED_REASON
        );
        let json = serde_json::to_value(&IpcResponse::SearchInspector(inspector)).unwrap();
        assert_eq!(json["kind"], "search_inspector");
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["model_loaded"], false);
        assert!(json["model_id"].is_null());
        assert_eq!(json["embedding_backend"], "none");
        assert_eq!(json["model_class"], "unclassified");
        assert_eq!(json["files_written"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "empty");
        let _ = (
            library::ENGINE_INSPECTOR_NOT_OWNED,
            library::INSPECTOR_READ_ONLY,
            library::INSPECTOR_MODEL_UNVERIFIED,
            library::SEMANTIC_SEARCH_UNVERIFIED,
        );
    }

    #[test]
    fn inspect_search_fixture_lexical_hits_match_physical_utf8() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t23-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("welcome.md"),
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n",
        )
        .unwrap();
        std::fs::write(dir.join("欢迎.md"), "# 欢迎\n\n第二篇夹具正文。\n").unwrap();
        std::fs::write(
            dir.join("alpha.md"),
            "# alpha\n\nEnglish body without CJK.\n",
        )
        .unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::SearchInspector(inspector) = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", None)),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!inspector.hits.is_empty());
        assert!(!inspector.semantic_enabled);
        assert!(!inspector.model_loaded);
        assert!(inspector.model_id.is_none());
        assert_eq!(inspector.embedding_backend, library::EmbeddingBackend::None);
        assert_eq!(inspector.model_class, library::ModelClass::Unclassified);
        assert!(!inspector.files_written);
        assert_eq!(
            inspector.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert!(inspector.observation.disk_verified);
        assert!(inspector.observation.envelope_is_not_disk_proof);
        for hit in &inspector.hits {
            assert_ne!(hit.lexical_score, hit.semantic_score);
            assert_eq!(hit.semantic_score, library::SEMANTIC_SCORE_DISABLED);
            assert!(hit.lexical_score > library::SEMANTIC_SCORE_DISABLED);
            assert!(!hit.identifier.contains('\\'));
            assert!(!hit.identifier.contains(':'));
            let path = dir.join(format!("{}.md", hit.identifier));
            let disk = std::fs::read_to_string(&path).unwrap();
            assert!(disk.contains("欢迎"));
        }
        assert!(inspector.hits.iter().any(|hit| hit.identifier == "welcome"));
        assert!(inspector.hits.iter().any(|hit| hit.identifier == "欢迎"));
        assert!(!inspector.hits.iter().any(|hit| hit.identifier == "alpha"));
        let json = serde_json::to_value(&IpcResponse::SearchInspector(inspector.clone())).unwrap();
        assert_eq!(json["kind"], "search_inspector");
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["model_loaded"], false);
        assert_eq!(json["embedding_backend"], "none");
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        let IpcResponse::SearchInspector(focused) = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", Some("欢迎"))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(focused.identifier.as_deref(), Some("欢迎"));
        assert_eq!(focused.hits.len(), 1);
        assert_eq!(focused.hits[0].identifier, "欢迎");
        assert!(!focused.model_loaded);
        let IpcResponse::SearchInspector(envelope) = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", None)),
            idle_snapshot(),
            &mut route,
            &EnvelopeInspectorLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            envelope.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        assert!(!envelope.observation.disk_verified);
        assert!(!envelope.model_loaded);
        assert!(envelope.observation.envelope_is_not_disk_proof);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn inspect_search_does_not_merge_engine_profiles_or_claim_loaded_model() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::SearchInspector(inspector) = dispatch_with_library(
            IpcCommand::InspectSearch(fixture_inspect_args("欢迎", None)),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::SearchInspector(inspector)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["model_loaded"], false);
        assert_eq!(json["embedding_backend"], "none");
        assert_eq!(json["kind"], "search_inspector");
        let IpcResponse::RuntimeState(state) = dispatch_with_library(
            IpcCommand::GetRuntimeState(EmptyArgs {}),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(preview),
                child_pid: Some(9),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(!state.semantic_model_loaded);
        let runtime_json = serde_json::to_value(&IpcResponse::RuntimeState(state)).unwrap();
        assert_eq!(runtime_json["semantic_model_loaded"], false);
        assert!(runtime_json.get("child_pid").is_none());
        let claimed = library::SearchInspectorDto {
            semantic_enabled: true,
            model_loaded: true,
            model_id: Some("unknown-model".to_owned()),
            ..library::empty_search_inspector("欢迎", None)
        };
        assert_eq!(
            library::accept_search_inspector(claimed).unwrap_err(),
            library::LibraryError::unsupported(library::UNSUPPORTED_TRUNCATED)
        );
        let _ = (
            library::ENGINE_INSPECTOR_NOT_OWNED,
            library::INSPECTOR_READ_ONLY,
            library::INSPECTOR_MODEL_UNVERIFIED,
        );
    }

    struct EnvelopePreviewLibrary;

    impl NoteLibrary for EnvelopePreviewLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<library::TreePageDto, library::LibraryError> {
            Ok(library::TreePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: false,
            })
        }

        fn read_note(
            &self,
            _identifier: &str,
        ) -> Result<library::NoteReadDto, library::LibraryError> {
            Err(library::LibraryError::unsupported(
                library::UNSUPPORTED_LIBRARY_UNAVAILABLE,
            ))
        }

        fn preview_context(
            &self,
            identifier: &str,
            query: Option<&str>,
        ) -> Result<library::ContextPreviewDto, library::LibraryError> {
            Ok(library::ContextPreviewDto {
                identifier: identifier.to_owned(),
                query: query.map(ToOwned::to_owned),
                snippet: "Saved successfully".to_owned(),
                executed: false,
                unsafe_html_present: false,
                observation: library::NoteCrudObservationDto {
                    classified_as: library::NoteCrudClass::AcceptedUnverified,
                    disk_verified: false,
                    envelope_is_not_disk_proof: true,
                },
                engine_context: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }
    }

    #[test]
    fn preview_context_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::ContextPreview(preview) = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("welcome", Some("欢迎"))),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(preview.snippet.is_empty());
        assert!(!preview.executed);
        assert!(!preview.engine_context);
        assert!(!preview.files_written);
        assert!(!preview.scanned_user_obsidian_vault);
        assert!(!preview.scanned_user_basic_memory_home);
        assert_eq!(
            preview.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!preview.observation.disk_verified);
        assert!(preview.observation.envelope_is_not_disk_proof);
        let json = serde_json::to_value(&IpcResponse::ContextPreview(preview)).unwrap();
        assert_eq!(json["kind"], "context_preview");
        assert_eq!(json["executed"], false);
        assert_eq!(json["engine_context"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        let missing = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("", None)),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap_err();
        assert_eq!(missing.category, ErrorCategory::Schema);
        let _ = (
            library::ENGINE_CONTEXT_NOT_OWNED,
            library::BUILD_CONTEXT_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn preview_context_fixture_snippet_matches_physical_utf8() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t22-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        let body = "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n<script>alert(1)</script>\n";
        std::fs::write(dir.join("welcome.md"), body).unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::ContextPreview(preview) = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("welcome", Some("欢迎"))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let disk = std::fs::read_to_string(dir.join("welcome.md")).unwrap();
        assert!(disk.contains(&preview.snippet));
        assert!(preview.snippet.contains("欢迎"));
        assert!(!preview.executed);
        assert!(preview.unsafe_html_present);
        assert!(!preview.engine_context);
        assert_eq!(
            preview.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert!(preview.observation.disk_verified);
        assert!(preview.observation.envelope_is_not_disk_proof);
        let json = serde_json::to_value(&IpcResponse::ContextPreview(preview)).unwrap();
        assert_eq!(json["kind"], "context_preview");
        assert_eq!(json["executed"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        let IpcResponse::ContextPreview(missing) = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("absent", None)),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(missing.snippet.is_empty());
        assert_eq!(
            missing.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        let IpcResponse::ContextPreview(envelope) = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("welcome", Some("欢迎"))),
            idle_snapshot(),
            &mut route,
            &EnvelopePreviewLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(
            envelope.observation.classified_as,
            library::NoteCrudClass::AcceptedUnverified
        );
        assert!(!envelope.observation.disk_verified);
        assert!(envelope.observation.envelope_is_not_disk_proof);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn list_activity_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::ActivityPage(page) = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(
                None,
                Some(library::DEFAULT_PAGE_SIZE),
            )),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(page.entries.is_empty());
        assert_eq!(page.next_cursor, None);
        assert!(!page.truncated);
        assert!(!page.engine_activity);
        assert!(!page.files_written);
        assert_eq!(
            page.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        let json = serde_json::to_value(&IpcResponse::ActivityPage(page)).unwrap();
        assert_eq!(json["kind"], "activity_page");
        assert_eq!(json["engine_activity"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        let _ = (
            library::ENGINE_ACTIVITY_NOT_OWNED,
            library::RECENT_ACTIVITY_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn list_activity_fixture_follows_physical_mtime_permalinks() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t22-activity-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("alpha.md"), "# alpha\n\nolder\n").unwrap();
        std::fs::write(dir.join("welcome.md"), "# welcome\n\nmiddle\n").unwrap();
        std::fs::write(dir.join("欢迎.md"), "# 欢迎\n\nnewer 中文\n").unwrap();
        let t0 = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_100);
        let t1 = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_200);
        let t2 = std::time::UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_300);
        std::fs::OpenOptions::new()
            .write(true)
            .open(dir.join("alpha.md"))
            .unwrap()
            .set_modified(t0)
            .unwrap();
        std::fs::OpenOptions::new()
            .write(true)
            .open(dir.join("welcome.md"))
            .unwrap()
            .set_modified(t1)
            .unwrap();
        std::fs::OpenOptions::new()
            .write(true)
            .open(dir.join("欢迎.md"))
            .unwrap()
            .set_modified(t2)
            .unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::ActivityPage(first) = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(None, Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(first.entries.len(), 2);
        assert_eq!(first.entries[0].identifier, "欢迎");
        assert_eq!(first.entries[1].identifier, "welcome");
        assert!(first.entries[0].observed_mtime >= first.entries[1].observed_mtime);
        assert!(!first.engine_activity);
        assert!(!first.truncated);
        assert_eq!(
            first.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        for entry in &first.entries {
            assert!(!entry.identifier.contains('\\'));
            assert!(!entry.identifier.contains(':'));
            assert!(dir.join(format!("{}.md", entry.identifier)).is_file());
        }
        let json = serde_json::to_value(&IpcResponse::ActivityPage(first.clone())).unwrap();
        assert_eq!(json["kind"], "activity_page");
        assert_eq!(json["engine_activity"], false);
        assert!(json.get("path").is_none());
        let IpcResponse::ActivityPage(second) = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(first.next_cursor.as_deref(), Some(2))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert!(second
            .entries
            .iter()
            .any(|entry| entry.identifier == "alpha"));
        let looped = library::accept_activity_page(
            first.next_cursor.as_deref(),
            library::ActivityPageDto {
                next_cursor: first.next_cursor.clone(),
                ..second
            },
        );
        assert_eq!(
            looped.unwrap_err(),
            library::LibraryError::schema(library::SCHEMA_INVALID_CURSOR)
        );
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn preview_and_activity_do_not_merge_engine_profiles_or_claim_engine_mcp() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview_profile = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(
            preview_profile.commit(),
            "3452c821d76c083823d020984d71e06904a1ff1e"
        );
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview_profile.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::ContextPreview(preview) = dispatch_with_library(
            IpcCommand::PreviewContext(fixture_preview_args("welcome", Some("欢迎"))),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let preview_json = serde_json::to_value(&IpcResponse::ContextPreview(preview)).unwrap();
        assert!(preview_json.get("expected_tools").is_none());
        assert!(preview_json.get("profile").is_none());
        assert_eq!(preview_json["engine_context"], false);
        assert_eq!(preview_json["executed"], false);
        let IpcResponse::ActivityPage(page) = dispatch_with_library(
            IpcCommand::ListActivity(fixture_activity_args(None, Some(2))),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(preview_profile),
                child_pid: Some(9),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let activity_json = serde_json::to_value(&IpcResponse::ActivityPage(page)).unwrap();
        assert!(activity_json.get("expected_tools").is_none());
        assert!(activity_json.get("profile").is_none());
        assert_eq!(activity_json["engine_activity"], false);
        let _ = (
            library::ENGINE_CONTEXT_NOT_OWNED,
            library::ENGINE_ACTIVITY_NOT_OWNED,
            library::RECENT_ACTIVITY_MCP_UNVERIFIED,
            library::BUILD_CONTEXT_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn run_recall_benchmark_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::RecallBenchmark(report) = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(None)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(report.k, library::DEFAULT_PAGE_SIZE);
        assert_eq!(report.query_count, 0);
        assert!(report.queries.is_empty());
        assert_eq!(report.recall_hits, 0);
        assert_eq!(report.recall_relevant, 0);
        assert!(!report.semantic_enabled);
        assert!(!report.engine_search);
        assert!(!report.native_gui);
        assert!(!report.files_written);
        assert!(!report.scanned_user_obsidian_vault);
        assert!(!report.scanned_user_basic_memory_home);
        assert_eq!(
            report.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!report.observation.disk_verified);
        let json = serde_json::to_value(&IpcResponse::RecallBenchmark(report)).unwrap();
        assert_eq!(json["kind"], "recall_benchmark");
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["engine_search"], false);
        assert_eq!(json["native_gui"], false);
        assert_eq!(json["files_written"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["observation"]["classified_as"], "empty");
        let _ = (
            library::ENGINE_RECALL_NOT_OWNED,
            library::RECALL_NATIVE_UNVERIFIED,
            library::OFFICIAL_CHINESE_RECALL_UNVERIFIED,
            library::NATIVE_GUI_UNVERIFIED,
        );
    }

    #[test]
    fn run_recall_benchmark_fixture_gold_matches_physical_utf8() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t24-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("welcome.md"),
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n",
        )
        .unwrap();
        std::fs::write(dir.join("欢迎.md"), "# 欢迎\n\n第二篇夹具正文。\n").unwrap();
        std::fs::write(
            dir.join("alpha.md"),
            "# alpha\n\nEnglish body without CJK.\n",
        )
        .unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::RecallBenchmark(report) = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(Some(20))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(report.k, 20);
        assert_eq!(report.query_count, 1);
        assert_eq!(report.queries[0].query, "欢迎");
        assert!(!report.semantic_enabled);
        assert!(!report.engine_search);
        assert!(!report.native_gui);
        assert_eq!(
            report.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert!(report.observation.disk_verified);
        assert!(report.observation.envelope_is_not_disk_proof);
        assert!(report.search_elapsed_ms < 60_000);
        assert!(report.expand_elapsed_ms < 60_000);
        assert!(report.chinese_permalinks.iter().any(|item| item == "欢迎"));
        for identifier in &report.queries[0].relevant {
            let path = dir.join(format!("{identifier}.md"));
            let disk = std::fs::read_to_string(&path).unwrap();
            assert!(disk.contains("欢迎"));
        }
        for identifier in &report.queries[0].hits {
            let path = dir.join(format!("{identifier}.md"));
            let disk = std::fs::read_to_string(&path).unwrap();
            assert!(disk.contains("欢迎"));
        }
        assert!(report.queries[0].relevant.iter().any(|item| item == "欢迎"));
        assert!(report.queries[0].hits.iter().any(|item| item == "欢迎"));
        assert_eq!(report.recall_hits, report.recall_relevant);
        assert_eq!(report.recall_relevant, 2);
        let IpcResponse::RecallBenchmark(limited) = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(Some(1))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(limited.k, 1);
        assert_eq!(limited.recall_hits, 1);
        assert_eq!(limited.recall_relevant, 2);
        let json = serde_json::to_value(&IpcResponse::RecallBenchmark(report)).unwrap();
        assert_eq!(json["kind"], "recall_benchmark");
        assert_eq!(json["native_gui"], false);
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("path").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn run_recall_benchmark_does_not_merge_engine_profiles_or_claim_native_gui() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::RecallBenchmark(report) = dispatch_with_library(
            IpcCommand::RunRecallBenchmark(fixture_recall_args(None)),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::RecallBenchmark(report)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["semantic_enabled"], false);
        assert_eq!(json["engine_search"], false);
        assert_eq!(json["native_gui"], false);
        assert_eq!(json["kind"], "recall_benchmark");
        let claimed = library::RecallBenchmarkDto {
            native_gui: true,
            engine_search: true,
            ..library::empty_recall_benchmark(library::DEFAULT_PAGE_SIZE)
        };
        assert_eq!(
            library::accept_recall_benchmark(claimed).unwrap_err(),
            library::LibraryError::unsupported(library::UNSUPPORTED_TRUNCATED)
        );
        let _ = (
            library::ENGINE_RECALL_NOT_OWNED,
            library::RECALL_NATIVE_UNVERIFIED,
            library::OFFICIAL_CHINESE_RECALL_UNVERIFIED,
        );
    }

    #[test]
    fn schema_validate_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::SchemaValidated(report) = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("welcome", None)),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(report.identifier, "welcome");
        assert_eq!(report.schema_id, library::DEFAULT_SCHEMA_ID);
        assert_eq!(report.verdict, library::SchemaVerdict::Empty);
        assert!(!report.observed_title);
        assert!(!report.observed_body);
        assert!(!report.engine_schema);
        assert!(!report.files_written);
        assert!(!report.scanned_user_obsidian_vault);
        assert!(!report.scanned_user_basic_memory_home);
        assert_eq!(
            report.observation.classified_as,
            library::NoteCrudClass::Empty
        );
        assert!(!report.observation.disk_verified);
        assert!(report.observation.envelope_is_not_disk_proof);
        let json = serde_json::to_value(&IpcResponse::SchemaValidated(report)).unwrap();
        assert_eq!(json["kind"], "schema_validated");
        assert_eq!(json["verdict"], "empty");
        assert_eq!(json["engine_schema"], false);
        assert!(json.get("path").is_none());
        assert!(json.get("expected_tools").is_none());
        let IpcResponse::SchemaValidated(unknown) = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("welcome", Some("unknown-schema"))),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(unknown.verdict, library::SchemaVerdict::Unsupported);
        let _ = (
            library::ENGINE_SCHEMA_NOT_OWNED,
            library::OFFICIAL_SCHEMA_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn schema_validate_fixture_note_matches_physical_utf8() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|duration| duration.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("bmdock-t25-{nanos}"));
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("welcome.md"),
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n",
        )
        .unwrap();
        std::fs::write(dir.join("missing-body.md"), "# 只有标题\n").unwrap();
        let library = library::FixtureLibrary::new(dir.clone());
        let mut route = RouteState::default();
        let IpcResponse::SchemaValidated(valid) = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("welcome", None)),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(valid.verdict, library::SchemaVerdict::Valid);
        assert!(valid.observed_title);
        assert!(valid.observed_body);
        assert!(valid.observation.disk_verified);
        assert_eq!(
            valid.observation.classified_as,
            library::NoteCrudClass::DiskVerified
        );
        assert!(valid.observation.envelope_is_not_disk_proof);
        assert!(!valid.engine_schema);
        let disk = std::fs::read_to_string(dir.join("welcome.md")).unwrap();
        let (title, body) = library::parse_fixture_fields(&disk);
        assert_eq!(title.as_deref(), Some("中文夹具笔记"));
        assert!(body.as_deref().is_some_and(|value| value.contains("欢迎")));
        let IpcResponse::SchemaValidated(invalid) = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("missing-body", Some("note"))),
            idle_snapshot(),
            &mut route,
            &library,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        assert_eq!(invalid.verdict, library::SchemaVerdict::Invalid);
        assert!(invalid.observed_title);
        assert!(!invalid.observed_body);
        assert_eq!(invalid.missing_fields, vec![library::SCHEMA_FIELD_BODY]);
        assert!(invalid.observation.disk_verified);
        assert_ne!(valid.verdict, invalid.verdict);
        let json = serde_json::to_value(&IpcResponse::SchemaValidated(valid)).unwrap();
        assert_eq!(json["kind"], "schema_validated");
        assert_eq!(json["engine_schema"], false);
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("path").is_none());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn schema_validate_does_not_merge_engine_profiles_or_claim_official_mcp() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.commit(), "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048");
        assert_eq!(preview.commit(), "3452c821d76c083823d020984d71e06904a1ff1e");
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        let mut route = RouteState::default();
        let IpcResponse::SchemaValidated(report) = dispatch_with_library(
            IpcCommand::SchemaValidate(fixture_schema_args("welcome", None)),
            RuntimeSnapshot {
                state: ConnectionState::Connected,
                profile: Some(release),
                child_pid: Some(7),
                failure: None,
                shutdown: None,
            },
            &mut route,
            &library::EmptyLibrary,
            &backups::EmptyBackupStore,
        )
        .unwrap() else {
            panic!("wrong response variant")
        };
        let json = serde_json::to_value(&IpcResponse::SchemaValidated(report)).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("tools").is_none());
        assert_eq!(json["engine_schema"], false);
        assert_eq!(json["kind"], "schema_validated");
        let claimed = library::SchemaValidateDto {
            engine_schema: true,
            ..library::empty_schema_report("welcome", library::DEFAULT_SCHEMA_ID)
        };
        assert_eq!(
            library::accept_schema_report(claimed).unwrap_err(),
            library::LibraryError::unsupported(library::UNSUPPORTED_TRUNCATED)
        );
        let _ = (
            library::ENGINE_SCHEMA_NOT_OWNED,
            library::OFFICIAL_SCHEMA_MCP_UNVERIFIED,
        );
    }
}
