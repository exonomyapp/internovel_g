# Identity & Account Management Specification (`did:peer:4` + NextGraph Wallet Auth)

This document specifies InterNovel’s decentralized identity and local-first authentication architecture, combining **DIF `did:peer:4`** with the **NextGraph (`ng-wallet`)** non-custodial wallet model and **Riot’s** hardware keystore and FFI isolation patterns.

---

## 1. Design Principle: Do Not Reinvent NextGraph’s Auth Wheel

Rather than inventing an ad-hoc browser key store, InterNovel adopts NextGraph’s battle-tested **`ng-wallet`** cryptographic lifecycle and upgrades its identity identifier layer to **`did:peer:4`** (`did-peer 0.7.5`):

* **Zero Central Auth Server:** Wallets are created, unlocked, and recovered 100% locally on the user’s device. Neither Iroh relays nor storage nodes ever receive plaintext private keys or credentials.
* **Multi-Factor Local Key Wrapping (`derive_key_from_pass`):**
  * User credentials never serve directly as signing keys. Instead, a random 32-byte **`master_key`** encrypts the sensitive wallet log (`XChaCha20Poly1305` / `ChaCha20Poly1305`).
  * Each configured login slot independently wraps a copy of `master_key` using **`Aes256GcmSiv`** (with `wallet_id` bound as Associated Data) under a 32-byte key derived via **Argon2id** (`m_cost = 40 MiB` on desktop/web, tunable mobile profile for iOS 120 MiB memory constraints, `t_cost = 40`, `p_cost = 1`, `wallet_id` as Argon2 Associated Data).
* **Supported Login & Recovery Slots (`LoginMethod`):**
  1. **Password Slot:** Standard passphrase-derived wrapping key.
  2. **12-Word BIP-39 Mnemonic + 4-Digit PIN:** Standard recovery phrase combined with a short device unlock PIN.
  3. **Visual Pazzle + 4-Digit PIN:** Category-shuffled visual symbol/emoji sequence (`ShuffledPazzle` + shuffled PIN pad to defeat shoulder-surfing and touchscreen residue).
  4. **Ephemeral In-Memory Session Mode:** Allows unlocking a wallet strictly in RAM with zero persistent disk/IndexedDB/SQLite state left behind on exit.
* **Anti-Phishing Security Banner (`SecurityImage` + `SecurityText`):**
  * Every signed `WalletContent` envelope stores a user-chosen anti-phishing phrase and normalized 400×400 JPEG security image in its signed public header.
  * When selecting an existing wallet on any device, InterNovel verifies the wallet’s Ed25519 signature and renders the security image/text **before** requesting a PIN, password, or pazzle.

---

## 2. Upgrading NextGraph’s Raw `PubKey` Identities to `did:peer:4`

NextGraph identifies users and devices via raw Ed25519 `PubKey` strings (`WalletId`, `UserId`, `ClientId`). InterNovel bridges that directly into standard **`did:peer` Method 4 (`numalgo:4`)** using the **`did-peer` (`0.7.5`)** crate:

1. **Input DID Document Composition:**
   * **`verificationMethod` (`Ed25519`):** Author signing key (`authentication`, `assertionMethod`) for Meadowcap capabilities, Willow entries, and Remix attribution.
   * **`keyAgreement` (`X25519`):** Derived/bound Diffie-Hellman key for encrypted wallet session sealing and `specs/e2e` key exchange.
   * **`service` (`WillowIrohEndpoint`):** Encodes the device’s active Iroh `EndpointId` (or ephemeral session `EndpointId`) and optional relay hints.
2. **Long-Form vs. Short-Form Exchange:**
   * **Long-Form (`did:peer:4z...`):** Embodies the complete multibase/multihash-encoded Input Document. Used during initial QR/TextCode pairing, Contact exchange, and Reading Room invites so peers resolve keys and endpoints with zero ledger or DNS lookup.
   * **Short-Form (`did:peer:4zQm...`):** Compact SHA-256 multihash digest of the long-form document. Used in ongoing Willow entry metadata, DIDComm headers, and UI displays while retaining cryptographic immutability.

---

## 3. Per-Device Sessions, Keychain & Multi-Device Pairing

### 3.1 `SessionPeerStorage` & `ClientV0` Isolation
Following `ng-wallet`:
* Each physical device generates its own **`ClientV0`** Ed25519 identity and local `storage_master_key`.
* Every active unlock creates a **`SessionPeerStorage`** record containing an ephemeral/device `peer_key` and a monotonic **`last_wallet_nonce`**, ensuring encrypted wallet state updates never reuse AEAD nonces across devices.
* **Riot Hardware Keystore Hardening:** On iOS/macOS, the device’s `ClientV0` wrapping key lives in the **Apple Keychain**; on Android, in **Android Keystore**; on Linux/Windows desktop, in the OS secret service / DPAPI with **`ZeroizeOnDrop`** on all in-memory secret structs (`SensitiveWallet`).

### 3.2 Cross-Device Pairing (Zero-Server Wallet Transfer)
To move an author’s identity from an Ubuntu laptop to a mobile phone (or vice versa):
1. **Live QR Rendezvous Pairing:** The source device generates an ephemeral encrypted rendezvous QR code; the target phone scans it (`ScanQR`) to transfer the signed `Wallet` envelope and immediately unlock with Pazzle/PIN/Password.
2. **Alphanumeric TextCode Fallback:** Copies the encrypted transfer payload as a short check-summed text block when a camera is unavailable.
3. **Offline Recovery Kit (`.ngw` Wallet File / Printable PDF):** Exports the signed, encrypted wallet bundle + BIP-39 mnemonic backup for USB/AirDrop or physical archival, sealed with `ChaCha20-Poly1305` (per Riot’s portable envelope pattern).

---

## 4. User Interface: Account Switcher, Profile & Settings

* **Account Switcher Chip:** Displays the active author’s `displayName`, avatar, and truncated `did:peer:4zQm...` short-form DID; allows switching local profiles, launching an ephemeral session, pairing via QR/TextCode, or exporting a sealed recovery bundle.
* **Real-Time Reading & Writing Settings:** Tri-state Theme toggle (Dark / Light / System) and dynamic **Text Scaling** (`25%` to `300%` in `10%` increments) persisted per `ClientV0` profile.
