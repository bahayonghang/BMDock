#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::{Component, Path, PathBuf};
#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use crate::library::{looks_like_filesystem_path, LibraryError};
#[cfg(test)]
use crate::library::{MAX_PAGE_SIZE, UNSUPPORTED_TRUNCATED};
#[cfg(test)]
use crate::routing::OWNED_KIND;

pub const UNSUPPORTED_BACKUP_UNAVAILABLE: &str = "engine/backup store unavailable";
pub const SCHEMA_BACKUP_ID: &str = "backup_id is required";
pub const POLICY_BACKUP_IDENTIFIER: &str =
    "Backup identifiers are generated fixture ids, not user vault filesystem paths";
#[cfg(test)]
pub const POLICY_FORBIDDEN_RESTORE_ROOT: &str =
    "Fixture restore cannot target user vaults, %APPDATA% Obsidian, or global Basic Memory config";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupRecordDto {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BackupCatalogDto {
    pub backups: Vec<BackupRecordDto>,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub cloud_or_credential_required: bool,
    pub local_offline: bool,
    pub files_written: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RestoreClass {
    Empty,
    DiskVerified,
    AcceptedUnverified,
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreObservationDto {
    pub classified_as: RestoreClass,
    pub disk_verified: bool,
    pub envelope_is_not_disk_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoredFileDto {
    pub identifier: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RestoreResultDto {
    pub backup_id: String,
    pub files: Vec<RestoredFileDto>,
    pub files_written: bool,
    pub observation: RestoreObservationDto,
}

pub trait BackupStore: Send + Sync {
    fn list_backups(&self) -> Result<BackupCatalogDto, LibraryError>;

    fn restore_fixture(&self, backup_id: &str) -> Result<RestoreResultDto, LibraryError>;
}

#[derive(Debug, Default)]
pub struct EmptyBackupStore;

impl BackupStore for EmptyBackupStore {
    fn list_backups(&self) -> Result<BackupCatalogDto, LibraryError> {
        Ok(empty_catalog())
    }

    fn restore_fixture(&self, backup_id: &str) -> Result<RestoreResultDto, LibraryError> {
        reject_backup_id(backup_id)?;
        Err(LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))
    }
}

pub fn empty_catalog() -> BackupCatalogDto {
    BackupCatalogDto {
        backups: Vec::new(),
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        cloud_or_credential_required: false,
        local_offline: true,
        files_written: false,
    }
}

pub fn reject_backup_id(backup_id: &str) -> Result<(), LibraryError> {
    if backup_id.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_BACKUP_ID));
    }
    if looks_like_filesystem_path(backup_id) || backup_id.contains('/') {
        return Err(LibraryError::policy(POLICY_BACKUP_IDENTIFIER));
    }
    Ok(())
}

#[cfg(test)]
pub fn restore_root_is_forbidden(path: &Path) -> bool {
    let text = path.to_string_lossy();
    if text.contains("%APPDATA%") || text.contains("%USERPROFILE%") {
        return true;
    }
    path.components().any(|component| match component {
        Component::Normal(name) => {
            let lower = name.to_string_lossy().to_ascii_lowercase();
            lower == ".obsidian"
                || lower == "obsidian"
                || lower == ".basic-memory"
                || lower == "basic-memory"
        }
        _ => false,
    })
}

#[cfg(test)]
pub fn reject_forbidden_restore_root(path: &Path) -> Result<(), LibraryError> {
    if restore_root_is_forbidden(path) {
        Err(LibraryError::policy(POLICY_FORBIDDEN_RESTORE_ROOT))
    } else {
        Ok(())
    }
}

#[cfg(test)]
fn observation(classified_as: RestoreClass, disk_verified: bool) -> RestoreObservationDto {
    RestoreObservationDto {
        classified_as,
        disk_verified,
        envelope_is_not_disk_proof: true,
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct FixtureBackupStore {
    snapshots_root: PathBuf,
    target_root: PathBuf,
    files_written: AtomicBool,
}

#[cfg(test)]
impl FixtureBackupStore {
    pub fn new(snapshots_root: PathBuf, target_root: PathBuf) -> Result<Self, LibraryError> {
        reject_forbidden_restore_root(&snapshots_root)?;
        reject_forbidden_restore_root(&target_root)?;
        let marker = snapshots_root.to_string_lossy().contains("bmdock")
            && target_root.to_string_lossy().contains("bmdock");
        if !marker {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_RESTORE_ROOT));
        }
        fs::create_dir_all(&snapshots_root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        fs::create_dir_all(&target_root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        Ok(Self {
            snapshots_root,
            target_root,
            files_written: AtomicBool::new(false),
        })
    }

    pub fn target_root(&self) -> &Path {
        &self.target_root
    }

    pub fn seed_backup(
        &self,
        backup_id: &str,
        files: &[(&str, &str)],
    ) -> Result<PathBuf, LibraryError> {
        reject_backup_id(backup_id)?;
        let dir = self.snapshots_root.join(backup_id);
        fs::create_dir_all(&dir)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        for (identifier, body) in files {
            reject_backup_id(identifier)?;
            fs::write(dir.join(format!("{identifier}.md")), body)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        }
        Ok(dir)
    }

    fn collect_backup_ids(&self) -> Result<Vec<BackupRecordDto>, LibraryError> {
        let reader = fs::read_dir(&self.snapshots_root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
        let mut backups = Vec::new();
        for item in reader {
            let item = item.map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            let file_type = item
                .file_type()
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            if file_type.is_symlink() {
                return Err(LibraryError::policy(POLICY_BACKUP_IDENTIFIER));
            }
            if !file_type.is_dir() {
                continue;
            }
            let name = item.file_name().to_string_lossy().into_owned();
            reject_backup_id(&name)?;
            backups.push(BackupRecordDto {
                id: name,
                kind: OWNED_KIND.to_owned(),
            });
        }
        backups.sort_by(|left, right| left.id.cmp(&right.id));
        if backups.len() > MAX_PAGE_SIZE as usize {
            return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
        }
        Ok(backups)
    }

    fn resolve_snapshot(&self, backup_id: &str) -> Result<PathBuf, LibraryError> {
        reject_backup_id(backup_id)?;
        reject_forbidden_restore_root(&self.target_root)?;
        let candidate = self.snapshots_root.join(backup_id);
        if candidate.is_absolute() && looks_like_filesystem_path(backup_id) {
            return Err(LibraryError::policy(POLICY_BACKUP_IDENTIFIER));
        }
        let root = self
            .snapshots_root
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        let resolved = candidate
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        if !resolved.starts_with(&root) {
            return Err(LibraryError::policy(POLICY_BACKUP_IDENTIFIER));
        }
        Ok(resolved)
    }

    fn collect_markdown(snapshot: &Path) -> Result<Vec<(String, PathBuf)>, LibraryError> {
        let reader = fs::read_dir(snapshot)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        let mut files = Vec::new();
        for item in reader {
            let item =
                item.map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
            let file_type = item
                .file_type()
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
            if file_type.is_symlink() {
                return Err(LibraryError::policy(POLICY_BACKUP_IDENTIFIER));
            }
            if !file_type.is_file() {
                continue;
            }
            let name = item.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".md") {
                continue;
            }
            let identifier = name.trim_end_matches(".md").to_string();
            reject_backup_id(&identifier)?;
            files.push((identifier, item.path()));
        }
        files.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(files)
    }
}

#[cfg(test)]
impl BackupStore for FixtureBackupStore {
    fn list_backups(&self) -> Result<BackupCatalogDto, LibraryError> {
        Ok(BackupCatalogDto {
            backups: self.collect_backup_ids()?,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            cloud_or_credential_required: false,
            local_offline: true,
            files_written: self.files_written.load(Ordering::Relaxed),
        })
    }

    fn restore_fixture(&self, backup_id: &str) -> Result<RestoreResultDto, LibraryError> {
        reject_backup_id(backup_id)?;
        reject_forbidden_restore_root(&self.target_root)?;
        let snapshot = self.resolve_snapshot(backup_id)?;
        let markdown = Self::collect_markdown(&snapshot)?;
        if markdown.is_empty() {
            return Ok(RestoreResultDto {
                backup_id: backup_id.to_owned(),
                files: Vec::new(),
                files_written: false,
                observation: observation(RestoreClass::Empty, false),
            });
        }
        fs::create_dir_all(&self.target_root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
        let mut files = Vec::new();
        let mut wrote = false;
        let mut all_verified = true;
        for (identifier, source) in markdown {
            let dest = self.target_root.join(format!("{identifier}.md"));
            if restore_root_is_forbidden(&dest) {
                return Err(LibraryError::policy(POLICY_FORBIDDEN_RESTORE_ROOT));
            }
            let body = fs::read_to_string(&source)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
            fs::write(&dest, &body)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE))?;
            wrote = true;
            match fs::read_to_string(&dest) {
                Ok(disk) if disk == body && dest.is_file() => {
                    files.push(RestoredFileDto {
                        identifier,
                        kind: OWNED_KIND.to_owned(),
                    });
                }
                _ => {
                    all_verified = false;
                    files.push(RestoredFileDto {
                        identifier,
                        kind: OWNED_KIND.to_owned(),
                    });
                }
            }
        }
        if wrote {
            self.files_written.store(true, Ordering::Relaxed);
        }
        let classified = if wrote && all_verified {
            RestoreClass::DiskVerified
        } else if wrote {
            RestoreClass::Unclassified
        } else {
            RestoreClass::AcceptedUnverified
        };
        Ok(RestoreResultDto {
            backup_id: backup_id.to_owned(),
            files,
            files_written: wrote,
            observation: observation(classified, wrote && all_verified),
        })
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct EnvelopeBackupStore {
    pub backup_id: String,
}

#[cfg(test)]
impl BackupStore for EnvelopeBackupStore {
    fn list_backups(&self) -> Result<BackupCatalogDto, LibraryError> {
        Ok(BackupCatalogDto {
            backups: vec![BackupRecordDto {
                id: self.backup_id.clone(),
                kind: OWNED_KIND.to_owned(),
            }],
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            cloud_or_credential_required: false,
            local_offline: true,
            files_written: false,
        })
    }

    fn restore_fixture(&self, backup_id: &str) -> Result<RestoreResultDto, LibraryError> {
        reject_backup_id(backup_id)?;
        Ok(RestoreResultDto {
            backup_id: backup_id.to_owned(),
            files: Vec::new(),
            files_written: false,
            observation: observation(RestoreClass::AcceptedUnverified, false),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static FIXTURE_SEQ: AtomicU64 = AtomicU64::new(0);

    struct TempOwned {
        dir: PathBuf,
    }

    impl TempOwned {
        fn create() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("bmdock-t12-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }
    }

    impl Drop for TempOwned {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    fn owned_store() -> (TempOwned, FixtureBackupStore) {
        let owned = TempOwned::create();
        let snapshots = owned.dir.join("snapshots");
        let target = owned.dir.join("target");
        let store = FixtureBackupStore::new(snapshots, target).unwrap();
        (owned, store)
    }

    #[test]
    fn empty_catalog_is_empty_state_not_user_vault() {
        let store = EmptyBackupStore;
        let catalog = store.list_backups().unwrap();
        assert!(catalog.backups.is_empty());
        assert!(!catalog.scanned_user_obsidian_vault);
        assert!(!catalog.scanned_user_basic_memory_home);
        assert!(!catalog.files_written);
        assert!(catalog.local_offline);
        assert!(!catalog.cloud_or_credential_required);
        let error = store.restore_fixture("fixture-welcome").unwrap_err();
        assert_eq!(
            error,
            LibraryError::unsupported(UNSUPPORTED_BACKUP_UNAVAILABLE)
        );
    }

    #[test]
    fn restore_observes_physical_markdown_not_envelope() {
        let (_owned, store) = owned_store();
        let body = "# 中文夹具备份\n\n这是 BMDock 自有恢复正文。参见 [[欢迎]]。\n";
        store
            .seed_backup("fixture-welcome", &[("welcome", body)])
            .unwrap();
        let before = store.list_backups().unwrap();
        assert_eq!(before.backups.len(), 1);
        assert_eq!(before.backups[0].id, "fixture-welcome");
        assert_eq!(before.backups[0].kind, OWNED_KIND);
        assert!(!before.files_written);
        assert!(!before.scanned_user_obsidian_vault);
        assert!(!before.scanned_user_basic_memory_home);

        let result = store.restore_fixture("fixture-welcome").unwrap();
        let dest = store.target_root().join("welcome.md");
        assert!(dest.is_file(), "restore must create a physical file");
        let disk = fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert!(result.files_written);
        assert!(result.observation.disk_verified);
        assert!(result.observation.envelope_is_not_disk_proof);
        assert_eq!(result.observation.classified_as, RestoreClass::DiskVerified);
        assert_eq!(result.files[0].identifier, "welcome");
        assert_eq!(result.files[0].kind, OWNED_KIND);

        let after = store.list_backups().unwrap();
        assert!(after.files_written);
        assert!(!after.scanned_user_obsidian_vault);
    }

    #[test]
    fn envelope_restored_text_is_not_disk_proof() {
        let store = EnvelopeBackupStore {
            backup_id: "fixture-welcome".to_owned(),
        };
        let result = store.restore_fixture("fixture-welcome").unwrap();
        assert!(!result.files_written);
        assert!(!result.observation.disk_verified);
        assert!(result.observation.envelope_is_not_disk_proof);
        assert_eq!(
            result.observation.classified_as,
            RestoreClass::AcceptedUnverified
        );
        let catalog = store.list_backups().unwrap();
        assert!(!catalog.files_written);
    }

    #[test]
    fn filesystem_backup_id_is_policy() {
        for backup_id in [
            r"C:\Users\someone\vault",
            r"%APPDATA%\Obsidian\vault",
            "/home/someone/.basic-memory/backup",
            "../secret",
            "folder/nested",
        ] {
            assert_eq!(
                reject_backup_id(backup_id).unwrap_err(),
                LibraryError::policy(POLICY_BACKUP_IDENTIFIER)
            );
            assert_eq!(
                EmptyBackupStore.restore_fixture(backup_id).unwrap_err(),
                LibraryError::policy(POLICY_BACKUP_IDENTIFIER)
            );
        }
        assert_eq!(
            reject_backup_id(""),
            Err(LibraryError::schema(SCHEMA_BACKUP_ID))
        );
        reject_backup_id("fixture-welcome").unwrap();
    }

    #[test]
    fn forbidden_restore_roots_are_policy() {
        for path in [
            Path::new(r"C:\Users\someone\AppData\Roaming\Obsidian"),
            Path::new(r"C:\Users\someone\.basic-memory"),
            Path::new("/home/someone/.basic-memory"),
            Path::new("%APPDATA%\\Obsidian"),
        ] {
            assert!(
                restore_root_is_forbidden(path),
                "expected forbidden restore root: {}",
                path.display()
            );
            assert_eq!(
                reject_forbidden_restore_root(path).unwrap_err(),
                LibraryError::policy(POLICY_FORBIDDEN_RESTORE_ROOT)
            );
        }
        let temp = std::env::temp_dir().join("bmdock-t12-owned-root");
        assert!(!restore_root_is_forbidden(&temp));
        let forbidden_store = FixtureBackupStore::new(
            PathBuf::from(r"C:\Users\someone\Documents\Obsidian"),
            PathBuf::from(r"C:\Users\someone\Documents\Obsidian"),
        );
        assert_eq!(
            forbidden_store.unwrap_err(),
            LibraryError::policy(POLICY_FORBIDDEN_RESTORE_ROOT)
        );
    }
}
