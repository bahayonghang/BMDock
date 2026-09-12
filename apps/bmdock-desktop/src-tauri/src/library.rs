#[cfg(test)]
use std::fs;
use std::path::Path;
#[cfg(test)]
use std::path::{Component, PathBuf};
#[cfg(test)]
use std::time::{Instant, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

pub const DEFAULT_PAGE_SIZE: u32 = 20;
pub const MAX_PAGE_SIZE: u32 = 64;
pub const UNSUPPORTED_LIBRARY_UNAVAILABLE: &str = "engine/library unavailable";
pub const SCHEMA_INVALID_CURSOR: &str = "invalid or repeated pagination cursor";
pub const SCHEMA_PAGE_SIZE: &str = "page_size must be bounded and greater than 0";
pub const SCHEMA_NOTE_IDENTIFIER: &str = "note identifier is required";
pub const SCHEMA_NOTE_TITLE: &str = "note title is required";
pub const SCHEMA_NOTE_DESTINATION: &str = "move destination is required";
pub const SCHEMA_MOVE_SAME_IDENTIFIER: &str = "move destination must differ from identifier";
pub const SCHEMA_SEARCH_QUERY: &str = "search query is required";
pub const SCHEMA_RECALL_K: &str = "k must be bounded and greater than 0";
pub const SCHEMA_INVALID_SCHEMA_ID: &str = "schema_id is required when provided";
pub const DEFAULT_SCHEMA_ID: &str = "note";
pub const SCHEMA_FIELD_TITLE: &str = "title";
pub const SCHEMA_FIELD_BODY: &str = "body";
pub const POLICY_FILESYSTEM_SCHEMA_ID: &str =
    "Schema ids are catalog identifiers, not user vault filesystem paths";
#[cfg(test)]
pub const CHINESE_RECALL_QUERIES: &[&str] = &["欢迎"];
#[cfg(test)]
pub const CHINESE_RECALL_PERMALINK: &str = "欢迎";
pub const SEMANTIC_DISABLED_REASON: &str =
    "semantic search is unavailable; semantic_enabled=false; no embedding backend; official semantic/model remain UNVERIFIED";
pub const UNSUPPORTED_TRUNCATED: &str = "truncated inventory is not a success";
pub const GRAPH_DEPTH: u32 = 1;
#[cfg(test)]
pub const PREVIEW_SNIPPET_LIMIT: usize = 240;
#[cfg(test)]
pub const PREVIEW_QUERY_PAD: usize = 80;
#[cfg(test)]
pub const LEXICAL_SCORE_TITLE: u32 = 2;
#[cfg(test)]
pub const LEXICAL_SCORE_BODY: u32 = 1;
#[cfg(test)]
pub const SEMANTIC_SCORE_DISABLED: u32 = 0;
#[cfg(test)]
pub const NATIVE_GUI_UNVERIFIED: &str =
    "native window freeze, WebView2 session, installer, and hosted CI remain UNVERIFIED";
#[cfg(test)]
pub const BOUNDED_HOST_EXPANSION: &str =
    "T20 proves bounded host expansion only; cargo test / npm build / UI copy are not native GUI";
#[cfg(test)]
pub const UNSUPPORTED_NOTE_MISSING: &str = "note identifier is not present in the fixture library";
#[cfg(test)]
pub const UNSUPPORTED_MOVE_DESTINATION_EXISTS: &str =
    "destination already exists; sequential dest-exists is unsupported and is not T16 same-target conflict";
pub const POLICY_FILESYSTEM_IDENTIFIER: &str =
    "Note identifiers are permalinks, not user vault filesystem paths";
pub const POLICY_FILESYSTEM_DESTINATION: &str =
    "Move destinations are permalinks, not user vault filesystem paths";
pub const POLICY_FILESYSTEM_QUERY: &str =
    "Search queries are lexical text, not user vault filesystem paths";
#[cfg(test)]
pub const ENGINE_GRAPH_NOT_OWNED: &str =
    "relations are derived from fixture markdown wiki-links, not a second database and not official engine graph MCP";
#[cfg(test)]
pub const ENGINE_SEARCH_NOT_OWNED: &str =
    "search is BMDock-owned fixture lexical matching, not a second database and not official engine search MCP";
#[cfg(test)]
pub const ENGINE_INSPECTOR_NOT_OWNED: &str =
    "inspect_search explains BMDock-owned fixture lexical search, not official engine semantic search, not an embedding backend, and not T24 recall";
#[cfg(test)]
pub const INSPECTOR_READ_ONLY: &str = "inspect_search is read-only; files_written=false";
#[cfg(test)]
pub const ENGINE_RECALL_NOT_OWNED: &str =
    "run_recall_benchmark is BMDock-owned fixture Chinese recall over disk gold, not official engine Chinese recall and not official search MCP";
#[cfg(test)]
pub const ENGINE_SCHEMA_NOT_OWNED: &str =
    "schema_validate is BMDock-owned fixture markdown/JSON-like frontmatter validation, not official MCP schema_validate / schema_infer / schema_diff";
#[cfg(test)]
pub const OFFICIAL_SCHEMA_MCP_UNVERIFIED: &str =
    "official schema_validate / schema_infer / schema_diff MCP remain UNVERIFIED";
#[cfg(test)]
pub const ENGINE_RESOURCES_NOT_OWNED: &str =
    "list_resources is BMDock-owned fixture markdown identifiers, not official MCP resources/list or resources/read";
#[cfg(test)]
pub const ENGINE_PROMPTS_NOT_OWNED: &str =
    "list_prompts is BMDock-owned fixture prompt sidecar templates, not official MCP prompts/list or prompts/get";
#[cfg(test)]
pub const OFFICIAL_RESOURCES_MCP_UNVERIFIED: &str =
    "official resources/list and resources/read MCP remain UNVERIFIED";
#[cfg(test)]
pub const OFFICIAL_PROMPTS_MCP_UNVERIFIED: &str =
    "official prompts/list and prompts/get MCP remain UNVERIFIED";
#[cfg(test)]
pub const ENGINE_TOOLS_NOT_OWNED: &str =
    "inspect_tools is a BMDock-owned comparison of the typed allowlist against one selected profile baseline, not live MCP tools/list and not call_tool";
#[cfg(test)]
pub const ENGINE_CLI_NOT_OWNED: &str =
    "list_cli_inventory is a BMDock-owned named leaf catalog from committed T01 artifacts, not live official CLI execution";
#[cfg(test)]
pub const OFFICIAL_TOOLS_MCP_UNVERIFIED: &str =
    "official MCP tool execution remains UNVERIFIED; unknown tools stay denied and are not auto-allowed";
#[cfg(test)]
pub const OFFICIAL_CLI_UNVERIFIED: &str =
    "live official CLI execution remains UNVERIFIED; catalog rows are not executed";
pub const SCHEMA_SOURCE_ID: &str = "source_id is required";
pub const POLICY_FILESYSTEM_SOURCE: &str =
    "Import source ids are BMDock-owned fixture identifiers, not user vault filesystem paths";
pub const ENGINE_IMPORT_NOT_OWNED: &str =
    "import_notes copies BMDock-owned fixture markdown into an owned fixture library, not official extras/document ingestion and not live CLI import";
#[cfg(test)]
pub const OFFICIAL_IMPORT_UNVERIFIED: &str =
    "official extras/document ingestion and live CLI import remain UNVERIFIED";
pub const SCHEMA_PROFILE_ID: &str = "profile_id must be release or main-preview";
pub const POLICY_FILESYSTEM_PROFILE: &str =
    "profile_id is release or main-preview, not a user vault filesystem path";
#[cfg(test)]
pub const SCHEMA_TOOL_NAME: &str = "tool names must be non-empty identifiers";
#[cfg(test)]
pub const SCHEMA_TOOL_BASELINE: &str = "tool baseline JSON is invalid";
pub const SCHEMA_CLI_LEAF: &str = "CLI leaves must be non-empty path segments";
pub const UNSUPPORTED_MIXED_PROFILES: &str =
    "release (21) and main-preview (27) must stay isolated; mixing or averaging is forbidden";
pub const UNSUPPORTED_CLI_BUCKET: &str =
    "CLI inventory must list leaf names; coarse buckets that hide missing commands are unsupported";
#[cfg(test)]
pub const TYPED_OFFICIAL_TOOL_ALLOWLIST: &[&str] = &[
    "delete_note",
    "edit_note",
    "move_note",
    "read_note",
    "schema_validate",
    "search_notes",
    "write_note",
];
#[cfg(test)]
pub const MAIN_PREVIEW_ONLY_TOOLS: &[&str] = &["cat", "find", "grep", "ls", "man", "tail"];
pub const SEARCH_IDENTITY: &str = "search";
pub const FETCH_IDENTITY: &str = "fetch";
pub const CALL_TOOL_IDENTITY: &str = "call_tool";
#[cfg(test)]
pub const PROMPT_SIDECAR_SUFFIX: &str = ".prompt.md";
#[cfg(test)]
pub const RECALL_NATIVE_UNVERIFIED: &str =
    "T24 records bounded in-process elapsed_ms only; cargo test / npm build / UI copy are not AC56 native proof";
#[cfg(test)]
pub const OFFICIAL_CHINESE_RECALL_UNVERIFIED: &str =
    "official engine Chinese recall remains UNVERIFIED";
#[cfg(test)]
pub const INSPECTOR_MODEL_UNVERIFIED: &str =
    "unknown or unavailable semantic model stays unclassified/unverified; model_loaded=false; never treat missing semantic as enabled success";
#[cfg(test)]
pub const ENGINE_CONTEXT_NOT_OWNED: &str =
    "context preview is BMDock-owned fixture markdown snippet, not official build_context MCP";
#[cfg(test)]
pub const ENGINE_ACTIVITY_NOT_OWNED: &str =
    "recent activity is BMDock-owned fixture markdown mtimes, not official recent_activity MCP";
#[cfg(test)]
pub const SEMANTIC_SEARCH_UNVERIFIED: &str =
    "official semantic search and embedding backend remain UNVERIFIED";
#[cfg(test)]
pub const OFFICIAL_SEARCH_MCP_UNVERIFIED: &str = "official search MCP remains UNVERIFIED";
#[cfg(test)]
pub const OFFICIAL_FETCH_MCP_UNVERIFIED: &str = "official fetch MCP remains UNVERIFIED";
#[cfg(test)]
pub const RECENT_ACTIVITY_MCP_UNVERIFIED: &str = "official recent_activity MCP remains UNVERIFIED";
#[cfg(test)]
pub const BUILD_CONTEXT_MCP_UNVERIFIED: &str = "official build_context MCP remains UNVERIFIED";
#[cfg(test)]
pub const POLICY_FORBIDDEN_LIBRARY_ROOT: &str =
    "Note CRUD cannot target user vaults, %APPDATA% Obsidian, or global Basic Memory config";

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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum NoteCrudClass {
    Empty,
    DiskVerified,
    AcceptedUnverified,
    Conflict,
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteCrudObservationDto {
    pub classified_as: NoteCrudClass,
    pub disk_verified: bool,
    pub envelope_is_not_disk_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteWriteDto {
    pub identifier: String,
    pub title: String,
    pub body: String,
    pub files_written: bool,
    pub engine_persisted: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub observation: NoteCrudObservationDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteEditDto {
    pub identifier: String,
    pub body: String,
    pub files_written: bool,
    pub engine_persisted: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub observation: NoteCrudObservationDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteMoveDto {
    pub identifier: String,
    pub destination: String,
    pub body: String,
    pub files_written: bool,
    pub engine_persisted: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub observation: NoteCrudObservationDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NoteDeleteDto {
    pub identifier: String,
    pub files_written: bool,
    pub engine_persisted: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub observation: NoteCrudObservationDto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RelationTargetClass {
    Present,
    Empty,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelationDto {
    pub identifier: String,
    pub classified_as: RelationTargetClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RelationListDto {
    pub identifier: String,
    pub relations: Vec<RelationDto>,
    pub observation: NoteCrudObservationDto,
    pub engine_graph: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_relation_list(identifier: &str) -> RelationListDto {
    RelationListDto {
        identifier: identifier.to_owned(),
        relations: Vec::new(),
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_graph: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum GraphNodeClass {
    Present,
    Empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphNodeDto {
    pub identifier: String,
    pub classified_as: GraphNodeClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphEdgeDto {
    pub source: String,
    pub target: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GraphPageDto {
    pub identifier: String,
    pub nodes: Vec<GraphNodeDto>,
    pub edges: Vec<GraphEdgeDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_graph: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
    pub depth: u32,
}

pub fn empty_graph_page(identifier: &str) -> GraphPageDto {
    GraphPageDto {
        identifier: identifier.to_owned(),
        nodes: Vec::new(),
        edges: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_graph: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
        depth: GRAPH_DEPTH,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchHitDto {
    pub identifier: String,
    pub lexical_score: u32,
    pub semantic_score: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchPageDto {
    pub query: String,
    pub hits: Vec<SearchHitDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub semantic_enabled: bool,
    pub engine_search: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_search_page(query: &str) -> SearchPageDto {
    SearchPageDto {
        query: query.to_owned(),
        hits: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        semantic_enabled: false,
        engine_search: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EmbeddingBackend {
    None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ModelClass {
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchInspectorDto {
    pub query: String,
    pub identifier: Option<String>,
    pub hits: Vec<SearchHitDto>,
    pub observation: NoteCrudObservationDto,
    pub semantic_enabled: bool,
    pub model_id: Option<String>,
    pub model_loaded: bool,
    pub embedding_backend: EmbeddingBackend,
    pub model_class: ModelClass,
    pub engine_search: bool,
    pub files_written: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub semantic_disabled_reason: String,
}

pub fn empty_search_inspector(query: &str, identifier: Option<&str>) -> SearchInspectorDto {
    SearchInspectorDto {
        query: query.to_owned(),
        identifier: identifier
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        hits: Vec::new(),
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        semantic_enabled: false,
        model_id: None,
        model_loaded: false,
        embedding_backend: EmbeddingBackend::None,
        model_class: ModelClass::Unclassified,
        engine_search: false,
        files_written: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        semantic_disabled_reason: SEMANTIC_DISABLED_REASON.to_owned(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecallQueryDto {
    pub query: String,
    pub relevant: Vec<String>,
    pub hits: Vec<String>,
    pub retrieved_relevant: u32,
    pub relevant_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecallBenchmarkDto {
    pub k: u32,
    pub query_count: u32,
    pub recall_hits: u32,
    pub recall_relevant: u32,
    pub queries: Vec<RecallQueryDto>,
    pub chinese_permalinks: Vec<String>,
    pub search_elapsed_ms: u64,
    pub expand_elapsed_ms: u64,
    pub observation: NoteCrudObservationDto,
    pub semantic_enabled: bool,
    pub engine_search: bool,
    pub native_gui: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_recall_benchmark(k: u32) -> RecallBenchmarkDto {
    RecallBenchmarkDto {
        k,
        query_count: 0,
        recall_hits: 0,
        recall_relevant: 0,
        queries: Vec::new(),
        chinese_permalinks: Vec::new(),
        search_elapsed_ms: 0,
        expand_elapsed_ms: 0,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        semantic_enabled: false,
        engine_search: false,
        native_gui: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SchemaVerdict {
    Valid,
    Invalid,
    Empty,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchemaValidateDto {
    pub identifier: String,
    pub schema_id: String,
    pub verdict: SchemaVerdict,
    pub required_fields: Vec<String>,
    pub missing_fields: Vec<String>,
    pub observed_title: bool,
    pub observed_body: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_schema: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn owned_schema_required_fields() -> Vec<String> {
    vec![SCHEMA_FIELD_TITLE.to_owned(), SCHEMA_FIELD_BODY.to_owned()]
}

pub fn empty_schema_report(identifier: &str, schema_id: &str) -> SchemaValidateDto {
    SchemaValidateDto {
        identifier: identifier.to_owned(),
        schema_id: schema_id.to_owned(),
        verdict: SchemaVerdict::Empty,
        required_fields: owned_schema_required_fields(),
        missing_fields: Vec::new(),
        observed_title: false,
        observed_body: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_schema: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

pub fn unsupported_schema_report(identifier: &str, schema_id: &str) -> SchemaValidateDto {
    SchemaValidateDto {
        identifier: identifier.to_owned(),
        schema_id: schema_id.to_owned(),
        verdict: SchemaVerdict::Unsupported,
        required_fields: Vec::new(),
        missing_fields: Vec::new(),
        observed_title: false,
        observed_body: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Unclassified,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_schema: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextPreviewDto {
    pub identifier: String,
    pub query: Option<String>,
    pub snippet: String,
    pub executed: bool,
    pub unsafe_html_present: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_context: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_context_preview(identifier: &str, query: Option<&str>) -> ContextPreviewDto {
    ContextPreviewDto {
        identifier: identifier.to_owned(),
        query: query
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned),
        snippet: String::new(),
        executed: false,
        unsafe_html_present: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_context: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivityEntryDto {
    pub identifier: String,
    pub observed_mtime: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActivityPageDto {
    pub entries: Vec<ActivityEntryDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_activity: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_activity_page() -> ActivityPageDto {
    ActivityPageDto {
        entries: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_activity: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourceEntryDto {
    pub identifier: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResourcePageDto {
    pub entries: Vec<ResourceEntryDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_resources: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_resource_page() -> ResourcePageDto {
    ResourcePageDto {
        entries: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_resources: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptEntryDto {
    pub identifier: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromptPageDto {
    pub entries: Vec<PromptEntryDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_prompts: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_prompt_page() -> PromptPageDto {
    PromptPageDto {
        entries: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_prompts: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ToolAdmission {
    Allowlisted,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InspectedToolDto {
    pub name: String,
    pub identity: String,
    pub admission: ToolAdmission,
    pub live_execution: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolInspectionDto {
    pub profile_id: String,
    pub expected_tool_count: u32,
    pub tools: Vec<InspectedToolDto>,
    pub mixed_profiles: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_tools: bool,
    pub call_tool_allowed: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_tool_inspection(profile_id: &str) -> ToolInspectionDto {
    ToolInspectionDto {
        profile_id: profile_id.to_owned(),
        expected_tool_count: 0,
        tools: Vec::new(),
        mixed_profiles: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_tools: false,
        call_tool_allowed: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliLeafDto {
    pub path: Vec<String>,
    pub executed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CliInventoryDto {
    pub profile_id: String,
    pub leaves: Vec<CliLeafDto>,
    pub next_cursor: Option<String>,
    pub page: u32,
    pub truncated: bool,
    pub observation: NoteCrudObservationDto,
    pub engine_cli: bool,
    pub executed: bool,
    pub mixed_profiles: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
    pub files_written: bool,
}

pub fn empty_cli_inventory(profile_id: &str) -> CliInventoryDto {
    CliInventoryDto {
        profile_id: profile_id.to_owned(),
        leaves: Vec::new(),
        next_cursor: None,
        page: 1,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: NoteCrudClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_cli: false,
        executed: false,
        mixed_profiles: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportedFileDto {
    pub identifier: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ImportClass {
    Empty,
    DiskVerified,
    AcceptedUnverified,
    Unclassified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportObservationDto {
    pub classified_as: ImportClass,
    pub disk_verified: bool,
    pub envelope_is_not_disk_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImportResultDto {
    pub source_id: String,
    pub files: Vec<ImportedFileDto>,
    pub files_written: bool,
    pub observation: ImportObservationDto,
    pub engine_import: bool,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
}

pub fn empty_import_result(source_id: &str) -> ImportResultDto {
    ImportResultDto {
        source_id: source_id.to_owned(),
        files: Vec::new(),
        files_written: false,
        observation: ImportObservationDto {
            classified_as: ImportClass::Empty,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
        engine_import: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
    }
}

pub fn reject_source_id(source_id: &str) -> Result<(), LibraryError> {
    if source_id.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_SOURCE_ID));
    }
    if looks_like_filesystem_path(source_id) || source_id.contains('/') {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_SOURCE));
    }
    Ok(())
}

pub fn accept_import_result(report: ImportResultDto) -> Result<ImportResultDto, LibraryError> {
    if report.engine_import {
        return Err(LibraryError::unsupported(ENGINE_IMPORT_NOT_OWNED));
    }
    if report.scanned_user_obsidian_vault || report.scanned_user_basic_memory_home {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_SOURCE));
    }
    if report
        .files
        .iter()
        .any(|file| looks_like_filesystem_path(&file.identifier) || file.identifier.contains('/'))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
    }
    let mut report = report;
    report.observation.envelope_is_not_disk_proof = true;
    if !report.files_written
        && (report.observation.disk_verified
            || report.observation.classified_as == ImportClass::DiskVerified)
    {
        report.observation.disk_verified = false;
        if report.observation.classified_as == ImportClass::DiskVerified {
            report.observation.classified_as = ImportClass::AcceptedUnverified;
        }
    }
    Ok(report)
}

pub fn parse_profile_id(value: &str) -> Result<&'static str, LibraryError> {
    if looks_like_filesystem_path(value) {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_PROFILE));
    }
    match value {
        "release" => Ok("release"),
        "main-preview" => Ok("main-preview"),
        _ => Err(LibraryError::schema(SCHEMA_PROFILE_ID)),
    }
}

#[cfg(test)]
pub fn tool_admission(name: &str) -> ToolAdmission {
    if name == SEARCH_IDENTITY || name == FETCH_IDENTITY || name == CALL_TOOL_IDENTITY {
        return ToolAdmission::Denied;
    }
    if TYPED_OFFICIAL_TOOL_ALLOWLIST.contains(&name) {
        ToolAdmission::Allowlisted
    } else {
        ToolAdmission::Denied
    }
}

#[cfg(test)]
pub fn inspect_tools_from_names(
    profile_id: &str,
    expected_tools: &[String],
    classified: NoteCrudClass,
    disk_verified: bool,
) -> Result<ToolInspectionDto, LibraryError> {
    let profile_id = parse_profile_id(profile_id)?;
    if expected_tools.len() >= 48 {
        return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
    }
    let mut seen: Vec<&str> = Vec::new();
    for name in expected_tools {
        if name.is_empty() {
            return Err(LibraryError::schema(SCHEMA_TOOL_NAME));
        }
        if looks_like_filesystem_path(name) {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        if seen.contains(&name.as_str()) {
            return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
        }
        seen.push(name);
    }
    let has_preview_only = expected_tools
        .iter()
        .any(|name| MAIN_PREVIEW_ONLY_TOOLS.contains(&name.as_str()));
    if profile_id == "release" && has_preview_only {
        return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
    }
    let tools = expected_tools
        .iter()
        .map(|name| InspectedToolDto {
            name: name.clone(),
            identity: name.clone(),
            admission: tool_admission(name),
            live_execution: false,
        })
        .collect::<Vec<_>>();
    let expected_tool_count = u32::try_from(tools.len())
        .map_err(|_| LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES))?;
    Ok(ToolInspectionDto {
        profile_id: profile_id.to_owned(),
        expected_tool_count,
        tools,
        mixed_profiles: false,
        observation: NoteCrudObservationDto {
            classified_as: classified,
            disk_verified,
            envelope_is_not_disk_proof: true,
        },
        engine_tools: false,
        call_tool_allowed: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    })
}

pub fn reject_coarse_cli_leaves(leaves: &[Vec<String>]) -> Result<(), LibraryError> {
    for (index, leaf) in leaves.iter().enumerate() {
        if leaf.is_empty() || leaf.iter().any(|part| part.is_empty()) {
            return Err(LibraryError::schema(SCHEMA_CLI_LEAF));
        }
        if leaf
            .iter()
            .any(|part| looks_like_filesystem_path(part) || part.contains('/'))
        {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        for (other_index, other) in leaves.iter().enumerate() {
            if index == other_index {
                continue;
            }
            if other.len() > leaf.len() && other.starts_with(leaf.as_slice()) {
                return Err(LibraryError::unsupported(UNSUPPORTED_CLI_BUCKET));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
pub fn cli_inventory_from_leaves(
    profile_id: &str,
    leaves: Vec<Vec<String>>,
    cursor: Option<&str>,
    page_size: u32,
    classified: NoteCrudClass,
    disk_verified: bool,
) -> Result<CliInventoryDto, LibraryError> {
    let profile_id = parse_profile_id(profile_id)?;
    let page_size = bound_page_size(Some(page_size))?;
    reject_coarse_cli_leaves(&leaves)?;
    let offset = parse_offset(cursor, leaves.len())?;
    let size = page_size as usize;
    let end = offset.saturating_add(size).min(leaves.len());
    let page_leaves = leaves[offset..end]
        .iter()
        .map(|path| CliLeafDto {
            path: path.clone(),
            executed: false,
        })
        .collect::<Vec<_>>();
    let next_cursor = if end < leaves.len() {
        Some(end.to_string())
    } else {
        None
    };
    let page = u32::try_from(offset / size)
        .map_err(|_| LibraryError::schema(SCHEMA_INVALID_CURSOR))?
        .saturating_add(1);
    let (classified, disk_verified) = if page_leaves.is_empty() {
        (NoteCrudClass::Empty, false)
    } else {
        (classified, disk_verified)
    };
    Ok(CliInventoryDto {
        profile_id: profile_id.to_owned(),
        leaves: page_leaves,
        next_cursor,
        page,
        truncated: false,
        observation: NoteCrudObservationDto {
            classified_as: classified,
            disk_verified,
            envelope_is_not_disk_proof: true,
        },
        engine_cli: false,
        executed: false,
        mixed_profiles: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        files_written: false,
    })
}

#[cfg(test)]
pub fn is_prompt_sidecar_name(name: &str) -> bool {
    name.ends_with(PROMPT_SIDECAR_SUFFIX)
}

#[cfg(test)]
pub fn preview_snippet(body: &str, query: Option<&str>) -> String {
    let query = query.map(str::trim).filter(|value| !value.is_empty());
    if let Some(needle) = query {
        if let Some(index) = body.find(needle) {
            let start = floor_char_boundary(body, index.saturating_sub(PREVIEW_QUERY_PAD));
            let end = ceil_char_boundary(
                body,
                (index + needle.len())
                    .saturating_add(PREVIEW_QUERY_PAD)
                    .min(body.len()),
            );
            return body[start..end].to_owned();
        }
    }
    prefix_snippet(body, PREVIEW_SNIPPET_LIMIT)
}

#[cfg(test)]
fn floor_char_boundary(body: &str, mut index: usize) -> usize {
    if index >= body.len() {
        return body.len();
    }
    while index > 0 && !body.is_char_boundary(index) {
        index -= 1;
    }
    index
}

#[cfg(test)]
fn ceil_char_boundary(body: &str, mut index: usize) -> usize {
    if index >= body.len() {
        return body.len();
    }
    while index < body.len() && !body.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[cfg(test)]
fn prefix_snippet(body: &str, max_bytes: usize) -> String {
    let end = ceil_char_boundary(body, max_bytes.min(body.len()));
    body[..end].to_owned()
}

#[cfg(test)]
pub fn extract_wiki_link_identifiers(body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut rest = body;
    while let Some(start) = rest.find("[[") {
        let after = &rest[start + 2..];
        match after.find("]]") {
            None => break,
            Some(end) => {
                let inner = after[..end].trim();
                let identifier = inner.split('|').next().unwrap_or(inner).trim();
                if !identifier.is_empty()
                    && !looks_like_filesystem_path(identifier)
                    && !out.iter().any(|existing| existing == identifier)
                {
                    out.push(identifier.to_owned());
                }
                rest = &after[end + 2..];
            }
        }
    }
    out
}

pub trait NoteLibrary: Send + Sync {
    fn list_tree(&self, cursor: Option<&str>, page_size: u32) -> Result<TreePageDto, LibraryError>;

    fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError>;

    fn list_relations(&self, identifier: &str) -> Result<RelationListDto, LibraryError> {
        reject_note_identifier(identifier)?;
        Ok(empty_relation_list(identifier))
    }

    fn expand_graph(
        &self,
        identifier: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<GraphPageDto, LibraryError> {
        reject_note_identifier(identifier)?;
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_graph_page(identifier))
    }

    fn search_notes(
        &self,
        query: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<SearchPageDto, LibraryError> {
        reject_search_query(query)?;
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_search_page(query))
    }

    fn inspect_search(
        &self,
        query: &str,
        identifier: Option<&str>,
    ) -> Result<SearchInspectorDto, LibraryError> {
        reject_search_query(query)?;
        let identifier = reject_inspect_identifier(identifier)?;
        Ok(empty_search_inspector(query, identifier))
    }

    fn run_recall_benchmark(&self, k: u32) -> Result<RecallBenchmarkDto, LibraryError> {
        let k = bound_recall_k(Some(k))?;
        Ok(empty_recall_benchmark(k))
    }

    fn schema_validate(
        &self,
        identifier: &str,
        schema_id: Option<&str>,
    ) -> Result<SchemaValidateDto, LibraryError> {
        reject_note_identifier(identifier)?;
        let schema_id = resolve_schema_id(schema_id)?;
        if !is_owned_schema(schema_id) {
            return Ok(unsupported_schema_report(identifier, schema_id));
        }
        Ok(empty_schema_report(identifier, schema_id))
    }

    fn preview_context(
        &self,
        identifier: &str,
        query: Option<&str>,
    ) -> Result<ContextPreviewDto, LibraryError> {
        reject_note_identifier(identifier)?;
        let query = reject_preview_query(query)?;
        Ok(empty_context_preview(identifier, query))
    }

    fn list_activity(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<ActivityPageDto, LibraryError> {
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_activity_page())
    }

    fn list_resources(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<ResourcePageDto, LibraryError> {
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_resource_page())
    }

    fn list_prompts(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<PromptPageDto, LibraryError> {
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_prompt_page())
    }

    fn inspect_tools(&self, profile_id: &str) -> Result<ToolInspectionDto, LibraryError> {
        let profile_id = parse_profile_id(profile_id)?;
        Ok(empty_tool_inspection(profile_id))
    }

    fn list_cli_inventory(
        &self,
        profile_id: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<CliInventoryDto, LibraryError> {
        let profile_id = parse_profile_id(profile_id)?;
        let _ = bound_page_size(Some(page_size))?;
        if cursor.is_some() {
            return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
        }
        Ok(empty_cli_inventory(profile_id))
    }

    fn import_notes(&self, source_id: &str) -> Result<ImportResultDto, LibraryError> {
        reject_source_id(source_id)?;
        Ok(empty_import_result(source_id))
    }

    fn write_note(
        &self,
        identifier: &str,
        title: &str,
        body: &str,
    ) -> Result<NoteWriteDto, LibraryError> {
        let _ = body;
        reject_note_identifier(identifier)?;
        reject_empty_title(title)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn edit_note(&self, identifier: &str, body: &str) -> Result<NoteEditDto, LibraryError> {
        let _ = body;
        reject_note_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn move_note(&self, identifier: &str, destination: &str) -> Result<NoteMoveDto, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_filesystem_destination(destination)?;
        if identifier == destination {
            return Err(LibraryError::schema(SCHEMA_MOVE_SAME_IDENTIFIER));
        }
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn delete_note(&self, identifier: &str) -> Result<NoteDeleteDto, LibraryError> {
        reject_note_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }
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
        reject_note_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn write_note(
        &self,
        identifier: &str,
        title: &str,
        body: &str,
    ) -> Result<NoteWriteDto, LibraryError> {
        let _ = body;
        reject_note_identifier(identifier)?;
        reject_empty_title(title)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn edit_note(&self, identifier: &str, body: &str) -> Result<NoteEditDto, LibraryError> {
        let _ = body;
        reject_note_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn move_note(&self, identifier: &str, destination: &str) -> Result<NoteMoveDto, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_filesystem_destination(destination)?;
        if identifier == destination {
            return Err(LibraryError::schema(SCHEMA_MOVE_SAME_IDENTIFIER));
        }
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }

    fn delete_note(&self, identifier: &str) -> Result<NoteDeleteDto, LibraryError> {
        reject_note_identifier(identifier)?;
        Err(LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))
    }
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct ToolBaselineFile {
    profile_id: String,
    expected_tools: Vec<String>,
}

#[cfg(test)]
#[derive(Debug, Deserialize)]
struct CliLeafFile {
    profile_id: String,
    leaves: Vec<Vec<String>>,
}

#[cfg(test)]
#[derive(Debug, Clone)]
pub struct FixtureLibrary {
    root: PathBuf,
    import_sources_root: PathBuf,
}

#[cfg(test)]
impl FixtureLibrary {
    pub fn new(root: PathBuf) -> Self {
        let import_sources_root = root.join("import-sources");
        Self {
            root,
            import_sources_root,
        }
    }

    pub fn with_import_sources(root: PathBuf, import_sources_root: PathBuf) -> Self {
        Self {
            root,
            import_sources_root,
        }
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
        let candidate = self.resolve_create_path(identifier)?;
        let root = self
            .root
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let resolved = candidate
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_NOTE_MISSING))?;
        if !resolved.starts_with(&root) {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        Ok(resolved)
    }

    fn resolve_create_path(&self, identifier: &str) -> Result<PathBuf, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_forbidden_library_root(&self.root)?;
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
        if library_root_is_forbidden(&candidate) {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        Ok(candidate)
    }

    fn require_crud_root(&self) -> Result<(), LibraryError> {
        reject_forbidden_library_root(&self.root)?;
        if !self.root.to_string_lossy().contains("bmdock-t15") {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        Ok(())
    }

    fn observe_exact_body(path: &Path, expected: &str) -> (bool, bool, NoteCrudClass) {
        let exists = path.is_file();
        let disk = crate::content_safety::read_exact_bytes(path).ok();
        let verified = exists
            && disk
                .as_deref()
                .is_some_and(|bytes| crate::content_safety::bytes_match_body(bytes, expected));
        let classified = if verified {
            NoteCrudClass::DiskVerified
        } else if exists {
            NoteCrudClass::Unclassified
        } else {
            NoteCrudClass::AcceptedUnverified
        };
        (exists, verified, classified)
    }

    fn relation_for_target(&self, target: &str) -> Option<RelationDto> {
        if looks_like_filesystem_path(target) {
            return None;
        }
        let present = self
            .resolve_create_path(target)
            .ok()
            .is_some_and(|path| path.is_file());
        Some(RelationDto {
            identifier: target.to_owned(),
            classified_as: if present {
                RelationTargetClass::Present
            } else {
                RelationTargetClass::Empty
            },
        })
    }

    fn collect_lexical_hits(&self, query: &str) -> Result<Vec<SearchHitDto>, LibraryError> {
        let entries = self.collect_entries()?;
        let mut hits = Vec::new();
        for entry in entries {
            let path = self.resolve_note_path(&entry.identifier)?;
            let disk = crate::content_safety::read_exact_text(&path)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            if !disk.contains(query) {
                continue;
            }
            if looks_like_filesystem_path(&entry.identifier) {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            let title = title_from_markdown(&disk).unwrap_or_else(|| entry.identifier.clone());
            let mut lexical_score = 0u32;
            if title.contains(query) {
                lexical_score = lexical_score.saturating_add(LEXICAL_SCORE_TITLE);
            }
            if disk.contains(query) {
                lexical_score = lexical_score.saturating_add(LEXICAL_SCORE_BODY);
            }
            hits.push(SearchHitDto {
                identifier: entry.identifier,
                lexical_score,
                semantic_score: SEMANTIC_SCORE_DISABLED,
            });
        }
        Ok(hits)
    }

    fn collect_disk_relevant(&self, query: &str) -> Result<Vec<String>, LibraryError> {
        let mut relevant = Vec::new();
        for entry in self.collect_entries()? {
            if looks_like_filesystem_path(&entry.identifier) {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            let path = self.resolve_create_path(&entry.identifier)?;
            if !path.is_file() {
                continue;
            }
            let disk = crate::content_safety::read_exact_text(&path)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            if disk.contains(query) {
                relevant.push(entry.identifier);
            }
        }
        Ok(relevant)
    }

    fn collect_activity_entries(&self) -> Result<Vec<ActivityEntryDto>, LibraryError> {
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
            if !path.is_file() {
                continue;
            }
            let metadata = fs::metadata(&path)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            let modified = metadata
                .modified()
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_TRUNCATED))?;
            let observed_mtime = modified
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or(0);
            entries.push(ActivityEntryDto {
                identifier,
                observed_mtime,
            });
        }
        entries.sort_by(|left, right| {
            right
                .observed_mtime
                .cmp(&left.observed_mtime)
                .then_with(|| left.identifier.cmp(&right.identifier))
        });
        Ok(entries)
    }

    fn collect_resource_entries(&self) -> Result<Vec<ResourceEntryDto>, LibraryError> {
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
            if is_prompt_sidecar_name(&name) || !name.ends_with(".md") {
                continue;
            }
            let identifier = name.strip_suffix(".md").unwrap_or(&name).to_string();
            if looks_like_filesystem_path(&identifier) {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            let path = item.path();
            if !path.is_file() {
                continue;
            }
            let title = fs::read_to_string(&path)
                .ok()
                .and_then(|body| title_from_markdown(&body))
                .unwrap_or_else(|| identifier.clone());
            entries.push(ResourceEntryDto { identifier, title });
        }
        entries.sort_by(|left, right| left.identifier.cmp(&right.identifier));
        Ok(entries)
    }

    fn collect_prompt_entries(&self) -> Result<Vec<PromptEntryDto>, LibraryError> {
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
            if !is_prompt_sidecar_name(&name) {
                continue;
            }
            let identifier = name
                .strip_suffix(PROMPT_SIDECAR_SUFFIX)
                .unwrap_or(&name)
                .to_string();
            if looks_like_filesystem_path(&identifier) {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
            }
            let path = item.path();
            if !path.is_file() {
                continue;
            }
            let title = fs::read_to_string(&path)
                .ok()
                .and_then(|body| title_from_markdown(&body))
                .unwrap_or_else(|| identifier.clone());
            entries.push(PromptEntryDto { identifier, title });
        }
        entries.sort_by(|left, right| left.identifier.cmp(&right.identifier));
        Ok(entries)
    }

    fn read_tool_baseline(
        &self,
        profile_id: &str,
    ) -> Result<Option<(String, Vec<String>)>, LibraryError> {
        reject_forbidden_library_root(&self.root)?;
        let path = self.root.join(format!("tools-{profile_id}.json"));
        if !path.is_file() {
            return Ok(None);
        }
        if library_root_is_forbidden(&path) {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        let text = fs::read_to_string(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let parsed: ToolBaselineFile =
            serde_json::from_str(&text).map_err(|_| LibraryError::schema(SCHEMA_TOOL_BASELINE))?;
        if parsed.profile_id != profile_id {
            return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
        }
        Ok(Some((parsed.profile_id, parsed.expected_tools)))
    }

    fn read_cli_leaves(&self, profile_id: &str) -> Result<Option<Vec<Vec<String>>>, LibraryError> {
        reject_forbidden_library_root(&self.root)?;
        let path = self.root.join(format!("cli-{profile_id}.json"));
        if !path.is_file() {
            return Ok(None);
        }
        if library_root_is_forbidden(&path) {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        let text = fs::read_to_string(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let parsed: CliLeafFile =
            serde_json::from_str(&text).map_err(|_| LibraryError::schema(SCHEMA_CLI_LEAF))?;
        if parsed.profile_id != profile_id {
            return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
        }
        Ok(Some(parsed.leaves))
    }

    fn resolve_prompt_sidecar_path(&self, identifier: &str) -> Result<PathBuf, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_forbidden_library_root(&self.root)?;
        if identifier.contains('/')
            || identifier.contains('\\')
            || identifier
                .split('/')
                .any(|part| part == "." || part == "..")
        {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
        }
        let candidate = self
            .root
            .join(format!("{identifier}{PROMPT_SIDECAR_SUFFIX}"));
        if library_root_is_forbidden(&candidate) {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        Ok(candidate)
    }

    fn require_import_root(&self) -> Result<(), LibraryError> {
        reject_forbidden_library_root(&self.root)?;
        reject_forbidden_library_root(&self.import_sources_root)?;
        let root_ok = self.root.to_string_lossy().contains("bmdock-t28");
        let source_ok = self
            .import_sources_root
            .to_string_lossy()
            .contains("bmdock-t28");
        if !root_ok || !source_ok {
            return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        }
        Ok(())
    }

    pub fn seed_import_source(
        &self,
        source_id: &str,
        files: &[(&str, &str)],
    ) -> Result<PathBuf, LibraryError> {
        reject_source_id(source_id)?;
        self.require_import_root()?;
        let dir = self.import_sources_root.join(source_id);
        fs::create_dir_all(&dir)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        for (identifier, body) in files {
            reject_note_identifier(identifier)?;
            crate::content_safety::persist_exact_utf8(&dir.join(format!("{identifier}.md")), body)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        }
        Ok(dir)
    }

    fn resolve_import_source(&self, source_id: &str) -> Result<Option<PathBuf>, LibraryError> {
        reject_source_id(source_id)?;
        reject_forbidden_library_root(&self.import_sources_root)?;
        let candidate = self.import_sources_root.join(source_id);
        if !candidate.exists() {
            return Ok(None);
        }
        if candidate.is_symlink() {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_SOURCE));
        }
        if !candidate.is_dir() {
            return Ok(None);
        }
        let root = self
            .import_sources_root
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let resolved = candidate
            .canonicalize()
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        if !resolved.starts_with(&root) {
            return Err(LibraryError::policy(POLICY_FILESYSTEM_SOURCE));
        }
        Ok(Some(resolved))
    }

    fn collect_import_markdown(snapshot: &Path) -> Result<Vec<(String, PathBuf)>, LibraryError> {
        let reader = fs::read_dir(snapshot)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let mut files = Vec::new();
        for item in reader {
            let item =
                item.map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            let file_type = item
                .file_type()
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            if file_type.is_symlink() {
                return Err(LibraryError::policy(POLICY_FILESYSTEM_SOURCE));
            }
            if !file_type.is_file() {
                continue;
            }
            let name = item.file_name().to_string_lossy().into_owned();
            if !name.ends_with(".md") || is_prompt_sidecar_name(&name) {
                continue;
            }
            let identifier = name.trim_end_matches(".md").to_string();
            reject_note_identifier(&identifier)?;
            files.push((identifier, item.path()));
        }
        files.sort_by(|left, right| left.0.cmp(&right.0));
        Ok(files)
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
        let disk = crate::content_safety::read_exact_text(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let title = title_from_markdown(&disk).unwrap_or_else(|| identifier.to_owned());
        Ok(NoteReadDto {
            title,
            identifier: identifier.to_owned(),
            body: disk.clone(),
            observation: observation_from_disk(&disk, &disk),
        })
    }

    fn list_relations(&self, identifier: &str) -> Result<RelationListDto, LibraryError> {
        let path = self.resolve_note_path(identifier)?;
        let disk = crate::content_safety::read_exact_text(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let mut relations = Vec::new();
        for target in extract_wiki_link_identifiers(&disk) {
            if let Some(relation) = self.relation_for_target(&target) {
                relations.push(relation);
            }
        }
        Ok(RelationListDto {
            identifier: identifier.to_owned(),
            relations,
            observation: crud_observation(NoteCrudClass::DiskVerified, true),
            engine_graph: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn expand_graph(
        &self,
        identifier: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<GraphPageDto, LibraryError> {
        let page_size = bound_page_size(Some(page_size))?;
        let path = self.resolve_note_path(identifier)?;
        let disk = crate::content_safety::read_exact_text(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let neighbors = extract_wiki_link_identifiers(&disk);
        let offset = parse_offset(cursor, neighbors.len())?;
        let size = page_size as usize;
        let end = offset.saturating_add(size).min(neighbors.len());
        let page_neighbors = &neighbors[offset..end];
        let next_cursor = if end < neighbors.len() {
            Some(end.to_string())
        } else {
            None
        };
        let page = u32::try_from(offset / size)
            .map_err(|_| LibraryError::schema(SCHEMA_INVALID_CURSOR))?
            .saturating_add(1);
        let mut nodes = vec![GraphNodeDto {
            identifier: identifier.to_owned(),
            classified_as: GraphNodeClass::Present,
        }];
        let mut edges = Vec::new();
        for target in page_neighbors {
            if let Some(relation) = self.relation_for_target(target) {
                let classified = match relation.classified_as {
                    RelationTargetClass::Present => GraphNodeClass::Present,
                    RelationTargetClass::Empty | RelationTargetClass::Unsupported => {
                        GraphNodeClass::Empty
                    }
                };
                if !nodes.iter().any(|node| node.identifier == *target) {
                    nodes.push(GraphNodeDto {
                        identifier: target.clone(),
                        classified_as: classified,
                    });
                }
                edges.push(GraphEdgeDto {
                    source: identifier.to_owned(),
                    target: target.clone(),
                });
            }
        }
        Ok(GraphPageDto {
            identifier: identifier.to_owned(),
            nodes,
            edges,
            next_cursor,
            page,
            truncated: false,
            observation: crud_observation(NoteCrudClass::DiskVerified, true),
            engine_graph: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
            depth: GRAPH_DEPTH,
        })
    }

    fn search_notes(
        &self,
        query: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<SearchPageDto, LibraryError> {
        reject_search_query(query)?;
        let page_size = bound_page_size(Some(page_size))?;
        let hits = self.collect_lexical_hits(query)?;
        let offset = parse_offset(cursor, hits.len())?;
        let size = page_size as usize;
        let end = offset.saturating_add(size).min(hits.len());
        let page_hits = hits[offset..end].to_vec();
        let next_cursor = if end < hits.len() {
            Some(end.to_string())
        } else {
            None
        };
        let page = u32::try_from(offset / size)
            .map_err(|_| LibraryError::schema(SCHEMA_INVALID_CURSOR))?
            .saturating_add(1);
        let (classified, disk_verified) = if page_hits.is_empty() {
            (NoteCrudClass::Empty, false)
        } else if page_hits.iter().all(|hit| {
            hit.lexical_score > SEMANTIC_SCORE_DISABLED
                && hit.semantic_score == SEMANTIC_SCORE_DISABLED
                && !looks_like_filesystem_path(&hit.identifier)
        }) {
            (NoteCrudClass::DiskVerified, true)
        } else {
            (NoteCrudClass::AcceptedUnverified, false)
        };
        Ok(SearchPageDto {
            query: query.to_owned(),
            hits: page_hits,
            next_cursor,
            page,
            truncated: false,
            observation: crud_observation(classified, disk_verified),
            semantic_enabled: false,
            engine_search: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn inspect_search(
        &self,
        query: &str,
        identifier: Option<&str>,
    ) -> Result<SearchInspectorDto, LibraryError> {
        reject_search_query(query)?;
        let identifier = reject_inspect_identifier(identifier)?;
        let mut hits = self.collect_lexical_hits(query)?;
        if let Some(target) = identifier {
            hits.retain(|hit| hit.identifier == target);
        }
        if hits.len() > MAX_PAGE_SIZE as usize {
            return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
        }
        let (classified, disk_verified) = if hits.is_empty() {
            (NoteCrudClass::Empty, false)
        } else if hits.iter().all(|hit| {
            hit.lexical_score > SEMANTIC_SCORE_DISABLED
                && hit.semantic_score == SEMANTIC_SCORE_DISABLED
                && !looks_like_filesystem_path(&hit.identifier)
        }) {
            (NoteCrudClass::DiskVerified, true)
        } else {
            (NoteCrudClass::AcceptedUnverified, false)
        };
        Ok(SearchInspectorDto {
            query: query.to_owned(),
            identifier: identifier.map(ToOwned::to_owned),
            hits,
            observation: crud_observation(classified, disk_verified),
            semantic_enabled: false,
            model_id: None,
            model_loaded: false,
            embedding_backend: EmbeddingBackend::None,
            model_class: ModelClass::Unclassified,
            engine_search: false,
            files_written: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            semantic_disabled_reason: SEMANTIC_DISABLED_REASON.to_owned(),
        })
    }

    fn run_recall_benchmark(&self, k: u32) -> Result<RecallBenchmarkDto, LibraryError> {
        let k = bound_recall_k(Some(k))?;
        let mut queries = Vec::new();
        let mut search_elapsed_ms = 0u64;
        let mut chinese_permalinks = Vec::new();
        for query in CHINESE_RECALL_QUERIES {
            let relevant = self.collect_disk_relevant(query)?;
            let search_start = Instant::now();
            let page = self.search_notes(query, None, k)?;
            search_elapsed_ms =
                search_elapsed_ms.saturating_add(search_start.elapsed().as_millis() as u64);
            let mut hits = Vec::new();
            for hit in page.hits.iter().take(k as usize) {
                if looks_like_filesystem_path(&hit.identifier) {
                    return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
                }
                let candidate = self.resolve_create_path(&hit.identifier)?;
                if !candidate.is_file() {
                    continue;
                }
                let disk = crate::content_safety::read_exact_text(&candidate)
                    .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
                if !disk.contains(*query) {
                    continue;
                }
                hits.push(hit.identifier.clone());
                push_chinese_permalink(&mut chinese_permalinks, &hit.identifier);
            }
            let retrieved_relevant = hits
                .iter()
                .filter(|identifier| relevant.iter().any(|item| item == *identifier))
                .count() as u32;
            let relevant_count = relevant.len() as u32;
            for identifier in &relevant {
                push_chinese_permalink(&mut chinese_permalinks, identifier);
            }
            queries.push(RecallQueryDto {
                query: (*query).to_owned(),
                relevant,
                hits,
                retrieved_relevant,
                relevant_count,
            });
        }
        let expand_start = Instant::now();
        let expand_page = match self.expand_graph(CHINESE_RECALL_PERMALINK, None, k) {
            Ok(page) => page,
            Err(LibraryError::Unsupported(_)) => empty_graph_page(CHINESE_RECALL_PERMALINK),
            Err(error) => return Err(error),
        };
        let expand_elapsed_ms = expand_start.elapsed().as_millis() as u64;
        for node in &expand_page.nodes {
            push_chinese_permalink(&mut chinese_permalinks, &node.identifier);
        }
        for edge in &expand_page.edges {
            push_chinese_permalink(&mut chinese_permalinks, &edge.source);
            push_chinese_permalink(&mut chinese_permalinks, &edge.target);
        }
        let recall_hits: u32 = queries.iter().map(|query| query.retrieved_relevant).sum();
        let recall_relevant: u32 = queries.iter().map(|query| query.relevant_count).sum();
        let (classified, disk_verified) = if recall_relevant == 0 {
            (NoteCrudClass::Empty, false)
        } else {
            (NoteCrudClass::DiskVerified, true)
        };
        Ok(RecallBenchmarkDto {
            k,
            query_count: queries.len() as u32,
            recall_hits,
            recall_relevant,
            queries,
            chinese_permalinks,
            search_elapsed_ms,
            expand_elapsed_ms,
            observation: crud_observation(classified, disk_verified),
            semantic_enabled: false,
            engine_search: false,
            native_gui: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn schema_validate(
        &self,
        identifier: &str,
        schema_id: Option<&str>,
    ) -> Result<SchemaValidateDto, LibraryError> {
        reject_note_identifier(identifier)?;
        let schema_id = resolve_schema_id(schema_id)?;
        if !is_owned_schema(schema_id) {
            return Ok(unsupported_schema_report(identifier, schema_id));
        }
        let candidate = self.resolve_create_path(identifier)?;
        if !candidate.is_file() {
            return Ok(empty_schema_report(identifier, schema_id));
        }
        let path = self.resolve_note_path(identifier)?;
        let disk = crate::content_safety::read_exact_text(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let (title, body) = parse_fixture_fields(&disk);
        let observed_title = title
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        let observed_body = body
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty());
        let mut missing_fields = Vec::new();
        if !observed_title {
            missing_fields.push(SCHEMA_FIELD_TITLE.to_owned());
        }
        if !observed_body {
            missing_fields.push(SCHEMA_FIELD_BODY.to_owned());
        }
        let verdict = if missing_fields.is_empty() {
            SchemaVerdict::Valid
        } else {
            SchemaVerdict::Invalid
        };
        Ok(SchemaValidateDto {
            identifier: identifier.to_owned(),
            schema_id: schema_id.to_owned(),
            verdict,
            required_fields: owned_schema_required_fields(),
            missing_fields,
            observed_title,
            observed_body,
            observation: crud_observation(NoteCrudClass::DiskVerified, true),
            engine_schema: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn preview_context(
        &self,
        identifier: &str,
        query: Option<&str>,
    ) -> Result<ContextPreviewDto, LibraryError> {
        reject_note_identifier(identifier)?;
        let query = reject_preview_query(query)?;
        let candidate = self.resolve_create_path(identifier)?;
        if !candidate.is_file() {
            return Ok(empty_context_preview(identifier, query));
        }
        let path = self.resolve_note_path(identifier)?;
        let disk = crate::content_safety::read_exact_text(&path)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let snippet = preview_snippet(&disk, query);
        if !disk.contains(&snippet) {
            return Ok(ContextPreviewDto {
                identifier: identifier.to_owned(),
                query: query.map(ToOwned::to_owned),
                snippet,
                executed: false,
                unsafe_html_present: crate::content_safety::classify_body(&disk)
                    .unsafe_html_present,
                observation: crud_observation(NoteCrudClass::AcceptedUnverified, false),
                engine_context: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            });
        }
        let safety = crate::content_safety::classify_body(&snippet);
        Ok(ContextPreviewDto {
            identifier: identifier.to_owned(),
            query: query.map(ToOwned::to_owned),
            snippet,
            executed: false,
            unsafe_html_present: safety.unsafe_html_present,
            observation: crud_observation(NoteCrudClass::DiskVerified, true),
            engine_context: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn list_activity(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<ActivityPageDto, LibraryError> {
        let page_size = bound_page_size(Some(page_size))?;
        let entries = self.collect_activity_entries()?;
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
        let (classified, disk_verified) = if page_entries.is_empty() {
            (NoteCrudClass::Empty, false)
        } else if page_entries.iter().all(|entry| {
            !looks_like_filesystem_path(&entry.identifier)
                && self
                    .resolve_create_path(&entry.identifier)
                    .ok()
                    .is_some_and(|path| path.is_file())
        }) {
            (NoteCrudClass::DiskVerified, true)
        } else {
            (NoteCrudClass::Empty, false)
        };
        Ok(ActivityPageDto {
            entries: page_entries,
            next_cursor,
            page,
            truncated: false,
            observation: crud_observation(classified, disk_verified),
            engine_activity: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn list_resources(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<ResourcePageDto, LibraryError> {
        let page_size = bound_page_size(Some(page_size))?;
        let entries = self.collect_resource_entries()?;
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
        let (classified, disk_verified) = if page_entries.is_empty() {
            (NoteCrudClass::Empty, false)
        } else if page_entries.iter().all(|entry| {
            !looks_like_filesystem_path(&entry.identifier)
                && self
                    .resolve_create_path(&entry.identifier)
                    .ok()
                    .is_some_and(|path| path.is_file())
        }) {
            (NoteCrudClass::DiskVerified, true)
        } else {
            (NoteCrudClass::Empty, false)
        };
        Ok(ResourcePageDto {
            entries: page_entries,
            next_cursor,
            page,
            truncated: false,
            observation: crud_observation(classified, disk_verified),
            engine_resources: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn list_prompts(
        &self,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<PromptPageDto, LibraryError> {
        let page_size = bound_page_size(Some(page_size))?;
        let entries = self.collect_prompt_entries()?;
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
        let (classified, disk_verified) = if page_entries.is_empty() {
            (NoteCrudClass::Empty, false)
        } else if page_entries.iter().all(|entry| {
            !looks_like_filesystem_path(&entry.identifier)
                && self
                    .resolve_prompt_sidecar_path(&entry.identifier)
                    .ok()
                    .is_some_and(|path| path.is_file())
        }) {
            (NoteCrudClass::DiskVerified, true)
        } else {
            (NoteCrudClass::Empty, false)
        };
        Ok(PromptPageDto {
            entries: page_entries,
            next_cursor,
            page,
            truncated: false,
            observation: crud_observation(classified, disk_verified),
            engine_prompts: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            files_written: false,
        })
    }

    fn inspect_tools(&self, profile_id: &str) -> Result<ToolInspectionDto, LibraryError> {
        let profile_id = parse_profile_id(profile_id)?;
        match self.read_tool_baseline(profile_id)? {
            None => Ok(empty_tool_inspection(profile_id)),
            Some((_, names)) => {
                inspect_tools_from_names(profile_id, &names, NoteCrudClass::DiskVerified, true)
            }
        }
    }

    fn list_cli_inventory(
        &self,
        profile_id: &str,
        cursor: Option<&str>,
        page_size: u32,
    ) -> Result<CliInventoryDto, LibraryError> {
        let profile_id = parse_profile_id(profile_id)?;
        let page_size = bound_page_size(Some(page_size))?;
        match self.read_cli_leaves(profile_id)? {
            None => {
                if cursor.is_some() {
                    return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
                }
                Ok(empty_cli_inventory(profile_id))
            }
            Some(leaves) => cli_inventory_from_leaves(
                profile_id,
                leaves,
                cursor,
                page_size,
                NoteCrudClass::DiskVerified,
                true,
            ),
        }
    }

    fn import_notes(&self, source_id: &str) -> Result<ImportResultDto, LibraryError> {
        reject_source_id(source_id)?;
        self.require_import_root()?;
        let Some(snapshot) = self.resolve_import_source(source_id)? else {
            return Ok(empty_import_result(source_id));
        };
        let markdown = Self::collect_import_markdown(&snapshot)?;
        if markdown.is_empty() {
            return Ok(empty_import_result(source_id));
        }
        fs::create_dir_all(&self.root)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let mut files = Vec::new();
        let mut wrote = false;
        let mut all_verified = true;
        for (identifier, source) in markdown {
            let dest = self.resolve_create_path(&identifier)?;
            if library_root_is_forbidden(&dest) {
                return Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
            }
            let body = crate::content_safety::read_exact_text(&source)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            crate::content_safety::persist_exact_utf8(&dest, &body)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
            wrote = true;
            let (exists, verified, _) = Self::observe_exact_body(&dest, &body);
            if !exists || !verified {
                all_verified = false;
            }
            files.push(ImportedFileDto {
                identifier,
                kind: crate::routing::OWNED_KIND.to_owned(),
            });
        }
        let classified = if wrote && all_verified {
            ImportClass::DiskVerified
        } else if wrote {
            ImportClass::Unclassified
        } else {
            ImportClass::AcceptedUnverified
        };
        Ok(ImportResultDto {
            source_id: source_id.to_owned(),
            files,
            files_written: wrote,
            observation: ImportObservationDto {
                classified_as: classified,
                disk_verified: wrote && all_verified,
                envelope_is_not_disk_proof: true,
            },
            engine_import: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
        })
    }

    fn write_note(
        &self,
        identifier: &str,
        title: &str,
        body: &str,
    ) -> Result<NoteWriteDto, LibraryError> {
        self.require_crud_root()?;
        reject_note_identifier(identifier)?;
        reject_empty_title(title)?;
        let dest = self.resolve_create_path(identifier)?;
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        }
        crate::content_safety::persist_exact_utf8(&dest, body)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let (exists, verified, classified) = Self::observe_exact_body(&dest, body);
        Ok(NoteWriteDto {
            identifier: identifier.to_owned(),
            title: title.to_owned(),
            body: body.to_owned(),
            files_written: exists,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(classified, verified),
        })
    }

    fn edit_note(&self, identifier: &str, body: &str) -> Result<NoteEditDto, LibraryError> {
        self.require_crud_root()?;
        let path = self.resolve_note_path(identifier)?;
        crate::content_safety::persist_exact_utf8(&path, body)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let (exists, verified, classified) = Self::observe_exact_body(&path, body);
        Ok(NoteEditDto {
            identifier: identifier.to_owned(),
            body: body.to_owned(),
            files_written: exists,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(classified, verified),
        })
    }

    fn move_note(&self, identifier: &str, destination: &str) -> Result<NoteMoveDto, LibraryError> {
        self.require_crud_root()?;
        reject_note_identifier(identifier)?;
        reject_filesystem_destination(destination)?;
        if identifier == destination {
            return Err(LibraryError::schema(SCHEMA_MOVE_SAME_IDENTIFIER));
        }
        let source = self.resolve_note_path(identifier)?;
        let dest = self.resolve_create_path(destination)?;
        if dest.exists() {
            return Err(LibraryError::unsupported(
                UNSUPPORTED_MOVE_DESTINATION_EXISTS,
            ));
        }
        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        }
        let body = crate::content_safety::read_exact_text(&source)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        fs::rename(&source, &dest)
            .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        let old_gone = !source.exists();
        let (exists, verified, classified) = Self::observe_exact_body(&dest, &body);
        let disk_verified = old_gone && verified;
        let classified = if disk_verified {
            NoteCrudClass::DiskVerified
        } else if exists {
            classified
        } else {
            NoteCrudClass::AcceptedUnverified
        };
        Ok(NoteMoveDto {
            identifier: identifier.to_owned(),
            destination: destination.to_owned(),
            body,
            files_written: exists && old_gone,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(classified, disk_verified),
        })
    }

    fn delete_note(&self, identifier: &str) -> Result<NoteDeleteDto, LibraryError> {
        self.require_crud_root()?;
        reject_note_identifier(identifier)?;
        let dest = self.resolve_create_path(identifier)?;
        let was_present = dest.is_file();
        if dest.exists() {
            fs::remove_file(&dest)
                .map_err(|_| LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE))?;
        }
        let missing = !dest.exists();
        let classified = if missing {
            NoteCrudClass::DiskVerified
        } else {
            NoteCrudClass::AcceptedUnverified
        };
        Ok(NoteDeleteDto {
            identifier: identifier.to_owned(),
            files_written: was_present && missing,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(classified, missing),
        })
    }
}

#[cfg(test)]
#[derive(Debug, Default)]
pub struct EnvelopeCrudLibrary;

#[cfg(test)]
impl NoteLibrary for EnvelopeCrudLibrary {
    fn list_tree(&self, cursor: Option<&str>, page_size: u32) -> Result<TreePageDto, LibraryError> {
        let _ = (cursor, page_size);
        Ok(TreePageDto {
            entries: Vec::new(),
            next_cursor: None,
            page: 1,
            truncated: false,
        })
    }

    fn read_note(&self, identifier: &str) -> Result<NoteReadDto, LibraryError> {
        Ok(NoteReadDto {
            title: identifier.to_owned(),
            identifier: identifier.to_owned(),
            body: "saved".to_owned(),
            observation: observation_from_envelope_only(),
        })
    }

    fn write_note(
        &self,
        identifier: &str,
        title: &str,
        _body: &str,
    ) -> Result<NoteWriteDto, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_empty_title(title)?;
        Ok(NoteWriteDto {
            identifier: identifier.to_owned(),
            title: title.to_owned(),
            body: "saved".to_owned(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(NoteCrudClass::AcceptedUnverified, false),
        })
    }

    fn edit_note(&self, identifier: &str, _body: &str) -> Result<NoteEditDto, LibraryError> {
        reject_note_identifier(identifier)?;
        Ok(NoteEditDto {
            identifier: identifier.to_owned(),
            body: "saved".to_owned(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(NoteCrudClass::AcceptedUnverified, false),
        })
    }

    fn move_note(&self, identifier: &str, destination: &str) -> Result<NoteMoveDto, LibraryError> {
        reject_note_identifier(identifier)?;
        reject_filesystem_destination(destination)?;
        Ok(NoteMoveDto {
            identifier: identifier.to_owned(),
            destination: destination.to_owned(),
            body: "saved".to_owned(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(NoteCrudClass::AcceptedUnverified, false),
        })
    }

    fn delete_note(&self, identifier: &str) -> Result<NoteDeleteDto, LibraryError> {
        reject_note_identifier(identifier)?;
        Ok(NoteDeleteDto {
            identifier: identifier.to_owned(),
            files_written: false,
            engine_persisted: false,
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
            observation: crud_observation(NoteCrudClass::AcceptedUnverified, false),
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

pub fn bound_recall_k(k: Option<u32>) -> Result<u32, LibraryError> {
    match k {
        None => Ok(DEFAULT_PAGE_SIZE),
        Some(0) => Err(LibraryError::schema(SCHEMA_RECALL_K)),
        Some(size) if size > MAX_PAGE_SIZE => Err(LibraryError::schema(SCHEMA_RECALL_K)),
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

pub fn accept_graph_page(
    request_cursor: Option<&str>,
    page: GraphPageDto,
) -> Result<GraphPageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.depth != GRAPH_DEPTH {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.edges.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
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

pub fn accept_search_page(
    request_cursor: Option<&str>,
    page: SearchPageDto,
) -> Result<SearchPageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.semantic_enabled || page.engine_search {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.hits.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page
        .hits
        .iter()
        .any(|hit| looks_like_filesystem_path(&hit.identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
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

pub fn accept_activity_page(
    request_cursor: Option<&str>,
    page: ActivityPageDto,
) -> Result<ActivityPageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.engine_activity {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.entries.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page
        .entries
        .iter()
        .any(|entry| looks_like_filesystem_path(&entry.identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
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

pub fn accept_resource_page(
    request_cursor: Option<&str>,
    page: ResourcePageDto,
) -> Result<ResourcePageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.engine_resources {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.entries.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page
        .entries
        .iter()
        .any(|entry| looks_like_filesystem_path(&entry.identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
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

pub fn accept_prompt_page(
    request_cursor: Option<&str>,
    page: PromptPageDto,
) -> Result<PromptPageDto, LibraryError> {
    if page.truncated {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.engine_prompts {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.entries.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page
        .entries
        .iter()
        .any(|entry| looks_like_filesystem_path(&entry.identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
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

pub fn accept_tool_inspection(
    report: ToolInspectionDto,
) -> Result<ToolInspectionDto, LibraryError> {
    parse_profile_id(&report.profile_id)?;
    if report.engine_tools
        || report.mixed_profiles
        || report.files_written
        || report.call_tool_allowed
        || report.tools.iter().any(|tool| tool.live_execution)
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if report.tools.len() >= 48 {
        return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
    }
    if report.expected_tool_count as usize != report.tools.len() {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if report.tools.iter().any(|tool| {
        looks_like_filesystem_path(&tool.name) || looks_like_filesystem_path(&tool.identity)
    }) {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
    }
    if report
        .tools
        .iter()
        .any(|tool| tool.name == CALL_TOOL_IDENTITY && tool.admission != ToolAdmission::Denied)
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    let search = report
        .tools
        .iter()
        .find(|tool| tool.name == SEARCH_IDENTITY);
    let fetch = report.tools.iter().find(|tool| tool.name == FETCH_IDENTITY);
    if let (Some(search), Some(fetch)) = (search, fetch) {
        if search.identity == fetch.identity {
            return Err(LibraryError::unsupported(UNSUPPORTED_MIXED_PROFILES));
        }
        if search.admission != ToolAdmission::Denied || fetch.admission != ToolAdmission::Denied {
            return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
        }
    }
    Ok(report)
}

pub fn accept_cli_inventory(
    request_cursor: Option<&str>,
    page: CliInventoryDto,
) -> Result<CliInventoryDto, LibraryError> {
    parse_profile_id(&page.profile_id)?;
    if page.truncated
        || page.engine_cli
        || page.executed
        || page.mixed_profiles
        || page.files_written
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if page.page == 0 {
        return Err(LibraryError::schema(SCHEMA_INVALID_CURSOR));
    }
    if page.leaves.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    let paths = page
        .leaves
        .iter()
        .map(|leaf| leaf.path.clone())
        .collect::<Vec<_>>();
    reject_coarse_cli_leaves(&paths)?;
    if page.leaves.iter().any(|leaf| leaf.executed) {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
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

pub fn accept_search_inspector(
    inspector: SearchInspectorDto,
) -> Result<SearchInspectorDto, LibraryError> {
    if inspector.semantic_enabled
        || inspector.engine_search
        || inspector.model_loaded
        || inspector.files_written
        || inspector.embedding_backend != EmbeddingBackend::None
        || inspector.model_id.is_some()
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if inspector.hits.len() > MAX_PAGE_SIZE as usize {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if inspector
        .hits
        .iter()
        .any(|hit| looks_like_filesystem_path(&hit.identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
    }
    Ok(inspector)
}

pub fn accept_recall_benchmark(
    report: RecallBenchmarkDto,
) -> Result<RecallBenchmarkDto, LibraryError> {
    if report.semantic_enabled || report.engine_search || report.native_gui || report.files_written
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if report.k == 0 || report.k > MAX_PAGE_SIZE {
        return Err(LibraryError::schema(SCHEMA_RECALL_K));
    }
    if report.query_count as usize != report.queries.len() {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if report.queries.iter().any(|query| {
        looks_like_filesystem_path(&query.query)
            || query
                .relevant
                .iter()
                .any(|identifier| looks_like_filesystem_path(identifier))
            || query
                .hits
                .iter()
                .any(|identifier| looks_like_filesystem_path(identifier))
    }) || report
        .chinese_permalinks
        .iter()
        .any(|identifier| looks_like_filesystem_path(identifier))
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
    }
    Ok(report)
}

pub fn resolve_schema_id(schema_id: Option<&str>) -> Result<&str, LibraryError> {
    match schema_id.map(str::trim) {
        None => Ok(DEFAULT_SCHEMA_ID),
        Some("") => Err(LibraryError::schema(SCHEMA_INVALID_SCHEMA_ID)),
        Some(value) if looks_like_filesystem_path(value) => {
            Err(LibraryError::policy(POLICY_FILESYSTEM_SCHEMA_ID))
        }
        Some(value) => Ok(value),
    }
}

pub fn is_owned_schema(schema_id: &str) -> bool {
    schema_id == DEFAULT_SCHEMA_ID
}

pub fn accept_schema_report(report: SchemaValidateDto) -> Result<SchemaValidateDto, LibraryError> {
    if report.engine_schema || report.files_written {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if looks_like_filesystem_path(&report.identifier)
        || looks_like_filesystem_path(&report.schema_id)
    {
        return Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER));
    }
    if report.verdict == SchemaVerdict::Valid && !report.observation.disk_verified {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    if report.verdict == SchemaVerdict::Valid
        && (!report.observed_title || !report.observed_body || !report.missing_fields.is_empty())
    {
        return Err(LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
    }
    Ok(report)
}

#[cfg(test)]
fn contains_cjk(value: &str) -> bool {
    value.chars().any(|ch| {
        let code = ch as u32;
        (0x3400..=0x9FFF).contains(&code) || (0xF900..=0xFAFF).contains(&code)
    })
}

#[cfg(test)]
fn push_chinese_permalink(out: &mut Vec<String>, identifier: &str) {
    if contains_cjk(identifier) && !out.iter().any(|existing| existing == identifier) {
        out.push(identifier.to_owned());
    }
}

pub fn reject_filesystem_identifier(identifier: &str) -> Result<(), LibraryError> {
    if looks_like_filesystem_path(identifier) {
        Err(LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER))
    } else {
        Ok(())
    }
}

pub fn reject_note_identifier(identifier: &str) -> Result<(), LibraryError> {
    if identifier.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_NOTE_IDENTIFIER));
    }
    reject_filesystem_identifier(identifier)
}

pub fn reject_empty_title(title: &str) -> Result<(), LibraryError> {
    if title.trim().is_empty() {
        Err(LibraryError::schema(SCHEMA_NOTE_TITLE))
    } else {
        Ok(())
    }
}

pub fn reject_filesystem_destination(destination: &str) -> Result<(), LibraryError> {
    if destination.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_NOTE_DESTINATION));
    }
    if looks_like_filesystem_path(destination) {
        Err(LibraryError::policy(POLICY_FILESYSTEM_DESTINATION))
    } else {
        Ok(())
    }
}

pub fn reject_search_query(query: &str) -> Result<(), LibraryError> {
    if query.trim().is_empty() {
        return Err(LibraryError::schema(SCHEMA_SEARCH_QUERY));
    }
    if looks_like_filesystem_path(query) {
        Err(LibraryError::policy(POLICY_FILESYSTEM_QUERY))
    } else {
        Ok(())
    }
}

pub fn reject_inspect_identifier(identifier: Option<&str>) -> Result<Option<&str>, LibraryError> {
    match identifier {
        None => Ok(None),
        Some(value) => {
            reject_note_identifier(value)?;
            Ok(Some(value.trim()))
        }
    }
}

pub fn reject_preview_query(query: Option<&str>) -> Result<Option<&str>, LibraryError> {
    match query.map(str::trim).filter(|value| !value.is_empty()) {
        None => Ok(None),
        Some(value) if looks_like_filesystem_path(value) => {
            Err(LibraryError::policy(POLICY_FILESYSTEM_QUERY))
        }
        Some(value) => Ok(Some(value)),
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

#[cfg(test)]
fn split_frontmatter(disk: &str) -> (Option<String>, &str) {
    let text = disk.strip_prefix('\u{feff}').unwrap_or(disk);
    let Some(after_open) = text.strip_prefix("---") else {
        return (None, disk);
    };
    let after_open = after_open.strip_prefix('\r').unwrap_or(after_open);
    let Some(after_open) = after_open.strip_prefix('\n') else {
        return (None, disk);
    };
    let Some(end) = after_open.find("\n---") else {
        return (None, disk);
    };
    let front = after_open[..end].trim().to_owned();
    let mut rest = &after_open[end + 1..];
    rest = rest.strip_prefix("---").unwrap_or(rest);
    rest = rest.strip_prefix("\r\n").unwrap_or(rest);
    rest = rest.strip_prefix('\n').unwrap_or(rest);
    (Some(front), rest)
}

#[cfg(test)]
fn json_like_fields(front: &str) -> (Option<String>, Option<String>) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(front) else {
        return (None, None);
    };
    let read = |key: &str| {
        value
            .get(key)
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    };
    (read("title"), read("body"))
}

#[cfg(test)]
fn yaml_like_fields(front: &str) -> (Option<String>, Option<String>) {
    let mut title = None;
    let mut body = None;
    for line in front.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_matches('"').trim();
        if value.is_empty() {
            continue;
        }
        if key == SCHEMA_FIELD_TITLE {
            title = Some(value.to_owned());
        }
        if key == SCHEMA_FIELD_BODY {
            body = Some(value.to_owned());
        }
    }
    (title, body)
}

#[cfg(test)]
fn markdown_body_without_heading(text: &str) -> String {
    let mut lines = text.lines().peekable();
    while let Some(line) = lines.peek() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("# ") {
            lines.next();
            continue;
        }
        break;
    }
    let body: Vec<&str> = lines.collect();
    body.join("\n").trim().to_owned()
}

#[cfg(test)]
pub fn parse_fixture_fields(disk: &str) -> (Option<String>, Option<String>) {
    let (front, rest) = split_frontmatter(disk);
    let mut title = None;
    let mut body = None;
    if let Some(front) = front.as_deref() {
        if front.starts_with('{') {
            let (json_title, json_body) = json_like_fields(front);
            title = json_title;
            body = json_body;
        } else {
            let (yaml_title, yaml_body) = yaml_like_fields(front);
            title = yaml_title;
            body = yaml_body;
        }
    }
    if title.is_none() {
        title = title_from_markdown(disk);
        if title.is_none() {
            title = title_from_markdown(rest);
        }
    }
    if body.is_none() {
        let remainder = markdown_body_without_heading(rest);
        if !remainder.is_empty() {
            body = Some(remainder);
        }
    }
    (title, body)
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
fn crud_observation(classified_as: NoteCrudClass, disk_verified: bool) -> NoteCrudObservationDto {
    NoteCrudObservationDto {
        classified_as,
        disk_verified,
        envelope_is_not_disk_proof: true,
    }
}

#[cfg(test)]
pub fn library_root_is_forbidden(path: &Path) -> bool {
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
pub fn reject_forbidden_library_root(path: &Path) -> Result<(), LibraryError> {
    if library_root_is_forbidden(path) {
        Err(LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT))
    } else {
        Ok(())
    }
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
pub fn follow_graph_pages(
    library: &dyn NoteLibrary,
    identifier: &str,
    page_size: u32,
) -> Result<(Vec<GraphEdgeDto>, u32), LibraryError> {
    use std::collections::HashSet;

    let mut edges = Vec::new();
    let mut cursor: Option<String> = None;
    let mut seen = HashSet::new();
    for page in 1..=128u32 {
        let listed = library.expand_graph(identifier, cursor.as_deref(), page_size)?;
        let listed = accept_graph_page(cursor.as_deref(), listed)?;
        edges.extend(listed.edges);
        match listed.next_cursor {
            None => return Ok((edges, page)),
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

        fn write_prompt(&self, identifier: &str, title: &str, body: &str) -> PathBuf {
            let path = self
                .dir
                .join(format!("{identifier}{PROMPT_SIDECAR_SUFFIX}"));
            fs::write(&path, format!("# {title}\n\n{body}\n")).unwrap();
            path
        }
    }

    fn set_file_mtime(path: &Path, time: SystemTime) {
        fs::OpenOptions::new()
            .write(true)
            .open(path)
            .unwrap()
            .set_modified(time)
            .unwrap();
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

        fn expand_graph(
            &self,
            identifier: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<GraphPageDto, LibraryError> {
            Ok(GraphPageDto {
                identifier: identifier.to_owned(),
                nodes: Vec::new(),
                edges: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_graph: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
                depth: GRAPH_DEPTH,
            })
        }

        fn search_notes(
            &self,
            query: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<SearchPageDto, LibraryError> {
            Ok(SearchPageDto {
                query: query.to_owned(),
                hits: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
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
        ) -> Result<ActivityPageDto, LibraryError> {
            Ok(ActivityPageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_activity: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_resources(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<ResourcePageDto, LibraryError> {
            Ok(ResourcePageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_resources: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_prompts(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<PromptPageDto, LibraryError> {
            Ok(PromptPageDto {
                entries: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_prompts: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_cli_inventory(
            &self,
            profile_id: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<CliInventoryDto, LibraryError> {
            Ok(CliInventoryDto {
                profile_id: profile_id.to_owned(),
                leaves: Vec::new(),
                next_cursor: None,
                page: 1,
                truncated: true,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_cli: false,
                executed: false,
                mixed_profiles: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
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

        fn search_notes(
            &self,
            query: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<SearchPageDto, LibraryError> {
            Ok(SearchPageDto {
                query: query.to_owned(),
                hits: vec![SearchHitDto {
                    identifier: "loop".to_owned(),
                    lexical_score: LEXICAL_SCORE_BODY,
                    semantic_score: SEMANTIC_SCORE_DISABLED,
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
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
        ) -> Result<ActivityPageDto, LibraryError> {
            Ok(ActivityPageDto {
                entries: vec![ActivityEntryDto {
                    identifier: "loop".to_owned(),
                    observed_mtime: 1,
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_activity: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_resources(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<ResourcePageDto, LibraryError> {
            Ok(ResourcePageDto {
                entries: vec![ResourceEntryDto {
                    identifier: "loop".to_owned(),
                    title: "loop".to_owned(),
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_resources: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_prompts(
            &self,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<PromptPageDto, LibraryError> {
            Ok(PromptPageDto {
                entries: vec![PromptEntryDto {
                    identifier: "loop".to_owned(),
                    title: "loop".to_owned(),
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_prompts: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
        }

        fn list_cli_inventory(
            &self,
            profile_id: &str,
            _cursor: Option<&str>,
            _page_size: u32,
        ) -> Result<CliInventoryDto, LibraryError> {
            Ok(CliInventoryDto {
                profile_id: profile_id.to_owned(),
                leaves: vec![CliLeafDto {
                    path: vec!["bm".to_owned(), "mcp".to_owned()],
                    executed: false,
                }],
                next_cursor: Some("same".to_owned()),
                page: 1,
                truncated: false,
                observation: crud_observation(NoteCrudClass::Unclassified, false),
                engine_cli: false,
                executed: false,
                mixed_profiles: false,
                scanned_user_obsidian_vault: false,
                scanned_user_basic_memory_home: false,
                files_written: false,
            })
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
        let relations = library.list_relations("welcome").unwrap();
        assert!(relations.relations.is_empty());
        assert!(!relations.engine_graph);
        assert!(!relations.files_written);
        assert!(!relations.scanned_user_obsidian_vault);
        assert!(!relations.scanned_user_basic_memory_home);
        assert_eq!(relations.observation.classified_as, NoteCrudClass::Empty);
        assert!(!relations.observation.disk_verified);
        assert!(relations.observation.envelope_is_not_disk_proof);
        let graph = library
            .expand_graph("welcome", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(graph.nodes.is_empty());
        assert!(graph.edges.is_empty());
        assert_eq!(graph.next_cursor, None);
        assert!(!graph.truncated);
        assert_eq!(graph.depth, GRAPH_DEPTH);
        assert!(!graph.engine_graph);
        assert_eq!(graph.observation.classified_as, NoteCrudClass::Empty);
        assert!(!graph.observation.disk_verified);
        let search = library
            .search_notes("欢迎", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(search.hits.is_empty());
        assert_eq!(search.next_cursor, None);
        assert!(!search.truncated);
        assert!(!search.semantic_enabled);
        assert!(!search.engine_search);
        assert_eq!(search.observation.classified_as, NoteCrudClass::Empty);
        assert!(!search.observation.disk_verified);
        assert!(search.observation.envelope_is_not_disk_proof);
        let inspector = library.inspect_search("欢迎", None).unwrap();
        assert!(inspector.hits.is_empty());
        assert!(inspector.identifier.is_none());
        assert!(!inspector.semantic_enabled);
        assert!(!inspector.model_loaded);
        assert!(inspector.model_id.is_none());
        assert_eq!(inspector.embedding_backend, EmbeddingBackend::None);
        assert_eq!(inspector.model_class, ModelClass::Unclassified);
        assert!(!inspector.engine_search);
        assert!(!inspector.files_written);
        assert_eq!(inspector.observation.classified_as, NoteCrudClass::Empty);
        assert!(!inspector.observation.disk_verified);
        assert_eq!(inspector.semantic_disabled_reason, SEMANTIC_DISABLED_REASON);
        let recall = library.run_recall_benchmark(DEFAULT_PAGE_SIZE).unwrap();
        assert_eq!(recall.query_count, 0);
        assert!(recall.queries.is_empty());
        assert_eq!(recall.recall_hits, 0);
        assert_eq!(recall.recall_relevant, 0);
        assert!(recall.chinese_permalinks.is_empty());
        assert!(!recall.semantic_enabled);
        assert!(!recall.engine_search);
        assert!(!recall.native_gui);
        assert!(!recall.files_written);
        assert_eq!(recall.observation.classified_as, NoteCrudClass::Empty);
        assert!(!recall.observation.disk_verified);
        assert!(recall.observation.envelope_is_not_disk_proof);
        let schema = library.schema_validate("welcome", None).unwrap();
        assert_eq!(schema.identifier, "welcome");
        assert_eq!(schema.schema_id, DEFAULT_SCHEMA_ID);
        assert_eq!(schema.verdict, SchemaVerdict::Empty);
        assert!(!schema.observed_title);
        assert!(!schema.observed_body);
        assert!(!schema.engine_schema);
        assert!(!schema.files_written);
        assert_eq!(schema.observation.classified_as, NoteCrudClass::Empty);
        assert!(!schema.observation.disk_verified);
        assert!(schema.observation.envelope_is_not_disk_proof);
        let preview = library.preview_context("welcome", None).unwrap();
        assert!(preview.snippet.is_empty());
        assert!(!preview.executed);
        assert!(!preview.engine_context);
        assert_eq!(preview.observation.classified_as, NoteCrudClass::Empty);
        assert!(!preview.observation.disk_verified);
        let activity = library.list_activity(None, DEFAULT_PAGE_SIZE).unwrap();
        assert!(activity.entries.is_empty());
        assert_eq!(activity.next_cursor, None);
        assert!(!activity.truncated);
        assert!(!activity.engine_activity);
        assert_eq!(activity.observation.classified_as, NoteCrudClass::Empty);
        let resources = library.list_resources(None, DEFAULT_PAGE_SIZE).unwrap();
        assert!(resources.entries.is_empty());
        assert_eq!(resources.next_cursor, None);
        assert!(!resources.truncated);
        assert!(!resources.engine_resources);
        assert_eq!(resources.observation.classified_as, NoteCrudClass::Empty);
        assert!(!resources.observation.disk_verified);
        assert!(resources.observation.envelope_is_not_disk_proof);
        let prompts = library.list_prompts(None, DEFAULT_PAGE_SIZE).unwrap();
        assert!(prompts.entries.is_empty());
        assert_eq!(prompts.next_cursor, None);
        assert!(!prompts.truncated);
        assert!(!prompts.engine_prompts);
        assert_eq!(prompts.observation.classified_as, NoteCrudClass::Empty);
        assert!(!prompts.observation.disk_verified);
        let tools = library.inspect_tools("release").unwrap();
        assert!(tools.tools.is_empty());
        assert_eq!(tools.expected_tool_count, 0);
        assert_eq!(tools.profile_id, "release");
        assert!(!tools.engine_tools);
        assert!(!tools.call_tool_allowed);
        assert!(!tools.mixed_profiles);
        assert!(!tools.files_written);
        assert_eq!(tools.observation.classified_as, NoteCrudClass::Empty);
        let preview_tools = library.inspect_tools("main-preview").unwrap();
        assert!(preview_tools.tools.is_empty());
        assert_eq!(preview_tools.profile_id, "main-preview");
        assert_eq!(preview_tools.expected_tool_count, 0);
        assert_eq!(
            library.inspect_tools("mixed").unwrap_err(),
            LibraryError::schema(SCHEMA_PROFILE_ID)
        );
        assert_eq!(
            library
                .inspect_tools(r"C:\Users\someone\vault")
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_PROFILE)
        );
        let cli = library
            .list_cli_inventory("release", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(cli.leaves.is_empty());
        assert_eq!(cli.next_cursor, None);
        assert!(!cli.truncated);
        assert!(!cli.engine_cli);
        assert!(!cli.executed);
        assert_eq!(cli.observation.classified_as, NoteCrudClass::Empty);
        let imported = library.import_notes("fixture-welcome").unwrap();
        assert!(imported.files.is_empty());
        assert!(!imported.files_written);
        assert!(!imported.engine_import);
        assert!(!imported.scanned_user_obsidian_vault);
        assert!(!imported.scanned_user_basic_memory_home);
        assert_eq!(imported.observation.classified_as, ImportClass::Empty);
        assert!(!imported.observation.disk_verified);
        assert!(imported.observation.envelope_is_not_disk_proof);
        assert_eq!(
            library
                .import_notes(r"C:\Users\someone\Documents\Obsidian")
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_SOURCE)
        );
        assert_eq!(
            library.import_notes("").unwrap_err(),
            LibraryError::schema(SCHEMA_SOURCE_ID)
        );
        let _ = (
            ENGINE_GRAPH_NOT_OWNED,
            ENGINE_SEARCH_NOT_OWNED,
            ENGINE_INSPECTOR_NOT_OWNED,
            INSPECTOR_READ_ONLY,
            INSPECTOR_MODEL_UNVERIFIED,
            ENGINE_RECALL_NOT_OWNED,
            RECALL_NATIVE_UNVERIFIED,
            OFFICIAL_CHINESE_RECALL_UNVERIFIED,
            ENGINE_SCHEMA_NOT_OWNED,
            OFFICIAL_SCHEMA_MCP_UNVERIFIED,
            ENGINE_RESOURCES_NOT_OWNED,
            ENGINE_PROMPTS_NOT_OWNED,
            OFFICIAL_RESOURCES_MCP_UNVERIFIED,
            OFFICIAL_PROMPTS_MCP_UNVERIFIED,
            ENGINE_TOOLS_NOT_OWNED,
            ENGINE_CLI_NOT_OWNED,
            OFFICIAL_TOOLS_MCP_UNVERIFIED,
            OFFICIAL_CLI_UNVERIFIED,
            ENGINE_IMPORT_NOT_OWNED,
            OFFICIAL_IMPORT_UNVERIFIED,
            ENGINE_CONTEXT_NOT_OWNED,
            ENGINE_ACTIVITY_NOT_OWNED,
            SEMANTIC_SEARCH_UNVERIFIED,
            OFFICIAL_SEARCH_MCP_UNVERIFIED,
            OFFICIAL_FETCH_MCP_UNVERIFIED,
            RECENT_ACTIVITY_MCP_UNVERIFIED,
            BUILD_CONTEXT_MCP_UNVERIFIED,
            NATIVE_GUI_UNVERIFIED,
            BOUNDED_HOST_EXPANSION,
        );
    }

    #[test]
    fn empty_library_rejects_cursor_as_schema() {
        let error = EmptyLibrary
            .list_tree(Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(error, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        let graph_cursor = EmptyLibrary
            .expand_graph("welcome", Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(graph_cursor, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        let search_cursor = EmptyLibrary
            .search_notes("欢迎", Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(search_cursor, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        let activity_cursor = EmptyLibrary
            .list_activity(Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(activity_cursor, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        let resource_cursor = EmptyLibrary
            .list_resources(Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(resource_cursor, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        let prompt_cursor = EmptyLibrary
            .list_prompts(Some("1"), DEFAULT_PAGE_SIZE)
            .unwrap_err();
        assert_eq!(prompt_cursor, LibraryError::schema(SCHEMA_INVALID_CURSOR));
        assert_eq!(
            EmptyLibrary
                .search_notes("", None, DEFAULT_PAGE_SIZE)
                .unwrap_err(),
            LibraryError::schema(SCHEMA_SEARCH_QUERY)
        );
        assert_eq!(
            EmptyLibrary
                .search_notes(r"C:\Users\someone\vault\note.md", None, DEFAULT_PAGE_SIZE)
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_QUERY)
        );
        assert_eq!(
            EmptyLibrary.inspect_search("", None).unwrap_err(),
            LibraryError::schema(SCHEMA_SEARCH_QUERY)
        );
        assert_eq!(
            EmptyLibrary
                .inspect_search(r"C:\Users\someone\vault\note.md", None)
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_QUERY)
        );
        assert_eq!(
            EmptyLibrary
                .inspect_search("欢迎", Some(r"C:\Users\someone\vault\note.md"))
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
        );
        assert_eq!(
            EmptyLibrary.inspect_search("欢迎", Some("")).unwrap_err(),
            LibraryError::schema(SCHEMA_NOTE_IDENTIFIER)
        );
        assert_eq!(
            EmptyLibrary.schema_validate("", None).unwrap_err(),
            LibraryError::schema(SCHEMA_NOTE_IDENTIFIER)
        );
        assert_eq!(
            EmptyLibrary
                .schema_validate(r"C:\Users\someone\vault\note.md", None)
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
        );
        assert_eq!(
            EmptyLibrary
                .schema_validate("welcome", Some(r"C:\Users\someone\vault\note.md"))
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_SCHEMA_ID)
        );
        assert_eq!(
            EmptyLibrary
                .schema_validate("welcome", Some(""))
                .unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_SCHEMA_ID)
        );
        let unknown = EmptyLibrary
            .schema_validate("welcome", Some("unknown-schema"))
            .unwrap();
        assert_eq!(unknown.verdict, SchemaVerdict::Unsupported);
        assert!(!unknown.engine_schema);
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
        assert_eq!(
            bound_recall_k(Some(0)).unwrap_err(),
            LibraryError::schema(SCHEMA_RECALL_K)
        );
        assert_eq!(
            bound_recall_k(Some(MAX_PAGE_SIZE + 1)).unwrap_err(),
            LibraryError::schema(SCHEMA_RECALL_K)
        );
        assert_eq!(bound_recall_k(None).unwrap(), DEFAULT_PAGE_SIZE);
        assert_eq!(
            EmptyLibrary.run_recall_benchmark(0).unwrap_err(),
            LibraryError::schema(SCHEMA_RECALL_K)
        );
        assert_eq!(
            EmptyLibrary
                .run_recall_benchmark(MAX_PAGE_SIZE + 1)
                .unwrap_err(),
            LibraryError::schema(SCHEMA_RECALL_K)
        );
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
        assert_eq!(
            accept_search_page(
                None,
                TruncatingLibrary.search_notes("欢迎", None, 2).unwrap()
            )
            .unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping_search = LoopingLibrary
            .search_notes("loop", Some("same"), 2)
            .unwrap();
        assert_eq!(
            accept_search_page(Some("same"), looping_search).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            accept_activity_page(None, TruncatingLibrary.list_activity(None, 2).unwrap())
                .unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping_activity = LoopingLibrary.list_activity(Some("same"), 2).unwrap();
        assert_eq!(
            accept_activity_page(Some("same"), looping_activity).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            accept_resource_page(None, TruncatingLibrary.list_resources(None, 2).unwrap())
                .unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping_resources = LoopingLibrary.list_resources(Some("same"), 2).unwrap();
        assert_eq!(
            accept_resource_page(Some("same"), looping_resources).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            accept_prompt_page(None, TruncatingLibrary.list_prompts(None, 2).unwrap()).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping_prompts = LoopingLibrary.list_prompts(Some("same"), 2).unwrap();
        assert_eq!(
            accept_prompt_page(Some("same"), looping_prompts).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        assert_eq!(
            accept_cli_inventory(
                None,
                TruncatingLibrary
                    .list_cli_inventory("release", None, 2)
                    .unwrap()
            )
            .unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let looping_cli = LoopingLibrary
            .list_cli_inventory("release", Some("same"), 2)
            .unwrap();
        assert_eq!(
            accept_cli_inventory(Some("same"), looping_cli).unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        let claimed_resources = ResourcePageDto {
            engine_resources: true,
            ..empty_resource_page()
        };
        assert_eq!(
            accept_resource_page(None, claimed_resources).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let claimed_prompts = PromptPageDto {
            engine_prompts: true,
            ..empty_prompt_page()
        };
        assert_eq!(
            accept_prompt_page(None, claimed_prompts).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
    }

    #[test]
    fn fixture_lexical_search_hits_match_physical_utf8_including_chinese() {
        let fixture = TempFixture::create();
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]]。",
        );
        fixture.write_note("欢迎", "欢迎", "第二篇中文正文，不含检索独有词。");
        fixture.write_note("alpha", "alpha", "English body without the CJK query.");
        let library = FixtureLibrary::new(fixture.dir.clone());
        let page = library.search_notes("欢迎", None, 2).unwrap();
        assert!(!page.hits.is_empty());
        assert!(!page.truncated);
        assert!(!page.semantic_enabled);
        assert!(!page.engine_search);
        assert_eq!(page.observation.classified_as, NoteCrudClass::DiskVerified);
        assert!(page.observation.disk_verified);
        assert!(page.observation.envelope_is_not_disk_proof);
        for hit in &page.hits {
            assert_ne!(hit.lexical_score, hit.semantic_score);
            assert_eq!(hit.semantic_score, SEMANTIC_SCORE_DISABLED);
            assert!(hit.lexical_score > SEMANTIC_SCORE_DISABLED);
            assert!(!looks_like_filesystem_path(&hit.identifier));
            let path = fixture.dir.join(format!("{}.md", hit.identifier));
            let disk = fs::read_to_string(&path).unwrap();
            assert!(
                disk.contains("欢迎"),
                "hit permalink must match a physical UTF-8 file that contains the query"
            );
        }
        assert!(page.hits.iter().any(|hit| hit.identifier == "welcome"));
        assert!(page.hits.iter().any(|hit| hit.identifier == "欢迎"));
        assert!(!page.hits.iter().any(|hit| hit.identifier == "alpha"));
        let missed = library
            .search_notes("独有哨兵词XYZ", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(missed.hits.is_empty());
        assert_eq!(missed.observation.classified_as, NoteCrudClass::Empty);
        assert!(!missed.semantic_enabled);
        let inspector = library.inspect_search("欢迎", None).unwrap();
        assert!(!inspector.hits.is_empty());
        assert!(!inspector.semantic_enabled);
        assert!(!inspector.model_loaded);
        assert!(inspector.model_id.is_none());
        assert_eq!(inspector.embedding_backend, EmbeddingBackend::None);
        assert_eq!(inspector.model_class, ModelClass::Unclassified);
        assert!(!inspector.files_written);
        assert_eq!(
            inspector.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        for hit in &inspector.hits {
            assert_ne!(hit.lexical_score, hit.semantic_score);
            assert_eq!(hit.semantic_score, SEMANTIC_SCORE_DISABLED);
            assert!(hit.lexical_score > SEMANTIC_SCORE_DISABLED);
            let path = fixture.dir.join(format!("{}.md", hit.identifier));
            let disk = fs::read_to_string(&path).unwrap();
            assert!(disk.contains("欢迎"));
        }
        let focused = library.inspect_search("欢迎", Some("欢迎")).unwrap();
        assert_eq!(focused.hits.len(), 1);
        assert_eq!(focused.hits[0].identifier, "欢迎");
        assert!(!focused.model_loaded);
        let claimed = SearchInspectorDto {
            model_loaded: true,
            model_id: Some("unknown-model".to_owned()),
            ..inspector
        };
        assert_eq!(
            accept_search_inspector(claimed).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
    }

    #[test]
    fn fixture_chinese_recall_at_k_matches_physical_utf8_gold() {
        let fixture = TempFixture::create();
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]]。",
        );
        fixture.write_note("欢迎", "欢迎", "第二篇中文正文，不含检索独有词。");
        fixture.write_note("alpha", "alpha", "English body without the CJK query.");
        let library = FixtureLibrary::new(fixture.dir.clone());
        let full = library.run_recall_benchmark(DEFAULT_PAGE_SIZE).unwrap();
        assert_eq!(full.k, DEFAULT_PAGE_SIZE);
        assert_eq!(full.query_count, 1);
        assert_eq!(full.queries.len(), 1);
        assert_eq!(full.queries[0].query, "欢迎");
        assert!(!full.semantic_enabled);
        assert!(!full.engine_search);
        assert!(!full.native_gui);
        assert!(!full.files_written);
        assert_eq!(full.observation.classified_as, NoteCrudClass::DiskVerified);
        assert!(full.observation.disk_verified);
        assert!(full.observation.envelope_is_not_disk_proof);
        assert!(full.search_elapsed_ms < 60_000);
        assert!(full.expand_elapsed_ms < 60_000);
        assert!(full.chinese_permalinks.iter().any(|item| item == "欢迎"));
        for identifier in &full.queries[0].relevant {
            assert!(!looks_like_filesystem_path(identifier));
            let path = fixture.dir.join(format!("{identifier}.md"));
            assert!(
                path.is_file(),
                "gold permalink must match a physical UTF-8 file"
            );
            let disk = fs::read_to_string(&path).unwrap();
            assert!(
                disk.contains("欢迎"),
                "gold permalink must contain the query on disk"
            );
        }
        for identifier in &full.queries[0].hits {
            let path = fixture.dir.join(format!("{identifier}.md"));
            let disk = fs::read_to_string(&path).unwrap();
            assert!(
                disk.contains("欢迎"),
                "a hit counts only if the physical file contains the query"
            );
        }
        assert!(full.queries[0]
            .relevant
            .iter()
            .any(|item| item == "welcome"));
        assert!(full.queries[0].relevant.iter().any(|item| item == "欢迎"));
        assert!(!full.queries[0].relevant.iter().any(|item| item == "alpha"));
        assert_eq!(full.recall_hits, full.recall_relevant);
        assert_eq!(full.recall_relevant, 2);
        let graph = library
            .expand_graph("欢迎", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(graph.nodes.iter().any(|node| node.identifier == "欢迎"));
        let search = library
            .search_notes("欢迎", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(search.hits.iter().any(|hit| hit.identifier == "欢迎"));
        let limited = library.run_recall_benchmark(1).unwrap();
        assert_eq!(limited.k, 1);
        assert_eq!(limited.queries[0].hits.len(), 1);
        assert_eq!(limited.recall_relevant, 2);
        assert_eq!(limited.recall_hits, 1);
        let missing = TempFixture::create();
        fixture_write_missing_gold(&missing);
        let empty_gold = FixtureLibrary::new(missing.dir.clone());
        let absent = empty_gold.run_recall_benchmark(DEFAULT_PAGE_SIZE).unwrap();
        assert_eq!(absent.recall_relevant, 0);
        assert_eq!(absent.observation.classified_as, NoteCrudClass::Empty);
        assert!(!absent.native_gui);
        let claimed = RecallBenchmarkDto {
            native_gui: true,
            semantic_enabled: true,
            ..empty_recall_benchmark(DEFAULT_PAGE_SIZE)
        };
        assert_eq!(
            accept_recall_benchmark(claimed).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let _ = (
            ENGINE_RECALL_NOT_OWNED,
            RECALL_NATIVE_UNVERIFIED,
            OFFICIAL_CHINESE_RECALL_UNVERIFIED,
            NATIVE_GUI_UNVERIFIED,
        );
    }

    #[test]
    fn fixture_schema_validate_observes_title_body_on_physical_utf8() {
        let fixture = TempFixture::create();
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]]。",
        );
        fs::write(
            fixture.dir.join("json-note.md"),
            "---\n{\"title\":\"欢迎\",\"body\":\"夹具 JSON-like 正文\"}\n---\n",
        )
        .unwrap();
        fs::write(fixture.dir.join("missing-body.md"), "# 只有标题\n").unwrap();
        fs::write(
            fixture.dir.join("missing-title.md"),
            "这是没有标题的夹具正文。\n",
        )
        .unwrap();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let valid = library.schema_validate("welcome", None).unwrap();
        assert_eq!(valid.schema_id, DEFAULT_SCHEMA_ID);
        assert_eq!(valid.verdict, SchemaVerdict::Valid);
        assert!(valid.observed_title);
        assert!(valid.observed_body);
        assert!(valid.missing_fields.is_empty());
        assert!(!valid.engine_schema);
        assert!(!valid.files_written);
        assert_eq!(valid.observation.classified_as, NoteCrudClass::DiskVerified);
        assert!(valid.observation.disk_verified);
        assert!(valid.observation.envelope_is_not_disk_proof);
        let disk = fs::read_to_string(fixture.dir.join("welcome.md")).unwrap();
        let (title, body) = parse_fixture_fields(&disk);
        assert_eq!(title.as_deref(), Some("中文夹具笔记"));
        assert!(body.as_deref().is_some_and(|value| value.contains("欢迎")));
        let json_note = library
            .schema_validate("json-note", Some(DEFAULT_SCHEMA_ID))
            .unwrap();
        assert_eq!(json_note.verdict, SchemaVerdict::Valid);
        assert!(json_note.observed_title);
        assert!(json_note.observed_body);
        let json_disk = fs::read_to_string(fixture.dir.join("json-note.md")).unwrap();
        let (json_title, json_body) = parse_fixture_fields(&json_disk);
        assert_eq!(json_title.as_deref(), Some("欢迎"));
        assert_eq!(json_body.as_deref(), Some("夹具 JSON-like 正文"));
        let missing_body = library.schema_validate("missing-body", None).unwrap();
        assert_eq!(missing_body.verdict, SchemaVerdict::Invalid);
        assert!(missing_body.observed_title);
        assert!(!missing_body.observed_body);
        assert_eq!(missing_body.missing_fields, vec![SCHEMA_FIELD_BODY]);
        assert_eq!(
            missing_body.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        let missing_title = library.schema_validate("missing-title", None).unwrap();
        assert_eq!(missing_title.verdict, SchemaVerdict::Invalid);
        assert!(!missing_title.observed_title);
        assert!(missing_title.observed_body);
        assert_eq!(missing_title.missing_fields, vec![SCHEMA_FIELD_TITLE]);
        let missing_file = library.schema_validate("absent", None).unwrap();
        assert_eq!(missing_file.verdict, SchemaVerdict::Empty);
        assert_eq!(missing_file.observation.classified_as, NoteCrudClass::Empty);
        let unknown = library
            .schema_validate("welcome", Some("unknown-schema"))
            .unwrap();
        assert_eq!(unknown.verdict, SchemaVerdict::Unsupported);
        assert!(!unknown.engine_schema);
        assert_ne!(SchemaVerdict::Valid, SchemaVerdict::Invalid);
        assert_ne!(SchemaVerdict::Valid, SchemaVerdict::Empty);
        assert_ne!(SchemaVerdict::Invalid, SchemaVerdict::Unsupported);
        let claimed = SchemaValidateDto {
            engine_schema: true,
            verdict: SchemaVerdict::Valid,
            observed_title: true,
            observed_body: true,
            ..empty_schema_report("welcome", DEFAULT_SCHEMA_ID)
        };
        assert_eq!(
            accept_schema_report(claimed).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let envelope_valid = SchemaValidateDto {
            verdict: SchemaVerdict::Valid,
            observed_title: true,
            observed_body: true,
            observation: NoteCrudObservationDto {
                classified_as: NoteCrudClass::AcceptedUnverified,
                disk_verified: false,
                envelope_is_not_disk_proof: true,
            },
            ..empty_schema_report("welcome", DEFAULT_SCHEMA_ID)
        };
        assert_eq!(
            accept_schema_report(envelope_valid).unwrap_err(),
            LibraryError::unsupported(UNSUPPORTED_TRUNCATED)
        );
        let _ = (ENGINE_SCHEMA_NOT_OWNED, OFFICIAL_SCHEMA_MCP_UNVERIFIED);
    }

    fn fixture_write_missing_gold(fixture: &TempFixture) {
        fixture.write_note("alpha", "alpha", "English body without the CJK query.");
    }

    #[test]
    fn fixture_preview_snippet_is_physical_substring_including_chinese() {
        let fixture = TempFixture::create();
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n<script>alert(1)</script>\n",
        );
        let library = FixtureLibrary::new(fixture.dir.clone());
        let preview = library.preview_context("welcome", Some("欢迎")).unwrap();
        let disk = fs::read_to_string(fixture.dir.join("welcome.md")).unwrap();
        assert!(disk.contains(&preview.snippet));
        assert!(preview.snippet.contains("欢迎"));
        assert!(!preview.executed);
        assert!(preview.unsafe_html_present);
        assert!(!preview.engine_context);
        assert_eq!(
            preview.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert!(preview.observation.disk_verified);
        assert!(preview.observation.envelope_is_not_disk_proof);
        let missing = library.preview_context("absent", None).unwrap();
        assert!(missing.snippet.is_empty());
        assert_eq!(missing.observation.classified_as, NoteCrudClass::Empty);
        assert!(!missing.observation.disk_verified);
        assert_eq!(
            library
                .preview_context(r"C:\Users\someone\vault\note.md", None)
                .unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
        );
    }

    #[test]
    fn fixture_activity_follows_physical_mtime_and_permalinks() {
        let fixture = TempFixture::create();
        let older = fixture.write_note("alpha", "alpha", "older fixture body");
        let newer = fixture.write_note("欢迎", "欢迎", "newer 中文夹具正文");
        let middle = fixture.write_note("welcome", "welcome", "middle fixture body");
        let t0 = UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_100);
        let t1 = UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_200);
        let t2 = UNIX_EPOCH + std::time::Duration::from_secs(1_700_000_300);
        set_file_mtime(&older, t0);
        set_file_mtime(&middle, t1);
        set_file_mtime(&newer, t2);
        let library = FixtureLibrary::new(fixture.dir.clone());
        let page = library.list_activity(None, 2).unwrap();
        assert_eq!(page.entries.len(), 2);
        assert_eq!(page.entries[0].identifier, "欢迎");
        assert_eq!(page.entries[1].identifier, "welcome");
        assert!(page.entries[0].observed_mtime >= page.entries[1].observed_mtime);
        assert!(!page.engine_activity);
        assert!(!page.truncated);
        assert_eq!(page.observation.classified_as, NoteCrudClass::DiskVerified);
        for entry in &page.entries {
            assert!(!looks_like_filesystem_path(&entry.identifier));
            assert!(fixture
                .dir
                .join(format!("{}.md", entry.identifier))
                .is_file());
        }
        let second = library
            .list_activity(page.next_cursor.as_deref(), 2)
            .unwrap();
        assert!(second
            .entries
            .iter()
            .any(|entry| entry.identifier == "alpha"));
        let missing_root = TempFixture::create();
        let empty = FixtureLibrary::new(missing_root.dir.clone())
            .list_activity(None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(empty.entries.is_empty());
        assert_eq!(empty.observation.classified_as, NoteCrudClass::Empty);
    }

    #[test]
    fn fixture_resources_and_prompts_match_physical_markdown_sidecars() {
        let fixture = TempFixture::create();
        fixture.write_note("alpha", "alpha", "resource body");
        fixture.write_note("欢迎", "欢迎资源", "中文资源正文");
        fixture.write_note("welcome", "welcome", "middle resource");
        fixture.write_prompt("summarize", "摘要模板", "请摘要这篇夹具笔记。");
        fixture.write_prompt("欢迎", "欢迎提示词", "中文提示词模板。");
        fixture.write_prompt("outline", "outline", "outline the fixture note");
        let library = FixtureLibrary::new(fixture.dir.clone());
        let resources = library.list_resources(None, 2).unwrap();
        assert_eq!(resources.entries.len(), 2);
        assert_eq!(resources.entries[0].identifier, "alpha");
        assert_eq!(resources.entries[1].identifier, "welcome");
        assert!(!resources.engine_resources);
        assert!(!resources.truncated);
        assert_eq!(
            resources.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert!(resources.observation.disk_verified);
        assert!(resources.observation.envelope_is_not_disk_proof);
        for entry in &resources.entries {
            assert!(!looks_like_filesystem_path(&entry.identifier));
            assert!(fixture
                .dir
                .join(format!("{}.md", entry.identifier))
                .is_file());
        }
        let resource_rest = library
            .list_resources(resources.next_cursor.as_deref(), 2)
            .unwrap();
        assert!(resource_rest
            .entries
            .iter()
            .any(|entry| entry.identifier == "欢迎"));
        assert!(fixture.dir.join("欢迎.md").is_file());
        let prompts = library.list_prompts(None, 2).unwrap();
        assert_eq!(prompts.entries.len(), 2);
        assert_eq!(prompts.entries[0].identifier, "outline");
        assert_eq!(prompts.entries[1].identifier, "summarize");
        assert!(!prompts.engine_prompts);
        assert!(!prompts.truncated);
        assert_eq!(
            prompts.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert!(prompts.observation.disk_verified);
        assert!(prompts.observation.envelope_is_not_disk_proof);
        for entry in &prompts.entries {
            assert!(!looks_like_filesystem_path(&entry.identifier));
            assert!(fixture
                .dir
                .join(format!("{}{PROMPT_SIDECAR_SUFFIX}", entry.identifier))
                .is_file());
        }
        let prompt_rest = library
            .list_prompts(prompts.next_cursor.as_deref(), 2)
            .unwrap();
        assert!(prompt_rest
            .entries
            .iter()
            .any(|entry| entry.identifier == "欢迎"));
        assert!(fixture
            .dir
            .join(format!("欢迎{PROMPT_SIDECAR_SUFFIX}"))
            .is_file());
        assert!(!resources
            .entries
            .iter()
            .any(|entry| entry.identifier == "summarize"));
        assert!(!prompts
            .entries
            .iter()
            .any(|entry| entry.identifier == "welcome"));
        let missing_root = TempFixture::create();
        let empty_resources = FixtureLibrary::new(missing_root.dir.clone())
            .list_resources(None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(empty_resources.entries.is_empty());
        assert_eq!(
            empty_resources.observation.classified_as,
            NoteCrudClass::Empty
        );
        let empty_prompts = FixtureLibrary::new(missing_root.dir.clone())
            .list_prompts(None, DEFAULT_PAGE_SIZE)
            .unwrap();
        assert!(empty_prompts.entries.is_empty());
        assert_eq!(
            empty_prompts.observation.classified_as,
            NoteCrudClass::Empty
        );
        let _ = (
            ENGINE_RESOURCES_NOT_OWNED,
            ENGINE_PROMPTS_NOT_OWNED,
            OFFICIAL_RESOURCES_MCP_UNVERIFIED,
            OFFICIAL_PROMPTS_MCP_UNVERIFIED,
        );
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

    struct TempCrud {
        dir: PathBuf,
    }

    impl TempCrud {
        fn create() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("bmdock-t15-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }
    }

    impl Drop for TempCrud {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    struct TempImport {
        dir: PathBuf,
    }

    impl TempImport {
        fn create() -> Self {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_nanos())
                .unwrap_or(0);
            let seq = FIXTURE_SEQ.fetch_add(1, Ordering::Relaxed);
            let dir = std::env::temp_dir().join(format!("bmdock-t28-{nanos}-{seq}"));
            fs::create_dir_all(&dir).unwrap();
            Self { dir }
        }
    }

    impl Drop for TempImport {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }

    fn chinese_body() -> &'static str {
        "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]]。\n"
    }

    #[test]
    fn empty_library_crud_is_unsupported() {
        let library = EmptyLibrary;
        let body = chinese_body();
        let write = library
            .write_note("welcome", "中文夹具笔记", body)
            .unwrap_err();
        assert_eq!(
            write,
            LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
        let edit = library.edit_note("welcome", body).unwrap_err();
        assert_eq!(
            edit,
            LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
        let moved = library.move_note("welcome", "renamed").unwrap_err();
        assert_eq!(
            moved,
            LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
        let deleted = library.delete_note("welcome").unwrap_err();
        assert_eq!(
            deleted,
            LibraryError::unsupported(UNSUPPORTED_LIBRARY_UNAVAILABLE)
        );
    }

    #[test]
    fn fixture_write_observes_physical_markdown_not_envelope() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let body = chinese_body();
        let written = library.write_note("welcome", "中文夹具笔记", body).unwrap();
        let dest = fixture.dir.join("welcome.md");
        assert!(dest.is_file(), "write must observe a physical owned file");
        let disk = fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("[[欢迎]]"));
        assert_eq!(written.body, body);
        assert_eq!(written.title, "中文夹具笔记");
        assert!(written.files_written);
        assert!(!written.engine_persisted);
        assert!(written.observation.disk_verified);
        assert!(written.observation.envelope_is_not_disk_proof);
        assert_eq!(
            written.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert!(!written.scanned_user_obsidian_vault);
        assert!(!written.scanned_user_basic_memory_home);
        let read = library.read_note("welcome").unwrap();
        assert_eq!(read.body, disk);
        assert_eq!(
            read.observation.classified_as,
            ObservationClass::BodyMatchesDisk
        );
    }

    #[test]
    fn edit_overwrites_existing_identifier_on_disk() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let original = chinese_body();
        library
            .write_note("welcome", "中文夹具笔记", original)
            .unwrap();
        let replacement = "# 覆盖后的笔记\n\n替换后的中文正文 [[欢迎]]。\n";
        let edited = library.edit_note("welcome", replacement).unwrap();
        let disk = fs::read_to_string(fixture.dir.join("welcome.md")).unwrap();
        assert_eq!(disk, replacement);
        assert_ne!(disk, original);
        assert_eq!(edited.body, replacement);
        assert!(edited.files_written);
        assert!(!edited.engine_persisted);
        assert!(edited.observation.disk_verified);
        assert_eq!(
            edited.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        let missing = library.edit_note("missing", replacement).unwrap_err();
        assert_eq!(missing, LibraryError::unsupported(UNSUPPORTED_NOTE_MISSING));
    }

    #[test]
    fn move_renames_identifier_within_owned_library() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let body = chinese_body();
        library.write_note("welcome", "中文夹具笔记", body).unwrap();
        let moved = library.move_note("welcome", "renamed").unwrap();
        let old_path = fixture.dir.join("welcome.md");
        let new_path = fixture.dir.join("renamed.md");
        assert!(!old_path.exists(), "old path must be gone after move");
        assert!(new_path.is_file(), "new path must exist after move");
        let disk = fs::read_to_string(&new_path).unwrap();
        assert_eq!(disk, body);
        assert_eq!(moved.body, body);
        assert_eq!(moved.identifier, "welcome");
        assert_eq!(moved.destination, "renamed");
        assert!(moved.files_written);
        assert!(!moved.engine_persisted);
        assert!(moved.observation.disk_verified);
        assert_eq!(moved.observation.classified_as, NoteCrudClass::DiskVerified);
        let filesystem = library
            .move_note("renamed", r"C:\Users\someone\vault\note.md")
            .unwrap_err();
        assert_eq!(
            filesystem,
            LibraryError::policy(POLICY_FILESYSTEM_DESTINATION)
        );
        assert!(new_path.is_file());
    }

    #[test]
    fn sequential_move_onto_existing_destination_is_unsupported() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let body = chinese_body();
        library.write_note("welcome", "中文夹具笔记", body).unwrap();
        library
            .write_note("renamed", "已有目标", "other body\n")
            .unwrap();
        let error = library.move_note("welcome", "renamed").unwrap_err();
        assert_eq!(
            error,
            LibraryError::unsupported(UNSUPPORTED_MOVE_DESTINATION_EXISTS)
        );
        let old_path = fixture.dir.join("welcome.md");
        let dest_path = fixture.dir.join("renamed.md");
        assert!(
            old_path.is_file(),
            "source must remain after sequential reject"
        );
        assert!(dest_path.is_file(), "existing destination must remain");
        assert_eq!(fs::read_to_string(&old_path).unwrap(), body);
        assert_eq!(fs::read_to_string(&dest_path).unwrap(), "other body\n");
    }

    #[test]
    fn delete_removes_physical_file_missing_is_observation() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let body = chinese_body();
        library.write_note("welcome", "中文夹具笔记", body).unwrap();
        let dest = fixture.dir.join("welcome.md");
        assert!(dest.is_file());
        let deleted = library.delete_note("welcome").unwrap();
        assert!(!dest.exists(), "delete must remove the physical file");
        assert!(deleted.files_written);
        assert!(!deleted.engine_persisted);
        assert!(deleted.observation.disk_verified);
        assert!(deleted.observation.envelope_is_not_disk_proof);
        assert_eq!(
            deleted.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        let missing = library.delete_note("welcome").unwrap();
        assert!(!dest.exists());
        assert!(!missing.files_written);
        assert!(missing.observation.disk_verified);
        assert_eq!(
            missing.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert!(!missing.scanned_user_obsidian_vault);
    }

    #[test]
    fn envelope_crud_saved_text_is_not_disk_proof() {
        let library = EnvelopeCrudLibrary;
        let written = library
            .write_note("welcome", "中文夹具笔记", chinese_body())
            .unwrap();
        assert_eq!(written.body, "saved");
        assert!(!written.files_written);
        assert!(!written.engine_persisted);
        assert!(!written.observation.disk_verified);
        assert_eq!(
            written.observation.classified_as,
            NoteCrudClass::AcceptedUnverified
        );
        let edited = library.edit_note("welcome", chinese_body()).unwrap();
        assert!(!edited.observation.disk_verified);
        assert_eq!(
            edited.observation.classified_as,
            NoteCrudClass::AcceptedUnverified
        );
        let moved = library.move_note("welcome", "renamed").unwrap();
        assert!(!moved.observation.disk_verified);
        let deleted = library.delete_note("welcome").unwrap();
        assert!(!deleted.files_written);
        assert!(!deleted.observation.disk_verified);
        assert_eq!(
            deleted.observation.classified_as,
            NoteCrudClass::AcceptedUnverified
        );
    }

    #[test]
    fn filesystem_identifier_and_destination_are_policy() {
        let library = EmptyLibrary;
        for identifier in [
            r"C:\Users\someone\vault\note.md",
            r"%APPDATA%\Obsidian\vault",
            "/home/someone/.basic-memory/note",
            "../secret",
        ] {
            assert_eq!(
                library.write_note(identifier, "title", "body").unwrap_err(),
                LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
            );
            assert_eq!(
                library.edit_note(identifier, "body").unwrap_err(),
                LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
            );
            assert_eq!(
                library.delete_note(identifier).unwrap_err(),
                LibraryError::policy(POLICY_FILESYSTEM_IDENTIFIER)
            );
            assert_eq!(
                library.move_note("welcome", identifier).unwrap_err(),
                LibraryError::policy(POLICY_FILESYSTEM_DESTINATION)
            );
        }
        assert_eq!(
            reject_note_identifier(""),
            Err(LibraryError::schema(SCHEMA_NOTE_IDENTIFIER))
        );
        assert_eq!(
            reject_empty_title("  "),
            Err(LibraryError::schema(SCHEMA_NOTE_TITLE))
        );
        assert_eq!(
            EmptyLibrary.move_note("welcome", "welcome").unwrap_err(),
            LibraryError::schema(SCHEMA_MOVE_SAME_IDENTIFIER)
        );
    }

    #[test]
    fn forbidden_crud_roots_and_t11_roots_are_policy() {
        for path in [
            Path::new(r"C:\Users\someone\AppData\Roaming\Obsidian"),
            Path::new(r"C:\Users\someone\.basic-memory"),
            Path::new("/home/someone/.basic-memory"),
            Path::new("%APPDATA%\\Obsidian"),
        ] {
            assert!(library_root_is_forbidden(path));
            assert_eq!(
                reject_forbidden_library_root(path).unwrap_err(),
                LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT)
            );
        }
        let t11 = TempFixture::create();
        t11.write_note("welcome", "笔记", "body");
        let t11_library = FixtureLibrary::new(t11.dir.clone());
        let denied = t11_library
            .write_note("welcome", "中文夹具笔记", chinese_body())
            .unwrap_err();
        assert_eq!(denied, LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT));
        let unmarked = FixtureLibrary::new(std::env::temp_dir().join("other-notes"));
        assert_eq!(
            unmarked.write_note("welcome", "title", "body").unwrap_err(),
            LibraryError::policy(POLICY_FORBIDDEN_LIBRARY_ROOT)
        );
    }

    #[test]
    fn unsafe_html_crlf_and_wiki_roundtrip_as_exact_bytes() {
        let fixture = TempCrud::create();
        let library = FixtureLibrary::new(fixture.dir.clone());
        let body = crate::content_safety::UNSAFE_HTML_WIKI_CRLF_BODY;
        let class = crate::content_safety::classify_body(body);
        assert!(class.unsafe_html_present);
        assert!(!class.executed);
        assert_eq!(
            class.line_endings,
            crate::content_safety::LineEndingClass::Crlf
        );
        let written = library.write_note("welcome", "中文夹具笔记", body).unwrap();
        let dest = fixture.dir.join("welcome.md");
        let disk = crate::content_safety::read_exact_bytes(&dest).unwrap();
        assert_eq!(disk, body.as_bytes());
        assert!(disk.windows(2).any(|pair| pair == b"\r\n"));
        let text = std::str::from_utf8(&disk).unwrap();
        assert!(text.contains("<script>"));
        assert!(text.contains("onerror="));
        assert!(text.contains("[[欢迎]]"));
        assert_eq!(written.body, body);
        assert!(written.observation.disk_verified);
        assert_eq!(
            written.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        let read = library.read_note("welcome").unwrap();
        assert_eq!(read.body, body);
        assert_eq!(read.body.as_bytes(), disk.as_slice());

        crate::content_safety::persist_exact_utf8(&dest, &body.replace("\r\n", "\n")).unwrap();
        let lost = crate::content_safety::read_exact_bytes(&dest).unwrap();
        assert!(crate::content_safety::crlf_normalized_to_lf(body, &lost));
        assert!(!crate::content_safety::disk_verified_from_bytes(
            body, &lost
        ));
        let edited = library.edit_note("welcome", body).unwrap();
        assert!(edited.observation.disk_verified);
        assert_eq!(
            crate::content_safety::read_exact_bytes(&dest).unwrap(),
            body.as_bytes()
        );
    }

    #[test]
    fn wiki_links_are_permalinks_not_filesystem_paths() {
        let extracted = extract_wiki_link_identifiers(
            "# 中文夹具笔记\n\n参见 [[欢迎]] 与 [[missing-target]]。\n[[欢迎|别名]]\n[[C:\\Users\\someone\\vault\\note.md]]\n",
        );
        assert_eq!(
            extracted,
            vec!["欢迎".to_string(), "missing-target".to_owned()]
        );
        assert!(!extracted
            .iter()
            .any(|item| item.contains('\\') || item.contains(':')));
    }

    #[test]
    fn fixture_relations_match_physical_wiki_links_and_missing_is_empty() {
        let fixture = TempFixture::create();
        let body =
            "# 中文夹具笔记\n\n这是 BMDock 自有夹具正文。参见 [[欢迎]] 与 [[missing-target]]。\n";
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]] 与 [[missing-target]]。",
        );
        fixture.write_note("欢迎", "欢迎", "目标正文");
        let library = FixtureLibrary::new(fixture.dir.clone());
        let listed = library.list_relations("welcome").unwrap();
        let disk = fs::read_to_string(fixture.dir.join("welcome.md")).unwrap();
        assert!(disk.contains("[[欢迎]]"));
        assert!(disk.contains("[[missing-target]]"));
        assert_eq!(
            listed
                .relations
                .iter()
                .map(|item| item.identifier.as_str())
                .collect::<Vec<_>>(),
            extract_wiki_link_identifiers(&disk)
        );
        assert_eq!(listed.relations.len(), 2);
        assert_eq!(listed.relations[0].identifier, "欢迎");
        assert_eq!(
            listed.relations[0].classified_as,
            RelationTargetClass::Present
        );
        assert_eq!(listed.relations[1].identifier, "missing-target");
        assert_eq!(
            listed.relations[1].classified_as,
            RelationTargetClass::Empty
        );
        assert!(!listed.relations.iter().any(|item| {
            item.identifier.contains('\\')
                || item.identifier.contains(':')
                || item.identifier.contains(".basic-memory")
        }));
        assert!(!listed.engine_graph);
        assert!(!listed.files_written);
        assert!(!listed.scanned_user_obsidian_vault);
        assert!(!listed.scanned_user_basic_memory_home);
        assert!(listed.observation.disk_verified);
        assert_eq!(
            listed.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        assert_ne!(listed.observation.classified_as, NoteCrudClass::Conflict);
        let missing_source = library.list_relations("absent").unwrap_err();
        assert_eq!(
            missing_source,
            LibraryError::unsupported(UNSUPPORTED_NOTE_MISSING)
        );
        let _ = body;
        let _ = (
            ENGINE_GRAPH_NOT_OWNED,
            RECENT_ACTIVITY_MCP_UNVERIFIED,
            BUILD_CONTEXT_MCP_UNVERIFIED,
        );
    }

    #[test]
    fn fixture_graph_one_hop_matches_physical_wiki_links_and_second_hop_reads_neighbor() {
        let fixture = TempFixture::create();
        fixture.write_note(
            "welcome",
            "中文夹具笔记",
            "这是 BMDock 自有夹具正文。参见 [[欢迎]] 与 [[alpha]] [[beta]] [[gamma]] [[delta]] 与 [[missing-target]]。",
        );
        fixture.write_note(
            "欢迎",
            "欢迎",
            "第二跳正文。参见 [[second-hop]] 与 [[missing-second]]。",
        );
        fixture.write_note("second-hop", "second-hop", "第三层正文");
        let library = FixtureLibrary::new(fixture.dir.clone());
        let first = library.expand_graph("welcome", None, 2).unwrap();
        let disk = fs::read_to_string(fixture.dir.join("welcome.md")).unwrap();
        let expected = extract_wiki_link_identifiers(&disk);
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
        assert_eq!(first.page, 1);
        assert_eq!(first.depth, GRAPH_DEPTH);
        assert!(!first.truncated);
        assert!(!first.engine_graph);
        assert_eq!(first.nodes[0].identifier, "welcome");
        assert_eq!(first.nodes[0].classified_as, GraphNodeClass::Present);
        assert_eq!(first.nodes[1].identifier, "欢迎");
        assert_eq!(first.nodes[1].classified_as, GraphNodeClass::Present);
        assert_eq!(first.nodes[2].identifier, "alpha");
        assert_eq!(first.nodes[2].classified_as, GraphNodeClass::Empty);
        assert!(!first.nodes.iter().any(|node| {
            node.identifier.contains('\\')
                || node.identifier.contains(':')
                || node.identifier.contains('/')
        }));
        assert_eq!(first.observation.classified_as, NoteCrudClass::DiskVerified);
        assert_ne!(first.observation.classified_as, NoteCrudClass::Conflict);
        assert!(first.observation.disk_verified);
        assert!(first.nodes.len() <= 3);
        assert!(!first
            .nodes
            .iter()
            .any(|node| node.identifier == "second-hop"));
        let second = library
            .expand_graph("welcome", first.next_cursor.as_deref(), 2)
            .unwrap();
        assert_eq!(second.page, 2);
        assert_eq!(
            second
                .edges
                .iter()
                .map(|edge| edge.target.clone())
                .collect::<Vec<_>>(),
            expected[2..4]
        );
        assert!(!second
            .nodes
            .iter()
            .any(|node| node.identifier == "second-hop"));
        let rejected = accept_graph_page(
            first.next_cursor.as_deref(),
            GraphPageDto {
                next_cursor: first.next_cursor.clone(),
                ..second.clone()
            },
        );
        assert_eq!(
            rejected.unwrap_err(),
            LibraryError::schema(SCHEMA_INVALID_CURSOR)
        );
        let (all_edges, pages) = follow_graph_pages(&library, "welcome", 2).unwrap();
        assert_eq!(all_edges.len(), expected.len());
        assert_eq!(pages, 3);
        assert!(all_edges.len() < 20);
        let hop = library
            .expand_graph("欢迎", None, DEFAULT_PAGE_SIZE)
            .unwrap();
        let neighbor_disk = fs::read_to_string(fixture.dir.join("欢迎.md")).unwrap();
        assert_eq!(
            hop.edges
                .iter()
                .map(|edge| edge.target.clone())
                .collect::<Vec<_>>(),
            extract_wiki_link_identifiers(&neighbor_disk)
        );
        assert_eq!(hop.identifier, "欢迎");
        assert_eq!(hop.nodes[0].identifier, "欢迎");
        assert!(hop.nodes.iter().any(|node| node.identifier == "second-hop"));
        assert_eq!(
            hop.nodes
                .iter()
                .find(|node| node.identifier == "missing-second")
                .unwrap()
                .classified_as,
            GraphNodeClass::Empty
        );
        assert!(!hop.nodes.iter().any(|node| node.identifier == "alpha"));
        assert!(!hop.nodes.iter().any(|node| node.identifier == "welcome"));
        assert_eq!(hop.observation.classified_as, NoteCrudClass::DiskVerified);
        assert_eq!(hop.depth, GRAPH_DEPTH);
        let truncated = accept_graph_page(
            None,
            TruncatingLibrary.expand_graph("welcome", None, 2).unwrap(),
        )
        .unwrap_err();
        assert_eq!(truncated, LibraryError::unsupported(UNSUPPORTED_TRUNCATED));
        let missing_source = library.expand_graph("absent", None, 2).unwrap_err();
        assert_eq!(
            missing_source,
            LibraryError::unsupported(UNSUPPORTED_NOTE_MISSING)
        );
        let _ = (NATIVE_GUI_UNVERIFIED, BOUNDED_HOST_EXPANSION);
    }

    #[test]
    fn fixture_import_copies_owned_markdown_and_observes_utf8() {
        let fixture = TempImport::create();
        let library_root = fixture.dir.join("library");
        let sources_root = fixture.dir.join("sources");
        fs::create_dir_all(&library_root).unwrap();
        fs::create_dir_all(&sources_root).unwrap();
        let library = FixtureLibrary::with_import_sources(library_root.clone(), sources_root);
        let body = "# 欢迎\n\n这是导入夹具正文。\n";
        library
            .seed_import_source("fixture-welcome", &[("欢迎", body)])
            .unwrap();
        let imported = library.import_notes("fixture-welcome").unwrap();
        assert_eq!(imported.source_id, "fixture-welcome");
        assert!(imported.files_written);
        assert!(!imported.engine_import);
        assert!(!imported.scanned_user_obsidian_vault);
        assert!(!imported.scanned_user_basic_memory_home);
        assert!(imported.observation.envelope_is_not_disk_proof);
        assert_eq!(
            imported.observation.classified_as,
            ImportClass::DiskVerified
        );
        assert!(imported.observation.disk_verified);
        assert_eq!(imported.files.len(), 1);
        assert_eq!(imported.files[0].identifier, "欢迎");
        assert_eq!(imported.files[0].kind, crate::routing::OWNED_KIND);
        let dest = library_root.join("欢迎.md");
        assert!(dest.is_file(), "import must observe a physical owned file");
        let disk = fs::read_to_string(&dest).unwrap();
        assert_eq!(disk, body);
        assert!(disk.contains("欢迎"));
        assert_ne!(
            imported.observation.classified_as,
            ImportClass::AcceptedUnverified
        );
        let missing = library.import_notes("fixture-absent").unwrap();
        assert!(missing.files.is_empty());
        assert!(!missing.files_written);
        assert_eq!(missing.observation.classified_as, ImportClass::Empty);
        assert_eq!(
            library.import_notes(r"%APPDATA%\Obsidian").unwrap_err(),
            LibraryError::policy(POLICY_FILESYSTEM_SOURCE)
        );
        let claimed = ImportResultDto {
            engine_import: true,
            ..empty_import_result("fixture-welcome")
        };
        assert_eq!(
            accept_import_result(claimed).unwrap_err(),
            LibraryError::unsupported(ENGINE_IMPORT_NOT_OWNED)
        );
        let envelope = ImportResultDto {
            files: vec![ImportedFileDto {
                identifier: "欢迎".to_owned(),
                kind: crate::routing::OWNED_KIND.to_owned(),
            }],
            files_written: false,
            observation: ImportObservationDto {
                classified_as: ImportClass::DiskVerified,
                disk_verified: true,
                envelope_is_not_disk_proof: false,
            },
            ..empty_import_result("fixture-welcome")
        };
        let accepted = accept_import_result(envelope).unwrap();
        assert!(!accepted.files_written);
        assert!(!accepted.observation.disk_verified);
        assert!(accepted.observation.envelope_is_not_disk_proof);
        assert_eq!(
            accepted.observation.classified_as,
            ImportClass::AcceptedUnverified
        );
        assert_ne!(
            accepted.observation.classified_as,
            ImportClass::DiskVerified
        );
    }
}
