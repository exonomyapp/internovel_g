//! `internovel_auth`: NextGraph-inspired non-custodial Wallet Auth + DIF `did:peer:4`.

pub mod did_peer;
pub mod wallet;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid wallet signature")]
    InvalidSignature,
    #[error("invalid PIN digit (must be 0..=9)")]
    InvalidPin,
    #[error("login method not configured on this wallet")]
    LoginMethodNotSupported,
    #[error("decryption failed (wrong credentials or corrupted envelope)")]
    DecryptionFailed,
    #[error("encryption failed")]
    EncryptionFailed,
    #[error("invalid mnemonic phrase")]
    InvalidMnemonic,
    #[error("invalid did:peer:4 document or identifier")]
    InvalidPeerDid,
    #[error("serialization error: {0}")]
    Serialization(String),
}
