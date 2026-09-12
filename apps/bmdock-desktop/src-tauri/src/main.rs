#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ipc;

#[tauri::command]
fn ipc_invoke(command: ipc::IpcCommand) -> ipc::IpcResponse {
    match ipc::dispatch(command) {
        Ok(response) => response,
        Err(error) => ipc::IpcResponse::Error(error),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ipc_invoke])
        .run(tauri::generate_context!())
        .expect("error while running BMDock application");
}
