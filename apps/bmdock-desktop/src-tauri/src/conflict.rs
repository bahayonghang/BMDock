use std::collections::HashSet;
use std::sync::Mutex;

use crate::library::{
    NoteCrudClass, NoteCrudObservationDto, NoteDeleteDto, NoteEditDto, NoteMoveDto, NoteWriteDto,
};
use crate::supervisor::{FailureKind, RuntimeSnapshot};

#[allow(dead_code)]
pub const OS_FILESYSTEM_RACE_UNVERIFIED: &str =
    "true concurrent OS-thread filesystem races remain UNVERIFIED; in-process inflight is host coordination, not an OS file lock";
#[allow(dead_code)]
pub const RECOVERY_NOT_T16: &str =
    "T16 records conflict and timeout_unknown; it is not T17/T37 recovery. Forced-kill, Job Object, and disk-failure remain UNVERIFIED";
#[allow(dead_code)]
pub const SEQUENTIAL_DEST_EXISTS_NOT_T16: &str =
    "sequential dest-exists is unsupported and is not T16 same-target conflict";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConflictError;

#[derive(Debug, Default)]
pub struct ConflictCoordinator {
    inflight: Mutex<HashSet<String>>,
}

pub struct InflightGuard<'a> {
    coordinator: &'a ConflictCoordinator,
    keys: Vec<String>,
}

impl ConflictCoordinator {
    pub fn try_acquire(&self, keys: Vec<String>) -> Result<InflightGuard<'_>, ConflictError> {
        let unique = unique_keys(keys);
        let mut inflight = self.inflight.lock().map_err(|_| ConflictError)?;
        if unique.iter().any(|key| inflight.contains(key)) {
            return Err(ConflictError);
        }
        for key in &unique {
            inflight.insert(key.clone());
        }
        Ok(InflightGuard {
            coordinator: self,
            keys: unique,
        })
    }

    #[cfg(test)]
    pub fn is_inflight(&self, identifier: &str) -> bool {
        let key = identifier.trim();
        self.inflight
            .lock()
            .map(|inflight| inflight.contains(key))
            .unwrap_or(true)
    }

    pub fn inflight_keys(&self) -> Vec<String> {
        match self.inflight.lock() {
            Ok(inflight) => {
                let mut keys: Vec<String> = inflight.iter().cloned().collect();
                keys.sort();
                keys
            }
            Err(_) => Vec::new(),
        }
    }
}

impl Drop for InflightGuard<'_> {
    fn drop(&mut self) {
        if let Ok(mut inflight) = self.coordinator.inflight.lock() {
            for key in &self.keys {
                inflight.remove(key);
            }
        }
    }
}

pub fn identifier_keys(identifier: &str) -> Vec<String> {
    vec![identifier.trim().to_owned()]
}

pub fn move_keys(identifier: &str, destination: &str) -> Vec<String> {
    let mut keys = identifier_keys(identifier);
    let destination = destination.trim();
    if !destination.is_empty() && destination != keys[0] {
        keys.push(destination.to_owned());
    }
    keys
}

pub fn timeout_unknown_present(snapshot: &RuntimeSnapshot) -> bool {
    snapshot.failure == Some(FailureKind::TimeoutUnknown)
        || snapshot
            .shutdown
            .as_ref()
            .map(|receipt| receipt.timeout_unknown)
            .unwrap_or(false)
}

pub fn should_auto_retry_non_idempotent_write(snapshot: &RuntimeSnapshot) -> bool {
    if timeout_unknown_present(snapshot) {
        return false;
    }
    false
}

pub fn auto_retry_non_idempotent_write<F: FnOnce()>(snapshot: &RuntimeSnapshot, retry: F) -> bool {
    if should_auto_retry_non_idempotent_write(snapshot) {
        retry();
        return true;
    }
    let _ = retry;
    false
}

pub fn refuse_auto_retry(snapshot: &RuntimeSnapshot) {
    let invoked = auto_retry_non_idempotent_write(snapshot, || {
        panic!("non-idempotent write retry helper must not run after timeout_unknown");
    });
    let _ = invoked;
}

