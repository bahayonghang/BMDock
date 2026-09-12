#[cfg(test)]
use std::fs;
#[cfg(test)]
use std::path::{Component, Path, PathBuf};
#[cfg(test)]
use std::sync::atomic::{AtomicBool, Ordering};

use serde::{Deserialize, Serialize};

use crate::library::{looks_like_filesystem_path, LibraryError};
#[cfg(test)]
use crate::library::{POLICY_FILESYSTEM_IDENTIFIER, UNSUPPORTED_TRUNCATED};

pub const UNSUPPORTED_DRAFT_UNAVAILABLE: &str = "engine/draft store unavailable";
pub const SCHEMA_DRAFT_IDENTIFIER: &str = "draft identifier is required";
pub const POLICY_DRAFT_IDENTIFIER: &str =
    "Draft identifiers are permalinks, not user vault filesystem paths";
#[cfg(test)]
pub const POLICY_FORBIDDEN_DRAFT_ROOT: &str =
    "Draft store cannot target user vaults, %APPDATA% Obsidian, or global Basic Memory config";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DraftClass {
    Empty,
    DiskVerified,
    AcceptedUnverified,
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DraftObservationDto {
    pub classified_as: DraftClass,
    pub disk_verified: bool,
    pub envelope_is_not_disk_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DraftResultDto {
    pub identifier: String,
    pub body: String,
    pub files_written: bool,
    pub engine_persisted: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub observation: DraftObservationDto,
}

pub trait DraftStore: Send + Sync {
    fn save_draft(&self, identifier: &str, body: &str) -> Result<DraftResultDto, LibraryError>;

    fn load_draft(&self, identifier: &str) -> Result<DraftResultDto, LibraryError>;
}

#[derive(Debug, Default)]
pub struct EmptyDraftStore;

impl DraftStore for EmptyDraftStore {
    fn save_draft(&self, identifier: &str, body: &str) -> Result<DraftResultDto, LibraryError> {
        let _ = body;
        reject_draft_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))
    }

    fn load_draft(&self, identifier: &str) -> Result<DraftResultDto, LibraryError> {
        reject_draft_identifier(identifier)?;
        Ok(empty_session(identifier))
    }
}

pub fn empty_session(identifier: &str) -> DraftResultDto {
    DraftResultDto {
        identifier: identifier.to_owned(),
        body: String::new(),
        files_written: false,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: observation(DraftClass::Empty, false),
    }
}

pub fn reject_draft_identifier(identifier: &str) -> Result<(), LibraryError> {
    if identifier.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_DRAFT_IDENTIFIER));
    }
    if looks_like_filesystem_path(identifier) {
        return Err(LibraryError::policy(POLICY_DRAFT_IDENTIFIER));
    }
    Ok(())
}

fn observation(classified_as: DraftClass, disk_verified: bool) -> DraftObservationDto {
    DraftObservationDto {
        classified_as,
        disk_verified,
        envelope_is_not_disk_proof: true,
    }
}

#[cfg(test)]
pub fn draft_root_is_forbidden(path: &Path) -> bool {
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
pub fn reject_forbidden_draft_root(path: &Path) -> Result<(), LibraryError> {
    if draft_root_is_forbidden(path) {
        Err(LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT))
    } else {
        Ok(())
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct FixtureDraftStore {
    root: PathBuf,
    files_written: AtomicBool,
}

#[cfg(test)]
impl FixtureDraftStore {
    pub fn new(root: PathBuf) -> Result<Self, LibraryError> {
        reject_forbidden_draft_root(&root)?;
        if !root.to_string_lossy().contains("bmdock-t14") {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT));
        }
        fs::create_dir_all(&root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))?;
        Ok(Self {
            root,
            files_written: AtomicBool::new(false),
        })
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn resolve_draft_path(&self, identifier: &str) -> Result<PathBuf, LibraryError> {
        reject_draft_identifier(identifier)?;
        reject_forbidden_draft_root(&self.root)?;
        if identifier
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || part.contains('\\'))
        {
            return Err(LibraryError::policy(POLICY_DRAFT_IDENTIFIER));
        }
        let mut relative = PathBuf::new();
        for part in identifier.split('/') {
            relative.push(part);
        }
        relative.set_extension("md");
        if relative.is_absolute() {
            return Err(LibraryError::policy(POLICY_DRAFT_IDENTIFIER));
        }
        let candidate = self.root.join(relative);
        if draft_root_is_forbidden(&candidate) {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT));
        }
        Ok(candidate)
    }

    fn owned_file(&self, path: &Path) -> Result<PathBuf, LibraryError> {
        let root = self
            .root
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))?;
        let resolved = path
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))?;
        if !resolved.starts_with(&root) {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        Ok(resolved)
    }
}

