//! Optional `specs/e2e` blind-relay codec: hierarchical path-component encryption + AEAD payloads.

use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Nonce,
};
use hkdf::Hkdf;
use rand::RngCore;
use sha2::Sha256;

use crate::GapError;

/// Derives a child symmetric key for a single Willow `Path` component so keys can be delegated
/// by subtree while keeping untrusted storage relays blind to path strings.
pub fn derive_component_key(parent_key: &[u8; 32], component: &[u8]) -> Result<[u8; 32], GapError> {
    let hk = Hkdf::<Sha256>::new(Some(b"willow-e2e-path-v1"), parent_key);
    let mut out = [0u8; 32];
    hk.expand(component, &mut out)
        .map_err(|_| GapError::CryptoFailure)?;
    Ok(out)
}

/// Encrypts payload bytes with `ChaCha20-Poly1305` (12-byte random nonce prepended) before
/// computing the Willow payload digest, keeping blind always-on relays unable to read payloads.
pub fn seal_payload(key: &[u8; 32], aad: &[u8], plaintext: &[u8]) -> Result<Vec<u8>, GapError> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let mut nonce_bytes = [0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(
            nonce,
            chacha20poly1305::aead::Payload {
                msg: plaintext,
                aad,
            },
        )
        .map_err(|_| GapError::CryptoFailure)?;

    let mut out = Vec::with_capacity(12 + ciphertext.len());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn open_payload(key: &[u8; 32], aad: &[u8], sealed: &[u8]) -> Result<Vec<u8>, GapError> {
    if sealed.len() < 12 {
        return Err(GapError::CryptoFailure);
    }
    let (nonce_bytes, ciphertext) = sealed.split_at(12);
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = Nonce::from_slice(nonce_bytes);
    cipher
        .decrypt(
            nonce,
            chacha20poly1305::aead::Payload {
                msg: ciphertext,
                aad,
            },
        )
        .map_err(|_| GapError::CryptoFailure)
}
