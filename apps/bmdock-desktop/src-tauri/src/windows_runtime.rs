use std::path::PathBuf;

const TAURI_CONF_JSON: &str = include_str!("../tauri.conf.json");

#[derive(Debug, Clone, Copy, serde::Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum HostOs {
    Windows,
    Other,
}

impl HostOs {
    pub fn current() -> Self {
        if cfg!(windows) {
            Self::Windows
        } else {
            Self::Other
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct WindowsRuntimeDto {
    pub host_os: HostOs,
    pub webview2_files_present: bool,
    pub webview2_session_verified: bool,
    pub job_object_assigned: bool,
    pub job_object_api_documented: bool,
    pub installer_bundle_active: bool,
    pub files_written: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
}

pub fn inspect_windows_runtime() -> WindowsRuntimeDto {
    let host_os = HostOs::current();
    WindowsRuntimeDto {
        host_os,
        webview2_files_present: observe_webview2_files(),
        // cargo test / npm build / a compiled exe are not a native GUI session.
        webview2_session_verified: false,
        // Documented Job Object APIs are not proof a job was created and a child assigned.
        job_object_assigned: false,
        job_object_api_documented: matches!(host_os, HostOs::Windows),
        installer_bundle_active: installer_bundle_active(),
        files_written: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
    }
}

pub fn installer_bundle_active() -> bool {
    serde_json::from_str::<serde_json::Value>(TAURI_CONF_JSON)
        .ok()
        .and_then(|value| value.get("bundle")?.get("active")?.as_bool())
        .unwrap_or(false)
}

fn observe_webview2_files() -> bool {
    if !cfg!(windows) {
        return false;
    }
    well_known_webview2_paths()
        .into_iter()
        .any(|path| path.is_dir() || path.is_file())
}

fn well_known_webview2_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    for key in ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"] {
        if let Ok(root) = std::env::var(key) {
            if root.is_empty() {
                continue;
            }
            paths.push(
                PathBuf::from(root)
                    .join("Microsoft")
                    .join("EdgeWebView")
                    .join("Application"),
            );
        }
    }
    paths
}

#[cfg(test)]
fn independent_webview2_install_dir_present() -> bool {
    if !cfg!(windows) {
        return false;
    }
    ["ProgramFiles(x86)", "ProgramFiles", "LOCALAPPDATA"]
        .into_iter()
        .filter_map(|key| std::env::var(key).ok())
        .filter(|root| !root.is_empty())
        .any(|root| {
            PathBuf::from(&root)
                .join("Microsoft")
                .join("EdgeWebView")
                .join("Application")
                .is_dir()
        })
}

#[cfg(test)]
fn on_disk_bundle_active() -> bool {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tauri.conf.json");
    let raw = std::fs::read_to_string(path).expect("tauri.conf.json must be readable");
    serde_json::from_str::<serde_json::Value>(&raw)
        .ok()
        .and_then(|value| value.get("bundle")?.get("active")?.as_bool())
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inspect_separates_observed_facts_from_unverified_claims() {
        let dto = inspect_windows_runtime();
        #[cfg(windows)]
        {
            assert_eq!(dto.host_os, HostOs::Windows);
            assert_eq!(
                dto.webview2_files_present,
                independent_webview2_install_dir_present()
            );
            assert!(dto.job_object_api_documented);
        }
        #[cfg(not(windows))]
        {
            assert_eq!(dto.host_os, HostOs::Other);
            assert!(!dto.webview2_files_present);
            assert!(!dto.job_object_api_documented);
        }
        assert!(!dto.webview2_session_verified);
        assert!(!dto.job_object_assigned);
        assert!(!dto.installer_bundle_active);
        assert!(!dto.files_written);
        assert!(!dto.scanned_user_obsidian_vault);
        assert!(!dto.scanned_user_basic_memory_home);
    }

    #[test]
    fn compiled_exe_npm_build_and_cargo_test_are_not_native_gui() {
        let dto = inspect_windows_runtime();
        assert!(
            !dto.webview2_session_verified,
            "compiled exe / npm build / cargo test are not a native GUI or WebView2 session"
        );
    }

    #[test]
    fn webview2_files_present_is_not_a_webview2_session() {
        let dto = inspect_windows_runtime();
        if dto.webview2_files_present {
            assert!(
                !dto.webview2_session_verified,
                "WebView2 files present is not a WebView2 session"
            );
        }
        assert!(!dto.webview2_session_verified);
    }

    #[test]
    fn job_object_api_docs_are_not_assignment() {
        let dto = inspect_windows_runtime();
        if dto.job_object_api_documented {
            assert!(
                !dto.job_object_assigned,
                "Job Object API/docs is not proof a job was created and a child assigned"
            );
        }
        assert!(!dto.job_object_assigned);
    }

    #[test]
    fn inspect_dto_is_not_just_contract_or_engine_inventory() {
        let dto = inspect_windows_runtime();
        let json = serde_json::to_value(&dto).unwrap();
        assert!(json.get("expected_tools").is_none());
        assert!(json.get("tools").is_none());
        assert!(json.get("profile").is_none());
        assert!(json.get("protocolVersion").is_none());
        assert!(json.get("protocol_version").is_none());
        assert_eq!(json["files_written"], false);
    }

    #[test]
    fn inspect_dto_is_not_t12_fixture_restore_or_windows_recovery() {
        let dto = inspect_windows_runtime();
        let json = serde_json::to_value(&dto).unwrap();
        assert!(!dto.files_written);
        assert!(!dto.job_object_assigned);
        assert!(json.get("backup_id").is_none());
        assert!(json.get("classified_as").is_none());
        assert!(json.get("disk_verified").is_none());
        assert_eq!(json["scanned_user_obsidian_vault"], false);
        assert_eq!(json["scanned_user_basic_memory_home"], false);
    }

    #[test]
    fn installer_bundle_active_matches_committed_tauri_conf() {
        assert!(
            !on_disk_bundle_active(),
            "T13 must not enable installer bundling; T37/T38 keep bundle.active=false and signing UNVERIFIED"
        );
        let dto = inspect_windows_runtime();
        assert_eq!(dto.installer_bundle_active, on_disk_bundle_active());
        assert_eq!(dto.installer_bundle_active, installer_bundle_active());
        assert!(!dto.installer_bundle_active);
    }

    #[test]
    fn json_field_names_are_snake_case() {
        let dto = inspect_windows_runtime();
        let json = serde_json::to_value(&dto).unwrap();
        for key in [
            "host_os",
            "webview2_files_present",
            "webview2_session_verified",
            "job_object_assigned",
            "job_object_api_documented",
            "installer_bundle_active",
            "files_written",
            "scanned_user_obsidian_vault",
            "scanned_user_basic_memory_home",
        ] {
            assert!(json.get(key).is_some(), "missing {key}");
        }
        match dto.host_os {
            HostOs::Windows => assert_eq!(json["host_os"], "windows"),
            HostOs::Other => assert_eq!(json["host_os"], "other"),
        }
        assert_eq!(json["webview2_session_verified"], false);
        assert_eq!(json["job_object_assigned"], false);
    }

    #[test]
    fn well_known_paths_do_not_scan_user_vaults() {
        for path in well_known_webview2_paths() {
            let rendered = path.to_string_lossy().to_ascii_lowercase();
            assert!(!rendered.contains("obsidian"));
            assert!(!rendered.contains(".basic-memory"));
            assert!(
                rendered.contains("edgewebview"),
                "only well-known EdgeWebView install dirs may be observed"
            );
        }
    }
}
