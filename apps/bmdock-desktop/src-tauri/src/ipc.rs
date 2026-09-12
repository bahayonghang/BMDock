use serde::{Deserialize, Serialize};

use crate::supervisor::{ConnectionState, FailureKind, RuntimeSnapshot, ShutdownReceipt};

pub const FIXTURE_PROJECT: &str = "bmdock-fixture";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IpcCommandName {
    GetCapabilities,
    GetRuntimeState,
    SelectProject,
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
#[serde(tag = "command", content = "args", rename_all = "snake_case")]
pub enum IpcCommand {
    GetCapabilities(EmptyArgs),
    GetRuntimeState(EmptyArgs),
    SelectProject(SelectProjectArgs),
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
    Error(IpcError),
}

pub fn dispatch(command: IpcCommand) -> Result<IpcResponse, IpcError> {
    dispatch_with_snapshot(
        command,
        RuntimeSnapshot {
            state: ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        },
    )
}

pub fn dispatch_with_snapshot(
    command: IpcCommand,
    snapshot: RuntimeSnapshot,
) -> Result<IpcResponse, IpcError> {
    match command {
        IpcCommand::GetCapabilities(_) => Ok(IpcResponse::Capabilities(CapabilitiesDto {
            commands: vec![
                IpcCommandName::GetCapabilities,
                IpcCommandName::GetRuntimeState,
                IpcCommandName::SelectProject,
            ],
            events: vec![IpcEventName::RuntimeState, IpcEventName::Policy],
            policy: PolicyDto {
                project: FIXTURE_PROJECT,
                arbitrary_paths_allowed: false,
                raw_call_tool_allowed: false,
            },
        })),
        IpcCommand::GetRuntimeState(_) => Ok(IpcResponse::RuntimeState(runtime_state(snapshot))),
        IpcCommand::SelectProject(args) if args.project == FIXTURE_PROJECT => {
            Ok(IpcResponse::ProjectSelected {
                project: FIXTURE_PROJECT,
            })
        }
        IpcCommand::SelectProject(_) => Err(IpcError {
            category: ErrorCategory::Policy,
            message: "Only the generated fixture project is allowed".to_owned(),
        }),
    }
}

fn runtime_state(snapshot: RuntimeSnapshot) -> RuntimeStateDto {
    let status = match snapshot.state {
        ConnectionState::NotStarted => "not_started",
        ConnectionState::Starting => "starting",
        ConnectionState::Connected => "connected",
        ConnectionState::Stopping => "stopping",
        ConnectionState::Stopped => "stopped",
        ConnectionState::Failed => "failed",
    };
    RuntimeStateDto {
        status: status.to_owned(),
        project: None,
        profile: snapshot.profile.map(|profile| profile.id().to_owned()),
        failure: snapshot.failure,
        shutdown: snapshot.shutdown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capabilities_are_explicit_and_fail_closed() {
        let IpcResponse::Capabilities(capabilities) =
            dispatch(IpcCommand::GetCapabilities(EmptyArgs {})).unwrap()
        else {
            panic!("wrong response variant")
        };
        assert_eq!(capabilities.policy.project, FIXTURE_PROJECT);
        assert!(!capabilities.policy.arbitrary_paths_allowed);
        assert!(!capabilities.policy.raw_call_tool_allowed);
        assert_eq!(capabilities.commands.len(), 3);
    }

    #[test]
    fn project_policy_rejects_non_fixture() {
        let result = dispatch(IpcCommand::SelectProject(SelectProjectArgs {
            project: "C:\\Users\\someone\\vault".to_owned(),
        }));
        assert_eq!(result.unwrap_err().category, ErrorCategory::Policy);
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
    }
}
