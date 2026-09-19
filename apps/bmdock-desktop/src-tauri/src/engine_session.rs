//! C02: one owned, isolated official-engine read session. No renderer launch API.
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    process::Stdio,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, Mutex,
    },
    time::Duration,
};

use rmcp::{
    model::ClientInfo,
    service::{Peer, RoleClient, RunningService},
    ServiceExt,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use tokio::{
    process::{Child, Command},
    sync::{watch, Notify, Semaphore},
    time::timeout,
};

use crate::{
    ipc::{ErrorCategory, IpcCommand, IpcError, IpcResponse, FIXTURE_PROJECT},
    library::{
        self, NoteObservationDto, NoteReadDto, ObservationClass, TreeEntryDto, TreeEntryKind,
        TreePageDto,
    },
    routing::{self, ExplicitRouteArgs},
    supervisor::{ConnectionState, EngineProfile, FailureKind, RuntimeSnapshot, ShutdownReceipt},
};

const CONNECT_TIMEOUT: Duration = Duration::from_secs(120);
const READ_TIMEOUT: Duration = Duration::from_secs(90);
const CLOSE_TIMEOUT: Duration = Duration::from_secs(30);
static NEXT_GENERATION: AtomicU32 = AtomicU32::new(1);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SessionIdentity {
    pub profile: EngineProfile,
    pub generation: u32,
}

pub(crate) fn error(category: ErrorCategory, message: &str) -> IpcError {
    IpcError {
        category,
        message: message.to_owned(),
    }
}
pub(crate) fn unavailable(message: &str) -> IpcError {
    error(ErrorCategory::Unsupported, message)
}
pub(crate) fn malformed() -> IpcError {
    error(
        ErrorCategory::Schema,
        "Official engine returned malformed structured data",
    )
}
fn launch_policy() -> IpcError {
    error(
        ErrorCategory::Policy,
        "Only a verified generated fixture and pinned local engine may start",
    )
}

/// The host derives all executable/environment paths. This type is never deserialized from IPC.
pub struct FixtureLaunch {
    profile: EngineProfile,
    sandbox: PathBuf,
    python: PathBuf,
    worker: PathBuf,
    env: BTreeMap<String, String>,
}

fn read_json(path: &Path) -> Result<Value, IpcError> {
    serde_json::from_slice(&std::fs::read(path).map_err(|_| launch_policy())?)
        .map_err(|_| launch_policy())
}

fn canonical(path: &Path) -> Result<PathBuf, IpcError> {
    let path = path.canonicalize().map_err(|_| launch_policy())?;
    // Python's sandbox marker stores ordinary Windows paths, not Win32 verbatim paths.
    #[cfg(windows)]
    {
        let text = path.to_string_lossy();
        if let Some(rest) = text.strip_prefix("\\\\?\\UNC\\") {
            return Ok(PathBuf::from(format!("\\\\{rest}")));
        }
        if let Some(rest) = text.strip_prefix("\\\\?\\") {
            return Ok(PathBuf::from(rest));
        }
    }
    Ok(path)
}

impl FixtureLaunch {
    pub fn verified(profile: EngineProfile, sandbox: &Path) -> Result<Self, IpcError> {
        let root = canonical(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../..")
                .as_path(),
        )?;
        let sandbox = canonical(sandbox)?;
        let allowed = canonical(&root.join(".work/g0"))?;
        if sandbox == allowed || !sandbox.starts_with(&allowed) {
            return Err(launch_policy());
        }
        let marker = read_json(&sandbox.join(".bmdock-g0-sandbox.json"))?;
        if marker["kind"] != "bmdock-g0"
            || marker["profile"] != profile.id()
            || canonical(Path::new(
                marker["root"].as_str().ok_or_else(launch_policy)?,
            ))? != sandbox
        {
            return Err(launch_policy());
        }
        for part in ["config", "vault", "home", "tmp", "cache"] {
            let path = sandbox.join(part);
            if std::fs::symlink_metadata(&path)
                .map_err(|_| launch_policy())?
                .file_type()
                .is_symlink()
                || !canonical(&path)?.starts_with(&sandbox)
            {
                return Err(launch_policy());
            }
        }
        let config = read_json(&sandbox.join("config/config.json"))?;
        let keys: BTreeSet<_> = config
            .as_object()
            .ok_or_else(launch_policy)?
            .keys()
            .map(String::as_str)
            .collect();
        let allowed_keys = BTreeSet::from([
            "projects",
            "default_project",
            "database_backend",
            "auto_update",
            "semantic_search_enabled",
        ]);
        if keys != allowed_keys
            || config["default_project"] != FIXTURE_PROJECT
            || config["database_backend"] != "sqlite"
            || config["auto_update"] != false
            || config["semantic_search_enabled"] != false
        {
            return Err(launch_policy());
        }
        let projects = config["projects"].as_object().ok_or_else(launch_policy)?;
        if projects.len() != 1 {
            return Err(launch_policy());
        }
        let project = projects.get(FIXTURE_PROJECT).ok_or_else(launch_policy)?;
        if project["mode"] != "local"
            || project.as_object().ok_or_else(launch_policy)?.len() != 2
            || canonical(Path::new(
                project["path"].as_str().ok_or_else(launch_policy)?,
            ))? != canonical(&sandbox.join("vault"))?
        {
            return Err(launch_policy());
        }
        let engine = root.join(".work/engines").join(profile.id());
        let marker = read_json(&engine.join(".bmdock-engine.json"))?;
        if marker["kind"] != "bmdock-engine" || marker["commit"] != profile.commit() {
            return Err(launch_policy());
        }
        let mut git = std::process::Command::new("git");
        git.args(["-C"]).arg(&engine).args(["rev-parse", "HEAD"]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            git.creation_flags(0x08000000);
        }
        let head = git.output().map_err(|_| launch_policy())?;
        if !head.status.success()
            || String::from_utf8_lossy(&head.stdout).trim() != profile.commit()
        {
            return Err(launch_policy());
        }
        let mut git = std::process::Command::new("git");
        git.arg("-C")
            .arg(&engine)
            .args(["status", "--porcelain", "--untracked-files=no"]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            git.creation_flags(0x08000000);
        }
        let status = git.output().map_err(|_| launch_policy())?;
        if !status.status.success() || !status.stdout.is_empty() {
            return Err(launch_policy());
        }
        // Do not canonicalize the executable itself: a venv symlink needs its own pyvenv.cfg.
        let python = engine.join(if cfg!(windows) {
            ".venv/Scripts/python.exe"
        } else {
            ".venv/bin/python"
        });
        let worker = root.join("scripts/engine_worker.py");
        if !python.is_file() || !worker.is_file() {
            return Err(launch_policy());
        }
        let env = isolated_env(&sandbox);
        Ok(Self {
            profile,
            sandbox,
            python,
            worker,
            env,
        })
    }
}

fn isolated_env(sandbox: &Path) -> BTreeMap<String, String> {
    let mut env = BTreeMap::new();
    for key in [
        "PATH",
        "SystemRoot",
        "WINDIR",
        "COMSPEC",
        "PATHEXT",
        "SystemDrive",
        "LANG",
        "LC_ALL",
    ] {
        if let Ok(value) = std::env::var(key) {
            env.insert(key.to_owned(), value);
        }
    }
    for (key, part) in [
        ("HOME", "home"),
        ("USERPROFILE", "home"),
        ("APPDATA", "home"),
        ("LOCALAPPDATA", "home"),
        ("XDG_CONFIG_HOME", "config"),
        ("XDG_CACHE_HOME", "cache"),
        ("TMP", "tmp"),
        ("TEMP", "tmp"),
        ("TMPDIR", "tmp"),
        ("BASIC_MEMORY_CONFIG_DIR", "config"),
    ] {
        env.insert(
            key.to_owned(),
            sandbox.join(part).to_string_lossy().into_owned(),
        );
    }
    for (key, value) in [
        ("BASIC_MEMORY_AUTO_UPDATE", "false"),
        ("BASIC_MEMORY_SEMANTIC_SEARCH_ENABLED", "false"),
        ("BASIC_MEMORY_FORCE_LOCAL", "true"),
        ("BASIC_MEMORY_EXPLICIT_ROUTING", "true"),
        ("BASIC_MEMORY_DATABASE_BACKEND", "sqlite"),
        ("BASIC_MEMORY_MCP_PROJECT", FIXTURE_PROJECT),
        ("PYTHONUTF8", "1"),
        ("PYTHONIOENCODING", "utf-8"),
        ("PYTHONUNBUFFERED", "1"),
        ("HF_HUB_OFFLINE", "1"),
        ("TRANSFORMERS_OFFLINE", "1"),
        ("LOGFIRE_SEND_TO_LOGFIRE", "false"),
    ] {
        env.insert(key.to_owned(), value.to_owned());
    }
    env
}

struct Inner {
    identity: SessionIdentity,
    state: watch::Sender<RuntimeSnapshot>,
    peer: Mutex<Option<Peer<RoleClient>>>,
    stop: Notify,
    // Bound retained reads. C03 can refine scheduling without changing lifecycle ownership.
    reads: Arc<Semaphore>,
    commands: AtomicU64,
}

#[derive(Clone)]
pub struct EngineSession(Arc<Inner>);

impl EngineSession {
    pub fn start(launch: FixtureLaunch) -> Result<Self, IpcError> {
        let generation = NEXT_GENERATION
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_add(1))
            .map_err(|_| unavailable("Session generation exhausted; restart the host"))?;
        let (state, _) = watch::channel(RuntimeSnapshot {
            state: ConnectionState::Starting,
            profile: Some(launch.profile),
            child_pid: None,
            failure: None,
            shutdown: None,
        });
        let session = Self(Arc::new(Inner {
            identity: SessionIdentity {
                profile: launch.profile,
                generation,
            },
            state,
            peer: Mutex::new(None),
            stop: Notify::new(),
            reads: Arc::new(Semaphore::new(8)),
            commands: AtomicU64::new(0),
        }));
        tokio::spawn(session.clone().run(launch));
        Ok(session)
    }

    pub fn identity(&self) -> SessionIdentity {
        self.0.identity.clone()
    }
    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.0.state.borrow().clone()
    }
    pub fn subscribe(&self) -> watch::Receiver<RuntimeSnapshot> {
        self.0.state.subscribe()
    }
    fn update(&self, f: impl FnOnce(&mut RuntimeSnapshot)) {
        self.0.state.send_modify(f);
    }
    fn fail(&self, kind: FailureKind) {
        self.update(|s| {
            if s.state == ConnectionState::Connected {
                s.state = ConnectionState::Failed;
                s.failure = Some(kind);
            }
        });
        self.0.stop.notify_one();
    }

    pub async fn wait_connected(&self) -> Result<(), IpcError> {
        let mut state = self.0.state.subscribe();
        loop {
            match state.borrow().state {
                ConnectionState::Connected => return Ok(()),
                ConnectionState::Starting => {}
                _ => return Err(unavailable("Official engine did not connect")),
            }
            state
                .changed()
                .await
                .map_err(|_| unavailable("Engine session closed"))?;
        }
    }

    pub async fn close(&self) -> ShutdownReceipt {
        let mut state = self.0.state.subscribe();
        self.update(|s| {
            s.state = if s.shutdown.is_none() {
                ConnectionState::Stopping
            } else {
                ConnectionState::Stopped
            };
        });
        self.0.stop.notify_one();
        loop {
            if let Some(receipt) = state.borrow().shutdown.clone() {
                return receipt;
            }
            if state.changed().await.is_err() {
                return unknown_receipt();
            }
        }
    }

    async fn run(self, launch: FixtureLaunch) {
        let mut command = Command::new(&launch.python);
        command
            .arg(&launch.worker)
            .arg("serve")
            .current_dir(&launch.sandbox)
            .env_clear()
            .envs(&launch.env)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(windows)]
        {
            command.creation_flags(0x08000000);
        }
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(_) => {
                self.update(|s| {
                    s.state = ConnectionState::Failed;
                    s.failure = Some(FailureKind::Process);
                    s.shutdown = Some(ShutdownReceipt {
                        transport_cancelled: true,
                        child_exited: true,
                        forced: false,
                        timeout_unknown: false,
                        exit_code: None,
                    });
                });
                return;
            }
        };
        self.update(|s| s.child_pid = child.id());
        let connected = async {
            let stdout = child.stdout.take().ok_or(FailureKind::Process)?;
            let stdin = child.stdin.take().ok_or(FailureKind::Process)?;
            let info: ClientInfo =
                serde_json::from_value(json!({"protocolVersion":"2025-11-25", "capabilities":{},
                "clientInfo":{"name":"BMDock-Desktop-Read", "version":env!("CARGO_PKG_VERSION")}}))
                .map_err(|_| FailureKind::Unverified)?;
            let mut service = info
                .serve((stdout, stdin))
                .await
                .map_err(|_| FailureKind::Transport)?;
            if let Err(failure) = discover(&service, launch.profile).await {
                let _ = service.close_with_timeout(CLOSE_TIMEOUT).await;
                return Err(failure);
            }
            Ok(service)
        };
        let connected = tokio::select! {
            result = timeout(CONNECT_TIMEOUT, connected) => result.unwrap_or(Err(FailureKind::TimeoutUnknown)),
            _ = self.0.stop.notified() => Err(FailureKind::Unverified),
        };
        let service = match connected {
            Ok(service) => service,
            Err(kind) => {
                self.update(|s| {
                    if s.state != ConnectionState::Stopping {
                        s.state = ConnectionState::Failed;
                        s.failure = Some(kind);
                    }
                });
                let receipt = close_child(false, &mut child, CLOSE_TIMEOUT).await;
                self.finish(receipt);
                return;
            }
        };
        if let Ok(mut peer) = self.0.peer.lock() {
            *peer = Some(service.peer().clone());
        }
        self.update(|s| {
            if s.state == ConnectionState::Starting {
                s.state = ConnectionState::Connected;
            }
        });
        self.own_connected(service, child, CLOSE_TIMEOUT).await;
    }

    async fn own_connected(
        self,
        service: RunningService<RoleClient, ClientInfo>,
        mut child: Child,
        close_budget: Duration,
    ) {
        let cancel = service.cancellation_token();
        let waiting = service.waiting();
        tokio::pin!(waiting);
        let mut transport_ended = None;
        tokio::select! {
            _ = self.0.stop.notified() => {},
            _ = child.wait() => self.update(|s| { if s.state != ConnectionState::Stopping { s.state = ConnectionState::Failed; s.failure = Some(FailureKind::Process); } }),
            result = &mut waiting => {
                transport_ended = Some(result.is_ok());
                self.update(|s| { if s.state != ConnectionState::Stopping { s.state = ConnectionState::Failed; s.failure = Some(FailureKind::Transport); } });
            },
        }
        if let Ok(mut peer) = self.0.peer.lock() {
            *peer = None;
        }
        self.0.reads.close();
        cancel.cancel();
        let transport_cancelled = match transport_ended {
            Some(closed) => closed,
            None => matches!(timeout(close_budget, &mut waiting).await, Ok(Ok(_))),
        };
        let receipt = close_child(transport_cancelled, &mut child, close_budget).await;
        self.finish(receipt);
    }

    fn finish(&self, receipt: ShutdownReceipt) {
        self.update(|s| {
            if s.state != ConnectionState::Failed {
                s.state = ConnectionState::Stopped;
            }
            s.shutdown = Some(receipt);
        });
    }

    fn require_identity(&self, expected: Option<&SessionIdentity>) -> Result<(), IpcError> {
        if self.snapshot().state != ConnectionState::Connected {
            return Err(unavailable("Official read session is unavailable"));
        }
        if expected != Some(&self.0.identity) {
            return Err(error(
                ErrorCategory::Policy,
                "Read session identity is missing or stale",
            ));
        }
        Ok(())
    }

    pub async fn read(&self, read: PreparedRead) -> Result<IpcResponse, IpcError> {
        self.read_with_budget(read, READ_TIMEOUT).await
    }

    async fn read_with_budget(
        &self,
        read: PreparedRead,
        budget: Duration,
    ) -> Result<IpcResponse, IpcError> {
        let mut response = self.request_with_budget(&read, budget).await?;
        // FastMCP also includes a text copy of structured content. Move the
        // structured value into decoding instead of serializing that copy again.
        let is_error = response.is_error.unwrap_or(false);
        let structured = response.structured_content.take().unwrap_or(Value::Null);
        drop(response);
        let envelope = serde_json::Map::from_iter([
            ("isError".to_owned(), Value::Bool(is_error)),
            ("structuredContent".to_owned(), structured),
        ]);
        read.decode(Value::Object(envelope), self.identity())
    }

    pub fn command_count(&self) -> u64 {
        self.0.commands.load(Ordering::Relaxed)
    }

    /// Host-only comparison entry; accepts the same prepared typed reads as dispatch.
    pub async fn raw_read(&self, read: &PreparedRead) -> Result<Value, IpcError> {
        serde_json::to_value(self.request_with_budget(read, READ_TIMEOUT).await?)
            .map_err(|_| malformed())
    }

    async fn request_with_budget(
        &self,
        read: &PreparedRead,
        budget: Duration,
    ) -> Result<rmcp::model::CallToolResult, IpcError> {
        self.require_identity(read.expected())?;
        let _permit = self
            .0
            .reads
            .clone()
            .try_acquire_owned()
            .map_err(|_| unavailable("Read session capacity is exhausted"))?;
        let peer = self
            .0
            .peer
            .lock()
            .map_err(|_| unavailable("Read session is unavailable"))?
            .clone()
            .ok_or_else(|| unavailable("Read session is unavailable"))?;
        let request =
            serde_json::from_value(json!({"method":"tools/call", "params":read.tool_call()}))
                .map_err(|_| malformed())?;
        let session = self.clone();
        let expected = read.expected().cloned();
        // The owner retains admission after a UI consumer drops its future. Dropping
        // a consumer is logical cancellation, not proof the engine stopped its RPC.
        tokio::spawn(async move {
            let _permit = _permit;
            session.0.commands.fetch_add(1, Ordering::Relaxed);
            let response = match timeout(budget, peer.send_request(request)).await {
                Ok(Ok(rmcp::model::ServerResult::CallToolResult(result))) => result,
                Ok(Ok(_)) => return Err(malformed()),
                Ok(Err(_)) => {
                    session.fail(FailureKind::Transport);
                    return Err(unavailable(
                        "Official engine transport failed; read was not retried",
                    ));
                }
                Err(_) => {
                    session.fail(FailureKind::TimeoutUnknown);
                    return Err(unavailable(
                        "Official read timed out; result unknown and session retired",
                    ));
                }
            };
            // close/replacement retires the old handle before another session is installed.
            session.require_identity(expected.as_ref())?;
            Ok(response)
        })
        .await
        .map_err(|_| unavailable("Owned read task failed"))?
    }
}

