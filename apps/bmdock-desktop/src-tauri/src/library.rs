#[cfg(test)]
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 64;
pub const UNSUPPORTED_LIBRARY_UNAVAILABLE: &str = "engine/library unavailable";
pub const SCHEMA_INVALID_CURSOR: &str = "invalid or repeated pagination cursor";
pub const SCHEMA_PAGE_SIZE: &str = "page_size must be bounded and greater than 0";
pub const UNSUPPORTED_TRUNCATED: &str = "truncated inventory is not a success";
pub const POLICY_FILESYSTEM_IDENTIFIER: &str =
    "Note identifiers are permalinks, not user vault filesystem paths";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LibraryError {
    Schema(String),
    Policy(String),
    Unsupported(String),
}

impl LibraryError {
    pub fn schema(message: impl Into<String>) -> Self {
        Self::Schema(message.into())
    }

    pub fn policy(message: impl Into<String>) -> Self {
        Self::Policy(message.into())
    }

    pub fn unsupported(message: impl Into<String>) -> Self {
        Self::Unsupported(message.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TreeEntryKind {
    Note,
    Directory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreeEntryDto {
    pub identifier: String,
    pub title: String,
    pub kind: TreeEntryKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TreePageDto {
    pub entries: Vec<TreeEntryDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ObservationClass {
    Empty,
    BodyMatchesDisk,
    AcceptedUnverified,
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteObservationDto {
    pub classified_as: ObservationClass,
    pub disk_verified: bool,
    pub envelope_is_not_disk_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteReadDto {
    pub title: String,
    pub identifier: String,
    pub body: String,
    pub observation: NoteObservationDto,
}

pub trait NoteLibrary: Send + Sync {
    fn list_tree(&self, cursor: Option<&str>, page_size: u32) -> Result<TreePageDto, LibraryError>;

    fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError>;
}

#[derive(Debug, Default)]
pub struct EmptyLibrary;

impl NoteLibrary for EmptyLibrary {
    fn list_tree(&self, cursor: Option<&str>, page_size: u32) -> Result<TreePageDto, LibraryError> {
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(TreePageDto {
            entries: Vec::new(),
            next_cursor: None,
            page: 1,
            truncated: false,
        })
    }

    fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError> {
        reject_filesystem_identifier(identifier)?;
        if identifier.is_empty() {
            return Err(LibraryError::schema("note identifier is required"));
        }
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct FixtureLibrary {
    root: PathBuf,
}

#[cfg(test)]
impl FixtureLibrary {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    fn collect_entries(&self) -> Result<Vec<TreeEntryDto>, LibraryError> {
        let reader = fs::read_dir(&self.root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
        let mut entries = Vec::new();
        for item in reader {
            let item = item.map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            let file_type = item
                .file_type()
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            if file_type.is_symlink() {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            if !file_type.is_file() {
                continue;
            }
            let name = item.file_name();
            let name = name.to_string_lossy();
            if !name.ends_with(".md") {
                continue;
            }
            let identifier = name.trim_end_matches(".md").to_string();
            if looks_like_filesystem_path(&identifier) {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            let path = item.path();
            let title = fs::read_to_string(&path)
                .ok()
                .and_then(|body| title_from_markdown(&body))
                .unwrap_or_else(|| identifier.clone());
            entries.push(TreeEntryDto {
                identifier,
                title,
                kind: TreeEntryKind::Note,
            });
        }
        entries.sort_by(|left, right| left.identifier.cmp(&right.identifier));
        Ok(entries)
    }

    fn resolve_note_path(&self, identifier: &str) -> Result<PathBuf, LibraryError> {
        reject_filesystem_identifier(identifier)?;
        if identifier.is_empty() {
            return Err(LibraryError::schema("note identifier is required"));
        }
        if identifier
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == ".." || part.contains('\\'))
        {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        let mut relative = PathBuf::new();
        for part in identifier.split('/') {
            relative.push(part);
        }
        relative.set_extension("md");
        if relative.is_absolute() {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        let candidate = self.root.join(relative);
        let root = self
            .root
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let resolved = candidate
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        if !resolved.starts_with(&root) {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        Ok(resolved)
    }
}

#[cfg(test)]
impl NoteLibrary for FixtureLibrary {
    fn list_tree(&self, cursor: Option<&str>, page_size: u32) -> Result<TreePageDto, LibraryError> {
        let page_size = bound_page_size(Some(page_size))?;
        let entries = self.collect_entries()?;
        let offset = parse_offset(cursor, entries.len())?;
        let size = page_size as usize;
        let end = offset.saturating_add(size).min(entries.len());
        let page_entries = entries[offset..end].to_vec();
        let next_cursor = if end < entries.len() {
            Some(end.to_string())
        } else {
            None
        };
        let page = u32::try_from(offset / size)
            .map_err(|_| LibraryError::schema(SCHEMA_INVALID_CURSOR))?
            .saturating_add(1);
        Ok(TreePageDto {
            entries: page_entries,
            next_cursor,
            page,
            truncated: false,
        })
    }

    fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError> {
        let path = self.resolve_note_path(identifier)?;
        let disk = fs::read_to_string(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let title = title_from_markdown(&disk).unwrap_or_else(|| identifier.to_owned());
        Ok(NoteReadDto {
            title,
            identifier: identifier.to_owned(),
            body: disk.clone(),
            observation: observation_from_disk(&disk, &disk),
        })
    }
}

pub fn bound_page_size(page_size: Option<u32>) -> Result<u32, LibraryError> {
    match page_size {
        None => Ok(DEFAULT_PAGE_SIZE),
        Some(0) => Err(LibraryError::schema(SCHEMA_PAGE_SIZE)),
        Some(size) if size > MAX_PAGE_SIZE => Err(LibraryError::schema(SCHEMA_PAGE_SIZE)),
        Some(size) => Ok(size),
    }
}

pub fn validate_request_cursor(cursor: Option<&str>) -> Result<Option<&str>, LibraryError> {
    match cursor {
        None => Ok(None),
        Some("") => Err(LibraryError::schema(SCHEMA_INVALID_CURSOR)),
        Some(value) => Ok(Some(value)),
    }
}

pub fn accept_tree_page(
    request_cursor: Option<&str>,
    page: TreePageDto,
) -> Result<TreePageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if let Some(next) = page.next_cursor.as_deref() {
        if next.is_empty() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        if request_cursor == Some(next) {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
    }
    Ok(page)
}

pub fn reject_filesystem_identifier(identifier: &str) -> Result<(), LibraryError> {
    if looks_like_filesystem_path(identifier) {
        Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER))
    } else {
        Ok(())
    }
}

#[cfg(test)]
pub fn observation_from_disk(body: &str, disk: &str) -> NoteObservationDto {
    if body == disk {
        NoteObservationDto {
            classified_as: ObservationClass::BodyMatchesDisk,
            disk_verified: true,
            envelope_is_not_disk_proof: true,
        }
    } else {
        NoteObservationDto {
            classified_as: ObservationClass::Unclassified,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        }
    }
}

#[cfg(test)]
pub fn observation_from_envelope_only() -> NoteObservationDto {
    NoteObservationDto {
        classified_as: ObservationClass::AcceptedUnverified,
        disk_verified: false,
        envelope_is_not_disk_proof: true,
    }
}

#[cfg(test)]
pub fn title_from_markdown(body: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(title) = trimmed.strip_prefix("# ") {
            let title = title.trim();
            if !title.is_empty() {
                return Some(title.to_owned());
            }
        }
        break;
    }
    None
}

pub fn looks_like_filesystem_path(value: &str) -> bool {
    let value = value.trim();
    if value.is_empty() {
        return false;
    }
    let bytes = value.as_bytes();
    value.contains("..")
        || value.contains('\\')
        || Path::new(value).is_absolute()
        || value.starts_with('/')
        || (bytes.len() >= 2 && bytes[1] == b':' && bytes[0].is_ascii_alphabetic())
        || value.contains("%APPDATA%")
        || value.contains("%USERPROFILE%")
        || value.contains(".obsidian")
        || value.contains(".basic-memory")
}

#[cfg(test)]
fn parse_offset(cursor: Option<&str>, len: usize) -> Result<usize, LibraryError> {
    match cursor {
        None => Ok(0),
        Some(raw) => {
            if raw.is_empty()
                || !raw.bytes().all(|byte| byte.is_ascii_digit())
                || (raw.starts_with('0') && raw != "0")
            {
                return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
            }
            let offset: usize = raw
                .parse()
                .map_err(|_| LibraryError::schema(SCHEMA_INVALID_CURSOR))?;
            if offset >= len && len > 0 {
                return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
            }
            if offset > len {
                return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
            }
            Ok(offset)
        }
    }
}

#[cfg(test)]
pub fn follow_tree_pages(
    library: &dyn NoteLibrary,
    page_size: u32,
) -> Result<(Vec<TreeEntryDto>, u32), LibraryError> {
    use std::collections::HashSet;

    let mut rows = Vec::new();
    let mut cursor: Option<String> = None;
    let mut seen = HashSet::new();
    for page in 1..=128u32 {
        let listed = library.list_tree(cursor.as_deref(), page_size)?;
        let listed = accept_tree_page(cursor.as_deref(), listed)?;
        rows.extend(listed.entries);
        match listed.next_cursor {
            None => return Ok((rows, page)),
            Some(next) => {
                if seen.contains(&next) {
                    return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
                }
                seen.insert(next.clone());
                cursor = Some(next);
            }
        }
    }
    Err(LibraryError::unsupported(
        "exceeded 128 pages; refusing partial inventory",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static FIXTURE_SEQ: AtomicU64 = AtomicU64::new(0);

    struct TempFixture {
        dir: PathBuf,
    }

    impl TempFixture {
        fn create() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("bmdock-t11-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }

        fn write_note(&self, identifier: &str, title: &str, body: &str) -> PathBuf {
            let path = self.dir.join(format!("{identifier}.md"));
            fs::write(&path, format!("# {title}\n\n{body}\n")).unwrap();
            path
        }
    }

    impl Drop for TempFixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    struct EnvelopeLibrary {
        body: String,
    }

    impl NoteLibrary for EnvelopeLibrary {
        fn list_tree(
            &self,
            cursor: Option<&str>,
            page_size: u32,
        ) -> Result<TreePageDto, LibraryError> {
            let _ = (cursor, page_size);
            Ok(TreePageDto {
                entries: vec![TreeEntryDto {
                    identifier: "envelope".to_owned(),
                    title: "envelope".to_owned(),
                    kind: TreeEntryKind::Note,
                }],
                next_cursor: None,
                page: 1,
                truncated: false,
            })
        }

        fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError> {
            Ok(NoteReadDto {
                title: identifier.to_owned(),
                identifier: identifier.to_owned(),
                body: self.body.clone(),
                observation: observation_from_envelope_only(),
            })
        }
    }

    struct TruncatingLibrary;

    impl NoteLibrary for TruncatingLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<TreePageDto, LibraryError> {
            Ok(TreePageDto {
                entries: vec![TreeEntryDto {
                    identifier: "partial".to_owned(),
                    title: "partial".to_owned(),
                    kind: TreeEntryKind::Note,
                }],
                next_cursor: None,
                page: 1,
                truncated: true,
            })
        }

        fn read_note(&self, _identifier: &str) -> Result<NoteReadDto, LibraryError> {
            Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
        }
    }

    struct LoopingLibrary;

    impl NoteLibrary for LoopingLibrary {
        fn list_tree(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<TreePageDto, LibraryError> {
            Ok(TreePageDto {
                entries: vec![TreeEntryDto {
                    identifier: "loop".to_owned(),
                    title: "loop".to_owned(),
                    kind: TreeEntryKind::Note,
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
            })
        }

        fn read_note(&self, _identifier: &str) -> Result<NoteReadDto, LibraryError> {
            Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
        }
    }

    #[test]
    fn empty_library_is_empty_state_not_user_vault() {
        let library = EmptyLibrary;
        let page = library.list_tree(None, DEFAULT_PAGE_SIZE).unwrap();
        assert!(page.entries.is_empty());
        assert_eq!(page.next_cursor, None);
        assert_eq!(page.page, 1);
        assert!(!page.truncated);
        let error = library.read_note("welcome").unwrap_err();
        assert_eq!(
            error,
            LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
    }

    #[test]
    fn empty_library_rejects_cursor_as_schema() {
        let error = EmptyLibrary
            .list_tree(Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(error, LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }

    #[test]
    fn page_size_zero_and_huge_are_schema() {
        assert_eq!(
            bound_page_size(Some(0)).unwrap_err(),
            LibraryError::schema(SCHEMA_PAGE_SIZE)
        );
        assert_eq!(
            bound_page_size(Some(MAX_PAGE_SIZE + 1)).unwrap_err(),
            LibraryError::schema(SCHEMA_PAGE_SIZE)
        );
        assert_eq!(bound_page_size(None).unwrap(), DEFAULT_PAGE_SIZE);
        assert_eq!(bound_page_size(Some(2)).unwrap(), 2);
    }

    #[test]
    fn fixture_pages_are_bounded_and_match_disk() {
        let fixture = TempFixture::create();
        for index in 1..=5 {
            fixture.write_note(
                &format!("note-{index:02}"),
                &format!("笔记 {index}"),
                "夹具正文 [[欢迎]]",
            );
        }
        let library = FixtureLibrary::new(fixture.dir.clone());
        let first = library.list_tree(None, 2).unwrap();
        assert_eq!(first.entries.len(), 2);
        assert_eq!(first.page, 1);
        assert!(!first.truncated);
        assert_eq!(first.next_cursor.as_deref(), Some("2"));
        assert_eq!(first.entries[0].identifier, "note-01");
        assert!(!first.entries[0].identifier.contains('\\'));
        assert!(!first.entries[0].identifier.contains(':'));

        let (all, pages) = follow_tree_pages(&library, 2).unwrap();
        assert_eq!(pages, 3);
        assert_eq!(all.len(), 5);
        assert_eq!(all[4].identifier, "note-05");

        let path = fixture.dir.join("note-01.md");
        let disk = fs::read_to_string(&path).unwrap();
        let note = library.read_note("note-01").unwrap();
        assert_eq!(note.body, disk);
        assert!(note.body.contains("夹具正文"));
        assert!(note.body.contains("[[欢迎]]"));
        assert_eq!(note.title, "笔记 1");
        assert_eq!(note.identifier, "note-01");
        assert!(note.observation.disk_verified);
        assert!(note.observation.envelope_is_not_disk_proof);
        assert_eq!(
            note.observation.classified_as,
            ObservationClass::BodyMatchesDisk
        );
    }

    #[test]
    fn invalid_cursor_and_truncation_fail_closed() {
        let fixture = TempFixture::create();
        fixture.write_note("only", "only", "body");
        let library = FixtureLibrary::new(fixture.dir.clone());
        assert_eq!(
            library.list_tree(Some("abc"), 2).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            library.list_tree(Some(""), 2).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            library.list_tree(Some("9"), 2).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            accept_tree_page(None, TruncatingLibrary.list_tree(None, 2).unwrap()).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping = LoopingLibrary.list_tree(Some("same"), 2).unwrap();
        assert_eq!(
            accept_tree_page(Some("same"), looping).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        let follow_loop = follow_tree_pages(&LoopingLibrary, 2).unwrap_err();
        assert_eq!(follow_loop, LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }

    #[test]
    fn envelope_success_is_not_disk_proof() {
        let library = EnvelopeLibrary {
            body: "Saved successfully".to_owned(),
        };
        let note = library.read_note("welcome").unwrap();
        assert!(!note.observation.disk_verified);
        assert!(note.observation.envelope_is_not_disk_proof);
        assert_eq!(
            note.observation.classified_as,
            ObservationClass::AcceptedUnverified
        );
        let mismatched = observation_from_disk("envelope", "disk");
        assert!(!mismatched.disk_verified);
        assert_eq!(mismatched.classified_as, ObservationClass::Unclassified);
    }

    #[test]
    fn filesystem_path_identifier_is_policy() {
        for identifier in [
            r"C:\Users\someone\vault\note.md",
            "/home/someone/.basic-memory/note",
            r"%APPDATA%\Obsidian\vault",
            "../secret",
        ] {
            assert!(looks_like_filesystem_path(identifier));
            assert_eq!(
                reject_filesystem_identifier(identifier).unwrap_err(),
                LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
            );
        }
        assert!(!looks_like_filesystem_path("welcome"));
        assert!(!looks_like_filesystem_path("folder/welcome"));
    }
}
