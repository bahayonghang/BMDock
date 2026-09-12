//! T18 editor content safety: note and draft bodies are opaque UTF-8 text.
//! Markdown/HTML must never execute. Fixture persist writes and re-reads exact
//! bytes, including CRLF. Replacing CRLF with LF is not `disk_verified`.
//! Native GUI / IME typing is UNVERIFIED; cargo test is not a native window.

use std::fs;
use std::path::Path;

pub const HTML_NEVER_EXECUTED: &str = "markdown/html bodies are never executed";
pub const NATIVE_GUI_IME_UNVERIFIED: &str =
    "native GUI / IME session remains UNVERIFIED; cargo test and npm build are not a typed native window";
pub const T39_HELP_NOT_THIS_TASK: &str = "T39 help completeness is not claimed by T18";
pub const CRLF_LOSS_IS_NOT_DISK_VERIFIED: &str =
    "CRLF loss from LF normalization is not disk_verified";

pub const UNSAFE_HTML_WIKI_CRLF_BODY: &str =
    "<p>欢迎</p>\r\n<script>alert(1)</script>\r\n<img src=x onerror=\"alert(1)\">\r\n参见 [[欢迎]]。\r\n";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineEndingClass {
    None,
    Lf,
    Crlf,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContentSafetyClass {
    pub unsafe_html_present: bool,
    pub executed: bool,
    pub line_endings: LineEndingClass,
}

pub fn classify_body(body: &str) -> ContentSafetyClass {
    ContentSafetyClass {
        unsafe_html_present: unsafe_html_present(body),
        executed: false,
        line_endings: classify_line_endings(body),
    }
}

pub fn unsafe_html_present(body: &str) -> bool {
    let lower = body.to_ascii_lowercase();
    lower.contains("<script")
        || lower.contains("onerror=")
        || lower.contains("onload=")
        || lower.contains("javascript:")
        || lower.contains("<iframe")
}

pub fn classify_line_endings(body: &str) -> LineEndingClass {
    let has_crlf = body.contains("\r\n");
    let without_crlf = body.replace("\r\n", "");
    let has_other = without_crlf.contains('\n') || without_crlf.contains('\r');
    match (has_crlf, has_other) {
        (false, false) => LineEndingClass::None,
        (true, false) => LineEndingClass::Crlf,
        (false, true) => LineEndingClass::Lf,
        (true, true) => LineEndingClass::Mixed,
    }
}

pub fn persist_exact_utf8(path: &Path, body: &str) -> std::io::Result<()> {
    fs::write(path, body.as_bytes())
}

pub fn read_exact_bytes(path: &Path) -> std::io::Result<Vec<u8>> {
    fs::read(path)
}

pub fn read_exact_text(path: &Path) -> std::io::Result<String> {
    let bytes = read_exact_bytes(path)?;
    String::from_utf8(bytes)
        .map_err(|err| std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

pub fn bytes_match_body(disk: &[u8], expected: &str) -> bool {
    disk == expected.as_bytes()
}

pub fn crlf_normalized_to_lf(expected: &str, disk: &[u8]) -> bool {
    if bytes_match_body(disk, expected) || !expected.contains("\r\n") {
        return false;
    }
    disk == expected.replace("\r\n", "\n").as_bytes()
}

pub fn disk_verified_from_bytes(expected: &str, disk: &[u8]) -> bool {
    bytes_match_body(disk, expected)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static SEQ: AtomicU64 = AtomicU64::new(0);

    struct TempFile {
        dir: PathBuf,
    }

    impl TempFile {
        fn create() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let seq = SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("bmdock-t18-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }

        fn path(&self) -> PathBuf {
            self.dir.join("welcome.md")
        }
    }

    impl Drop for TempFile {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    #[test]
    fn unsafe_html_and_wiki_link_are_classified_not_executed() {
        let body = UNSAFE_HTML_WIKI_CRLF_BODY;
        let class = classify_body(body);
        assert!(class.unsafe_html_present);
        assert!(!class.executed);
        assert_eq!(class.line_endings, LineEndingClass::Crlf);
        assert!(body.contains("<script>"));
        assert!(body.contains("onerror="));
        assert!(body.contains("[[欢迎]]"));
        assert!(HTML_NEVER_EXECUTED.contains("never executed"));
        assert!(NATIVE_GUI_IME_UNVERIFIED.contains("UNVERIFIED"));
        assert!(T39_HELP_NOT_THIS_TASK.contains("T39"));
        let safe = classify_body("# 中文\n\n参见 [[欢迎]]。\n");
        assert!(!safe.unsafe_html_present);
        assert!(!safe.executed);
        assert_eq!(safe.line_endings, LineEndingClass::Lf);
    }

    #[test]
    fn crlf_persists_as_exact_bytes_normalized_lf_is_not_disk_verified() {
        let owned = TempFile::create();
        let path = owned.path();
        let body = UNSAFE_HTML_WIKI_CRLF_BODY;
        persist_exact_utf8(&path, body).unwrap();
        let disk = read_exact_bytes(&path).unwrap();
        assert_eq!(disk, body.as_bytes());
        assert!(disk.windows(2).any(|pair| pair == b"\r\n"));
        assert_eq!(read_exact_text(&path).unwrap(), body);
        assert!(disk_verified_from_bytes(body, &disk));
        assert!(!crlf_normalized_to_lf(body, &disk));

        let normalized = body.replace("\r\n", "\n");
        fs::write(&path, normalized.as_bytes()).unwrap();
        let lost = read_exact_bytes(&path).unwrap();
        assert_ne!(lost, body.as_bytes());
        assert!(crlf_normalized_to_lf(body, &lost));
        assert!(!disk_verified_from_bytes(body, &lost));
        assert!(CRLF_LOSS_IS_NOT_DISK_VERIFIED.contains("not disk_verified"));
    }
}