pub fn write_conflict(identifier: &str, title: &str, body: &str) -> NoteWriteDto {
    NoteWriteDto {
        identifier: identifier.to_owned(),
        title: title.to_owned(),
        body: body.to_owned(),
        files_written: false,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: conflict_observation(),
    }
}

pub fn edit_conflict(identifier: &str, body: &str) -> NoteEditDto {
    NoteEditDto {
        identifier: identifier.to_owned(),
        body: body.to_owned(),
        files_written: false,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: conflict_observation(),
    }
}

pub fn move_conflict(identifier: &str, destination: &str, body: &str) -> NoteMoveDto {
    NoteMoveDto {
        identifier: identifier.to_owned(),
        destination: destination.to_owned(),
        body: body.to_owned(),
        files_written: false,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: conflict_observation(),
    }
}

pub fn delete_conflict(identifier: &str) -> NoteDeleteDto {
    NoteDeleteDto {
        identifier: identifier.to_owned(),
        files_written: false,
        engine_persisted: false,
        scanned_user_obsidian_vault: false,
        scanned_user_basic_memory_home: false,
        observation: conflict_observation(),
    }
}

fn conflict_observation() -> NoteCrudObservationDto {
    NoteCrudObservationDto {
        classified_as: NoteCrudClass::Conflict,
        disk_verified: false,
        envelope_is_not_disk_proof: true,
    }
}

