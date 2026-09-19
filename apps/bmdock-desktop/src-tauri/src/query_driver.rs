//! Headless, host-only acceptance control. No raw renderer MCP surface.
use crate::{
    dispatch_host,
    engine_session::{EngineSession, FixtureLaunch, PreparedRead},
    ipc::{self, IpcCommand, IpcError, IpcResponse},
    AppState,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    io::{self, BufRead, Read, Write},
    sync::Mutex,
    time::Instant,
};

#[derive(Deserialize)]
#[serde(untagged)]
enum Line {
    Request(Box<Request>),
    Control(Control),
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    id: String,
    lane: Lane,
    command: IpcCommand,
}
#[derive(Clone, Copy, Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
enum Lane {
    Direct,
    Adapter,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Control {
    id: String,
    control: ControlKind,
}
#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum ControlKind {
    Status,
    Shutdown,
}

fn emit(value: Value) -> Result<(), IpcError> {
    let stdout = io::stdout();
    let mut output = stdout.lock();
    serde_json::to_writer(&mut output, &value).map_err(|_| io_error())?;
    output
        .write_all(b"\n")
        .and_then(|()| output.flush())
        .map_err(|_| io_error())
}
fn io_error() -> IpcError {
    IpcError {
        category: ipc::ErrorCategory::Unsupported,
        message: "Query driver control I/O failed".into(),
    }
}

pub async fn run(launch: FixtureLaunch) -> Result<(), IpcError> {
    let session = EngineSession::start(launch)?;
    let host = Mutex::new(AppState {
        session: Some(session.clone()),
        ..AppState::default()
    });
    let result = async {
        session.wait_connected().await?;
        emit(json!({"event":"ready","session":session.identity(),
            "runtime":dispatch_host(&host,IpcCommand::GetRuntimeState(ipc::EmptyArgs{})).await,
            "engine_sha":session.identity().profile.commit()}))?;
        let stdin=io::stdin();
        let mut input=stdin.lock();
        loop {
            let mut buffer=Vec::new();
            let n=(&mut input).take(65_537).read_until(b'\n',&mut buffer).map_err(|_|io_error())?;
            if n==0 {break;}
            if n>65_536 {return Err(ipc::IpcError {category:ipc::ErrorCategory::Schema,message:"Driver line exceeds 64 KiB".into()});}
            let line:Line=serde_json::from_slice(&buffer).map_err(|_|ipc::IpcError {category:ipc::ErrorCategory::Schema,message:"Invalid typed query driver line".into()})?;
            match line {
                Line::Control(Control{id,control:ControlKind::Status}) => emit(json!({"id":id,"session":session.identity(),
                    "runtime":dispatch_host(&host,IpcCommand::GetRuntimeState(ipc::EmptyArgs{})).await}))?,
                Line::Control(Control{id,control:ControlKind::Shutdown}) => {
                    let shutdown=session.close().await;
                    emit(json!({"id":id,"shutdown":shutdown,
                        "stopped":dispatch_host(&host,IpcCommand::GetRuntimeState(ipc::EmptyArgs{})).await}))?;
                    break;
                }
                Line::Request(request) => {
                    let Request {id,lane,command}=*request;
                    let before=session.command_count();
                    let prepared=PreparedRead::from_command(&command).unwrap_or_else(||Err(ipc::IpcError {
                        category:ipc::ErrorCategory::Policy,message:"Driver accepts only typed engine read commands".into()}));
                    let request=prepared.as_ref().ok().map(PreparedRead::tool_call);
                    let started=Instant::now();
                    let response=match prepared {
                        Err(error)=>json!(IpcResponse::Error(error)),
                        Ok(read)=>match lane {
                            Lane::Direct=>match session.raw_read(&read).await {
                                Ok(response)=>response,Err(error)=>json!(IpcResponse::Error(error)),
                            },
                            Lane::Adapter=>json!(dispatch_host(&host,command).await),
                        },
                    };
                    let elapsed_ms=started.elapsed().as_secs_f64()*1000.0;
                    emit(json!({"id":id,"lane":lane,"request":request,"response":response,
                        "elapsed_ms":elapsed_ms,"command_count":session.command_count()-before}))?;
                }
            }
        }
        Ok(())
    }.await;
    session.close().await;
    result
}
