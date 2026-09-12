use crate::supervisor::{ConnectionState, EngineProfile, RuntimeSnapshot};

#[cfg(test)]
use std::path::{Component, Path};

#[cfg(test)]
pub const POLICY_NON_OWNED_PATH: &str = "Only BMDock-owned fixture paths may be inspected";
pub const DISCOVERY_ROOT_NONE: &str = "none";
#[cfg(test)]
const OWNED_CONFIG_KIND: &str = "bmdock_owned";
#[cfg(test)]
const OWNED_PATH_COMPONENT: &str = "bmdock-fixture";

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct ProfileRecordDto {
    pub id: String,
    pub commit: String,
    pub expected_tools: usize,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct PreflightHostDto {
    pub profiles_metadata_present: bool,
    pub arbitrary_paths_allowed: bool,
    pub raw_call_tool_allowed: bool,
    pub supervisor_status: String,
    pub supervisor_idle: bool,
    pub cloud_or_credential_required: bool,
    pub engine_spawned: bool,
    pub files_written: bool,
    pub local_offline: bool,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct PreflightDto {
    pub profiles: Vec<ProfileRecordDto>,
    pub host: PreflightHostDto,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct ConfigCandidateDto {
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct ConfigDiscoveryDto {
    pub root: String,
    pub candidates: Vec<ConfigCandidateDto>,
    pub scanned_user_basic_memory_home: bool,
    pub copied_or_rewrote_production_config: bool,
}

pub fn isolated_profiles() -> Vec<ProfileRecordDto> {
    [EngineProfile::Release, EngineProfile::MainPreview]
        .into_iter()
        .map(|profile| ProfileRecordDto {
            id: profile.id().to_owned(),
            commit: profile.commit().to_owned(),
            expected_tools: profile.expected_tools(),
        })
        .collect()
}

pub fn run_preflight(snapshot: &RuntimeSnapshot) -> PreflightDto {
    let profiles = isolated_profiles();
    let profiles_metadata_present = profiles.len() == 2
        && profiles[0].id == EngineProfile::Release.id()
        && profiles[0].expected_tools == 21
        && profiles[1].id == EngineProfile::MainPreview.id()
        && profiles[1].expected_tools == 27
        && profiles[0].commit != profiles[1].commit
        && !profiles[0].commit.is_empty()
        && !profiles[1].commit.is_empty();
    let supervisor_status = snapshot.state.as_str();
    PreflightDto {
        profiles,
        host: PreflightHostDto {
            profiles_metadata_present,
            arbitrary_paths_allowed: false,
            raw_call_tool_allowed: false,
            supervisor_status: supervisor_status.to_owned(),
            supervisor_idle: snapshot.state == ConnectionState::NotStarted,
            cloud_or_credential_required: false,
            // Honest lifecycle projection; T09 still does not start/stop.
            engine_spawned: engine_spawned_from_lifecycle(snapshot.state),
            // T09 never writes; do not infer vault mutation from a live engine.
            files_written: false,
            local_offline: true,
        },
    }
}

fn engine_spawned_from_lifecycle(state: ConnectionState) -> bool {
    match state {
        ConnectionState::NotStarted => false,
        ConnectionState::Starting
        | ConnectionState::Connected
        | ConnectionState::Stopping
        | ConnectionState::Stopped
        | ConnectionState::Failed => true,
    }
}

#[cfg(test)]
pub fn is_bmdock_owned_config_root(path: &str) -> bool {
    if path.is_empty() || path.contains("..") {
        return false;
    }
    Path::new(path)
        .components()
        .any(|component| match component {
            Component::Normal(part) => part == OWNED_PATH_COMPONENT,
            _ => false,
        })
}

#[cfg(test)]
pub fn owned_config_root_or_policy(path: &str) -> Result<(), &'static str> {
    if is_bmdock_owned_config_root(path) {
        Ok(())
    } else {
        Err(POLICY_NON_OWNED_PATH)
    }
}

pub fn empty_discovery() -> ConfigDiscoveryDto {
    ConfigDiscoveryDto {
        root: DISCOVERY_ROOT_NONE.to_owned(),
        candidates: Vec::new(),
        scanned_user_basic_memory_home: false,
        copied_or_rewrote_production_config: false,
    }
}

#[cfg(test)]
pub fn discover_config(root: Option<&str>) -> Result<ConfigDiscoveryDto, &'static str> {
    match root {
        None => Ok(empty_discovery()),
        Some(path) => {
            owned_config_root_or_policy(path)?;
            Ok(list_owned_config_candidates(path))
        }
    }
}

#[cfg(test)]
fn list_owned_config_candidates(root: &str) -> ConfigDiscoveryDto {
    let config = Path::new(root).join("config.json");
    let mut candidates = Vec::new();
    if let Ok(meta) = std::fs::symlink_metadata(&config) {
        if meta.is_file() && !meta.file_type().is_symlink() {
            candidates.push(ConfigCandidateDto {
                path: config.to_string_lossy().replace('\\', "/"),
                kind: OWNED_CONFIG_KIND.to_owned(),
            });
        }
    }
    ConfigDiscoveryDto {
        root: root.replace('\\', "/"),
        candidates,
        scanned_user_basic_memory_home: false,
        copied_or_rewrote_production_config: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::supervisor::{
        ConnectionState, EngineProfile, FailureKind, RuntimeSnapshot, ShutdownReceipt,
    };

    fn idle_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            state: ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        }
    }

    #[test]
    fn preflight_does_not_spawn_or_write() {
        let snapshot = idle_snapshot();
        let report = run_preflight(&snapshot);
        assert!(!report.host.engine_spawned);
        assert!(!report.host.files_written);
        assert!(report.host.supervisor_idle);
        assert_eq!(report.host.supervisor_status, "not_started");
        assert!(!report.host.cloud_or_credential_required);
        assert!(report.host.local_offline);
        assert!(!report.host.arbitrary_paths_allowed);
        assert!(!report.host.raw_call_tool_allowed);
        assert_eq!(snapshot, idle_snapshot());
    }

    #[test]
    fn preflight_profiles_stay_isolated() {
        let report = run_preflight(&idle_snapshot());
        assert_eq!(report.profiles.len(), 2);
        assert_eq!(report.profiles[0].id, "release");
        assert_eq!(
            report.profiles[0].commit,
            "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048"
        );
        assert_eq!(report.profiles[0].expected_tools, 21);
        assert_eq!(report.profiles[1].id, "main-preview");
        assert_eq!(
            report.profiles[1].commit,
            "3452c821d76c083823d020984d71e06904a1ff1e"
        );
        assert_eq!(report.profiles[1].expected_tools, 27);
        assert_ne!(report.profiles[0].commit, report.profiles[1].commit);
        assert_ne!(
            report.profiles[0].expected_tools,
            report.profiles[1].expected_tools
        );
        let json = serde_json::to_value(&report).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert_eq!(json["profiles"][0]["expected_tools"], 21);
        assert_eq!(json["profiles"][1]["expected_tools"], 27);
        assert!(report.host.profiles_metadata_present);
        assert_eq!(EngineProfile::Release.expected_tools(), 21);
        assert_eq!(EngineProfile::MainPreview.expected_tools(), 27);
    }

    #[test]
    fn preflight_does_not_stop_an_already_started_snapshot() {
        let snapshot = RuntimeSnapshot {
            state: ConnectionState::Connected,
            profile: Some(EngineProfile::Release),
            child_pid: Some(42),
            failure: Some(FailureKind::Unverified),
            shutdown: Some(ShutdownReceipt {
                transport_cancelled: false,
                child_exited: false,
                forced: false,
                timeout_unknown: false,
                exit_code: None,
            }),
        };
        let before = snapshot.clone();
        let report = run_preflight(&snapshot);
        assert_eq!(report.host.supervisor_status, "connected");
        assert!(!report.host.supervisor_idle);
        assert!(report.host.engine_spawned);
        assert!(!report.host.files_written);
        assert_eq!(snapshot, before);
        assert_eq!(snapshot.state, ConnectionState::Connected);
        assert_eq!(snapshot.child_pid, Some(42));
    }

    #[test]
    fn preflight_reports_stopped_lifecycle_without_writing() {
        let snapshot = RuntimeSnapshot {
            state: ConnectionState::Stopped,
            profile: Some(EngineProfile::MainPreview),
            child_pid: None,
            failure: Some(FailureKind::TimeoutUnknown),
            shutdown: Some(ShutdownReceipt {
                transport_cancelled: true,
                child_exited: true,
                forced: false,
                timeout_unknown: true,
                exit_code: None,
            }),
        };
        let before = snapshot.clone();
        let report = run_preflight(&snapshot);
        assert_eq!(report.host.supervisor_status, "stopped");
        assert!(report.host.engine_spawned);
        assert!(!report.host.files_written);
        assert!(!report.host.supervisor_idle);
        assert_eq!(snapshot, before);
    }

    #[test]
    fn extra_user_paths_are_not_owned() {
        for path in [
            r"C:\Users\someone\Documents\vault",
            r"C:\Users\someone\.basic-memory",
            r"C:\Users\someone\Documents\Obsidian\.obsidian",
            "/home/someone/.basic-memory/config.json",
            "",
            "../vault",
        ] {
            assert!(
                !is_bmdock_owned_config_root(path),
                "path should not be owned: {path}"
            );
            assert_eq!(
                owned_config_root_or_policy(path).unwrap_err(),
                POLICY_NON_OWNED_PATH
            );
        }
    }

    #[test]
    fn non_owned_discovery_is_policy_without_user_vault_success() {
        let error = discover_config(Some(r"C:\Users\someone\vault")).unwrap_err();
        assert_eq!(error, POLICY_NON_OWNED_PATH);
        let empty = discover_config(None).unwrap();
        assert_eq!(empty.root, DISCOVERY_ROOT_NONE);
        assert!(empty.candidates.is_empty());
        assert!(!empty.scanned_user_basic_memory_home);
        assert!(!empty.copied_or_rewrote_production_config);
        let json = serde_json::to_value(&empty).unwrap();
        assert_ne!(json["root"], r"C:\Users\someone\vault");
        assert_ne!(json["root"], "/home/someone/.basic-memory");
    }

    #[test]
    fn owned_discovery_lists_without_rewriting() {
        let root = std::env::temp_dir()
            .join(format!("t09-{}", std::process::id()))
            .join("bmdock-fixture");
        std::fs::create_dir_all(&root).unwrap();
        let config = root.join("config.json");
        let original = "{\"kind\":\"bmdock-owned\"}";
        std::fs::write(&config, original).unwrap();
        let result = discover_config(Some(root.to_str().unwrap())).unwrap();
        let after = std::fs::read_to_string(&config).unwrap();
        let _ = std::fs::remove_dir_all(&root);
        assert_eq!(after, original);
        assert_eq!(result.candidates.len(), 1);
        assert_eq!(result.candidates[0].kind, OWNED_CONFIG_KIND);
        assert!(!result.copied_or_rewrote_production_config);
        assert!(!result.scanned_user_basic_memory_home);
    }

    #[test]
    fn owned_missing_config_is_empty_not_user_vault() {
        let root = std::env::temp_dir()
            .join(format!("t09-empty-{}", std::process::id()))
            .join("bmdock-fixture");
        std::fs::create_dir_all(&root).unwrap();
        let result = discover_config(Some(root.to_str().unwrap())).unwrap();
        let _ = std::fs::remove_dir_all(&root);
        assert!(result.candidates.is_empty());
        assert!(!result.scanned_user_basic_memory_home);
    }
}