fn unique_keys(keys: Vec<String>) -> Vec<String> {
    let mut unique = Vec::new();
    for key in keys {
        let key = key.trim().to_owned();
        if key.is_empty() || unique.iter().any(|existing| existing == &key) {
            continue;
        }
        unique.push(key);
    }
    unique
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::supervisor::{ConnectionState, ShutdownReceipt};
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;

    fn timeout_unknown_snapshot() -> RuntimeSnapshot {
        RuntimeSnapshot {
            state: ConnectionState::Stopped,
            profile: Some(crate::supervisor::EngineProfile::MainPreview),
            child_pid: None,
            failure: Some(FailureKind::TimeoutUnknown),
            shutdown: Some(ShutdownReceipt {
                transport_cancelled: false,
                child_exited: false,
                forced: true,
                timeout_unknown: true,
                exit_code: None,
            }),
        }
    }

    #[test]
    fn overlapping_same_target_inflight_is_conflict() {
        let coordinator = ConflictCoordinator::default();
        let first = coordinator
            .try_acquire(identifier_keys("welcome"))
            .expect("first writer occupies the identifier");
        assert!(coordinator.is_inflight("welcome"));
        let second = coordinator.try_acquire(identifier_keys("welcome"));
        assert_eq!(second.err(), Some(ConflictError));
        let conflict = write_conflict("welcome", "中文夹具笔记", "body");
        assert_eq!(conflict.observation.classified_as, NoteCrudClass::Conflict);
        assert!(!conflict.observation.disk_verified);
        assert!(!conflict.files_written);
        assert_ne!(
            conflict.observation.classified_as,
            NoteCrudClass::DiskVerified
        );
        drop(first);
        assert!(!coordinator.is_inflight("welcome"));
        coordinator
            .try_acquire(identifier_keys("welcome"))
            .expect("released identifier can be acquired again");
    }

    #[test]
    fn overlapping_os_threads_observe_conflict_class() {
        let coordinator = Arc::new(ConflictCoordinator::default());
        let started = Arc::new(Barrier::new(2));
        let released = Arc::new(Barrier::new(2));
        let conflicts = Arc::new(AtomicUsize::new(0));
        let acquired = Arc::new(AtomicUsize::new(0));
        thread::scope(|scope| {
            for _ in 0..2 {
                let coordinator = Arc::clone(&coordinator);
                let started = Arc::clone(&started);
                let released = Arc::clone(&released);
                let conflicts = Arc::clone(&conflicts);
                let acquired = Arc::clone(&acquired);
                scope.spawn(move || {
                    started.wait();
                    match coordinator.try_acquire(identifier_keys("welcome")) {
                        Ok(guard) => {
                            acquired.fetch_add(1, Ordering::SeqCst);
                            released.wait();
                            drop(guard);
                        }
                        Err(ConflictError) => {
                            conflicts.fetch_add(1, Ordering::SeqCst);
                            released.wait();
                        }
                    }
                });
            }
        });
        assert_eq!(acquired.load(Ordering::SeqCst), 1);
        assert_eq!(conflicts.load(Ordering::SeqCst), 1);
        assert!(OS_FILESYSTEM_RACE_UNVERIFIED.contains("UNVERIFIED"));
        assert!(RECOVERY_NOT_T16.contains("not T17/T37"));
    }

    #[test]
    fn move_keys_overlap_source_and_destination() {
        let coordinator = ConflictCoordinator::default();
        let _guard = coordinator
            .try_acquire(move_keys("welcome", "renamed"))
            .unwrap();
        assert_eq!(
            coordinator.try_acquire(identifier_keys("renamed")).err(),
            Some(ConflictError)
        );
        assert_eq!(
            coordinator.try_acquire(identifier_keys("welcome")).err(),
            Some(ConflictError)
        );
        coordinator
            .try_acquire(identifier_keys("other"))
            .expect("distinct identifier is not the same target");
    }

    #[test]
    fn timeout_unknown_does_not_invoke_retry_helper() {
        let snapshot = timeout_unknown_snapshot();
        assert!(timeout_unknown_present(&snapshot));
        assert!(!should_auto_retry_non_idempotent_write(&snapshot));
        let retried = AtomicBool::new(false);
        let invoked = auto_retry_non_idempotent_write(&snapshot, || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        refuse_auto_retry(&snapshot);
        assert_eq!(snapshot.failure, Some(FailureKind::TimeoutUnknown));
        assert_eq!(
            snapshot
                .shutdown
                .as_ref()
                .map(|receipt| receipt.timeout_unknown),
            Some(true)
        );
    }

    #[test]
    fn idle_snapshot_still_does_not_auto_retry() {
        let snapshot = RuntimeSnapshot {
            state: ConnectionState::NotStarted,
            profile: None,
            child_pid: None,
            failure: None,
            shutdown: None,
        };
        let retried = AtomicBool::new(false);
        let invoked = auto_retry_non_idempotent_write(&snapshot, || {
            retried.store(true, Ordering::SeqCst);
        });
        assert!(!invoked);
        assert!(!retried.load(Ordering::SeqCst));
        assert!(!timeout_unknown_present(&snapshot));
    }

    #[test]
    fn conflict_dto_is_not_timeout_unknown_or_policy() {
        let written = write_conflict("welcome", "title", "body");
        let edited = edit_conflict("welcome", "body");
        let moved = move_conflict("welcome", "renamed", "body");
        let deleted = delete_conflict("welcome");
        for classified in [
            written.observation.classified_as,
            edited.observation.classified_as,
            moved.observation.classified_as,
            deleted.observation.classified_as,
        ] {
            assert_eq!(classified, NoteCrudClass::Conflict);
            assert_ne!(classified, NoteCrudClass::DiskVerified);
            assert_ne!(classified, NoteCrudClass::AcceptedUnverified);
        }
        let json = serde_json::to_value(&written).unwrap();
        assert_eq!(json["observation"]["classified_as"], "conflict");
        assert_ne!(json["observation"]["classified_as"], "disk_verified");
        assert_ne!(json["observation"]["classified_as"], "timeout_unknown");
        assert_ne!(json["observation"]["classified_as"], "policy");
        assert_eq!(json["engine_persisted"], false);
        assert_eq!(json["files_written"], false);
        assert!(json.get("path").is_none());
        assert!(SEQUENTIAL_DEST_EXISTS_NOT_T16.contains("not T16"));
    }

    #[test]
    fn dual_profiles_stay_isolated_on_unknown_snapshot() {
        let release = crate::supervisor::EngineProfile::Release;
        let preview = crate::supervisor::EngineProfile::MainPreview;
        assert_eq!(release.expected_tools(), 21);
        assert_eq!(preview.expected_tools(), 27);
        assert_ne!(release.commit(), preview.commit());
        let snapshot = timeout_unknown_snapshot();
        assert_eq!(
            snapshot.profile.map(|profile| profile.id()),
            Some("main-preview")
        );
        assert_ne!(
            snapshot.profile.map(|profile| profile.expected_tools()),
            Some(release.expected_tools())
        );
    }
}
