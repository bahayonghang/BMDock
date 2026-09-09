//! Developer-only G0 probe. Never expose this control channel as desktop IPC.
//! The official SDK owns MCP framing; BMDock owns the child and shutdown evidence.
use rmcp::{model::ClientInfo, ServiceExt};
use serde_json::{json, Value};
use std::{error::Error, io::Write, path::PathBuf, process::Stdio, time::Duration};
use tokio::{io::AsyncBufReadExt, process::Command, time::timeout};

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;
const RPC_TIMEOUT: Duration = Duration::from_secs(90);
const CLOSE_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CONTROL_LINE: usize = 1_048_576;

fn emit(value: Value) -> Result<()> {
    let mut out = std::io::stdout().lock();
    serde_json::to_writer(&mut out, &value)?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}

fn validate_request(request: &Value) -> Result<()> {
    let method = request["method"].as_str().ok_or("Missing method")?;
    if !matches!(
        method,
        "tools/list"
            | "resources/list"
            | "resources/templates/list"
            | "prompts/list"
            | "prompts/get"
            | "resources/read"
            | "tools/call"
    ) {
        return Err("Method not allowed in the G0 probe".into());
    }
    if method == "tools/call" {
        let name = request["params"]["name"]
            .as_str()
            .ok_or("Missing tool name")?;
        if !matches!(
            name,
            "list_memory_projects"
                | "read_note"
                | "read_content"
                | "search_notes"
                | "write_note"
                | "edit_note"
                | "search"
                | "fetch"
                | "__bmdock_missing_tool__"
        ) {
            return Err("Tool not allowed in the G0 probe".into());
        }
        if matches!(name, "write_note" | "edit_note")
            && request["params"]["arguments"]["project"] != "bmdock-fixture"
        {
            return Err("Writes are restricted to the generated fixture project".into());
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args == ["--version"] {
        println!("bmdock-probe {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    if args.len() != 3 {
        return Err(
            "usage: bmdock-probe <managed-python> <engine-worker.py> <owned-sandbox>".into(),
        );
    }
    let python = PathBuf::from(&args[0]);
    let worker = PathBuf::from(&args[1]);
    let sandbox = PathBuf::from(&args[2]).canonicalize()?;
    if !python.is_absolute() || !python.is_file() || !worker.is_absolute() || !worker.is_file() {
        return Err("Absolute existing Python and worker paths are required".into());
    }
    let marker: Value =
        serde_json::from_slice(&std::fs::read(sandbox.join(".bmdock-g0-sandbox.json"))?)?;
    if marker["kind"] != "bmdock-g0" {
        return Err("Refusing a non-sandbox configuration".into());
    }
    let configured = PathBuf::from(std::env::var("BASIC_MEMORY_CONFIG_DIR")?).canonicalize()?;
    if configured != sandbox.join("config").canonicalize()? {
        return Err("Configuration directory does not match the sandbox".into());
    }
    for (key, value) in [
        ("BASIC_MEMORY_AUTO_UPDATE", "false"),
        ("BASIC_MEMORY_SEMANTIC_SEARCH_ENABLED", "false"),
        ("BASIC_MEMORY_FORCE_LOCAL", "true"),
    ] {
        if std::env::var(key).as_deref() != Ok(value) {
            return Err(format!("Required isolation setting missing: {key}").into());
        }
    }
    let mut child = Command::new(python)
        .arg(worker)
        .arg("serve")
        .current_dir(&sandbox)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .kill_on_drop(true)
        .spawn()?;
    let stdout = child.stdout.take().ok_or("No child stdout")?;
    let stdin = child.stdin.take().ok_or("No child stdin")?;
    let info: ClientInfo = serde_json::from_value(json!({
        "protocolVersion": "2025-11-25", "capabilities": {},
        "clientInfo": {"name": "BMDock-G0", "version": env!("CARGO_PKG_VERSION")}
    }))?;
    let service = timeout(Duration::from_secs(120), info.serve((stdout, stdin))).await??;
    emit(json!({"event": "connected", "server": service.peer_info(), "childPid": child.id()}))?;
    let mut control = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buffer = Vec::new();
    let mut result = Ok(());
    loop {
        buffer.clear();
        // take() also bounds allocation on a malicious/accidental giant control line.
        use tokio::io::AsyncReadExt;
        let n = (&mut control)
            .take((MAX_CONTROL_LINE + 1) as u64)
            .read_until(b'\n', &mut buffer)
            .await?;
        if n == 0 {
            break;
        }
        if n > MAX_CONTROL_LINE {
            result = Err("Control line too large".into());
            break;
        }
        let request: Value = serde_json::from_slice(&buffer)?;
        let id = request["id"].clone();
        if let Err(error) = validate_request(&request) {
            emit(json!({"id": id, "error": {"kind": "policy", "message": error.to_string()}}))?;
            continue;
        }
        let payload = json!({"method": request["method"], "params": request.get("params").cloned().unwrap_or(json!({}))});
        match serde_json::from_value(payload) {
            Ok(typed) => match timeout(RPC_TIMEOUT, service.send_request(typed)).await {
                Ok(Ok(response)) => emit(json!({"id": id, "result": response}))?,
                Ok(Err(error)) => emit(
                    json!({"id": id, "error": {"kind": "rpc_or_transport", "message": error.to_string()}}),
                )?,
                Err(_) => {
                    emit(
                        json!({"id": id, "error": {"kind": "timeout_unknown", "message": "Result unknown. No retry was performed."}}),
                    )?;
                    // Stop this session: later replies must not be mistaken for a retry.
                    break;
                }
            },
            Err(error) => {
                emit(json!({"id": id, "error": {"kind": "schema", "message": error.to_string()}}))?
            }
        }
    }
    let sdk_closed = matches!(timeout(CLOSE_TIMEOUT, service.cancel()).await, Ok(Ok(_)));
    let exit = match timeout(CLOSE_TIMEOUT, child.wait()).await {
        Ok(status) => json!({"forced": false, "exitCode": status?.code()}),
        Err(_) => {
            child.kill().await?;
            let status = child.wait().await?;
            json!({"forced": true, "exitCode": status.code()})
        }
    };
    emit(
        json!({"event": "shutdown", "sdkClosed": sdk_closed, "process": exit,
        "materialization": "not_proven_by_process_exit"}),
    )?;
    if !sdk_closed || exit["forced"] == true || exit["exitCode"] != 0 {
        return Err("Engine did not shut down cleanly".into());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_method_is_denied() {
        assert!(validate_request(&json!({"method": "shell/exec"})).is_err());
    }
    #[test]
    fn unknown_tool_is_denied() {
        assert!(validate_request(
            &json!({"method": "tools/call", "params": {"name": "delete_project"}})
        )
        .is_err());
    }
    #[test]
    fn fixture_write_requires_explicit_project() {
        assert!(validate_request(&json!({"method": "tools/call", "params": {"name": "write_note", "arguments": {"project": "production"}}})).is_err());
        assert!(validate_request(&json!({"method": "tools/call", "params": {"name": "write_note", "arguments": {"project": "bmdock-fixture"}}})).is_ok());
    }
    #[test]
    fn discovery_is_allowed() {
        for method in [
            "tools/list",
            "resources/list",
            "resources/templates/list",
            "prompts/list",
        ] {
            assert!(validate_request(&json!({"method": method})).is_ok());
        }
    }
}