#[cfg(test)]
impl DraftStore for FixtureDraftStore {
    fn save_draft(&self, identifier: &str, body: &str) -> Result<DraftResultDto, LibraryError> {
        reject_draft_identifier(identifier)?;
        reject_forbidden_draft_root(&self.root)?;
        let dest = self.resolve_draft_path(identifier)?;
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))?;
        }
        fs::write(&dest, body)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE))?;
        let exists = dest.is_file();
        let disk = fs::read_to_string(&dest).ok();
        let verified = exists && disk.as_deref() == Some(body);
        if exists {
            self.files_written.store(true, Ordering::Relaxed);
        }
        let classified = if verified {
            DraftClass::DiskVerified
        } else if exists {
            DraftClass::Unclassified
        } else {
            DraftClass::AcceptedUnverified
        };
        Ok(DraftResultDto {
            identifier: identifier.to_owned(),
            body: body.to_owned(),
            files_written: exists,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: observation(classified, verified),
        })
    }

    fn load_draft(&self, identifier: &str) -> Result<DraftResultDto, LibraryError> {
        reject_draft_identifier(identifier)?;
        reject_forbidden_draft_root(&self.root)?;
        let dest = self.resolve_draft_path(identifier)?;
        if !dest.exists() {
            return Ok(empty_session(identifier));
        }
        let resolved = self.owned_file(&dest)?;
        let disk = fs::read_to_string(&resolved)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
        Ok(DraftResultDto {
            identifier: identifier.to_owned(),
            body: disk.clone(),
            files_written: self.files_written.load(Ordering::Relaxed),
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: observation(DraftClass::DiskVerified, true),
        })
    }
}

#[cfg(test)]
#[derive(Debug)]
pub struct EnvelopeDraftStore {
    pub body: String,
}

