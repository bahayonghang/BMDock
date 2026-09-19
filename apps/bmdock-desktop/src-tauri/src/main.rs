//! T05 desktop host: window bootstrap for `bmdock-app`.
//! Typed IPC (`ipc`) is T06; engine Supervisor is T07. Do not strip those modules
//! to recreate an empty T05 `main`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{Emitter, Manager, State};

mod backups;
mod conflict;
#[cfg(test)]
mod content_safety;
mod drafts;
mod drain;
mod engine_queries;
mod engine_session;
mod ipc;
mod library;
mod preflight;
mod query_driver;
mod routing;
mod supervisor;
mod windows_runtime;

struct AppState {
    supervisor: supervisor::Supervisor,
    session: Option<engine_session::EngineSession>,
    route: routing::RouteState,
    library: Box<dyn library::NoteLibrary>,
    backups: Box<dyn backups::BackupStore>,
    drafts: Box<dyn drafts::DraftStore>,
    conflicts: conflict::ConflictCoordinator,
    drain: drain::HostDrain,
}

#[tauri::command]
async fn ipc_invoke(
    state: State<'_, Mutex<AppState>>,
    command: ipc::IpcCommand,
) -> Result<ipc::IpcResponse, ()> {
    Ok(dispatch_host(state.inner(), command).await)
}

/// Shared by Tauri and the isolated headless acceptance entry. All engine awaits
/// are outside the app lock; no parallel test-only client bypasses this boundary.
async fn dispatch_host(state: &Mutex<AppState>, command: ipc::IpcCommand) -> ipc::IpcResponse {
    if let Some(prepared) = engine_session::PreparedRead::from_command(&command) {
        let prepared = match prepared {
            Ok(read) => read,
            Err(error) => return ipc::IpcResponse::Error(error),
        };
        let session = match state.lock() {
            Ok(host) => host.session.clone(),
            Err(_) => return host_unavailable(),
        };
        return match session {
            Some(session) => session
                .read(prepared)
                .await
                .unwrap_or_else(ipc::IpcResponse::Error),
            None => ipc::IpcResponse::Error(ipc::IpcError {
                category: ipc::ErrorCategory::Unsupported,
                message: "Official read session is unavailable".to_owned(),
            }),
        };
    }
    let mut host = match state.lock() {
        Ok(host) => host,
        Err(_) => {
            return ipc::IpcResponse::Error(ipc::IpcError {
                category: ipc::ErrorCategory::Unsupported,
                message: "Supervisor state is unavailable".to_owned(),
            })
        }
    };
    let snapshot = host
        .session
        .as_ref()
        .map(|session| session.snapshot())
        .unwrap_or_else(|| host.supervisor.snapshot());
    let session_generation = host
        .session
        .as_ref()
        .map(|session| session.identity().generation);
    let live_shutdown = host
        .session
        .as_ref()
        .map(|session| session.snapshot().shutdown);
    let AppState {
        route,
        library,
        backups,
        drafts,
        conflicts,
        drain,
        supervisor: _,
        session: _,
    } = &mut *host;
    match ipc::dispatch_with_drain(
        command,
        snapshot,
        route,
        library.as_ref(),
        backups.as_ref(),
        drafts.as_ref(),
        conflicts,
        drain,
    ) {
        Ok(ipc::IpcResponse::RuntimeState(mut response)) => {
            response.session_generation = session_generation;
            if let Some(shutdown) = live_shutdown {
                response.shutdown = shutdown;
            }
            ipc::IpcResponse::RuntimeState(response)
        }
        Ok(response) => response,
        Err(error) => ipc::IpcResponse::Error(error),
    }
}

fn host_unavailable() -> ipc::IpcResponse {
    ipc::IpcResponse::Error(ipc::IpcError {
        category: ipc::ErrorCategory::Unsupported,
        message: "Host state is unavailable".to_owned(),
    })
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            supervisor: supervisor::Supervisor::default(),
            session: None,
            route: routing::RouteState::default(),
            library: Box::new(library::EmptyLibrary),
            backups: Box::new(backups::EmptyBackupStore),
            drafts: Box::new(drafts::EmptyDraftStore),
            conflicts: conflict::ConflictCoordinator::default(),
            drain: drain::HostDrain::default(),
        }
    }
}

