use serde::{Deserialize, Serialize};
#[cfg(test)]
use std::{
    collections::BTreeMap,
    path::PathBuf,
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineProfile {
    Release,
    MainPreview,
}

impl EngineProfile {
    pub const fn id(self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::MainPreview => "main-preview",
        }
    }
    pub const fn commit(self) -> &'static str {
        match self {
            Self::Release => "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048",
            Self::MainPreview => "3452c821d76c083823d020984d71e06904a1ff1e",
        }
    }
    pub const fn expected_tools(self) -> usize {
        match self {
            Self::Release => 21,
            Self::MainPreview => 27,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionState {
    NotStarted,
    Starting,
    Connected,
    Stopping,
    Stopped,
    Failed,
}

impl ConnectionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotStarted => "not_started",
            Self::Starting => "starting",
            Self::Connected => "connected",
            Self::Stopping => "stopping",
            Self::Stopped => "stopped",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailureKind {
    Policy,
    Transport,
    TimeoutUnknown,
    Process,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShutdownReceipt {
    pub transport_cancelled: bool,
    pub child_exited: bool,
    pub forced: bool,
    pub timeout_unknown: bool,
    pub exit_code: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RuntimeSnapshot {
    pub state: ConnectionState,
    pub profile: Option<EngineProfile>,
    pub child_pid: Option<u32>,
    pub failure: Option<FailureKind>,
    pub shutdown: Option<ShutdownReceipt>,
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineLaunchSpec {
    pub profile: EngineProfile,
    pub program: PathBuf,
    pub args: Vec<String>,
    pub cwd: Option<PathBuf>,
    pub env: BTreeMap<String, String>,
}

#[cfg(test)]
pub trait ChildHandle: Send {
    fn pid(&self) -> Option<u32>;
    fn wait(&mut self, timeout: Duration) -> WaitOutcome;
    fn kill(&mut self) -> bool;
}

#[cfg(test)]
pub struct OwnedChild {
    child: Child,
}

#[cfg(test)]
impl OwnedChild {
    pub fn spawn(spec: &EngineLaunchSpec) -> std::io::Result<Self> {
        let mut command = Command::new(&spec.program);
        command
            .args(&spec.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit());
        if let Some(cwd) = &spec.cwd {
            command.current_dir(cwd);
        }
        command.envs(&spec.env);
        Ok(Self {
            child: command.spawn()?,
        })
    }
}

#[cfg(test)]
impl Drop for OwnedChild {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

#[cfg(test)]
impl ChildHandle for OwnedChild {
    fn pid(&self) -> Option<u32> {
        Some(self.child.id())
    }

    fn wait(&mut self, timeout: Duration) -> WaitOutcome {
        let deadline = Instant::now() + timeout;
        loop {
            match self.child.try_wait() {
                Ok(Some(status)) => return WaitOutcome::Exited(status.code()),
                Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
                Ok(None) => return WaitOutcome::TimedOut,
                Err(_) => return WaitOutcome::Unknown,
            }
        }
    }

    fn kill(&mut self) -> bool {
        self.child.kill().is_ok()
    }
}

#[cfg(test)]
pub trait Transport: Send {
    fn cancel(&mut self) -> CancelOutcome;
}
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WaitOutcome {
    Exited(Option<i32>),
    TimedOut,
    Unknown,
}
#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CancelOutcome {
    Cancelled,
    TimedOut,
    Failed,
}

pub struct Supervisor {
    snapshot: RuntimeSnapshot,
    #[cfg(test)]
    child: Option<Box<dyn ChildHandle>>,
}

impl Default for Supervisor {
    fn default() -> Self {
        Self {
            snapshot: RuntimeSnapshot {
                state: ConnectionState::NotStarted,
                profile: None,
                child_pid: None,
                failure: None,
                shutdown: None,
            },
            #[cfg(test)]
            child: None,
        }
    }
}

impl Supervisor {
    pub fn snapshot(&self) -> RuntimeSnapshot {
        self.snapshot.clone()
    }

    #[cfg(test)]
    fn can_start(&self) -> bool {
        matches!(
            self.snapshot.state,
            ConnectionState::NotStarted | ConnectionState::Stopped
        )
    }

    #[cfg(test)]
    pub fn start(
        &mut self,
        spec: EngineLaunchSpec,
        child: Box<dyn ChildHandle>,
    ) -> Result<(), FailureKind> {
        if !self.can_start() || spec.program.as_os_str().is_empty() {
            return Err(FailureKind::Policy);
        }
        self.snapshot = RuntimeSnapshot {
            state: ConnectionState::Starting,
            profile: Some(spec.profile),
            child_pid: child.pid(),
            failure: None,
            shutdown: None,
        };
        self.child = Some(child);
        Ok(())
    }

    #[cfg(test)]
    pub fn spawn(&mut self, spec: EngineLaunchSpec) -> Result<(), FailureKind> {
        if !self.can_start() || spec.program.as_os_str().is_empty() {
            return Err(FailureKind::Policy);
        }
        let child = OwnedChild::spawn(&spec).map_err(|_| FailureKind::Process)?;
        self.start(spec, Box::new(child))
    }
    #[cfg(test)]
    pub fn mark_connected(&mut self) -> Result<(), FailureKind> {
        if self.snapshot.state != ConnectionState::Starting {
            return Err(FailureKind::Policy);
        }
        self.snapshot.state = ConnectionState::Connected;
        Ok(())
    }
    #[cfg(test)]
    pub fn mark_failed(&mut self, kind: FailureKind) -> Result<(), FailureKind> {
        if !matches!(
            self.snapshot.state,
            ConnectionState::Starting | ConnectionState::Connected
        ) {
            return Err(FailureKind::Policy);
        }
        self.snapshot.state = ConnectionState::Failed;
        self.snapshot.failure = Some(kind);
        Ok(())
    }
    #[cfg(test)]
    pub fn shutdown<T: Transport>(
        &mut self,
        transport: &mut T,
        timeout: Duration,
    ) -> Result<ShutdownReceipt, FailureKind> {
        if !matches!(
            self.snapshot.state,
            ConnectionState::Starting | ConnectionState::Connected | ConnectionState::Failed
        ) {
            return Err(FailureKind::Policy);
        }
        self.snapshot.state = ConnectionState::Stopping;
        let cancel = transport.cancel();
        let mut receipt = ShutdownReceipt {
            transport_cancelled: matches!(cancel, CancelOutcome::Cancelled),
            child_exited: false,
            forced: false,
            timeout_unknown: matches!(cancel, CancelOutcome::TimedOut | CancelOutcome::Failed),
            exit_code: None,
        };
        if let Some(child) = self.child.as_mut() {
            match child.wait(timeout) {
                WaitOutcome::Exited(code) => {
                    receipt.child_exited = true;
                    receipt.exit_code = code;
                }
                WaitOutcome::TimedOut | WaitOutcome::Unknown => {
                    receipt.forced = child.kill();
                    receipt.timeout_unknown = true;
                }
            }
        }
        self.child = None;
        self.snapshot.state = ConnectionState::Stopped;
        self.snapshot.shutdown = Some(receipt.clone());
        Ok(receipt)
    }
}

#[cfg(test)]
impl Drop for Supervisor {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = child.kill();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    struct FakeChild {
        pid: Option<u32>,
        wait: WaitOutcome,
        killed: bool,
    }
    impl ChildHandle for FakeChild {
        fn pid(&self) -> Option<u32> {
            self.pid
        }
        fn wait(&mut self, _: Duration) -> WaitOutcome {
            self.wait
        }
        fn kill(&mut self) -> bool {
            self.killed = true;
            true
        }
    }
    struct FakeTransport {
        outcome: CancelOutcome,
        calls: usize,
    }
    impl Transport for FakeTransport {
        fn cancel(&mut self) -> CancelOutcome {
            self.calls += 1;
            self.outcome
        }
    }
    fn spec(profile: EngineProfile) -> EngineLaunchSpec {
        EngineLaunchSpec {
            profile,
            program: "engine".into(),
            args: vec![],
            cwd: None,
            env: BTreeMap::new(),
        }
    }
    #[test]
    fn profile_constants_are_isolated() {
        assert_eq!(EngineProfile::Release.id(), "release");
        assert_eq!(EngineProfile::MainPreview.id(), "main-preview");
        assert_eq!(
            EngineProfile::Release.commit(),
            "c0bd87c6d5a4a58034b1d6c8c5018e443b0bd048"
        );
        assert_eq!(
            EngineProfile::MainPreview.commit(),
            "3452c821d76c083823d020984d71e06904a1ff1e"
        );
        assert_ne!(
            EngineProfile::Release.commit(),
            EngineProfile::MainPreview.commit()
        );
        assert_eq!(EngineProfile::Release.expected_tools(), 21);
        assert_eq!(EngineProfile::MainPreview.expected_tools(), 27);
    }
    #[test]
    fn lifecycle_rejects_invalid_transitions() {
        let mut s = Supervisor::default();
        assert_eq!(s.mark_connected(), Err(FailureKind::Policy));
        assert_eq!(
            s.mark_failed(FailureKind::Process),
            Err(FailureKind::Policy)
        );
        s.start(
            spec(EngineProfile::Release),
            Box::new(FakeChild {
                pid: Some(7),
                wait: WaitOutcome::Exited(Some(0)),
                killed: false,
            }),
        )
        .unwrap();
        assert_eq!(
            s.start(
                spec(EngineProfile::Release),
                Box::new(FakeChild {
                    pid: None,
                    wait: WaitOutcome::Unknown,
                    killed: false
                })
            ),
            Err(FailureKind::Policy)
        );
        assert_eq!(s.snapshot().child_pid, Some(7));
        s.mark_connected().unwrap();
        assert_eq!(
            s.spawn(spec(EngineProfile::MainPreview)),
            Err(FailureKind::Policy)
        );
        assert_eq!(
            s.start(
                spec(EngineProfile::MainPreview),
                Box::new(FakeChild {
                    pid: Some(99),
                    wait: WaitOutcome::Exited(Some(0)),
                    killed: false,
                }),
            ),
            Err(FailureKind::Policy)
        );
        assert_eq!(s.snapshot().profile, Some(EngineProfile::Release));
        assert_eq!(s.snapshot().child_pid, Some(7));
        s.mark_failed(FailureKind::Transport).unwrap();
        assert_eq!(s.snapshot().failure, Some(FailureKind::Transport));
        assert_eq!(s.snapshot().profile, Some(EngineProfile::Release));
        assert_eq!(s.snapshot().state, ConnectionState::Failed);
    }
    #[test]
    fn shutdown_cancels_before_wait_and_records_exit() {
        let mut s = Supervisor::default();
        s.start(
            spec(EngineProfile::Release),
            Box::new(FakeChild {
                pid: Some(7),
                wait: WaitOutcome::Exited(Some(0)),
                killed: false,
            }),
        )
        .unwrap();
        let mut t = FakeTransport {
            outcome: CancelOutcome::Cancelled,
            calls: 0,
        };
        let r = s.shutdown(&mut t, Duration::from_secs(1)).unwrap();
        assert_eq!(t.calls, 1);
        assert!(r.transport_cancelled && r.child_exited && !r.forced && !r.timeout_unknown);
    }
    #[test]
    fn timeout_forces_kill_and_is_unknown() {
        let mut s = Supervisor::default();
        s.start(
            spec(EngineProfile::MainPreview),
            Box::new(FakeChild {
                pid: Some(8),
                wait: WaitOutcome::TimedOut,
                killed: false,
            }),
        )
        .unwrap();
        let mut t = FakeTransport {
            outcome: CancelOutcome::TimedOut,
            calls: 0,
        };
        let r = s.shutdown(&mut t, Duration::from_millis(1)).unwrap();
        assert!(r.forced && r.timeout_unknown && !r.child_exited);
        assert_eq!(s.snapshot().state, ConnectionState::Stopped);
    }

    #[test]
    fn transport_failed_and_wait_unknown_stay_timeout_unknown() {
        let mut s = Supervisor::default();
        s.start(
            spec(EngineProfile::Release),
            Box::new(FakeChild {
                pid: Some(8),
                wait: WaitOutcome::Unknown,
                killed: false,
            }),
        )
        .unwrap();
        let mut t = FakeTransport {
            outcome: CancelOutcome::Failed,
            calls: 0,
        };
        let r = s.shutdown(&mut t, Duration::from_millis(1)).unwrap();
        assert_eq!(t.calls, 1);
        assert!(!r.transport_cancelled && r.timeout_unknown && r.forced && !r.child_exited);
    }

    #[test]
    fn shutdown_is_rejected_before_start() {
        let mut s = Supervisor::default();
        let mut t = FakeTransport {
            outcome: CancelOutcome::Cancelled,
            calls: 0,
        };
        assert_eq!(
            s.shutdown(&mut t, Duration::from_millis(1)),
            Err(FailureKind::Policy)
        );
        assert_eq!(t.calls, 0);
    }

    #[test]
    fn owned_child_can_be_started_and_stopped() {
        let mut s = Supervisor::default();
        let mut spec = spec(EngineProfile::Release);
        #[cfg(windows)]
        {
            spec.program = "cmd".into();
            spec.args = vec!["/C".into(), "exit".into(), "0".into()];
        }
        #[cfg(not(windows))]
        {
            spec.program = "sh".into();
            spec.args = vec!["-c".into(), "exit 0".into()];
        }
        s.spawn(spec).unwrap();
        s.mark_connected().unwrap();
        let mut t = FakeTransport {
            outcome: CancelOutcome::Cancelled,
            calls: 0,
        };
        let receipt = s.shutdown(&mut t, Duration::from_secs(2)).unwrap();
        assert!(receipt.transport_cancelled && receipt.child_exited);
    }

    #[test]
    fn failed_shutdown_then_start_keeps_profiles_sequential() {
        let mut s = Supervisor::default();
        s.start(
            spec(EngineProfile::Release),
            Box::new(FakeChild {
                pid: Some(7),
                wait: WaitOutcome::Exited(Some(0)),
                killed: false,
            }),
        )
        .unwrap();
        s.mark_failed(FailureKind::TimeoutUnknown).unwrap();
        let mut t = FakeTransport {
            outcome: CancelOutcome::Cancelled,
            calls: 0,
        };
        let receipt = s.shutdown(&mut t, Duration::from_millis(1)).unwrap();
        assert!(receipt.child_exited && !receipt.forced);
        assert_eq!(s.snapshot().state, ConnectionState::Stopped);
        s.start(
            spec(EngineProfile::MainPreview),
            Box::new(FakeChild {
                pid: Some(8),
                wait: WaitOutcome::Exited(Some(1)),
                killed: false,
            }),
        )
        .unwrap();
        assert_eq!(s.snapshot().profile, Some(EngineProfile::MainPreview));
        assert_eq!(s.snapshot().state, ConnectionState::Starting);
        assert_eq!(s.snapshot().failure, None);
    }

    #[test]
    fn empty_program_is_policy_without_process_error() {
        let mut s = Supervisor::default();
        let mut empty = spec(EngineProfile::Release);
        empty.program = PathBuf::new();
        assert_eq!(s.spawn(empty), Err(FailureKind::Policy));
        assert_eq!(s.snapshot().state, ConnectionState::NotStarted);
    }
}
