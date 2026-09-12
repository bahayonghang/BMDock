use serde::{Deserialize, Serialize};

use crate::library::{self, NoteLibrary, NoteReadDto, TreePageDto};
use crate::preflight::{self, ConfigDiscoveryDto, PreflightDto};
use crate::routing::{self, ExplicitRouteArgs, ProjectCatalogDto, RouteState};
use crate::supervisor::{FailureKind, RuntimeSnapshot, ShutdownReceipt};

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
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    dispatch_with_library(command, snapshot, route, &library::EmptyLibrary)
}

pub fn dispatch_with_library(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
    library: &dyn NoteLibrary,
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
        IpcCommand::GetRuntimeState(_) => {
            Ok(IpcResponse::RuntimeState(runtime_state(snapshot, route)))
        }
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
    }
}

fn require_explicit_fixture_route(route: &ExplicitRouteArgs) -> Result<(), IpcError> {
    routing::require_explicit_fixture_route(route).map_err(|message| IpcError {
        category: ErrorCategory::Policy,
        message: message.to_owned(),
    })
}

fn runtime_state(snapshot: RuntimeSnapshot, route: &RouteState) -> RuntimeStateDto {
    RuntimeStateDto {
        status: snapshot.state.as_str().to_owned(),
        project: route.project.clone(),
        profile: snapshot.profile.map(|profile| profile.id().to_owned()),
        failure: snapshot.failure,
        shutdown: snapshot.shutdown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preflight::POLICY_NON_OWNED_PATH;
    use crate::supervisor::ConnectionState;

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
            ]
        );
        assert_eq!(capabilities.commands.len(), 8);
        assert_eq!(
            capabilities.events,
            vec![IpcEventName::RuntimeState, IpcEventName::Policy]
        );
        let json = serde_json::to_value(&IpcResponse::Capabilities(capabilities)).unwrap();
        let commands = json["commands"].as_array().unwrap();
        assert_eq!(commands.len(), 8);
        assert!(commands.iter().any(|command| command == "list_projects"));
        assert!(commands.iter().any(|command| command == "select_project"));
        assert!(commands.iter().any(|command| command == "list_tree"));
        assert!(commands.iter().any(|command| command == "read_note"));
        assert!(!commands.iter().any(|command| {
            command == "search_notes"
                || command == "call_tool"
                || command == "search"
                || command == "write_note"
        }));
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
        let json = serde_json::to_value(&IpcResponse::RuntimeState(state)).unwrap();
        assert!(json.get("child_pid").is_none());
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
        let search = serde_json::from_str::<IpcCommand>(
            r#"{"command":"search_notes","args":{"query":"cross-project"}}"#,
        );
        assert!(search.is_err());
        let write_note = serde_json::from_str::<IpcCommand>(
            r#"{"command":"write_note","args":{"project":"bmdock-fixture"}}"#,
        );
        assert!(write_note.is_err());
        let incomplete_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"project":"bmdock-fixture"}}"#,
        );
        assert!(incomplete_read.is_err());
        let extra_path_on_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome","path":"C:\\vault\\note.md"}}"#,
        );
        assert!(extra_path_on_read.is_err());
        let extra_path_on_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","path":"C:\\vault"}}"#,
        );
        assert!(extra_path_on_tree.is_err());
        let extra_fs_path_on_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","filesystem_path":"%APPDATA%\\\\Obsidian"}}"#,
        );
        assert!(extra_fs_path_on_tree.is_err());
        let well_formed_read = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","identifier":"welcome"}}"#,
        );
        assert!(well_formed_read.is_ok());
        let well_formed_tree = serde_json::from_str::<IpcCommand>(
            r#"{"command":"list_tree","args":{"workspace":"bmdock-workspace","project":"bmdock-fixture","page_size":2}}"#,
        );
        assert!(well_formed_tree.is_ok());
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

    #[test]
    fn list_tree_empty_library_is_empty_not_user_vault() {
        let mut route = RouteState::default();
        let IpcResponse::TreePage(page) = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(library::DEFAULT_PAGE_SIZE))),
            idle_snapshot(),
            &mut route,
            &library::EmptyLibrary,
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
        )
        .unwrap_err();
        assert_eq!(workspace.category, ErrorCategory::Policy);
        let identifier = dispatch_with_library(
            IpcCommand::ReadNote(ReadNoteArgs {
                workspace: crate::routing::OWNED_WORKSPACE_ID.to_owned(),
                project: FIXTURE_PROJECT.to_owned(),
                identifier: r"C:\Users\someone\vault\note.md".to_owned(),
            }),
            snapshot,
            &mut route,
            &PanicLibrary,
        )
        .unwrap_err();
        assert_eq!(identifier.category, ErrorCategory::Policy);
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
        )
        .unwrap_err();
        assert_eq!(zero.category, ErrorCategory::Schema);
        let huge = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(library::MAX_PAGE_SIZE + 1))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
        )
        .unwrap_err();
        assert_eq!(huge.category, ErrorCategory::Schema);
        let empty_cursor = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(Some(""), Some(2))),
            idle_snapshot(),
            &mut route,
            &PanicLibrary,
        )
        .unwrap_err();
        assert_eq!(empty_cursor.category, ErrorCategory::Schema);
        let truncated = dispatch_with_library(
            IpcCommand::ListTree(fixture_tree_args(None, Some(2))),
            idle_snapshot(),
            &mut route,
            &TruncatingLibrary,
        )
        .unwrap_err();
        assert_eq!(truncated.category, ErrorCategory::Unsupported);
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
}