fn unknown_receipt() -> ShutdownReceipt {
    ShutdownReceipt {
        transport_cancelled: false,
        child_exited: false,
        forced: false,
        timeout_unknown: true,
        exit_code: None,
    }
}

async fn close_child(
    transport_cancelled: bool,
    child: &mut Child,
    budget: Duration,
) -> ShutdownReceipt {
    let mut receipt = ShutdownReceipt {
        transport_cancelled,
        ..unknown_receipt()
    };
    match timeout(budget, child.wait()).await {
        Ok(Ok(status)) => {
            receipt.child_exited = true;
            receipt.exit_code = status.code();
            receipt.timeout_unknown = !transport_cancelled;
        }
        _ => {
            receipt.forced = child.start_kill().is_ok();
            if let Ok(Ok(status)) = timeout(budget, child.wait()).await {
                receipt.child_exited = true;
                receipt.exit_code = status.code();
            }
        }
    }
    receipt
}

async fn discover(
    service: &RunningService<RoleClient, ClientInfo>,
    profile: EngineProfile,
) -> Result<(), FailureKind> {
    let info = serde_json::to_value(service.peer_info()).map_err(|_| FailureKind::Unverified)?;
    if info["protocolVersion"] != "2025-11-25" {
        return Err(FailureKind::Unverified);
    }
    let profiles: Value =
        serde_json::from_str(include_str!("../../../../compatibility/profiles.json"))
            .map_err(|_| FailureKind::Unverified)?;
    let expected = profiles["engines"][profile.id()]["expected_tools"]
        .as_array()
        .ok_or(FailureKind::Unverified)?;
    let expected: BTreeSet<_> = expected
        .iter()
        .filter_map(Value::as_str)
        .map(str::to_owned)
        .collect();
    let mut actual = BTreeSet::new();
    let mut cursor = None;
    let mut seen = BTreeSet::new();
    for _ in 0..128 {
        let request =
            serde_json::from_value(json!({"method":"tools/list", "params": {"cursor":cursor}}))
                .map_err(|_| FailureKind::Unverified)?;
        let response = service
            .send_request(request)
            .await
            .map_err(|_| FailureKind::Transport)?;
        let response = serde_json::to_value(response).map_err(|_| FailureKind::Unverified)?;
        for tool in response["tools"]
            .as_array()
            .ok_or(FailureKind::Unverified)?
        {
            let name = tool["name"].as_str().ok_or(FailureKind::Unverified)?;
            if !actual.insert(name.to_owned()) {
                return Err(FailureKind::Unverified);
            }
            let required: &[&str] = match name {
                "read_note" => &[
                    "identifier",
                    "project",
                    "output_format",
                    "include_frontmatter",
                ],
                "list_directory" => &[
                    "dir_name",
                    "project",
                    "output_format",
                    "page",
                    "page_size",
                    "depth",
                ],
                "search_notes" => &[
                    "query",
                    "search_type",
                    "entity_types",
                    "note_types",
                    "categories",
                    "tags",
                    "metadata_filters",
                    "status",
                    "after_date",
                    "project",
                    "page",
                    "page_size",
                    "output_format",
                ],
                "build_context" => &[
                    "url",
                    "depth",
                    "max_related",
                    "timeframe",
                    "project",
                    "page",
                    "page_size",
                    "output_format",
                ],
                "recent_activity" => &[
                    "type",
                    "depth",
                    "timeframe",
                    "project",
                    "page",
                    "page_size",
                    "output_format",
                ],
                _ => &[],
            };
            if required
                .iter()
                .any(|key| tool["inputSchema"]["properties"].get(key).is_none())
            {
                return Err(FailureKind::Unverified);
            }
            if profile == EngineProfile::MainPreview {
                let preview_fields: &[&str] = match name {
                    "search_notes" => &["compact", "valid_at", "valid_overlaps", "time_kind"],
                    "build_context" => &["compact"],
                    _ => &[],
                };
                if preview_fields
                    .iter()
                    .any(|key| tool["inputSchema"]["properties"].get(key).is_none())
                {
                    return Err(FailureKind::Unverified);
                }
            }
        }
        match response.get("nextCursor") {
            None | Some(Value::Null) => {
                return if actual == expected && actual.len() == profile.expected_tools() {
                    Ok(())
                } else {
                    Err(FailureKind::Unverified)
                }
            }
            Some(Value::String(next)) if !next.is_empty() && seen.insert(next.clone()) => {
                cursor = Some(next.clone())
            }
            _ => return Err(FailureKind::Unverified),
        }
    }
    Err(FailureKind::Unverified)
}

