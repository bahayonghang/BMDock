use std::sync::Mutex;

use serde::{Deserialize, Serialize};

use crate::conflict::{self, ConflictCoordinator};
use crate::supervisor::{ConnectionState, RuntimeSnapshot, ShutdownReceipt};

pub const UNSUPPORTED_HOST_DRAINING: &str = "host is draining";
#[allow(dead_code)]
pub const FORCED_KILL_UNVERIFIED: &str =
    "forced-kill recovery remains UNVERIFIED; a successful host drain is not that proof";
#[allow(dead_code)]
pub const JOB_OBJECT_UNVERIFIED: &str =
    "Job Object assignment remains UNVERIFIED; T07 unit fakes are not native process-tree evidence";
#[allow(dead_code)]
pub const SLEEP_RESUME_UNVERIFIED: &str = "sleep-resume recovery remains UNVERIFIED";
#[allow(dead_code)]
pub const DISK_FAILURE_UNVERIFIED: &str = "disk-failure injection remains UNVERIFIED";
#[allow(dead_code)]
pub const T12_RESTORE_IS_NOT_T17_DRAIN: &str =
    "T12 fixture restore is not T17 drain; restore_fixture remains a distinct command";
#[allow(dead_code)]
pub const T16_CONFLICT_IS_NOT_T17_DRAIN: &str =
    "T16 classified_as=conflict is not T17 host drain; drain refuses new CRUD as unsupported";
#[allow(dead_code)]
pub const T07_FAKES_ARE_NOT_NATIVE_PROCESS_TREE: &str =
    "T07/T17 deterministic fakes are not a native process-tree; Job Object and live engine lifespan remain UNVERIFIED";
