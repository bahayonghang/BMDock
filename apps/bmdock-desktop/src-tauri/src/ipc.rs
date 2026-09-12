use serde::{Deserialize, Serialize};

use crate::preflight::{self, ConfigDiscoveryDto, PreflightDto};
use crate::routing::{self, ProjectCatalogDto, RouteState};
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
}

pub fn allowed_commands() -> Vec<IpcCommandName> {
    vec![
        IpcCommandName::GetCapabilities,
        IpcCommandName::GetRuntimeState,
        IpcCommandName::SelectProject,
        IpcCommandName::ListProjects,
        IpcCommandName::RunPreflight,
        IpcCommandName::DiscoverConfig,
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
    Error(IpcError),
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

pub fn dispatch_with_route(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
    route: &mut RouteState,
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
    }
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
            ]
        );
        assert_eq!(capabilities.commands.len(), 6);
        assert_eq!(
            capabilities.events,
            vec![IpcEventName::RuntimeState, IpcEventName::Policy]
        );
        let json = serde_json::to_value(&IpcResponse::Capabilities(capabilities)).unwrap();
        let commands = json["commands"].as_array().unwrap();
        assert_eq!(commands.len(), 6);
        assert!(commands.iter().any(|command| command == "list_projects"));
        assert!(commands.iter().any(|command| command == "select_project"));
        assert!(!commands.iter().any(|command| {
            command == "search_notes" || command == "call_tool" || command == "search"
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
        let read_note = serde_json::from_str::<IpcCommand>(
            r#"{"command":"read_note","args":{"project":"bmdock-fixture"}}"#,
        );
        assert!(read_note.is_err());
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
}
