//! Typed projections of the two pinned engines. The official index owns ranking.
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Number, Value};

use crate::{
    engine_session::{error, malformed, require_route, unavailable, SessionIdentity},
    ipc::{ErrorCategory, IpcCommand, IpcError, IpcResponse, FIXTURE_PROJECT},
    library,
    routing::OWNED_WORKSPACE_ID,
    supervisor::EngineProfile,
};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct SearchOptions {
    pub mode: Option<String>,
    pub entity_types: Option<Vec<String>>,
    pub note_types: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub metadata_filters: Option<Map<String, Value>>,
    pub status: Option<String>,
    pub after_date: Option<String>,
    pub min_similarity: Option<Number>,
    pub compact: Option<bool>,
    pub valid_at: Option<String>,
    pub valid_overlaps: Option<String>,
    pub time_kind: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ContextOptions {
    pub depth: Option<u32>,
    pub max_related: Option<u32>,
    pub timeframe: Option<String>,
    pub compact: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ActivityOptions {
    pub types: Option<Vec<String>>,
    pub depth: Option<u32>,
    pub timeframe: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct QueryIdentity {
    pub session: SessionIdentity,
    pub workspace: &'static str,
    pub project: &'static str,
    pub operation: &'static str,
    /// Exact, normalized arguments sent to the selected official tool.
    pub arguments: Value,
    /// Consumer-owned epoch, echoed only; the backend has no result cache.
    pub request_generation: u32,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EngineHitDto {
    pub result_kind: String,
    /// Identity of this result, distinct from its owning note.
    pub identifier: String,
    pub note_identifier: Option<String>,
    pub title: Option<String>,
    pub excerpt: Option<String>,
    pub score: Option<Number>,
    pub file_path: Option<String>,
    pub category: Option<String>,
    pub relation_type: Option<String>,
    pub from_entity: Option<String>,
    pub to_entity: Option<String>,
    pub to_name: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EngineSearchPageDto {
    pub engine_search: bool,
    pub session: SessionIdentity,
    pub request: QueryIdentity,
    pub query: String,
    pub hits: Vec<EngineHitDto>,
    pub page: u32,
    pub page_size: u32,
    pub next_cursor: Option<String>,
    pub has_more: bool,
    pub total: u64,
    pub total_is_exact: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct ContextGroupDto {
    pub primary_result: EngineHitDto,
    pub observations: Vec<EngineHitDto>,
    pub related_results: Vec<EngineHitDto>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EngineContextDto {
    pub engine_context: bool,
    pub session: SessionIdentity,
    pub request: QueryIdentity,
    pub identifier: String,
    pub results: Vec<ContextGroupDto>,
    pub metadata: Map<String, Value>,
    pub page: u32,
    pub page_size: u32,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct EngineActivityDto {
    pub engine_activity: bool,
    pub session: SessionIdentity,
    pub request: QueryIdentity,
    pub entries: Vec<EngineHitDto>,
    pub page: u32,
    pub page_size: u32,
    /// The captured list has no continuation or count metadata.
    pub next_cursor: Option<String>,
    pub has_more: Option<bool>,
    pub total: Option<u64>,
    pub total_is_exact: Option<bool>,
}

pub struct PreparedQuery {
    expected: Option<SessionIdentity>,
    operation: &'static str,
    tool: &'static str,
    arguments: Value,
    request_generation: u32,
    subject: String,
    page: u32,
    page_size: u32,
}

fn page(cursor: Option<&str>) -> Result<u32, IpcError> {
    match cursor {
        None => Ok(1),
        Some(cursor) => cursor
            .parse::<u32>()
            .ok()
            .filter(|n| *n > 1)
            .ok_or_else(|| error(ErrorCategory::Schema, "Invalid official page cursor")),
    }
}

fn depth(value: Option<u32>) -> Result<u32, IpcError> {
    let value = value.unwrap_or(1);
    if !(1..=3).contains(&value) {
        return Err(error(
            ErrorCategory::Schema,
            "Relation depth must be between 1 and 3",
        ));
    }
    Ok(value)
}

fn require_preview(expected: Option<&SessionIdentity>, used: bool) -> Result<(), IpcError> {
    if used && !expected.is_some_and(|s| s.profile == EngineProfile::MainPreview) {
        return Err(unavailable(
            "This query option requires the main-preview profile",
        ));
    }
    Ok(())
}

impl PreparedQuery {
    pub fn from_command(command: &IpcCommand) -> Option<Result<Self, IpcError>> {
        match command {
            IpcCommand::SearchNotes(args) => Some((|| {
                require_route(&args.workspace, &args.project)?;
                let page = page(args.cursor.as_deref())?;
                let page_size = library::bound_page_size(Some(args.page_size.unwrap_or(50)))?;
                let options = &args.options;
                let query = if args.query.trim().is_empty() {
                    let has_filter = [
                        &options.entity_types,
                        &options.note_types,
                        &options.categories,
                        &options.tags,
                    ]
                    .iter()
                    .any(|value| value.as_ref().is_some_and(|items| !items.is_empty()))
                        || options
                            .metadata_filters
                            .as_ref()
                            .is_some_and(|filters| !filters.is_empty())
                        || [
                            &options.status,
                            &options.after_date,
                            &options.valid_at,
                            &options.valid_overlaps,
                            &options.time_kind,
                        ]
                        .iter()
                        .any(|value| value.as_ref().is_some_and(|text| !text.trim().is_empty()));
                    if !has_filter {
                        return Err(error(
                            ErrorCategory::Schema,
                            "A query or a supported filter is required",
                        ));
                    }
                    None
                } else {
                    library::reject_search_query(&args.query)?;
                    Some(args.query.as_str())
                };
                let mode = options.mode.as_deref().unwrap_or("text");
                if !matches!(mode, "text" | "title" | "permalink") {
                    return Err(unavailable("Semantic retrieval is disabled; only text, title and permalink modes are available"));
                }
                if options.min_similarity.is_some() {
                    return Err(unavailable(
                        "Minimum similarity requires unavailable semantic retrieval",
                    ));
                }
                require_preview(
                    args.expected_session.as_ref(),
                    options.compact.is_some()
                        || options.valid_at.is_some()
                        || options.valid_overlaps.is_some()
                        || options.time_kind.is_some(),
                )?;
                if options.valid_at.is_some() && options.valid_overlaps.is_some() {
                    return Err(error(
                        ErrorCategory::Schema,
                        "valid_at and valid_overlaps are mutually exclusive",
                    ));
                }
                let mut arguments = serde_json::to_value(options).map_err(|_| malformed())?;
                let object = arguments.as_object_mut().ok_or_else(malformed)?;
                object.retain(|key, value| key != "mode" && !value.is_null());
                object.extend(
                    json!({"project":FIXTURE_PROJECT,"query":query,
                    "search_type":mode,"page":page,"page_size":page_size,"output_format":"json"})
                    .as_object()
                    .ok_or_else(malformed)?
                    .clone(),
                );
                Ok(Self {
                    expected: args.expected_session.clone(),
                    operation: "search_notes",
                    tool: "search_notes",
                    arguments,
                    request_generation: args.request_generation,
                    subject: args.query.clone(),
                    page,
                    page_size,
                })
            })()),
            IpcCommand::PreviewContext(args) => Some((|| {
                require_route(&args.workspace, &args.project)?;
                library::reject_note_identifier(&args.identifier)?;
                if args.query.as_ref().is_some_and(|q| !q.is_empty()) {
                    return Err(unavailable(
                        "Official build_context does not support a query filter",
                    ));
                }
                let page = page(args.cursor.as_deref())?;
                let page_size = library::bound_page_size(Some(args.page_size.unwrap_or(50)))?;
                let options = &args.options;
                let max_related = options.max_related.unwrap_or(20);
                if !(1..=100).contains(&max_related) {
                    return Err(error(
                        ErrorCategory::Schema,
                        "max_related must be between 1 and 100",
                    ));
                }
                require_preview(args.expected_session.as_ref(), options.compact.is_some())?;
                let mut arguments = json!({"project":FIXTURE_PROJECT,"url":format!("memory://{}",args.identifier),
                    "depth":depth(options.depth)?,"max_related":max_related,
                    "timeframe":options.timeframe.as_deref().unwrap_or("7d"),
                    "page":page,"page_size":page_size,"output_format":"json"});
                if let Some(compact) = options.compact {
                    arguments["compact"] = json!(compact);
                }
                Ok(Self {
                    expected: args.expected_session.clone(),
                    operation: "preview_context",
                    tool: "build_context",
                    arguments,
                    request_generation: args.request_generation,
                    subject: args.identifier.clone(),
                    page,
                    page_size,
                })
            })()),
            IpcCommand::ListActivity(args) => Some((|| {
                require_route(&args.workspace, &args.project)?;
                let page = page(args.cursor.as_deref())?;
                let page_size = library::bound_page_size(Some(args.page_size.unwrap_or(50)))?;
                let options = &args.options;
                let arguments = json!({"project":FIXTURE_PROJECT,
                    "type":options.types.as_ref().map_or(json!(""), |types| json!(types)),
                    "depth":depth(options.depth)?,"timeframe":options.timeframe.as_deref().unwrap_or("7d"),
                    "page":page,"page_size":page_size,"output_format":"json"});
                Ok(Self {
                    expected: args.expected_session.clone(),
                    operation: "list_activity",
                    tool: "recent_activity",
                    arguments,
                    request_generation: args.request_generation,
                    subject: String::new(),
                    page,
                    page_size,
                })
            })()),
            _ => None,
        }
    }

    pub fn expected(&self) -> Option<&SessionIdentity> {
        self.expected.as_ref()
    }
    pub fn tool_call(&self) -> Value {
        json!({"name":self.tool,"arguments":self.arguments})
    }
    fn identity(&self, session: SessionIdentity) -> QueryIdentity {
        QueryIdentity {
            session,
            workspace: OWNED_WORKSPACE_ID,
            project: FIXTURE_PROJECT,
            operation: self.operation,
            arguments: self.arguments.clone(),
            request_generation: self.request_generation,
        }
    }

    pub fn decode(
        &self,
        result: &Value,
        session: SessionIdentity,
    ) -> Result<IpcResponse, IpcError> {
        if result.is_string() {
            return Err(unavailable(
                "Official query returned guidance instead of structured results",
            ));
        }
        let request = self.identity(session.clone());
        match self.operation {
            "search_notes" => {
                self.check_page(result, "current_page")?;
                let hits = rows(&result["results"])?;
                if hits.len() > self.page_size as usize {
                    return Err(malformed());
                }
                let has_more = result["has_more"].as_bool().ok_or_else(malformed)?;
                Ok(IpcResponse::EngineSearchPage(EngineSearchPageDto {
                    engine_search: true,
                    session,
                    request,
                    query: self.subject.clone(),
                    hits,
                    page: self.page,
                    page_size: self.page_size,
                    next_cursor: self.next(has_more)?,
                    has_more,
                    total: result["total"].as_u64().ok_or_else(malformed)?,
                    total_is_exact: result["total_is_exact"].as_bool().ok_or_else(malformed)?,
                }))
            }
            "preview_context" => {
                self.check_page(result, "page")?;
                let groups = result["results"].as_array().ok_or_else(malformed)?;
                if groups.len() > self.page_size as usize {
                    return Err(malformed());
                }
                let results = groups
                    .iter()
                    .map(|group| {
                        Ok(ContextGroupDto {
                            primary_result: hit(&group["primary_result"])?,
                            observations: rows(&group["observations"])?,
                            related_results: rows(&group["related_results"])?,
                        })
                    })
                    .collect::<Result<_, IpcError>>()?;
                let has_more = result["has_more"].as_bool().ok_or_else(malformed)?;
                Ok(IpcResponse::EngineContext(EngineContextDto {
                    engine_context: true,
                    session,
                    request,
                    identifier: self.subject.clone(),
                    results,
                    metadata: result["metadata"]
                        .as_object()
                        .ok_or_else(malformed)?
                        .clone(),
                    page: self.page,
                    page_size: self.page_size,
                    next_cursor: self.next(has_more)?,
                    has_more,
                }))
            }
            "list_activity" => {
                let entries = rows(result)?;
                if entries.len() > self.page_size as usize {
                    return Err(malformed());
                }
                Ok(IpcResponse::EngineActivity(EngineActivityDto {
                    engine_activity: true,
                    session,
                    request,
                    entries,
                    page: self.page,
                    page_size: self.page_size,
                    next_cursor: None,
                    has_more: None,
                    total: None,
                    total_is_exact: None,
                }))
            }
            _ => Err(malformed()),
        }
    }
    fn check_page(&self, result: &Value, key: &str) -> Result<(), IpcError> {
        if result[key].as_u64() != Some(u64::from(self.page))
            || result["page_size"].as_u64() != Some(u64::from(self.page_size))
        {
            return Err(malformed());
        }
        Ok(())
    }
    fn next(&self, has_more: bool) -> Result<Option<String>, IpcError> {
        if has_more {
            Ok(Some(
                self.page.checked_add(1).ok_or_else(malformed)?.to_string(),
            ))
        } else {
            Ok(None)
        }
    }
}

fn text(value: &Value, key: &str) -> Result<Option<String>, IpcError> {
    match value.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.clone())),
        _ => Err(malformed()),
    }
}
fn rows(value: &Value) -> Result<Vec<EngineHitDto>, IpcError> {
    value
        .as_array()
        .ok_or_else(malformed)?
        .iter()
        .map(hit)
        .collect()
}
fn hit(value: &Value) -> Result<EngineHitDto, IpcError> {
    let kind = value["type"].as_str().ok_or_else(malformed)?;
    let identifier = match text(value, "permalink")?.filter(|v| !v.is_empty()) {
        Some(id) => id,
        None => {
            let key = match kind {
                "entity" => "entity_id",
                "observation" => "observation_id",
                "relation" => "relation_id",
                _ => return Err(malformed()),
            };
            format!("{kind}:{}", value[key].as_u64().ok_or_else(malformed)?)
        }
    };
    let file_path = text(value, "file_path")?;
    let note_identifier = text(value, "external_id")?
        .or(text(value, "entity_external_id")?)
        .or(text(value, "from_entity_external_id")?)
        .or(text(value, "entity")?)
        .or_else(|| file_path.clone());
    let score = match value.get("score") {
        None | Some(Value::Null) => None,
        Some(Value::Number(n)) => Some(n.clone()),
        _ => return Err(malformed()),
    };
    let excerpt = match value.get("content") {
        None | Some(Value::Null) => None,
        Some(Value::String(body)) => Some(body.chars().take(240).collect()),
        _ => return Err(malformed()),
    };
    Ok(EngineHitDto {
        result_kind: kind.to_owned(),
        identifier,
        note_identifier,
        title: text(value, "title")?,
        excerpt,
        score,
        file_path,
        category: text(value, "category")?,
        relation_type: text(value, "relation_type")?,
        from_entity: text(value, "from_entity")?,
        to_entity: text(value, "to_entity")?,
        to_name: text(value, "to_name")?,
        created_at: text(value, "created_at")?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity(profile: EngineProfile) -> SessionIdentity {
        SessionIdentity {
            profile,
            generation: 7,
        }
    }
    fn command(name: &str, extra: Value, profile: EngineProfile) -> IpcCommand {
        let mut args = json!({"workspace":OWNED_WORKSPACE_ID,"project":FIXTURE_PROJECT,
            "expected_session":identity(profile),"request_generation":3});
        args.as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        serde_json::from_value(json!({"command":name,"args":args})).unwrap()
    }
    fn search(extra: Value) -> PreparedQuery {
        PreparedQuery::from_command(&command("search_notes", extra, EngineProfile::Release))
            .unwrap()
            .unwrap()
    }

    #[test]
    fn captured_profiles_replay_rank_context_and_activity_without_full_body_projection() {
        for profile in [EngineProfile::Release, EngineProfile::MainPreview] {
            let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(format!(
                "../../../tests/fixtures/c01-{}-query-replay.json",
                profile.id()
            ));
            let capture: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            for (id, name, extra) in [
                (
                    "ordering-probe",
                    "search_notes",
                    json!({"query":"orderingprobe","options":{"entity_types":["entity"]}}),
                ),
                (
                    "context",
                    "preview_context",
                    json!({"identifier":"corpus/note-00010","options":{"timeframe":"2026-01-01"}}),
                ),
                (
                    "activity",
                    "list_activity",
                    json!({"options":{"types":["entity"],"timeframe":"2026-01-01"}}),
                ),
            ] {
                let record = capture["records"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|r| r["id"] == id)
                    .unwrap();
                let query = PreparedQuery::from_command(&command(name, extra, profile))
                    .unwrap()
                    .unwrap();
                let result = query
                    .decode(
                        &record["response"]["result"]["structuredContent"]["result"],
                        identity(profile),
                    )
                    .unwrap();
                match result {
                    IpcResponse::EngineSearchPage(page) => {
                        assert_eq!(
                            page.hits
                                .iter()
                                .map(|h| h.identifier.as_str())
                                .collect::<Vec<_>>(),
                            vec!["corpus/note-00099", "corpus/note-00020"]
                        );
                        let raw = &record["response"]["result"]["structuredContent"]["result"];
                        for (hit, source) in
                            page.hits.iter().zip(raw["results"].as_array().unwrap())
                        {
                            assert_eq!(hit.score.as_ref(), source["score"].as_number());
                            assert!(hit.excerpt.as_ref().unwrap().chars().count() <= 240);
                            assert_eq!(
                                hit.note_identifier.as_deref(),
                                source["external_id"].as_str()
                            );
                        }
                    }
                    IpcResponse::EngineContext(context) => {
                        assert_eq!(context.results[0].primary_result.result_kind, "entity");
                        assert_eq!(
                            context.results[0].observations[0].result_kind,
                            "observation"
                        );
                        assert_eq!(
                            context.results[0].related_results[0].result_kind,
                            "relation"
                        );
                        assert!(context.results[0].related_results[0]
                            .identifier
                            .starts_with("relation:"));
                    }
                    IpcResponse::EngineActivity(activity) => {
                        assert_eq!(activity.entries.len(), 50);
                        assert!(
                            activity.has_more.is_none()
                                && activity.total.is_none()
                                && activity.next_cursor.is_none()
                        );
                    }
                    _ => panic!("wrong projection"),
                }
            }
        }
    }

    #[test]
    fn page_two_unknown_zero_total_and_mixed_hit_identities_keep_order_and_scores() {
        let query =
            search(json!({"query":"Exact OR 热压","cursor":"2","options":{"categories":["fact"]}}));
        assert_eq!(query.tool_call()["arguments"]["query"], "Exact OR 热压");
        assert!(query.tool_call()["arguments"].get("entity_types").is_none());
        let payload = json!({"current_page":2,"page_size":50,"total":0,"total_is_exact":false,"has_more":true,
            "results":[
                {"type":"observation","permalink":"z/observations/a","entity":"owner","external_id":"owner-uuid","score":1.3},
                {"type":"observation","permalink":"z/observations/b","entity":"owner","external_id":"owner-uuid","score":-6.5},
                {"type":"relation","permalink":"","relation_id":17,"entity":"owner","score":null}]});
        let IpcResponse::EngineSearchPage(page) = query
            .decode(&payload, identity(EngineProfile::Release))
            .unwrap()
        else {
            panic!()
        };
        assert_eq!(page.page, 2);
        assert_eq!(page.total, 0);
        assert!(!page.total_is_exact);
        assert!(page.has_more);
        assert_eq!(page.next_cursor.as_deref(), Some("3"));
        assert_eq!(page.hits[0].score.as_ref().unwrap().as_f64(), Some(1.3));
        assert_eq!(page.hits[1].score.as_ref().unwrap().as_f64(), Some(-6.5));
        assert_ne!(page.hits[0].identifier, page.hits[1].identifier);
        assert_eq!(page.hits[0].note_identifier, page.hits[1].note_identifier);
        assert_eq!(page.hits[2].identifier, "relation:17");
        let mut malformed = payload;
        malformed["current_page"] = json!(1);
        assert!(query
            .decode(&malformed, identity(EngineProfile::Release))
            .is_err());
    }

    #[test]
    fn filter_only_search_preserves_official_null_query_and_separate_filter_domains() {
        let prepared = search(
            json!({"query":"","options":{"categories":["fact"],"note_types":["note"],"tags":["fixture"]}}),
        );
        assert_eq!(prepared.tool_call()["arguments"]["query"], Value::Null);
        assert_eq!(
            prepared.tool_call()["arguments"]["categories"],
            json!(["fact"])
        );
        assert!(prepared.tool_call()["arguments"]
            .get("entity_types")
            .is_none());
        assert!(PreparedQuery::from_command(&command(
            "search_notes",
            json!({"query":"  ","options":{"tags":[]}}),
            EngineProfile::Release
        ))
        .unwrap()
        .is_err());
    }

    #[test]
    fn profile_and_disabled_modes_fail_without_silent_downgrade() {
        for options in [
            json!({"compact":false}),
            json!({"valid_at":"2026-01-01"}),
            json!({"time_kind":"due"}),
            json!({"mode":"semantic"}),
            json!({"mode":"hybrid"}),
            json!({"min_similarity":0.2}),
        ] {
            assert!(PreparedQuery::from_command(&command(
                "search_notes",
                json!({"query":"heat","options":options}),
                EngineProfile::Release
            ))
            .unwrap()
            .is_err());
        }
        let cmd = command(
            "search_notes",
            json!({"query":"heat","options":{"compact":true,"valid_at":"2026-01-01","time_kind":"due"}}),
            EngineProfile::MainPreview,
        );
        let prepared = PreparedQuery::from_command(&cmd).unwrap().unwrap();
        assert_eq!(prepared.tool_call()["arguments"]["compact"], true);
        let release = search(json!({"query":"heat"}));
        assert!(release.tool_call()["arguments"].get("compact").is_none());
        assert!(release
            .decode(
                &json!("semantic search is disabled"),
                identity(EngineProfile::Release)
            )
            .is_err());
        assert!(release
            .decode(&json!({"results":[]}), identity(EngineProfile::Release))
            .is_err());
    }

    #[test]
    fn every_query_identity_dimension_is_echoed_without_coalescing() {
        let base =
            search(json!({"query":"Heat OR 热压"})).identity(identity(EngineProfile::Release));
        for extra in [
            json!({"query":"heat OR 热压"}),
            json!({"query":"Heat OR 热压","cursor":"2"}),
            json!({"query":"Heat OR 热压","page_size":20}),
            json!({"query":"Heat OR 热压","options":{"mode":"title"}}),
            json!({"query":"Heat OR 热压","request_generation":4}),
            json!({"query":"Heat OR 热压","options":{"entity_types":["observation"],"note_types":["note"],"categories":["fact"],"tags":["tag"],"metadata_filters":{"status":"open"},"after_date":"2026-01-01"}}),
        ] {
            assert_ne!(
                base,
                search(extra).identity(identity(EngineProfile::Release))
            );
        }
        let mut replaced = identity(EngineProfile::Release);
        replaced.generation += 1;
        assert_ne!(
            base,
            search(json!({"query":"Heat OR 热压"})).identity(replaced)
        );
        assert_ne!(
            base,
            search(json!({"query":"Heat OR 热压"})).identity(identity(EngineProfile::MainPreview))
        );
    }
}