/// Only prepared typed reads can reach MCP. C03 extends this owner for search/context/activity.
pub enum PreparedRead {
    Query(crate::engine_queries::PreparedQuery),
    Tree {
        directory: String,
        page: u32,
        page_size: u32,
        expected: Option<SessionIdentity>,
    },
    Note {
        identifier: String,
        expected: Option<SessionIdentity>,
    },
}

impl PreparedRead {
    pub fn from_command(command: &IpcCommand) -> Option<Result<Self, IpcError>> {
        match command {
            IpcCommand::ListTree(args) => Some((|| {
                require_route(&args.workspace, &args.project)?;
                let directory = args.directory.as_deref().unwrap_or("");
                validate_directory(directory)?;
                let page_size = library::bound_page_size(Some(args.page_size.unwrap_or(50)))?;
                let page = match args.cursor.as_deref() {
                    None => 1,
                    Some(cursor) => cursor
                        .parse::<u32>()
                        .ok()
                        .filter(|page| *page > 1)
                        .ok_or_else(|| {
                            error(ErrorCategory::Schema, "Invalid official tree page cursor")
                        })?,
                };
                Ok(Self::Tree {
                    directory: directory.to_owned(),
                    page,
                    page_size,
                    expected: args.expected_session.clone(),
                })
            })()),
            IpcCommand::ReadNote(args) => Some((|| {
                require_route(&args.workspace, &args.project)?;
                library::reject_note_identifier(&args.identifier)?;
                Ok(Self::Note {
                    identifier: args.identifier.clone(),
                    expected: args.expected_session.clone(),
                })
            })()),
            _ => crate::engine_queries::PreparedQuery::from_command(command)
                .map(|result| result.map(Self::Query)),
        }
    }
    fn expected(&self) -> Option<&SessionIdentity> {
        match self {
            Self::Query(query) => query.expected(),
            Self::Tree { expected, .. } | Self::Note { expected, .. } => expected.as_ref(),
        }
    }
    pub fn tool_call(&self) -> Value {
        match self {
            Self::Query(query) => query.tool_call(),
            Self::Tree {
                directory,
                page,
                page_size,
                ..
            } => json!({"name":"list_directory", "arguments":{
                "project":FIXTURE_PROJECT, "dir_name":format!("/{directory}"), "depth":1, "page":page, "page_size":page_size, "output_format":"json"}}),
            Self::Note { identifier, .. } => json!({"name":"read_note", "arguments":{
                "project":FIXTURE_PROJECT, "identifier":identifier, "include_frontmatter":true, "output_format":"json"}}),
        }
    }
    fn decode(&self, response: Value, identity: SessionIdentity) -> Result<IpcResponse, IpcError> {
        let result = structured_result(&response)?;
        match self {
            Self::Query(query) => query.decode(result, identity),
            Self::Tree {
                page, page_size, ..
            } => decode_tree(result, *page, *page_size, identity).map(IpcResponse::TreePage),
            Self::Note { identifier, .. } => {
                let note = decode_note(result, identity)?;
                // The engine's non-UUID branch can fall back to a different note
                // with an exact matching title. Do not publish that as this target.
                if !is_uuid(identifier)
                    && result["permalink"].as_str() != Some(identifier)
                    && result["file_path"].as_str() != Some(identifier)
                {
                    return Err(unavailable("Requested note identity was not found"));
                }
                Ok(IpcResponse::NoteRead(note))
            }
        }
    }
}

