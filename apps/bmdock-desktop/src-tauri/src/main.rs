//! T05 desktop host: window bootstrap for `bmdock-app`.
//! Typed IPC (`ipc`) is T06; engine Supervisor is T07. Do not strip those modules
//! to recreate an empty T05 `main`.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::State;

mod ipc;
mod preflight;
mod supervisor;

#[tauri::command]
fn ipc_invoke(
    state: State<'_, Mutex<supervisor::Supervisor>>,
    command: ipc::IpcCommand,
) -> ipc::IpcResponse {
    let snapshot = match state.lock() {
        Ok(supervisor) => supervisor.snapshot(),
        Err(_) => {
            return ipc::IpcResponse::Error(ipc::IpcError {
                category: ipc::ErrorCategory::Unsupported,
                message: "Supervisor state is unavailable".to_owned(),
            })
        }
    };
    match ipc::dispatch_with_snapshot(command, snapshot) {
        Ok(response) => response,
        Err(error) => ipc::IpcResponse::Error(error),
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(supervisor::Supervisor::default()))
        .invoke_handler(tauri::generate_handler![ipc_invoke])
        .run(tauri::generate_context!())
        .expect("error while running BMDock application");
}
