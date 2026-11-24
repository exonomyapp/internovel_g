//! Iroh 1.5 QUIC transport binding, TLS exporter receiver-key proof, and `TokioStreamBridge`.

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use iroh::endpoint::Connection;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::info;

use crate::GapError;

pub const WILLOW_WTP_PIO_ALPN: &[u8] = b"willow/wtp-pio/1";
pub const TLS_EXPORTER_LABEL: &[u8] = b"willow-iroh-pio-v1";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum BindingMode {
    /// Receiver key == Iroh `EndpointId` proven natively by TLS 1.3 handshake.
    DirectEndpointId,
    /// Decoupled Willow `Receiver` key proven by signing the connection's TLS exporter digest.
    TlsExporterProof {
        receiver_pk: [u8; 32],
        signature: Vec<u8>,
    },
}

#[derive(Clone, Debug)]
pub struct SessionKeyingMaterial {
    pub session_binding_digest: [u8; 32],
    pub ini_salt: [u8; 32],
    pub res_salt: [u8; 32],
}

/// Derives `session_binding_digest`, `ini_salt`, and `res_salt` deterministically from
/// Iroh's TLS 1.3 exporter secret and both peers' `EndpointId`s.
pub fn derive_session_keying(
    conn: &Connection,
    local_endpoint_id: [u8; 32],
) -> Result<SessionKeyingMaterial, GapError> {
    let remote_endpoint_id = *conn.remote_id().as_bytes();
    let mut exporter = [0u8; 96];
    conn.export_keying_material(&mut exporter, TLS_EXPORTER_LABEL, WILLOW_WTP_PIO_ALPN)
        .map_err(|e| GapError::Transport(format!("tls exporter failed: {e}")))?;

    let mut hasher = blake3::Hasher::new();
    hasher.update(b"willow-iroh-binding-v1");
    hasher.update(&local_endpoint_id);
    hasher.update(&remote_endpoint_id);
    hasher.update(&exporter[..32]);
    let session_binding_digest = *hasher.finalize().as_bytes();

    let mut ini_salt = [0u8; 32];
    let mut res_salt = [0u8; 32];
    ini_salt.copy_from_slice(&exporter[32..64]);
    res_salt.copy_from_slice(&exporter[64..96]);

    info!(target: "willow::iroh", "derived TLS exporter session binding & directional PIO salts");
    Ok(SessionKeyingMaterial {
        session_binding_digest,
        ini_salt,
        res_salt,
    })
}

pub fn sign_binding_proof(signing_key: &SigningKey, session_binding_digest: &[u8; 32]) -> BindingMode {
    let sig = signing_key.sign(session_binding_digest);
    BindingMode::TlsExporterProof {
        receiver_pk: signing_key.verifying_key().to_bytes(),
        signature: sig.to_bytes().to_vec(),
    }
}

pub fn verify_binding_proof(
    mode: &BindingMode,
    remote_endpoint_id: &[u8; 32],
    session_binding_digest: &[u8; 32],
) -> Result<[u8; 32], GapError> {
    match mode {
        BindingMode::DirectEndpointId => Ok(*remote_endpoint_id),
        BindingMode::TlsExporterProof {
            receiver_pk,
            signature,
        } => {
            let vk = VerifyingKey::from_bytes(receiver_pk).map_err(|_| GapError::BindingFailed)?;
            let sig = Signature::from_slice(signature).map_err(|_| GapError::BindingFailed)?;
            vk.verify(session_binding_digest, &sig)
                .map_err(|_| GapError::BindingFailed)?;
            Ok(*receiver_pk)
        }
    }
}

/// Bridges Iroh's `tokio::io` QUIC streams into length-prefixed frames ready for `ufotofu` consumers.
pub struct TokioStreamBridge {
    pub send: iroh::endpoint::SendStream,
    pub recv: iroh::endpoint::RecvStream,
}

impl TokioStreamBridge {
    pub fn new(send: iroh::endpoint::SendStream, recv: iroh::endpoint::RecvStream) -> Self {
        Self { send, recv }
    }

    pub async fn write_frame(&mut self, bytes: &[u8]) -> Result<(), GapError> {
        let len = (bytes.len() as u32).to_be_bytes();
        self.send
            .write_all(&len)
            .await
            .map_err(|e| GapError::Transport(e.to_string()))?;
        self.send
            .write_all(bytes)
            .await
            .map_err(|e| GapError::Transport(e.to_string()))?;
        Ok(())
    }

    pub async fn read_frame(&mut self, max_frame_bytes: usize) -> Result<Vec<u8>, GapError> {
        let mut len_buf = [0u8; 4];
        self.recv
            .read_exact(&mut len_buf)
            .await
            .map_err(|e| GapError::Transport(e.to_string()))?;
        let len = u32::from_be_bytes(len_buf) as usize;
        if len > max_frame_bytes {
            return Err(GapError::BudgetExceeded("max_frame_bytes"));
        }
        let mut buf = vec![0u8; len];
        self.recv
            .read_exact(&mut buf)
            .await
            .map_err(|e| GapError::Transport(e.to_string()))?;
        Ok(buf)
    }
}
