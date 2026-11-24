//! `willow_gap_core`: Confidential Willow Transfer Protocol (WTP + PIO + e2e) over Iroh.
//!
//! Designed as a Wasm-clean core crate (`persistent-storage` and `iroh-transport` are opt-in
//! Cargo features) implementing the layered architecture in `docs/willow-gap.md`.

pub mod admission;
pub mod e2e;
pub mod pio;
pub mod wtp;

#[cfg(feature = "iroh-transport")]
pub mod iroh_transport;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum GapError {
    #[error("session binding verification failed")]
    BindingFailed,
    #[error("private interest overlap rejected or capability handle {0} unknown")]
    UnauthorizedHandle(u64),
    #[error("requested area is outside granted capability boundary")]
    CapabilityScopeViolation,
    #[error("admission budget exceeded: {0}")]
    BudgetExceeded(&'static str),
    #[error("duplicate entry or message id in staged batch")]
    DuplicateId,
    #[error("cryptographic operation failed")]
    CryptoFailure,
    #[error("transport error: {0}")]
    Transport(String),
}