#[cfg(test)]
impl DraftStore for EnvelopeDraftStore {
    fn save_draft(&self, identifier: &str, _body: &str) -> Result<DraftResultDto, LibraryError> {
        reject_draft_identifier(identifier)?;
        Ok(DraftResultDto {
            identifier: identifier.to_owned(),
            body: "saved".to_owned(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: observation(DraftClass::AcceptedUnverified, false),
        })
    }

    fn load_draft(&self, identifier: &str) -> Result<DraftResultDto, LibraryError> {
        reject_draft_identifier(identifier)?;
        Ok(DraftResultDto {
            identifier: identifier.to_owned(),
            body: self.body.clone(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: observation(DraftClass::AcceptedUnverified, false),
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
            let dir = std::env::temp_dir().join(format!("bmdock-t14-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }
    }

    impl Drop for TempOwned {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    fn owned_store() -> (TempOwned, FixtureDraftStore) {
        let owned = TempOwned::create();
        let store = FixtureDraftStore::new(owned.dir.clone()).unwrap();
        (owned, store)
    }

    #[test]
    fn empty_store_load_is_empty_session_save_is_unsupported() {
        let store = EmptyDraftStore;
        let loaded = store.load_draft("welcome").unwrap();
        assert_eq!(loaded.identifier, "welcome");
        assert!(loaded.body.is_empty());
        assert!(!loaded.files_written);
        assert!(!loaded.engine_persisted);
        assert!(!loaded.scanned_user_obsidian_vault);
        assert!(!loaded.scanned_user_basic_memory_home);
        assert_eq!(loaded.observation.classified_as, DraftClass::Empty);
        assert!(!loaded.observation.disk_verified);
        let error = store
            .save_draft("welcome", "# 中文草稿\n\n参见 [[欢迎]]。\n")
            .unwrap_err();
        assert_eq!(
            error,
            LibraryError::unsupported(UNSUPPORTED_DRAFT_UNAVAILABLE)
        );
        assert_ne!(
            error,
            LibraryError::unsupported(crate::library::UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
        assert_ne!(
            error,
            LibraryError::unsupported(crate::backups::UNSUPPORTED_BACKUP_UNAVAILABLE)
        );
    }

    #[test]
    fn save_observes_physical_markdown_not_envelope() {
        let (_owned, store) = owned_store();
        let body = "# 中文草稿\n\n这是 BMDock 自有草稿正文。参见 [[欢迎]]。\n";
        let saved = store.save_draft("welcome", body).unwrap();
        let dest = store.root().join("welcome.md");
        assert!(
            dest.is_file(),
            "save must create a physical owned draft file"
        );
        let disk = fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert_eq!(saved.body, body);
        assert!(saved.files_written);
        assert!(!saved.engine_persisted);
        assert!(saved.observation.disk_verified);
        assert!(saved.observation.envelope_is_not_disk_proof);
        assert_eq!(saved.observation.classified_as, DraftClass::DiskVerified);

        let loaded = store.load_draft("welcome").unwrap();
        assert_eq!(loaded.body, disk);
        assert_eq!(loaded.body, body);
        assert!(!loaded.engine_persisted);
        assert!(loaded.observation.disk_verified);
        assert_eq!(loaded.observation.classified_as, DraftClass::DiskVerified);
    }

    #[test]
    fn envelope_saved_text_is_not_disk_proof() {
        let store = EnvelopeDraftStore {
            body: "saved".to_owned(),
        };
        let saved = store.save_draft("welcome", "ignored").unwrap();
        assert_eq!(saved.body, "saved");
        assert!(!saved.files_written);
        assert!(!saved.engine_persisted);
        assert!(!saved.observation.disk_verified);
        assert!(saved.observation.envelope_is_not_disk_proof);
        assert_eq!(
            saved.observation.classified_as,
            DraftClass::AcceptedUnverified
        );
        let loaded = store.load_draft("welcome").unwrap();
        assert!(!loaded.files_written);
        assert!(!loaded.engine_persisted);
        assert_eq!(
            loaded.observation.classified_as,
            DraftClass::AcceptedUnverified
        );
    }

    #[test]
    fn filesystem_draft_identifier_is_policy() {
        for identifier in [
            r"C:\Users\someone\vault\note.md",
            r"%APPDATA%\Obsidian\vault",
            "/home/someone/.basic-memory/note",
            "../secret",
        ] {
            assert_eq!(
                reject_draft_identifier(identifier).unwrap_err(),
                LibraryError::policy(POLICY_DRAFT_IDENTIFIER)
            );
            assert_eq!(
                EmptyDraftStore.save_draft(identifier, "body").unwrap_err(),
                LibraryError::policy(POLICY_DRAFT_IDENTIFIER)
            );
            assert_eq!(
                EmptyDraftStore.load_draft(identifier).unwrap_err(),
                LibraryError::policy(POLICY_DRAFT_IDENTIFIER)
            );
        }
        assert_eq!(
            reject_draft_identifier(""),
            Err(LibraryError::schema(SCHEMA_DRAFT_IDENTIFIER))
        );
        reject_draft_identifier("welcome").unwrap();
    }

    #[test]
    fn forbidden_draft_roots_are_policy() {
        for path in [
            Path::new(r"C:\Users\someone\AppData\Roaming\Obsidian"),
            Path::new(r"C:\Users\someone\.basic-memory"),
            Path::new("/home/someone/.basic-memory"),
            Path::new("%APPDATA%\\Obsidian"),
        ] {
            assert!(
                draft_root_is_forbidden(path),
                "expected forbidden draft root: {}",
                path.display()
            );
            assert_eq!(
                reject_forbidden_draft_root(path).unwrap_err(),
                LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT)
            );
        }
        let forbidden_store =
            FixtureDraftStore::new(PathBuf::from(r"C:\Users\someone\Documents\Obsidian"));
        assert_eq!(
            forbidden_store.unwrap_err(),
            LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT)
        );
        let unmarked = FixtureDraftStore::new(std::env::temp_dir().join("other-drafts"));
        assert_eq!(
            unmarked.unwrap_err(),
            LibraryError::policy(POLICY_FORBIDDEN_DRAFT_ROOT)
        );
    }

    #[test]
    fn missing_fixture_draft_is_empty_session_not_user_vault() {
        let (_owned, store) = owned_store();
        let loaded = store.load_draft("missing").unwrap();
        assert!(loaded.body.is_empty());
        assert!(!loaded.files_written);
        assert!(!loaded.engine_persisted);
        assert_eq!(loaded.observation.classified_as, DraftClass::Empty);
        assert!(!loaded.scanned_user_obsidian_vault);
    }
}