pub(crate) fn require_route(workspace: &str, project: &str) -> Result<(), IpcError> {
    routing::require_explicit_fixture_route(&ExplicitRouteArgs {
        workspace: workspace.to_owned(),
        project: project.to_owned(),
    })
    .map_err(|message| error(ErrorCategory::Policy, message))
}

fn validate_directory(directory: &str) -> Result<(), IpcError> {
    if directory.is_empty() {
        return Ok(());
    }
    if directory.starts_with('/')
        || directory.contains(['\\', ':'])
        || directory
            .split('/')
            .any(|s| s.is_empty() || s == "." || s == "..")
    {
        return Err(error(
            ErrorCategory::Policy,
            "Directory must be a project-relative logical identifier",
        ));
    }
    Ok(())
}

fn structured_result(response: &Value) -> Result<&Value, IpcError> {
    if response["isError"] == true {
        return Err(unavailable("Official engine rejected the read"));
    }
    // Both pinned captures use FastMCP structuredContent.result; prose is never a decoder fallback.
    let result = response
        .pointer("/structuredContent/result")
        .ok_or_else(malformed)?;
    if result.get("error").is_some_and(|value| !value.is_null()) {
        return Err(unavailable("Official engine could not resolve the read"));
    }
    Ok(result)
}

fn decode_note(result: &Value, identity: SessionIdentity) -> Result<NoteReadDto, IpcError> {
    let content = result.get("content").ok_or_else(malformed)?;
    if content.is_null() {
        return Err(unavailable("Note was not found in the connected fixture"));
    }
    let body = result["content"].as_str().ok_or_else(malformed)?.to_owned();
    let title = result["title"].as_str().ok_or_else(malformed)?.to_owned();
    let identifier = result["permalink"]
        .as_str()
        .or_else(|| result["file_path"].as_str())
        .ok_or_else(malformed)?
        .to_owned();
    library::reject_note_identifier(&identifier)?;
    Ok(NoteReadDto {
        session: Some(identity),
        title,
        identifier,
        body,
        observation: NoteObservationDto {
            classified_as: ObservationClass::Unclassified,
            disk_verified: false,
            envelope_is_not_disk_proof: true,
        },
    })
}

