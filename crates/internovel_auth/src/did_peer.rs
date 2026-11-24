//! DIF `did:peer:4` (`numalgo:4`) Long-Form & Short-Form Identity Derivation + `did-peer` interop.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::{PublicKey as X25519PublicKey, StaticSecret};

use crate::AuthError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerDidDocument {
    #[serde(rename = "@context")]
    pub context: Vec<String>,
    #[serde(rename = "verificationMethod")]
    pub verification_method: Vec<VerificationMethod>,
    pub authentication: Vec<String>,
    #[serde(rename = "assertionMethod")]
    pub assertion_method: Vec<String>,
    #[serde(rename = "keyAgreement")]
    pub key_agreement: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub service: Vec<PeerService>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VerificationMethod {
    pub id: String,
    #[serde(rename = "type")]
    pub method_type: String,
    #[serde(rename = "publicKeyMultibase")]
    pub public_key_multibase: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerService {
    pub id: String,
    #[serde(rename = "type")]
    pub service_type: String,
    #[serde(rename = "serviceEndpoint")]
    pub service_endpoint: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PeerDid4Bundle {
    /// Complete long-form `did:peer:4z...` used during initial pairing/invites.
    pub long_form: String,
    /// Compact short-form `did:peer:4zQm...` SHA-256 multihash used in ongoing sync & UI.
    pub short_form: String,
    pub input_document: PeerDidDocument,
}

/// Multicodec prefix `0xed 0x01` (Ed25519 pub) / `0xec 0x01` (X25519 pub) encoded as `z` base58btc.
fn multibase_key(codec: [u8; 2], key: &[u8; 32]) -> String {
    let mut raw = Vec::with_capacity(34);
    raw.extend_from_slice(&codec);
    raw.extend_from_slice(key);
    format!("z{}", bs58::encode(raw).into_string())
}

pub fn derive_x25519_from_ed25519(ed_secret: &[u8; 32]) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"internovel-x25519-keyagreement-v1");
    digest.update(ed_secret);
    let key_bytes: [u8; 32] = digest.finalize().into();
    let secret = StaticSecret::from(key_bytes);
    *X25519PublicKey::from(&secret).as_bytes()
}

impl PeerDid4Bundle {
    /// Generates a DIF `did:peer:4` long-form & short-form pair embedding Ed25519 + X25519
    /// keys and an optional Willow+Iroh `EndpointId` service endpoint.
    pub fn create(
        ed25519_pk: &[u8; 32],
        x25519_pk: &[u8; 32],
        iroh_endpoint_id: Option<&str>,
    ) -> Self {
        let ed_mb = multibase_key([0xed, 0x01], ed25519_pk);
        let x_mb = multibase_key([0xec, 0x01], x25519_pk);

        let service = iroh_endpoint_id
            .map(|ep| {
                vec![PeerService {
                    id: "#willow-iroh".into(),
                    service_type: "WillowIrohEndpoint".into(),
                    service_endpoint: format!("iroh://{ep}?alpn=willow/wtp-pio/1"),
                }]
            })
            .unwrap_or_default();

        let input_document = PeerDidDocument {
            context: vec![
                "https://www.w3.org/ns/did/v1".into(),
                "https://w3id.org/security/multikey/v1".into(),
            ],
            verification_method: vec![
                VerificationMethod {
                    id: "#key-1".into(),
                    method_type: "Multikey".into(),
                    public_key_multibase: ed_mb,
                },
                VerificationMethod {
                    id: "#key-2".into(),
                    method_type: "Multikey".into(),
                    public_key_multibase: x_mb,
                },
            ],
            authentication: vec!["#key-1".into()],
            assertion_method: vec!["#key-1".into()],
            key_agreement: vec!["#key-2".into()],
            service,
        };

        // DIF did:peer:4 input document multicodec JSON prefix (0x02 0x00) + base58btc (`z`)
        let json_bytes = serde_json::to_vec(&input_document).unwrap_or_default();
        let mut encoded_doc = Vec::with_capacity(2 + json_bytes.len());
        encoded_doc.extend_from_slice(&[0x80, 0x04]); // varint(0x0200 = JSON multicodec)
        encoded_doc.extend_from_slice(&json_bytes);
        let encoded_suffix = format!("z{}", bs58::encode(&encoded_doc).into_string());

        // Short-form hash: SHA-256 multihash (0x12 0x20) over UTF-8 of encoded suffix
        let hash = Sha256::digest(encoded_suffix.as_bytes());
        let mut mh = Vec::with_capacity(34);
        mh.extend_from_slice(&[0x12, 0x20]);
        mh.extend_from_slice(&hash);
        let short_suffix = format!("z{}", bs58::encode(mh).into_string());

        Self {
            long_form: format!("did:peer:4{short_suffix}:{encoded_suffix}"),
            short_form: format!("did:peer:4{short_suffix}"),
            input_document,
        }
    }

    /// Resolves a `did:peer:4` long-form string back into its verified short-form hash and document.
    pub fn resolve_long_form(did: &str) -> Result<Self, AuthError> {
        let rest = did.strip_prefix("did:peer:4").ok_or(AuthError::InvalidPeerDid)?;
        let (short_suffix, encoded_suffix) = rest.split_once(':').ok_or(AuthError::InvalidPeerDid)?;

        let expected_hash = Sha256::digest(encoded_suffix.as_bytes());
        let mut mh = Vec::with_capacity(34);
        mh.extend_from_slice(&[0x12, 0x20]);
        mh.extend_from_slice(&expected_hash);
        let expected_short = format!("z{}", bs58::encode(mh).into_string());
        if short_suffix != expected_short {
            return Err(AuthError::InvalidPeerDid);
        }

        let b58_payload = encoded_suffix.strip_prefix('z').ok_or(AuthError::InvalidPeerDid)?;
        let raw = bs58::decode(b58_payload)
            .into_vec()
            .map_err(|_| AuthError::InvalidPeerDid)?;
        if raw.len() < 3 || raw[..2] != [0x80, 0x04] {
            return Err(AuthError::InvalidPeerDid);
        }
        let input_document: PeerDidDocument =
            serde_json::from_slice(&raw[2..]).map_err(|_| AuthError::InvalidPeerDid)?;

        Ok(Self {
            long_form: did.to_string(),
            short_form: format!("did:peer:4{short_suffix}"),
            input_document,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_long_form_to_matching_short_form() {
        let bundle = PeerDid4Bundle::create(&[7; 32], &[9; 32], Some("ep-test-123"));
        assert!(bundle.short_form.starts_with("did:peer:4zQm"));
        let resolved = PeerDid4Bundle::resolve_long_form(&bundle.long_form).unwrap();
        assert_eq!(resolved.short_form, bundle.short_form);
        assert_eq!(resolved.input_document.service.len(), 1);
    }
}
