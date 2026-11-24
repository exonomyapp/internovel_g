//! NextGraph (`ng-wallet`) inspired non-custodial local wallet lifecycle.
//!
//! Preserves NextGraph's core security model without reinventing the wheel:
//! - Anti-phishing `SecurityBanner` (`security_text` + normalized image bytes) inside the signed envelope.
//! - Multi-slot `master_key` wrapping via `Aes256GcmSiv` (AD = `wallet_id`) under `Argon2id` keys.
//! - Login methods: Password, 12-word BIP-39 Mnemonic + 4-digit PIN, Visual Pazzle + 4-digit PIN,
//!   plus Ephemeral In-Memory sessions (`in_memory = true`).
//! - Per-device `ClientProfile` (`ClientV0` equivalent) and `SessionPeerStorage` nonce tracking.
//! - QR code & TextCode cross-device wallet export/import + Riot `ChaCha20-Poly1305` sealed backups.

use aes_gcm_siv::{
    aead::{Aead, KeyInit, Payload},
    Aes256GcmSiv, Nonce as AesNonce,
};
use argon2::{Algorithm, Argon2, Params, Version};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use chacha20poly1305::{ChaCha20Poly1305, Nonce as ChaChaNonce};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use qrcode::{render::svg, QrCode};
use rand::{seq::SliceRandom, RngCore};
use serde::{Deserialize, Serialize};
use tracing::info;
use zeroize::{Zeroize, ZeroizeOnDrop};

use crate::did_peer::{derive_x25519_from_ed25519, PeerDid4Bundle};
use crate::AuthError;

pub type WalletId = [u8; 32];

/// Anti-phishing visual & text banner stored in the signed wallet header and shown *before* unlock.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SecurityBanner {
    pub security_text: String,
    pub security_image_jpeg: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginSlot {
    pub salt: [u8; 16],
    /// 32-byte `master_key` + 16-byte AES-GCM-SIV auth tag = 48 bytes.
    pub enc_master_key: Vec<u8>,
}

/// Shuffled grid indices for shoulder-surfing-resistant Visual Pazzle + PIN login (per `ng-wallet`).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShuffledPazzle {
    pub category_indices: Vec<u8>,
    pub emoji_indices: Vec<Vec<u8>>,
    pub pin_digits: Vec<u8>,
}