fn parse_fixture_launch(
    args: &[String],
) -> Result<Option<engine_session::FixtureLaunch>, ipc::IpcError> {
    if args.is_empty() {
        return Ok(None);
    }
    let invalid = || ipc::IpcError {
        category: ipc::ErrorCategory::Policy,
        message:
            "Use --fixture-session or --session-check with one pinned profile and generated sandbox"
                .to_owned(),
    };
    if args.len() != 3
        || !matches!(
            args[0].as_str(),
            "--fixture-session" | "--session-check" | "--query-driver"
        )
    {
        return Err(invalid());
    }
    let profile = match args[1].as_str() {
        "release" => supervisor::EngineProfile::Release,
        "main-preview" => supervisor::EngineProfile::MainPreview,
        _ => return Err(invalid()),
    };
    engine_session::FixtureLaunch::verified(profile, std::path::Path::new(&args[2])).map(Some)
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let launch = match parse_fixture_launch(&args) {
        Ok(launch) => launch,
        Err(error) => {
            eprintln!("{}", error.message);
            std::process::exit(2);
        }
    };
    if args.first().is_some_and(|arg| arg == "--query-driver") {
        let runtime = tokio::runtime::Runtime::new().expect("create runtime");
        if let Err(error) = runtime.block_on(query_driver::run(
            launch.expect("validated fixture arguments"),
        )) {
            eprintln!("{}", error.message);
            std::process::exit(1);
        }
        return;
    }
    if args.first().is_some_and(|arg| arg == "--session-check") {
        let runtime = tokio::runtime::Runtime::new().expect("create runtime");
        let result = runtime.block_on(session_check(launch.expect("validated fixture arguments")));
        match result {
            Ok(report) => println!("{report}"),
            Err(error) => {
                eprintln!("{}", error.message);
                std::process::exit(1);
            }
        }
        return;
    }
    let app = tauri::Builder::default()
        .manage(Mutex::new(AppState::default()))
        .setup(move |app| {
            if let Some(launch) = launch {
                let handle = app.handle().clone();
                tauri::async_runtime::spawn(async move {
                    match engine_session::EngineSession::start(launch) {
                        Ok(session) => {
                            let mut changes = session.subscribe();
                            if let Ok(mut host) = handle.state::<Mutex<AppState>>().lock() {
                                host.session = Some(session.clone());
                            }
                            loop {
                                let state = handle.state::<Mutex<AppState>>();
                                if let ipc::IpcResponse::RuntimeState(runtime) = dispatch_host(
                                    state.inner(),
                                    ipc::IpcCommand::GetRuntimeState(ipc::EmptyArgs {}),
                                )
                                .await
                                {
                                    if let Err(error) = handle.emit("runtime_state", runtime) {
                                        eprintln!("Runtime event delivery failed: {error}");
                                    }
                                }
                                if session.snapshot().shutdown.is_some()
                                    || changes.changed().await.is_err()
                                {
                                    break;
                                }
                            }
                        }
                        Err(error) => eprintln!("{}", error.message),
                    }
                });
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![ipc_invoke])
        .build(tauri::generate_context!())
        .expect("error while running BMDock application");
    app.run(|handle, event| {
        if let tauri::RunEvent::ExitRequested { api, .. } = event {
            let session = handle
                .state::<Mutex<AppState>>()
                .lock()
                .ok()
                .and_then(|host| host.session.clone());
            if let Some(session) = session.filter(|session| session.snapshot().shutdown.is_none()) {
                api.prevent_exit();
                let handle = handle.clone();
                tauri::async_runtime::spawn(async move {
                    session.close().await;
                    handle.exit(0);
                });
            }
        }
    });
}

async fn session_check(
    launch: engine_session::FixtureLaunch,
) -> Result<serde_json::Value, ipc::IpcError> {
    use serde_json::json;
    let session = engine_session::EngineSession::start(launch)?;
    let host = Mutex::new(AppState {
        session: Some(session.clone()),
        ..AppState::default()
    });
    let starting = dispatch_host(&host, ipc::IpcCommand::GetRuntimeState(ipc::EmptyArgs {})).await;
    let result = async {
        session.wait_connected().await?;
        let identity = session.identity();
        let pid = session.snapshot().child_pid;
        let tree = |directory: &str| ipc::IpcCommand::ListTree(ipc::ListTreeArgs {
            workspace: routing::OWNED_WORKSPACE_ID.to_owned(), project: ipc::FIXTURE_PROJECT.to_owned(),
            directory: Some(directory.to_owned()), expected_session: Some(identity.clone()), cursor: None, page_size: Some(50),
        });
        let read = || ipc::IpcCommand::ReadNote(ipc::ReadNoteArgs {
            workspace: routing::OWNED_WORKSPACE_ID.to_owned(), project: ipc::FIXTURE_PROJECT.to_owned(),
            identifier: "corpus/note-00000".to_owned(), expected_session: Some(identity.clone()),
        });
        // Fixture acceptance waits for upstream's initial background indexing.
        // Connected-empty remains an honest product result; readiness polling is
        // test-entry work, not an implicit retry added to the product dispatcher.
        let ready_started = std::time::Instant::now();
        let mut readiness_attempts = 0;
        let first = loop {
            readiness_attempts += 1;
            let response = dispatch_host(&host, read()).await;
            if matches!(response, ipc::IpcResponse::NoteRead(_)) { break response; }
            if ready_started.elapsed() >= std::time::Duration::from_secs(120) {
                return Err(ipc::IpcError { category: ipc::ErrorCategory::Unsupported,
                    message: "Generated fixture index did not become ready".to_owned() });
            }
            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
        };
        let root = dispatch_host(&host, tree("")).await;
        let nested = dispatch_host(&host, tree("group-000/item-000")).await;
        let drain = dispatch_host(&host, ipc::IpcCommand::BeginShutdown(ipc::EmptyArgs {})).await;
        let second = dispatch_host(&host, read()).await;
        let mut stale = identity.clone(); stale.generation = stale.generation.saturating_add(1);
        let stale = dispatch_host(&host, ipc::IpcCommand::ReadNote(ipc::ReadNoteArgs {
            workspace: routing::OWNED_WORKSPACE_ID.to_owned(), project: ipc::FIXTURE_PROJECT.to_owned(),
            identifier: "corpus/note-00000".to_owned(), expected_session: Some(stale),
        })).await;
        let runtime = dispatch_host(&host, ipc::IpcCommand::GetRuntimeState(ipc::EmptyArgs {})).await;
        let wrong_route = dispatch_host(&host, ipc::IpcCommand::ReadNote(ipc::ReadNoteArgs {
            workspace: "user-vault".to_owned(), project: ipc::FIXTURE_PROJECT.to_owned(),
            identifier: "corpus/note-00000".to_owned(), expected_session: Some(identity.clone()),
        })).await;
        Ok::<_, ipc::IpcError>(json!({"starting":starting, "runtime":runtime, "root":root, "nested":nested,
            "first":first, "second":second, "drain":drain, "stale":stale, "wrong_route":wrong_route,
            "child_reused":pid == session.snapshot().child_pid,
            "fixture_readiness_attempts":readiness_attempts,
            "session":identity, "engine_sha":identity.profile.commit(), "dispatch":"main.rs::dispatch_host"}))
    }.await;
    let shutdown = session.close().await;
    let mut report = result?;
    report["shutdown"] = json!(shutdown);
    report["stopped"] = serde_json::to_value(
        dispatch_host(&host, ipc::IpcCommand::GetRuntimeState(ipc::EmptyArgs {})).await,
    )
    .map_err(|_| ipc::IpcError {
        category: ipc::ErrorCategory::Schema,
        message: "Cannot serialize session result".into(),
    })?;
    Ok(report)
}
