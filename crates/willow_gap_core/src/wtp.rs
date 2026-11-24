//! Sans-I/O Willow Transfer Protocol (`willow_transfer_protocol`) state machine & RBSR frames.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::debug;

use crate::pio::{PioGate, PrivateInterest, SessionCapabilityHandle};
use crate::GapError;

pub type WtpMessageId = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PreferredOrdering {
    NewestThenSubspaceThenPath,
    SubspaceThenPathThenNewest,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponseInformationProvenance {
    pub parent: Option<WtpMessageId>,
    pub is_final_response: bool,
    pub root: WtpMessageId,
    pub is_root_by_sender: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WtpConfig {
    pub threshold_count: u64,
    pub threshold_size: u64,
    pub payload_lazyness: u64,
    pub allow_symmetric_rbsr: bool,
    pub preferred_ordering: PreferredOrdering,
}

impl Default for WtpConfig {
    fn default() -> Self {
        Self {
            threshold_count: 64,
            threshold_size: 256 * 1024,
            payload_lazyness: 16 * 1024,
            allow_symmetric_rbsr: true,
            preferred_ordering: PreferredOrdering::NewestThenSubspaceThenPath,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum WtpFrame {
    RequestEntries {
        request_id: Option<WtpMessageId>,
        capability_handle: SessionCapabilityHandle,
        grouping: PrivateInterest,
        is_oneshot: bool,
        fingerprint: [u8; 32],
    },
    RespondToRequestEntries {
        provenance: ResponseInformationProvenance,
        entry_count: u64,
        total_size: u64,
        fingerprint: [u8; 32],
        immediate_entries: Vec<WtpEntryWire>,
    },
    SendEntry {
        subscription_id: WtpMessageId,
        entry: WtpEntryWire,
    },
    RequestPayloadSlice {
        request_id: WtpMessageId,
        capability_handle: SessionCapabilityHandle,
        payload_digest: [u8; 32],
        offset: u64,
        length: u64,
    },
    RespondToRequestPayloadSlice {
        request_id: WtpMessageId,
        status: PayloadSliceStatus,
    },
    SendPayloadSlice {
        request_id: WtpMessageId,
        offset: u64,
        bytes: Vec<u8>,
        bab_proof: Vec<[u8; 32]>,
    },
    CancelOwnRequest {
        request_id: WtpMessageId,
    },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PayloadSliceStatus {
    Found,
    Done,
    Nope { retry: bool },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WtpEntryWire {
    pub namespace_id: [u8; 32],
    pub subspace_id: [u8; 32],
    pub path: Vec<Vec<u8>>,
    pub timestamp: u64,
    pub payload_length: u64,
    pub payload_digest: [u8; 32],
    pub inline_payload: Option<Vec<u8>>,
    pub authorisation_token: Vec<u8>,
}

/// Coordinates inbound WTP admission against `PioGate`, active subscriptions, and RBSR thresholds.
pub struct WtpSession {
    pub config: WtpConfig,
    active_subscriptions: HashMap<WtpMessageId, PrivateInterest>,
}

impl WtpSession {
    pub fn new(config: WtpConfig) -> Self {
        Self {
            config,
            active_subscriptions: HashMap::new(),
        }
    }

    pub fn validate_inbound_frame(
        &mut self,
        pio: &PioGate,
        frame: &WtpFrame,
    ) -> Result<(), GapError> {
        match frame {
            WtpFrame::RequestEntries {
                request_id,
                capability_handle,
                grouping,
                is_oneshot,
                ..
            } => {
                pio.authorize_request(*capability_handle, grouping)?;
                if !is_oneshot {
                    if let Some(id) = request_id {
                        self.active_subscriptions.insert(*id, grouping.clone());
                        debug!(target: "willow::wtp", subscription_id = id, "registered live area subscription");
                    }
                }
                Ok(())
            }
            WtpFrame::CancelOwnRequest { request_id } => {
                self.active_subscriptions.remove(request_id);
                Ok(())
            }
            WtpFrame::RequestPayloadSlice {
                capability_handle, ..
            } => {
                // Ensure the handle itself is still active in PioGate.
                if pio.authorize_request(
                    *capability_handle,
                    &PrivateInterest {
                        namespace_id: [0; 32],
                        subspace_id: None,
                        path_prefix: vec![],
                    },
                ) == Err(GapError::UnauthorizedHandle(*capability_handle))
                {
                    return Err(GapError::UnauthorizedHandle(*capability_handle));
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub fn should_send_immediate(&self, count: u64, total_size: u64) -> bool {
        count <= self.config.threshold_count && total_size <= self.config.threshold_size
    }

    pub fn should_inline_payload(&self, payload_len: u64) -> bool {
        payload_len <= self.config.payload_lazyness
    }
}