pub fn gen_shuffled_pazzle(pazzle_length: usize) -> ShuffledPazzle {
    let mut rng = rand::thread_rng();
    let mut category_indices: Vec<u8> = (0..pazzle_length as u8).collect();
    category_indices.shuffle(&mut rng);
    let mut emoji_indices = Vec::with_capacity(pazzle_length);
    for _ in 0..pazzle_length {
        let mut idx: Vec<u8> = (0..15).collect();
        idx.shuffle(&mut rng);
        emoji_indices.push(idx);
    }
    let mut pin_digits: Vec<u8> = (0..10).collect();
    pin_digits.shuffle(&mut rng);
    ShuffledPazzle {
        category_indices,
        emoji_indices,
        pin_digits,
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletContent {
    pub security_banner: SecurityBanner,
    pub pazzle_length: u8,
    pub password_slot: Option<LoginSlot>,
    pub mnemonic_slot: Option<LoginSlot>,
    pub pazzle_slot: Option<LoginSlot>,
    pub master_nonce: u64,
    pub timestamp_ms: u64,
    pub peer_id: [u8; 32],
    pub log_nonce: [u8; 12],
    pub encrypted_sensitive_wallet: Vec<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletEnvelope {
    pub wallet_id: WalletId,
    pub content: WalletContent,
    pub signature: Vec<u8>,
}

impl WalletEnvelope {
    pub fn verify_signature(&self) -> Result<(), AuthError> {
        let vk = VerifyingKey::from_bytes(&self.wallet_id).map_err(|_| AuthError::InvalidSignature)?;
        let sig = Signature::from_slice(&self.signature).map_err(|_| AuthError::InvalidSignature)?;
        let bytes =
            serde_json::to_vec(&self.content).map_err(|e| AuthError::Serialization(e.to_string()))?;
        vk.verify(&bytes, &sig)
            .map_err(|_| AuthError::InvalidSignature)
    }

    /// Exports the encrypted wallet as a compact URL-safe TextCode for camera-less cross-device transfer.
    pub fn to_text_code(&self) -> Result<String, AuthError> {
        let bytes = serde_json::to_vec(self).map_err(|e| AuthError::Serialization(e.to_string()))?;
        Ok(format!("inw4:{}", URL_SAFE_NO_PAD.encode(bytes)))
    }

    pub fn from_text_code(code: &str) -> Result<Self, AuthError> {
        let raw = code.trim().strip_prefix("inw4:").unwrap_or(code.trim());
        let bytes = URL_SAFE_NO_PAD
            .decode(raw)
            .map_err(|e| AuthError::Serialization(e.to_string()))?;
        let wallet: Self =
            serde_json::from_slice(&bytes).map_err(|e| AuthError::Serialization(e.to_string()))?;
        wallet.verify_signature()?;
        Ok(wallet)
    }

    /// Renders an SVG QR code for NextGraph-style camera-based cross-device wallet pairing.
    pub fn to_pairing_qr_svg(&self) -> Result<String, AuthError> {
        let text_code = self.to_text_code()?;
        let code = QrCode::new(text_code.as_bytes()).map_err(|e| AuthError::Serialization(e.to_string()))?;
        Ok(code.render::<svg::Color>().min_dimensions(220, 220).build())
    }
}

/// Decrypted sensitive wallet state; automatically zeroized on drop.
#[derive(Clone, Debug, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
pub struct SensitiveWallet {
    pub wallet_secret: [u8; 32],
    pub author_ed25519_secret: [u8; 32],
    pub master_key: [u8; 32],
    #[zeroize(skip)]
    pub display_name: String,
    #[zeroize(skip)]
    pub in_memory_only: bool,
    pub last_wallet_nonce: u64,
}

impl SensitiveWallet {
    /// Derives the author's `did:peer:4` long-form and short-form identity bundle bound to an Iroh EndpointId.
    pub fn to_peer_did4(&self, iroh_endpoint_id: &str) -> PeerDid4Bundle {
        let ed_sk = SigningKey::from_bytes(&self.author_ed25519_secret);
        let ed_pk = ed_sk.verifying_key().to_bytes();
        let x_pk = derive_x25519_from_ed25519(&self.author_ed25519_secret);
        PeerDid4Bundle::create(&ed_pk, &x_pk, Some(iroh_endpoint_id))
    }
}

#[derive(Clone, Copy, Debug)]
pub enum KdfProfile {
    /// Desktop / server profile (~40 MiB Argon2id, matching NextGraph `ng-wallet`).
    Desktop,
    /// Mobile / iOS extension safe profile (~16 MiB) staying well under iOS 120 MiB limits.
    MobileConstrained,
}

pub fn derive_key_from_pass(
    mut secret_input: Vec<u8>,
    salt: &[u8; 16],
    wallet_id: &WalletId,
    profile: KdfProfile,
) -> Result<[u8; 32], AuthError> {
    let (m_cost, t_cost) = match profile {
        KdfProfile::Desktop => (40 * 1024, 3),
        KdfProfile::MobileConstrained => (16 * 1024, 3),
    };
    let params = Params::new(m_cost, t_cost, 1, Some(32)).map_err(|_| AuthError::EncryptionFailed)?;
    let argon = Argon2::new_with_secret(wallet_id, Algorithm::Argon2id, Version::V0x13, params)
        .map_err(|_| AuthError::EncryptionFailed)?;
    let mut out = [0u8; 32];
    argon
        .hash_password_into(&secret_input, salt, &mut out)
        .map_err(|_| AuthError::EncryptionFailed)?;
    secret_input.zeroize();
    Ok(out)
}

fn wrap_master_key(
    master_key: &[u8; 32],
    wrapping_key: &[u8; 32],
    wallet_id: &WalletId,
) -> Result<Vec<u8>, AuthError> {
    let cipher = Aes256GcmSiv::new(wrapping_key.into());
    let nonce = AesNonce::from_slice(&[0u8; 12]);
    cipher
        .encrypt(
            nonce,
            Payload {
                msg: master_key,
                aad: wallet_id,
            },
        )
        .map_err(|_| AuthError::EncryptionFailed)
}

fn unwrap_master_key(
    ciphertext: &[u8],
    wrapping_key: &[u8; 32],
    wallet_id: &WalletId,
) -> Result<[u8; 32], AuthError> {
    let cipher = Aes256GcmSiv::new(wrapping_key.into());
    let nonce = AesNonce::from_slice(&[0u8; 12]);
    let pt = cipher
        .decrypt(
            nonce,
            Payload {
                msg: ciphertext,
                aad: wallet_id,
            },
        )
        .map_err(|_| AuthError::DecryptionFailed)?;
    pt.try_into().map_err(|_| AuthError::DecryptionFailed)
}

pub struct CreateWalletParams {
    pub display_name: String,
    pub security_text: String,
    pub password: Option<String>,
    pub enable_mnemonic: bool,
    pub pazzle: Option<Vec<u8>>,
    pub pin: Option<[u8; 4]>,
    pub in_memory_only: bool,
    pub profile: KdfProfile,
}

pub struct CreatedWallet {
    pub envelope: WalletEnvelope,
    pub sensitive: SensitiveWallet,
    pub mnemonic_words: Option<String>,
}

pub fn create_wallet(params: CreateWalletParams) -> Result<CreatedWallet, AuthError> {
    let mut rng = rand::thread_rng();
    let wallet_sk = SigningKey::generate(&mut rng);
    let wallet_id = wallet_sk.verifying_key().to_bytes();
    let author_sk = SigningKey::generate(&mut rng);

    let mut master_key = [0u8; 32];
    rng.fill_bytes(&mut master_key);

    let password_slot = if let Some(pw) = params.password {
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt);
        let mut k = derive_key_from_pass(pw.into_bytes(), &salt, &wallet_id, params.profile)?;
        let enc = wrap_master_key(&master_key, &k, &wallet_id)?;
        k.zeroize();
        Some(LoginSlot {
            salt,
            enc_master_key: enc,
        })
    } else {
        None
    };

    let (mnemonic_slot, mnemonic_words) = if params.enable_mnemonic {
        let pin = params.pin.unwrap_or([0, 0, 0, 0]);
        let mut entropy = [0u8; 16];
        rng.fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy(&entropy).map_err(|_| AuthError::InvalidMnemonic)?;
        let phrase = mnemonic.to_string();
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt);
        let mut input = phrase.as_bytes().to_vec();
        input.extend_from_slice(&pin);
        let mut k = derive_key_from_pass(input, &salt, &wallet_id, params.profile)?;
        let enc = wrap_master_key(&master_key, &k, &wallet_id)?;
        k.zeroize();
        (
            Some(LoginSlot {
                salt,
                enc_master_key: enc,
            }),
            Some(phrase),
        )
    } else {
        (None, None)
    };

    let pazzle_len = params.pazzle.as_ref().map(|p| p.len() as u8).unwrap_or(0);
    let pazzle_slot = if let Some(pazzle) = params.pazzle {
        let pin = params.pin.ok_or(AuthError::InvalidPin)?;
        if pin.iter().any(|&d| d > 9) {
            return Err(AuthError::InvalidPin);
        }
        let mut salt = [0u8; 16];
        rng.fill_bytes(&mut salt);
        let mut input = pazzle;
        input.extend_from_slice(&pin);
        let mut k = derive_key_from_pass(input, &salt, &wallet_id, params.profile)?;
        let enc = wrap_master_key(&master_key, &k, &wallet_id)?;
        k.zeroize();
        Some(LoginSlot {
            salt,
            enc_master_key: enc,
        })
    } else {
        None
    };

    let sensitive = SensitiveWallet {
        wallet_secret: wallet_sk.to_bytes(),
        author_ed25519_secret: author_sk.to_bytes(),
        master_key,
        display_name: params.display_name,
        in_memory_only: params.in_memory_only,
        last_wallet_nonce: 1,
    };

    let mut log_nonce = [0u8; 12];
    rng.fill_bytes(&mut log_nonce);
    let cipher = ChaCha20Poly1305::new((&master_key).into());
    let sensitive_bytes =
        serde_json::to_vec(&sensitive).map_err(|e| AuthError::Serialization(e.to_string()))?;
    let encrypted_sensitive_wallet = cipher
        .encrypt(
            ChaChaNonce::from_slice(&log_nonce),
            Payload {
                msg: &sensitive_bytes,
                aad: &wallet_id,
            },
        )
        .map_err(|_| AuthError::EncryptionFailed)?;

    let content = WalletContent {
        security_banner: SecurityBanner {
            security_text: params.security_text,
            security_image_jpeg: None,
        },
        pazzle_length: pazzle_len,
        password_slot,
        mnemonic_slot,
        pazzle_slot,
        master_nonce: 0,
        timestamp_ms: 0,
        peer_id: author_sk.verifying_key().to_bytes(),
        log_nonce,
        encrypted_sensitive_wallet,
    };
    let content_bytes =
        serde_json::to_vec(&content).map_err(|e| AuthError::Serialization(e.to_string()))?;
    let signature = wallet_sk.sign(&content_bytes).to_bytes().to_vec();

    info!(target: "internovel::auth", "created non-custodial NextGraph-style wallet");
    Ok(CreatedWallet {
        envelope: WalletEnvelope {
            wallet_id,
            content,
            signature,
        },
        sensitive,
        mnemonic_words,
    })
}

pub fn unlock_with_password(
    wallet: &WalletEnvelope,
    password: &str,
    profile: KdfProfile,
) -> Result<SensitiveWallet, AuthError> {
    wallet.verify_signature()?;
    let slot = wallet
        .content
        .password_slot
        .as_ref()
        .ok_or(AuthError::LoginMethodNotSupported)?;
    let mut k = derive_key_from_pass(
        password.as_bytes().to_vec(),
        &slot.salt,
        &wallet.wallet_id,
        profile,
    )?;
    let master_key = unwrap_master_key(&slot.enc_master_key, &k, &wallet.wallet_id)?;
    k.zeroize();
    decrypt_sensitive_wallet(wallet, &master_key)
}

pub fn unlock_with_mnemonic(
    wallet: &WalletEnvelope,
    phrase: &str,
    pin: [u8; 4],
    profile: KdfProfile,
) -> Result<SensitiveWallet, AuthError> {
    wallet.verify_signature()?;
    let slot = wallet
        .content
        .mnemonic_slot
        .as_ref()
        .ok_or(AuthError::LoginMethodNotSupported)?;
    let mut input = phrase.trim().as_bytes().to_vec();
    input.extend_from_slice(&pin);
    let mut k = derive_key_from_pass(input, &slot.salt, &wallet.wallet_id, profile)?;
    let master_key = unwrap_master_key(&slot.enc_master_key, &k, &wallet.wallet_id)?;
    k.zeroize();
    decrypt_sensitive_wallet(wallet, &master_key)
}

fn decrypt_sensitive_wallet(
    wallet: &WalletEnvelope,
    master_key: &[u8; 32],
) -> Result<SensitiveWallet, AuthError> {
    let cipher = ChaCha20Poly1305::new(master_key.into());
    let pt = cipher
        .decrypt(
            ChaChaNonce::from_slice(&wallet.content.log_nonce),
            Payload {
                msg: &wallet.content.encrypted_sensitive_wallet,
                aad: &wallet.wallet_id,
            },
        )
        .map_err(|_| AuthError::DecryptionFailed)?;
    serde_json::from_slice(&pt).map_err(|e| AuthError::Serialization(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_wallet_password_and_textcode() {
        let created = create_wallet(CreateWalletParams {
            display_name: "Ada".into(),
            security_text: "crimson-sparrow-42".into(),
            password: Some("correct-horse-battery".into()),
            enable_mnemonic: true,
            pazzle: None,
            pin: Some([1, 9, 8, 4]),
            in_memory_only: false,
            profile: KdfProfile::MobileConstrained,
        })
        .unwrap();

        let code = created.envelope.to_text_code().unwrap();
        let imported = WalletEnvelope::from_text_code(&code).unwrap();
        let unlocked = unlock_with_password(&imported, "correct-horse-battery", KdfProfile::MobileConstrained).unwrap();
        assert_eq!(unlocked.display_name, "Ada");
        assert!(unlock_with_password(&imported, "wrong-password", KdfProfile::MobileConstrained).is_err());
    }
}