fn is_uuid(identifier: &str) -> bool {
    identifier.len() == 36
        && identifier.bytes().enumerate().all(|(i, b)| {
            if matches!(i, 8 | 13 | 18 | 23) {
                b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
}

fn decode_tree(
    result: &Value,
    page: u32,
    page_size: u32,
    identity: SessionIdentity,
) -> Result<TreePageDto, IpcError> {
    if result["page"].as_u64() != Some(u64::from(page))
        || result["page_size"].as_u64() != Some(u64::from(page_size))
    {
        return Err(malformed());
    }
    let has_more = result["has_more"].as_bool().ok_or_else(malformed)?;
    let nodes = result["nodes"].as_array().ok_or_else(malformed)?;
    if nodes.len() > page_size as usize || (nodes.is_empty() && has_more) {
        return Err(malformed());
    }
    let mut entries = Vec::with_capacity(nodes.len());
    for node in nodes {
        let (identifier, title, kind) = match node["type"].as_str() {
            Some("directory") => {
                let directory = node["directory_path"]
                    .as_str()
                    .ok_or_else(malformed)?
                    .trim_start_matches('/');
                validate_directory(directory)?;
                (
                    directory,
                    node["name"].as_str().ok_or_else(malformed)?,
                    TreeEntryKind::Directory,
                )
            }
            Some("file") => {
                let identifier = node["permalink"]
                    .as_str()
                    .or_else(|| node["file_path"].as_str())
                    .ok_or_else(malformed)?;
                library::reject_note_identifier(identifier)?;
                (
                    identifier,
                    node["title"]
                        .as_str()
                        .or_else(|| node["name"].as_str())
                        .ok_or_else(malformed)?,
                    TreeEntryKind::Note,
                )
            }
            _ => return Err(malformed()),
        };
        entries.push(TreeEntryDto {
            note_identifier: if kind == TreeEntryKind::Note {
                let id = node["external_id"]
                    .as_str()
                    .filter(|id| is_uuid(id))
                    .ok_or_else(malformed)?;
                Some(id.to_owned())
            } else {
                None
            },
            identifier: identifier.to_owned(),
            title: title.to_owned(),
            kind,
        });
    }
    Ok(TreePageDto {
        session: Some(identity),
        entries,
        page,
        next_cursor: if has_more {
            Some(page.checked_add(1).ok_or_else(malformed)?.to_string())
        } else {
            None
        },
        truncated: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn identity() -> SessionIdentity {
        SessionIdentity {
            profile: EngineProfile::Release,
            generation: 9,
        }
    }
    #[test]
    fn full_source_is_unsliced_and_preserves_delivered_crlf_yaml_utf8() {
        let read = PreparedRead::Note {
            identifier: "nested/中文".into(),
            expected: Some(identity()),
        };
        let args = &read.tool_call()["arguments"];
        assert_eq!(args["include_frontmatter"], true);
        assert!(args.get("start_line").is_none());
        let body = "---\r\ntitle: 中文\r\n---\r\n正文\r\n";
        let result = decode_note(
            &json!({"title":"中文", "permalink":"nested/中文", "content":body}),
            identity(),
        )
        .unwrap();
        assert_eq!(result.body.as_bytes(), body.as_bytes());
        assert!(!result.observation.disk_verified);
        assert_eq!(result.session, Some(identity()));
    }
    #[test]
    fn structured_errors_and_missing_are_not_successful_empty_reads() {
        assert_eq!(
            structured_result(&json!({"isError":true}))
                .unwrap_err()
                .category,
            ErrorCategory::Unsupported
        );
        assert_eq!(
            structured_result(&json!({"content":[{"text":"helpful guidance"}]}))
                .unwrap_err()
                .category,
            ErrorCategory::Schema
        );
        assert_eq!(
            structured_result(&json!({"structuredContent":{"result":{"error":"NOTE_NOT_FOUND"}}}))
                .unwrap_err()
                .category,
            ErrorCategory::Unsupported
        );
        assert_eq!(
            decode_note(&json!({"content":null, "title":null}), identity())
                .unwrap_err()
                .category,
            ErrorCategory::Unsupported
        );
        assert_eq!(
            decode_note(&json!({"content":123}), identity())
                .unwrap_err()
                .category,
            ErrorCategory::Schema
        );
    }
    #[test]
    fn nested_directory_is_logical_and_tree_empty_is_a_connected_result() {
        for denied in [
            "/vault",
            "../vault",
            "C:/vault",
            "nested/../vault",
            "nested\\vault",
        ] {
            assert!(validate_directory(denied).is_err());
        }
        validate_directory("group-000/item-000").unwrap();
        let page = decode_tree(
            &json!({"nodes":[],"page":1,"page_size":20,"has_more":false}),
            1,
            20,
            identity(),
        )
        .unwrap();
        assert!(page.entries.is_empty());
        assert_eq!(page.session, Some(identity()));
    }
    #[test]
    fn tree_open_uses_uuid_and_stale_permalink_cannot_accept_another_notes_title_match() {
        let uuid = "12345678-1234-5678-9012-123456789abc";
        let tree=decode_tree(&json!({"page":1,"page_size":50,"has_more":false,"nodes":[
            {"type":"file","permalink":"deleted-note","file_path":"folder/deleted.md","title":"Original","external_id":uuid}
        ]}),1,50,identity()).unwrap();
        assert_eq!(tree.entries[0].identifier, "deleted-note");
        assert_eq!(tree.entries[0].note_identifier.as_deref(), Some(uuid));
        let stale = PreparedRead::Note {
            identifier: "deleted-note".into(),
            expected: Some(identity()),
        };
        let fallback = json!({"structuredContent":{"result":{"title":"deleted-note","permalink":"unrelated-note","file_path":"other.md","content":"wrong body"}}});
        assert_eq!(
            stale.decode(fallback, identity()).unwrap_err().category,
            ErrorCategory::Unsupported
        );
        let stable = PreparedRead::Note {
            identifier: uuid.into(),
            expected: Some(identity()),
        };
        let renamed = json!({"structuredContent":{"result":{"title":"Original","permalink":"renamed-note","file_path":"folder/renamed.md","content":"body"}}});
        assert!(matches!(
            stable.decode(renamed, identity()).unwrap(),
            IpcResponse::NoteRead(_)
        ));
    }

    #[test]
    fn generated_environment_drops_untrusted_host_settings() {
        let env = isolated_env(Path::new("owned-fixture"));
        for key in [
            "OPENAI_API_KEY",
            "PYTHONPATH",
            "PYTHONHOME",
            "BASIC_MEMORY_FORCE_CLOUD",
            "BASIC_MEMORY_DATABASE_URL",
        ] {
            assert!(!env.contains_key(key));
        }
        assert_eq!(env["BASIC_MEMORY_MCP_PROJECT"], FIXTURE_PROJECT);
        assert_eq!(env["BASIC_MEMORY_AUTO_UPDATE"], "false");
    }

    async fn controlled_session() -> (
        EngineSession,
        tokio::sync::oneshot::Receiver<()>,
        tokio::sync::oneshot::Sender<()>,
        tokio::task::JoinHandle<()>,
    ) {
        controlled_session_with_child(None).await
    }

    async fn controlled_session_with_child(
        child: Option<Child>,
    ) -> (
        EngineSession,
        tokio::sync::oneshot::Receiver<()>,
        tokio::sync::oneshot::Sender<()>,
        tokio::task::JoinHandle<()>,
    ) {
        use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
        let (client, server) = tokio::io::duplex(8192);
        let (seen_tx, seen_rx) = tokio::sync::oneshot::channel();
        let (reply_tx, reply_rx) = tokio::sync::oneshot::channel();
        let server_task = tokio::spawn(async move {
            let (reader, mut writer) = tokio::io::split(server);
            let mut lines = BufReader::new(reader).lines();
            let initialize: Value =
                serde_json::from_str(&lines.next_line().await.unwrap().unwrap()).unwrap();
            let response = json!({"jsonrpc":"2.0", "id":initialize["id"], "result":{
                "protocolVersion":"2025-11-25", "capabilities":{"tools":{}}, "serverInfo":{"name":"controlled-test-peer", "version":"1"}}});
            writer
                .write_all(format!("{response}\n").as_bytes())
                .await
                .unwrap();
            while let Some(line) = lines.next_line().await.unwrap() {
                let call: Value = serde_json::from_str(&line).unwrap();
                if call["method"] == "tools/call" {
                    let _ = seen_tx.send(());
                    if reply_rx.await.is_ok() {
                        let response = json!({"jsonrpc":"2.0", "id":call["id"], "result":{
                            "content":[], "structuredContent":{"result":{"title":"Controlled", "permalink":"controlled", "content":"body"}}}});
                        let _ = writer.write_all(format!("{response}\n").as_bytes()).await;
                    }
                    break;
                }
            }
        });
        let info: ClientInfo =
            serde_json::from_value(json!({"protocolVersion":"2025-11-25", "capabilities":{},
            "clientInfo":{"name":"controlled-read-test", "version":"1"}}))
            .unwrap();
        let mut service = info.serve(tokio::io::split(client)).await.unwrap();
        let (state, _) = watch::channel(RuntimeSnapshot {
            state: ConnectionState::Connected,
            profile: Some(EngineProfile::Release),
            child_pid: child.as_ref().and_then(Child::id),
            failure: None,
            shutdown: None,
        });
        let session = EngineSession(Arc::new(Inner {
            identity: identity(),
            state,
            peer: Mutex::new(Some(service.peer().clone())),
            stop: Notify::new(),
            reads: Arc::new(Semaphore::new(8)),
            commands: AtomicU64::new(0),
        }));
        let owner = session.clone();
        if let Some(child) = child {
            tokio::spawn(owner.own_connected(service, child, Duration::from_millis(200)));
        } else {
            tokio::spawn(async move {
                owner.0.stop.notified().await;
                let transport_cancelled = matches!(
                    service.close_with_timeout(Duration::from_secs(1)).await,
                    Ok(Some(_))
                );
                owner.finish(ShutdownReceipt {
                    transport_cancelled,
                    child_exited: false,
                    forced: false,
                    timeout_unknown: !transport_cancelled,
                    exit_code: None,
                });
            });
        }
        (session, seen_rx, reply_tx, server_task)
    }

    #[tokio::test]
    async fn pending_production_dispatch_leaves_global_lock_free_and_close_retires_it() {
        let (session, seen, reply, server) = controlled_session().await;
        let host = Arc::new(Mutex::new(crate::AppState {
            session: Some(session.clone()),
            ..crate::AppState::default()
        }));
        let pending_host = host.clone();
        let pending = tokio::spawn(async move {
            crate::dispatch_host(
                &pending_host,
                IpcCommand::ReadNote(crate::ipc::ReadNoteArgs {
                    workspace: routing::OWNED_WORKSPACE_ID.into(),
                    project: FIXTURE_PROJECT.into(),
                    identifier: "controlled".into(),
                    expected_session: Some(identity()),
                }),
            )
            .await
        });
        seen.await.unwrap();
        assert!(
            host.try_lock().is_ok(),
            "pending transport must release the app lock"
        );
        let runtime = timeout(
            Duration::from_millis(200),
            crate::dispatch_host(&host, IpcCommand::GetRuntimeState(crate::ipc::EmptyArgs {})),
        )
        .await
        .unwrap();
        let IpcResponse::RuntimeState(runtime) = runtime else {
            panic!("runtime unavailable")
        };
        assert_eq!(runtime.status, "connected");
        assert_eq!(runtime.session_generation, Some(identity().generation));
        let receipt = timeout(Duration::from_secs(2), session.close())
            .await
            .unwrap();
        assert!(receipt.transport_cancelled);
        let _ = reply.send(());
        let response = timeout(Duration::from_secs(1), pending)
            .await
            .unwrap()
            .unwrap();
        assert!(
            matches!(response, IpcResponse::Error(_)),
            "retired result must not publish"
        );
        assert_eq!(session.snapshot().state, ConnectionState::Stopped);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn abandoned_consumers_remain_bounded_until_owned_rpc_finishes() {
        let (session, seen, reply, server) = controlled_session().await;
        let mut consumers = Vec::new();
        for _ in 0..8 {
            let owner = session.clone();
            consumers.push(tokio::spawn(async move {
                owner
                    .read(PreparedRead::Note {
                        identifier: "controlled".into(),
                        expected: Some(identity()),
                    })
                    .await
            }));
        }
        seen.await.unwrap();
        timeout(Duration::from_secs(1), async {
            while session.command_count() != 8 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        for consumer in consumers {
            consumer.abort();
            let _ = consumer.await;
        }
        assert_eq!(
            session.0.reads.available_permits(),
            0,
            "abandoning consumers must not release outstanding engine work"
        );
        let overflow = session
            .read(PreparedRead::Note {
                identifier: "controlled".into(),
                expected: Some(identity()),
            })
            .await
            .unwrap_err();
        assert_eq!(overflow.category, ErrorCategory::Unsupported);
        assert!(overflow.message.contains("capacity"));
        assert_eq!(
            session.command_count(),
            8,
            "overflow must not enter transport"
        );
        let host = Mutex::new(crate::AppState {
            session: Some(session.clone()),
            ..crate::AppState::default()
        });
        assert!(matches!(
            timeout(
                Duration::from_millis(200),
                crate::dispatch_host(&host, IpcCommand::GetRuntimeState(crate::ipc::EmptyArgs {}))
            )
            .await
            .unwrap(),
            IpcResponse::RuntimeState(_)
        ));
        let _ = reply.send(());
        server.await.unwrap();
        timeout(Duration::from_secs(1), async {
            while session.0.reads.available_permits() != 8 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap();
        session.close().await;
    }

    #[tokio::test]
    async fn missing_wrong_profile_and_wrong_generation_never_enter_transport() {
        let (session, seen, reply, server) = controlled_session().await;
        for expected in [
            None,
            Some(SessionIdentity {
                profile: EngineProfile::MainPreview,
                ..identity()
            }),
            Some(SessionIdentity {
                generation: 10,
                ..identity()
            }),
        ] {
            let result = session
                .read(PreparedRead::Note {
                    identifier: "controlled".into(),
                    expected,
                })
                .await
                .unwrap_err();
            assert_eq!(result.category, ErrorCategory::Policy);
        }
        let mut seen = seen;
        assert!(matches!(
            seen.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        session.close().await;
        drop(reply);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn read_deadline_retires_session_without_retry() {
        let (session, seen, reply, server) = controlled_session().await;
        let reader = session.clone();
        let pending = tokio::spawn(async move {
            reader
                .read_with_budget(
                    PreparedRead::Note {
                        identifier: "controlled".into(),
                        expected: Some(identity()),
                    },
                    Duration::from_millis(100),
                )
                .await
        });
        seen.await.unwrap();
        let result = pending.await.unwrap().unwrap_err();
        assert_eq!(result.category, ErrorCategory::Unsupported);
        assert_eq!(
            session.snapshot().failure,
            Some(FailureKind::TimeoutUnknown)
        );
        assert!(session
            .read(PreparedRead::Note {
                identifier: "controlled".into(),
                expected: Some(identity())
            })
            .await
            .is_err());
        session.close().await;
        drop(reply);
        server.await.unwrap();
    }

    #[tokio::test]
    async fn closed_transport_is_runtime_failure_not_successful_empty_note() {
        let (session, seen, reply, server) = controlled_session().await;
        let reader = session.clone();
        let pending = tokio::spawn(async move {
            reader
                .read(PreparedRead::Note {
                    identifier: "controlled".into(),
                    expected: Some(identity()),
                })
                .await
        });
        seen.await.unwrap();
        drop(reply); // peer closes after accepting the request, without a response
        let result = pending.await.unwrap().unwrap_err();
        assert_eq!(result.category, ErrorCategory::Unsupported);
        assert_eq!(session.snapshot().failure, Some(FailureKind::Transport));
        session.close().await;
        server.await.unwrap();
    }

    #[tokio::test]
    async fn production_without_session_is_unavailable_not_connected_empty() {
        let host = Mutex::new(crate::AppState::default());
        let response = crate::dispatch_host(
            &host,
            IpcCommand::ListTree(crate::ipc::ListTreeArgs {
                workspace: routing::OWNED_WORKSPACE_ID.into(),
                project: FIXTURE_PROJECT.into(),
                directory: None,
                expected_session: None,
                cursor: None,
                page_size: None,
            }),
        )
        .await;
        assert!(matches!(
            response,
            IpcResponse::Error(IpcError {
                category: ErrorCategory::Unsupported,
                ..
            })
        ));
        let IpcResponse::RuntimeState(runtime) =
            crate::dispatch_host(&host, IpcCommand::GetRuntimeState(crate::ipc::EmptyArgs {}))
                .await
        else {
            panic!("runtime state")
        };
        assert_eq!(runtime.status, "not_started");
        assert_eq!(runtime.session_generation, None);
    }

    #[test]
    #[ignore = "owned subprocess fixture; invoked only by lifecycle regression"]
    fn lifecycle_child_fixture() {
        std::thread::sleep(Duration::from_secs(60));
    }

    #[tokio::test]
    async fn transport_eof_with_live_child_retires_without_waiting_for_another_read() {
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--ignored",
                "--exact",
                "engine_session::tests::lifecycle_child_fixture",
            ])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .kill_on_drop(true);
        #[cfg(windows)]
        {
            command.creation_flags(0x08000000);
        }
        let mut child = command.spawn().unwrap();
        assert!(child.try_wait().unwrap().is_none());
        let (session, _seen, _reply, server) = controlled_session_with_child(Some(child)).await;
        let mut changes = session.subscribe();
        server.abort(); // remote stdio disappears, but the owned OS child is still alive
        timeout(Duration::from_secs(3), async {
            loop {
                if session.snapshot().shutdown.is_some() {
                    break;
                }
                changes.changed().await.unwrap();
            }
        })
        .await
        .unwrap();
        let snapshot = session.snapshot();
        assert_eq!(snapshot.state, ConnectionState::Failed);
        assert_eq!(snapshot.failure, Some(FailureKind::Transport));
        let receipt = snapshot.shutdown.unwrap();
        assert!(receipt.transport_cancelled);
        assert!(receipt.child_exited);
        assert!(receipt.forced);
        assert!(receipt.timeout_unknown);
        assert!(session.require_identity(Some(&identity())).is_err());
    }
}