#[allow(dead_code)]
pub const T17_IS_NOT_T37_T38: &str =
    "a successful drain test is not T37 install/upgrade recovery or T38 native installer proof";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DrainPhase {
    Idle,
    Draining,
    Drained,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DrainClass {
    IdleNotStarted,
    InflightUnknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrainResultDto {
    pub host_drain: DrainPhase,
    pub supervisor_status: String,
    pub engine_spawned: bool,
    pub child_killed: bool,
    pub files_written: bool,
    pub classified_as: DrainClass,
    pub inflight_unknown: Vec<String>,
    pub shutdown: ShutdownReceipt,
    pub scanned_user_obsidian_vault: bool,
    pub scanned_user_basic_memory_home: bool,
}

#[derive(Debug)]
pub struct HostDrain {
    phase: Mutex<DrainPhase>,
    last: Mutex<Option<DrainResultDto>>,
}

impl Default for HostDrain {
    fn default() -> Self {
        Self {
            phase: Mutex::new(DrainPhase::Idle),
            last: Mutex::new(None),
        }
    }
}

impl HostDrain {
    pub fn phase(&self) -> DrainPhase {
        self.phase
            .lock()
            .map(|phase| *phase)
            .unwrap_or(DrainPhase::Draining)
    }

    pub fn is_closed(&self) -> bool {
        !matches!(self.phase(), DrainPhase::Idle)
    }

    pub fn last_shutdown(&self) -> Option<ShutdownReceipt> {
        self.last
            .lock()
            .ok()
            .and_then(|last| last.as_ref().map(|dto| dto.shutdown.clone()))
    }

    pub fn begin(
        &self,
        snapshot: &RuntimeSnapshot,
        conflicts: &ConflictCoordinator,
    ) -> DrainResultDto {
        conflict::refuse_auto_retry(snapshot);
        let inflight_unknown = conflicts.inflight_keys();
        let unknown = !inflight_unknown.is_empty();
        let phase = if unknown {
            DrainPhase::Draining
        } else {
            DrainPhase::Drained
        };
        let dto = DrainResultDto {
            host_drain: phase,
            supervisor_status: snapshot.state.as_str().to_owned(),
            engine_spawned: engine_spawned_from_lifecycle(snapshot.state),
            child_killed: false,
            files_written: false,
            classified_as: if unknown {
                DrainClass::InflightUnknown
            } else {
                DrainClass::IdleNotStarted
            },
            inflight_unknown,
            shutdown: drain_receipt(snapshot, unknown),
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
        };
        if let Ok(mut current) = self.phase.lock() {
            *current = phase;
        }
        if let Ok(mut last) = self.last.lock() {
            *last = Some(dto.clone());
        }
        dto
    }
}

pub fn idle_not_started_receipt() -> ShutdownReceipt {
    ShutdownReceipt {
        transport_cancelled: false,
        child_exited: false,
        forced: false,
        timeout_unknown: false,
        exit_code: None,
    }
}

fn drain_receipt(snapshot: &RuntimeSnapshot, inflight_unknown: bool) -> ShutdownReceipt {
    if inflight_unknown {
        return ShutdownReceipt {
            transport_cancelled: false,
            child_exited: false,
            forced: false,
            timeout_unknown: true,
            exit_code: None,
        };
    }
    if snapshot.state == ConnectionState::NotStarted {
        return idle_not_started_receipt();
    }
    snapshot
        .shutdown
        .clone()
        .unwrap_or_else(idle_not_started_receipt)
}

fn engine_spawned_from_lifecycle(state: ConnectionState) -> bool {
    !matches!(state, ConnectionState::NotStarted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::library::NoteCrudClass;
    use crate::supervisor::{
        CancelOutcome, ChildHandle, ConnectionState, EngineLaunchSpec, EngineProfile, FailureKind,
        Supervisor, Transport, WaitOutcome,
    };
    use std::collections::BTreeMap;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::time::Duration;

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

    fn idle_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            state: ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        }
    }

    #[test]
    fn idle_not_started_receipt_reuses_t07_fields_forced_false() {
        let receipt = idle_not_started_receipt();
        assert!(!receipt.transport_cancelled);
        assert!(!receipt.child_exited);
        assert!(!receipt.forced);
        assert!(!receipt.timeout_unknown);
        assert_eq!(receipt.exit_code, None);
        let json = serde_json::to_value(&receipt).unwrap();
        assert_eq!(json["transport_cancelled"], false);
        assert_eq!(json["child_exited"], false);
        assert_eq!(json["forced"], false);
        assert_eq!(json["timeout_unknown"], false);
        assert!(json["exit_code"].is_null());
        assert!(T07_FAKES_ARE_NOT_NATIVE_PROCESS_TREE.contains("not a native process-tree"));
    }

    #[test]
    fn host_begin_on_idle_supervisor_does_not_spawn_or_kill() {
        let supervisor = Supervisor::default();
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        assert!(!drain.is_closed());
        let before = supervisor.snapshot();
        let dto = drain.begin(&supervisor.snapshot(), &conflicts);
        let after = supervisor.snapshot();
        assert_eq!(before, after);
        assert_eq!(after.state, ConnectionState::NotStarted);
        assert_eq!(after.child_pid, None);
        assert_eq!(dto.host_drain, DrainPhase::Drained);
        assert_eq!(dto.classified_as, DrainClass::IdleNotStarted);
        assert_eq!(dto.supervisor_status, "not_started");
        assert!(!dto.engine_spawned);
        assert!(!dto.child_killed);
        assert!(!dto.files_written);
        assert!(!dto.shutdown.forced);
        assert!(!dto.shutdown.timeout_unknown);
        assert!(dto.inflight_unknown.is_empty());
        assert!(drain.is_closed());
        assert_eq!(drain.phase(), DrainPhase::Drained);
        assert_ne!(dto.classified_as, DrainClass::InflightUnknown);
        assert!(T12_RESTORE_IS_NOT_T17_DRAIN.contains("not T17 drain"));
        assert!(T16_CONFLICT_IS_NOT_T17_DRAIN.contains("not T17 host drain"));
    }

    #[test]
    fn begin_does_not_kill_a_live_fake_child() {
        let mut supervisor = Supervisor::default();
        supervisor
            .start(
                spec(EngineProfile::Release),
                Box::new(FakeChild {
                    pid: Some(7),
                    wait: WaitOutcome::Exited(Some(0)),
                    killed: false,
                }),
            )
            .unwrap();
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let dto = drain.begin(&supervisor.snapshot(), &conflicts);
        assert_eq!(supervisor.snapshot().child_pid, Some(7));
        assert_eq!(supervisor.snapshot().state, ConnectionState::Starting);
        assert!(dto.engine_spawned);
        assert!(!dto.child_killed);
        assert!(!dto.shutdown.forced);
        assert_ne!(dto.host_drain, DrainPhase::Idle);
        assert!(T17_IS_NOT_T37_T38.contains("not T37"));
    }

    #[test]
    fn graceful_fake_shutdown_forced_false_is_not_native_process_tree() {
        let mut supervisor = Supervisor::default();
        supervisor
            .start(
                spec(EngineProfile::Release),
                Box::new(FakeChild {
                    pid: Some(7),
                    wait: WaitOutcome::Exited(Some(0)),
                    killed: false,
                }),
            )
            .unwrap();
        supervisor.mark_connected().unwrap();
        let mut transport = FakeTransport {
            outcome: CancelOutcome::Cancelled,
            calls: 0,
        };
        let receipt = supervisor
            .shutdown(&mut transport, Duration::from_secs(1))
            .unwrap();
        assert_eq!(transport.calls, 1);
        assert!(receipt.transport_cancelled);
        assert!(receipt.child_exited);
        assert!(!receipt.forced);
        assert!(!receipt.timeout_unknown);
        assert_eq!(receipt.exit_code, Some(0));
        assert!(JOB_OBJECT_UNVERIFIED.contains("UNVERIFIED"));
        assert!(T07_FAKES_ARE_NOT_NATIVE_PROCESS_TREE.contains("UNVERIFIED"));
    }

    #[test]
    fn inflight_keys_are_recorded_unknown_and_not_retried() {
        let drain = HostDrain::default();
        let conflicts = ConflictCoordinator::default();
        let guard = conflicts
            .try_acquire(conflict::identifier_keys("welcome"))
            .unwrap();
        let retried = AtomicBool::new(false);
        let invoked = conflict::auto_retry_non_idempotent_write(&idle_snapshot(), || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        let dto = drain.begin(&idle_snapshot(), &conflicts);
        assert_eq!(dto.host_drain, DrainPhase::Draining);
        assert_eq!(dto.classified_as, DrainClass::InflightUnknown);
        assert_eq!(dto.inflight_unknown, vec!["welcome".to_owned()]);
        assert!(dto.shutdown.timeout_unknown);
        assert!(!dto.shutdown.forced);
        assert!(!dto.child_killed);
        assert!(drain.is_closed());
        drop(guard);
        let invoked_again = conflict::auto_retry_non_idempotent_write(&idle_snapshot(), || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked_again);
        assert!(!retried.load(Ordering::SeqCst));
        assert_ne!(NoteCrudClass::Conflict, NoteCrudClass::DiskVerified);
    }

    #[test]
    fn drain_classes_stay_distinct_from_conflict_restore_and_timeout() {
        let idle = DrainClass::IdleNotStarted;
        let unknown = DrainClass::InflightUnknown;
        assert_ne!(
            serde_json::to_value(idle).unwrap(),
            serde_json::to_value(NoteCrudClass::Conflict).unwrap()
        );
        assert_ne!(
            serde_json::to_value(unknown).unwrap(),
            serde_json::to_value("disk_verified").unwrap()
        );
        assert_ne!(
            serde_json::to_value(unknown).unwrap(),
            serde_json::to_value("conflict").unwrap()
        );
        assert_ne!(DrainPhase::Idle, DrainPhase::Draining);
        assert_ne!(DrainPhase::Idle, DrainPhase::Drained);
        assert_ne!(FailureKind::TimeoutUnknown, FailureKind::Process);
        assert!(FORCED_KILL_UNVERIFIED.contains("UNVERIFIED"));
        assert!(SLEEP_RESUME_UNVERIFIED.contains("UNVERIFIED"));
        assert!(DISK_FAILURE_UNVERIFIED.contains("UNVERIFIED"));
        let json = serde_json::to_value(&DrainResultDto {
            host_drain: DrainPhase::Drained,
            supervisor_status: "not_started".to_owned(),
            engine_spawned: false,
            child_killed: false,
            files_written: false,
            classified_as: DrainClass::IdleNotStarted,
            inflight_unknown: Vec::new(),
            shutdown: idle_not_started_receipt(),
            scanned_user_obsidian_vault: false,
            scanned_user_basic_memory_home: false,
        })
        .unwrap();
        assert_eq!(json["classified_as"], "idle_not_started");
        assert_ne!(json["classified_as"], "conflict");
        assert_ne!(json["classified_as"], "disk_verified");
        assert_ne!(json["kind"], "fixture_restored");
        assert_eq!(json["child_killed"], false);
        assert_eq!(json["shutdown"]["forced"], false);
    }

    #[test]
    fn dual_profiles_stay_isolated_on_drain() {
        let release = EngineProfile::Release;
        let preview = EngineProfile::MainPreview;
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        assert_ne!(release.commit(), preview.commit());
        let mut supervisor = Supervisor::default();
        supervisor
            .start(
                spec(EngineProfile::MainPreview),
                Box::new(FakeChild {
                    pid: Some(8),
                    wait: WaitOutcome::Exited(Some(0)),
                    killed: false,
                }),
            )
            .unwrap();
        let dto =
            HostDrain::default().begin(&supervisor.snapshot(), &ConflictCoordinator::default());
        assert_eq!(
            supervisor.snapshot().profile,
            Some(EngineProfile::MainPreview)
        );
        assert_ne!(
            supervisor
                .snapshot()
                .profile
                .map(|profile| profile.expected_tools()),
            Some(release.expected_tools())
        );
        assert!(!dto.child_killed);
    }
}
