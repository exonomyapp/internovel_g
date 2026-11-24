//! Riot-inspired FFI boundary: opaque handles backed by `Arc<Mutex<SessionArbiter>>`
//! wrapped in `std::panic::catch_unwind` so malformed peer/wallet inputs quarantine a session
//! rather than crashing Swift (iOS/macOS) or Kotlin (Android) host processes.

use std::collections::HashMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::{Arc, Mutex};

use internovel_auth::wallet::{
    create_wallet, unlock_with_password, CreateWalletParams, KdfProfile, SensitiveWallet,
    WalletEnvelope,
};
use willow_gap_core::admission::{inspect_and_plan_batch, AdmissionBudgets};
use willow_gap_core::wtp::WtpEntryWire;

uniffi::setup_scaffolding!();

#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum FfiError {
    #[error("session panicked and was quarantined")]
    SessionQuarantined,
    #[error("unknown session handle {0}")]
    InvalidHandle(u64),
    #[error("{0}")]
    OperationFailed(String),
}

struct SessionState {
    quarantined: bool,
    unlocked: Option<SensitiveWallet>,
}

pub struct SessionArbiter {
    next_id: u64,
    sessions: HashMap<u64, Arc<Mutex<SessionState>>>,
}

impl SessionArbiter {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            sessions: HashMap::new(),
        }
    }
}

#[derive(uniffi::Object)]
pub struct InterNovelMobileCore {
    arbiter: Mutex<SessionArbiter>,
}

#[uniffi::export]
impl InterNovelMobileCore {
    #[uniffi::constructor]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            arbiter: Mutex::new(SessionArbiter::new()),
        })
    }

    pub fn open_session(&self) -> u64 {
        let mut guard = self.arbiter.lock().expect("arbiter mutex");
        let id = guard.next_id;
        guard.next_id += 1;
        guard.sessions.insert(
            id,
            Arc::new(Mutex::new(SessionState {
                quarantined: false,
                unlocked: None,
            })),
        );
        id
    }

    pub fn create_mobile_wallet(
        &self,
        handle: u64,
        display_name: String,
        security_text: String,
        password: String,
    ) -> Result<String, FfiError> {
        self.with_session(handle, |state| {
            let created = create_wallet(CreateWalletParams {
                display_name,
                security_text,
                password: Some(password),
                enable_mnemonic: true,
                pazzle: None,
                pin: Some([0, 0, 0, 0]),
                in_memory_only: false,
                profile: KdfProfile::MobileConstrained,
            })
            .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            let text_code = created
                .envelope
                .to_text_code()
                .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            state.unlocked = Some(created.sensitive);
            Ok(text_code)
        })
    }

    pub fn unlock_mobile_wallet(
        &self,
        handle: u64,
        text_code: String,
        password: String,
        iroh_endpoint_id: String,
    ) -> Result<String, FfiError> {
        self.with_session(handle, |state| {
            let env = WalletEnvelope::from_text_code(&text_code)
                .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            let sensitive = unlock_with_password(&env, &password, KdfProfile::MobileConstrained)
                .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            let did = sensitive.to_peer_did4(&iroh_endpoint_id).short_form;
            state.unlocked = Some(sensitive);
            Ok(did)
        })
    }

    pub fn stage_sync_batch_json(&self, handle: u64, entries_json: String) -> Result<u64, FfiError> {
        self.with_session(handle, |_state| {
            let entries: Vec<WtpEntryWire> = serde_json::from_str(&entries_json)
                .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            let plan = inspect_and_plan_batch(&AdmissionBudgets::default(), entries)
                .map_err(|e| FfiError::OperationFailed(e.to_string()))?;
            Ok(plan.entries.len() as u64)
        })
    }
}

impl InterNovelMobileCore {
    fn with_session<T, F>(&self, handle: u64, f: F) -> Result<T, FfiError>
    where
        F: FnOnce(&mut SessionState) -> Result<T, FfiError>,
    {
        let session_arc = {
            let guard = self.arbiter.lock().map_err(|_| FfiError::SessionQuarantined)?;
            guard
                .sessions
                .get(&handle)
                .cloned()
                .ok_or(FfiError::InvalidHandle(handle))?
        };
        let mut session = session_arc.lock().map_err(|_| FfiError::SessionQuarantined)?;
        if session.quarantined {
            return Err(FfiError::SessionQuarantined);
        }
        match catch_unwind(AssertUnwindSafe(|| f(&mut session))) {
            Ok(res) => res,
            Err(_) => {
                session.quarantined = true;
                Err(FfiError::SessionQuarantined)
            }
        }
    }
}
