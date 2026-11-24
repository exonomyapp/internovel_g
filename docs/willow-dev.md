# Willow + Iroh Transport, Confidentiality & Native Core Architecture (`willow-dev`)

This document serves as both **InterNovel’s deep technical blueprint** for migrating onto the modern **Willow Protocol (`willow25`) + Iroh (`iroh`)** stack and a **formal upstream engineering proposal** targeting `codeberg.org/worm-blossom/willow_rs`. It specifies the exact crate baseline, the missing confidentiality and transfer-protocol layers required to ship **Confidential Willow Transfer Protocol (WTP) over Iroh**, and the production-hardened engineering patterns adopted from auditing **`rabble/riot`**.

---

## 1. Current Ecosystem Baseline & Crate Targets

Rather than building on legacy `earthstar@10.2.2` (npm) or waiting on the stalled `11.0.0-beta` wrapper (JSR), InterNovel targets the actively maintained Rust implementations and standardized **Willow’25** parameter suite:

| Component | Target Version / Source | Current Capabilities & Role |
| :--- | :--- | :--- |
| **`willow25`** | `0.7.12-alpha.1` (`codeberg.org/worm-blossom/willow_rs`, `main` Nov 17, 2026 / `crates.io` Nov 18, 2026) | Pre-configured **Willow’25** parameters (Ed25519 + `WILLIAM3` + Meadowcap), `drop_format`, `MemoryStore`, `PersistentStore` (`fjall`), `PayloadPrefixStore`, `SummarisableStore`, `SplittableStore`, and the `reconciliation` module (`AreaSlice`, `ReconciliationRange`, `LengthyAuthorisedEntry`, `Fingerprint`, `Summary`, `Monoid`). |
| **`willow_transfer_protocol`** | `wtp` branch (`codeberg.org/worm-blossom/willow_rs`, commit `6f4b367`, Oct 12, 2026; WTP spec Oct 16, 2026) | Defines the core WTP message structs (`RequestEntries`, `RespondToRequestEntries`, `RequestPayloadSlice`, `RespondToRequestPayloadSlice`, `SendEntry`, `SendPayloadSlice`, `CancelOwnRequest`, `ResponseInformationProvenance`) aligned with the WTP specification (*Proposal*, Oct 16, 2026). Lacks an async session runner, transport binding, and confidentiality layer. |
| **`iroh`** | `1.4.2` (`crates.io` / `n0-computer/iroh`, Nov 19, 2026 UTC) | QUIC-based P2P networking with `Endpoint`, Ed25519 `EndpointId`, TLS 1.3 authentication (`conn.remote_id()`), `Router` + `ProtocolHandler` ALPN dispatch, hole-punching + relay fallback, and native stream multiplexing (`open_bi`, `accept_bi`, `open_uni`). |
| **`bab_rs`** | `0.8.2` (`crates.io` / Codeberg, Nov 17, 2026; `ufotofu-compat` feature) | Verifiable streaming `WILLIAM3` hashing (`PartialVerification`, `k`-grouped slice streams) for incremental payload verification. |
| **`ufotofu`** | `0.12.7` (`crates.io` / Codeberg, Sep 29, 2026) | Symmetric async `Producer` / `Consumer` / `BulkProducer` / `BulkConsumer` stream traits used throughout `willow25` and `bab_rs`. |
| **Ecosystem Test & Interop Vectors** | `willow_test_vectors` (Sep 22, 2026), `Deln0r/willow-go` (`8c5b56c`, Nov 10, 2026), `sneakerweb` (Nov 18, 2026), `Pillow` (grant Aug 2026) | Byte-for-byte wire compatibility fixtures (`willow_rs` / `willow-go` zero-CGO mobile bindings), Drop Format `.snk` packaging (`sneakerweb`), and NLnet-funded WTP relay/support-node research (`Pillow` / `p2panda`). |

### Why Confidential Sync Was Paused — and How We Close the Gap
On `willowprotocol.org`, the original **Confidential Sync (WGPS)** specification is marked deprecated/paused while the authors finish the simpler **Willow Transfer Protocol (WTP)**. However, WTP explicitly transmits `namespace_id`, `grouping` (`AreaOfInterest` / `3dRange`), and `ReadCapability` tokens **in the open**, leaving metadata exposed to peers and intermediaries.

Our design bridges that exact gap for **Rust + Iroh**: layering **Private Interest Overlap Detection (PIO)** and **Encrypted Willow (`e2e`)** cleanly around WTP over Iroh QUIC streams.

---

## 2. Architecture: Confidential WTP over Iroh (`willow-iroh-wtp`)

### 2.1 What Iroh Replaces Out of the Box
Willow’s standalone specs define a custom Noise XX handshake (**`Handshake and Encryption Scheme`**) and a stream multiplexer (**`LCMUX`**) solely because raw TCP/Unix pipes provide neither transport encryption nor framing. On **Iroh `1.4.2`**, we deliberately eliminate both layers:

1. **No Noise XX Inside TLS:** Iroh establishes TLS 1.3 over QUIC authenticated to each peer’s Ed25519 `EndpointId`. Double-encrypting every frame inside an already-encrypted QUIC connection wastes CPU and complicates flow control.
2. **No `LCMUX` Framing:** QUIC provides native stream multiplexing, independent per-stream flow control, and zero head-of-line blocking:
   * **Stream `0` (Bidirectional Control Stream):** Session binding proof, PIO salted interest negotiation, capability reveal, and `RequestEntries` / `RespondToRequestEntries` / `CancelOwnRequest` / RBSR control messages.
   * **Parallel Bidirectional / Unidirectional Streams:** Dedicated QUIC streams for `SendEntry` eager subscriptions and `RequestPayloadSlice` / `SendPayloadSlice` `bab_rs` chunk streaming so large media transfers never stall metadata reconciliation.
3. **ALPN Service Dispatch:** One `iroh::protocol::ProtocolHandler` registered on Iroh’s `Router` under a versioned ALPN (e.g., `b"willow/wtp-pio/1"`).

```
┌──────────────────────────────────────────────────────────────────────┐
│ Application Layer (InterNovel / Native / Wasm)                       │
│  • Yjs CRDT updates, Novel Spine, Meadowcap ACLs, E2E keys           │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 4: Encrypted Willow (specs/e2e) [Optional for Untrusted Relays]│
│  • Per-subspace / per-path-component symmetric encryption            │
│  • Preserves timestamps & prefix-pruning order for blind relays      │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 3: WTP + Symmetric/Asymmetric RBSR + Bab Payload Streaming     │
│  • RequestEntries / SendEntry / RequestPayloadSlice / CancelRequest  │
│  • Driven via SummarisableStore + SplittableStore + PayloadPrefix    │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 2: Confidential PIO Gate & Receiver Key Binding                │
│  • TLS Exporter signature binds Willow Receiver <-> Iroh EndpointId  │
│  • Salted WILLIAM3 Private Interest hashes (exact + relaxed parents) │
│  • SessionCapabilityHandle unlock ONLY on confirmed mutual overlap   │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 1: Iroh 1.4.2 QUIC Transport (Router + ProtocolHandler)        │
│  • TLS 1.3 peer auth (remote_id), NAT traversal, native QUIC streams │
├──────────────────────────────────────────────────────────────────────┤
│ Adapter: TokioStreamBridge (tokio::io <-> ufotofu::BulkProducer/Cons)│
└──────────────────────────────────────────────────────────────────────┘
```

---

### 2.2 Layer 1 & 2: Session Binding, Anti-MITM Invariant, and Salts

The Willow **Read Access and Confidentiality (PIO)** specification requires a strict anti-MITM invariant:
> *All `ReadCapability` tokens presented by a peer in a session must share a single `Receiver` public key, and the peer must prove possession of that `Receiver` secret key bound directly to the encrypted session.*

We support two clean binding modes on Iroh without running a redundant Noise handshake:

1. **Direct Ephemeral Binding (`Receiver == EndpointId`):**
   * Because an Iroh `EndpointId` is an Ed25519 verifying key proven during the QUIC TLS 1.3 handshake (`conn.remote_id()`), a peer can delegate an ephemeral session `McCapability` directly to its `EndpointId`.
   * Zero additional signature round-trips are required, and using ephemeral per-session `EndpointId`s (for dialers/followers) prevents cross-session social graph linkability.
2. **Decoupled Identity Binding (TLS Exporter Proof):**
   * When a peer keeps its Iroh networking `EndpointId` separate from its Willow `Receiver` key (e.g., for stable relay connections or multi-account switching), both peers call `conn.export_keying_material(...)` immediately after QUIC handshake completion to derive:
     * `session_binding_digest` (32 bytes, incorporating both `EndpointId`s + ALPN + exporter secret)
     * `ini_salt` (32 bytes) and `res_salt` (32 bytes) — directional PIO salts derived deterministically from the TLS exporter so neither peer can reuse/replay stale hashes or salts.
   * Each peer sends `(receiver_pk: SubspaceId, ed25519_sig(receiver_sk, session_binding_digest))` as Frame 0 on Control Stream `0`. Any peer failing verification is disconnected immediately (`abort_session`).

---

### 2.3 Layer 2: Private Interest Overlap Detection (PIO) Gate

To uphold **Private Interest Overlap** before any WTP `RequestEntries` or `ReadCapability` leaks metadata:

1. **Interest Normalization (Exact + Awkward Overlaps):**
   * Each peer gathers its local `(NamespaceId, AreaOfInterest)` pairs.
   * Because two peers may hold overlapping hierarchical interests of different specificity (e.g., Peer A holds `/novel-1/*` while Peer B holds `/novel-1/chapters/03`), each interest is expanded into its canonical **Private Interest** set plus the **relaxed parent/prefix variants** defined in `specs/pio`.
   * Where a peer wishes to hide which specific `SubspaceId`s it is querying inside a shared namespace, it prepares an **Enumeration Capability** flow per the PIO spec.
2. **Salted Hash Exchange:**
   * Initiator and Responder exchange compact frames containing `WILLIAM3(own_salt || canonical_encode(private_interest))` digests—never raw `NamespaceId`, `SubspaceId`, or `Path` bytes.
3. **Local Intersection Verification:**
   * Each peer hashes its own local Private Interests with the **peer’s salt** (`WILLIAM3(peer_salt || canonical_encode(local_interest))`) and checks for set intersection.
   * Only matching digests reveal a mutual interest—non-overlapping novels, chapters, and subspaces remain cryptographically opaque.
4. **Scoped Capability Unlock (`SessionCapabilityHandle`):**
   * Only after both sides independently observe the overlap match does each peer transmit the `ReadCapability` (bound to its authenticated `Receiver` key) for that matched region, assigning it a compact `u64` `SessionCapabilityHandle`.
   * Subsequent WTP messages on the connection reference the matched overlap and enforce that `grouping ⊆ granted_area(read_capability)` strictly at the admission boundary.

---

### 2.4 Layer 3: Async WTP + RBSR State Machine

Using the message definitions from `willow_rs` (`wtp` branch) and the storage/reconciliation traits from `willow25` (`main` branch):

* **Trait-Generic Store Boundary:** Parameterize the sync engine over:
  ```rust
  S: SplittableStore + SummarisableStore + PayloadPrefixStore + Clone + Send + Sync + 'static
  ```
  guaranteeing zero-panic behavior under adversarial wire inputs (every decoding, budget, and signature failure surfaces as a typed session abort without panicking).
* **Request & RBSR Flow (`RequestEntries` / `RespondToRequestEntries`):**
  1. **Fingerprint Short-Circuit:** If `own_fingerprint` matches `store.summarise(&area_slice)` for the requested grouping, reply with `ResponseForRequestEntries::Metadata` (zero entries transferred).
  2. **Threshold Evaluation:** If entry count `< threshold_count` and summed claimed payload sizes `< threshold_size`, transition to `ImmediateEntries` and stream `LengthyAuthorisedEntry` items (respecting `preferred_ordering`: `NewestThenSubspaceThenPath` or `SubspaceThenPathThenNewest`).
  3. **Symmetric / Asymmetric RBSR Bisection:** When thresholds are exceeded:
     * If `allow_symmetric_rbsr == true`, partition the `AreaSlice` via `SplittableStore` and emit recursive child `RequestEntries` messages stamped with `ResponseInformationProvenance { parent, is_final_response, root, is_root_by_sender }`, enabling $O(1)$-memory completion tracking across peers.
     * If `allow_symmetric_rbsr == false`, return `Metadata { fingerprint, count, size }` so the requester drives the range split.
* **Eager Live Subscriptions (`is_oneshot = false`):**
  * Keep the logical `WtpMessageId` request registered against a store mutation stream (`watch_area`), pushing new `SendEntry` frames as collaborative edits arrive until `CancelOwnRequest` or QUIC stream closure.
* **Stateless vs. Stateful Flexibility:**
  * Support `request_id: None` for fire-and-forget stateless requests alongside monotonic `u64` `WtpMessageId` counters for pipelined, cancellable requests.

---

### 2.5 Layer 4: Verifiable `bab_rs` Payload Streaming & `ufotofu` Bridge

1. **`TokioStreamBridge` (`tokio::io` ↔ `ufotofu`):**
   * Implement a lightweight, zero-copy adapter wrapping Iroh’s `(iroh::endpoint::SendStream, iroh::endpoint::RecvStream)` into `ufotofu::BulkProducer<Item = u8>` and `ufotofu::BulkConsumer<Item = u8>` so `willow25`’s canonical `compact_u64` encoders/decoders and `bab_rs` stream directly over QUIC without intermediate full-buffer allocations.
2. **Inline vs. Streamed Payloads (`payload_lazyness`):**
   * Small payloads (such as Yjs CRDT update deltas or short prose notes `<= payload_lazyness`) accompany `SendEntry` directly on the control/entry stream.
   * Larger assets (cover art, Character Travel Trunk media, long chapter blobs) are fetched lazily via `RequestPayloadSlice` / `RespondToRequestPayloadSlice` (`Found` / `Done` / `Nope { retry }`) over a dedicated QUIC stream.
3. **Incremental Slice Verification:**
   * Senders stream `k`-grouped `bab_rs` `PartialVerification` proofs alongside raw slice bytes from `PayloadPrefixStore`.
   * Receivers verify each chunk against the entry’s `WILLIAM3` `PayloadDigest` *as bytes arrive*, writing verified prefixes atomically into `PayloadPrefixStore` and immediately aborting if a peer sends corrupted bytes.

---

### 2.6 Layer 5: Blind Always-On Storage Relays (`specs/e2e`)

PIO + Iroh TLS protects confidentiality *between two syncing peers* and against passive Iroh network relays (which only forward encrypted QUIC packets). However, an **always-on Willow storage relay** (a third-party node that actually runs a `Store` and holds entries while authors are offline) must hold a `ReadCapability` to participate in RBSR.

To keep untrusted cloud relays completely blind to novel titles, chapter names, character bios, and prose:
* Implement **`specs/e2e` (Encrypted Willow)** as a storage/codec wrapper above `Store`:
  * Derive symmetric keys per namespace, per subspace, or hierarchically per path component (`HKDF` over `(subspace_key, component)`).
  * Encrypt each `Path` component individually (preserving structural prefix matching for authorized holders while making components indistinguishable random bytes to the relay).
  * Encrypt payloads with AEAD (`ChaCha20-Poly1305` + random nonce) before computing `WILLIAM3` digests.
  * Keep `Timestamp` and entry structure intact so blind relays can still run Willow prefix pruning, `Summary` fingerprints, and RBSR reconciliation without ever holding decryption keys.

---

### 2.7 Upstream Codeberg Contribution Checklist (`worm-blossom/willow_rs`)

To ensure this work is structured as an upstream-ready PR/RFC for `worm-blossom` on Codeberg:
* **Crate split:**
  * Finish `willow_transfer_protocol` (`wtp` branch): pure sans-I/O protocol codec + `WtpSession` state machine parameterized over `SplittableStore + PayloadPrefixStore` and `ufotofu` streams (zero dependency on `tokio` or `iroh`).
  * Separate `willow_pio` / `willow_confidential`: pure PIO salt/interest/capability gate composable with WTP.
  * Separate `willow_iroh`: thin `iroh 1.4.2` transport crate (`Router`, `ProtocolHandler`, `TokioStreamBridge`, TLS exporter binding).
* **Upstream `willow25` Wasm fix:** Feature-gate `fjall`, `async-fs`, and `bab_rs/storage-fs` behind an opt-in `persistent-storage` Cargo feature (matching Riot’s patch) so `willow25` compiles cleanly to `wasm32-unknown-unknown` without pulling filesystem/LSM crates.
* **Verification:** Property/fuzz targets under `willow_rs/fuzz/` for adversarial wire inputs, canonical encoding round-trips, `willow_test_vectors` byte-parity tests, and zero `unwrap()`/`panic!()` on untrusted network bytes.

---

## 3. Capabilities & Engineering Patterns Adopted from `rabble/riot`

Auditing `github.com/rabble/riot` (318 Rust files, 126 Swift files, 97 Kotlin files, FFI bindings, and Python gateway) surfaced a set of battle-tested local-first patterns and multi-platform capabilities that directly upgrade InterNovel’s architecture. We adopt **all of the above (AOTA)**:

### 3.1 Feature-Gated `willow25` & Wasm Dependency-Graph Enforcement
* **Pattern:** Riot vendors a minimal feature-gating patch on `willow25` (`persistent-storage` feature gating `fjall`/`lsm-tree`/`async-fs`/`bab_rs/storage-fs`) and enforces it in CI via a Wasm graph check (`check-wasm-graph.mjs`).
* **InterNovel Adoption:** Keep one shared Rust core crate (`internovel-core`) compiling cleanly to **both** native targets (iOS/macOS/Android/Desktop with persistent `fjall`/SQLite) and **`wasm32-unknown-unknown`** (web browser backed by `wa-sqlite` on OPFS/IndexedDB). Gate CI so no filesystem or `tokio::net` dependency ever accidentally leaks into the browser Wasm target.

### 3.2 Copy-on-Write Atomic Admission (`preview → plan → commit`)
* **Pattern:** Riot’s `RiotSession` never mutates live storage directly from incoming sync frames or Drop bundles. Instead, imports run through a deterministic 3-step boundary (`inspect()` → `plan()` → `commit()`) enforcing strict byte, entry-count, capability-size, and path-depth budgets (`MAX_ENTRY_BYTES`, `MAX_CAPABILITY_BYTES`, `MAX_PATH_COMPONENTS`, duplicate ID rejection), computing the join against a cloned/staged plan, and installing changes with a single atomic pointer/transaction swap.
* **InterNovel Adoption:** All incoming Willow entries, Yjs snapshots, Character Travel Trunks, and `.zip`/`.sqlite` Drop restores pass through an explicit preview-and-budgeted-plan step before touching active novel state.

### 3.3 Hardened SQLite Foundation (`wa-sqlite` + Native SQLite Parity)
* **Pattern:** Riot’s `RiotDatabase` runs SQLite in WAL mode with versioned migrations, fail-closed schema/integrity checks on open, and transactional `backup_to()` / `restore_from()` routines.
* **InterNovel Adoption:** Replace `idb-keyval` with `@journeyapps/wa-sqlite` (OPFS VFS + IndexedDB fallback) in the browser and native SQLite in the Rust core, implementing fail-closed integrity checks and unified `.sqlite` + Willow Drop backup/restore workflows in user Settings.

### 3.4 Sealed Identity Portability & Native Keystore Security
* **Pattern:** Riot seals exported signing keys using **`ChaCha20-Poly1305`** under a user-held wrapping secret (`EvidenceAuthor`) and stores device keys in iOS Keychain / Android Keystore.
* **InterNovel Adoption:** Upgrade `did:peer` / Ed25519 / Meadowcap keychain export/import to use authenticated `ChaCha20-Poly1305` sealed envelopes on web and Keychain / Android Keystore on native mobile/desktop.

### 3.5 Panic-Catching Handle + Arbiter FFI Boundary (`UniFFI` + `JNA`)
* **Pattern:** Riot’s `crates/riot-ffi/` exposes opaque handle IDs backed by `Arc<Mutex<SessionState>>` arbiters, compiled with `panic = "unwind"` and `std::panic::catch_unwind` at every FFI boundary—quarantining a session rather than crashing the host app if malformed peer data triggers an internal fault.
* **InterNovel Adoption:** Structure our Rust-to-Swift (`UniFFI`), Rust-to-Kotlin (`JNA`), and Rust-to-JS (`wasm-bindgen`) bindings behind the exact same handle + arbiter + fail-closed boundary.

### 3.6 Multi-Transport Abstraction & Privacy-Preserving Peer Dialing
Riot demonstrates how a single transport-neutral sync core can operate across radically different network conditions:
* **Iroh (QUIC) + Signed Invite Tickets & ALPN Router:** Used as InterNovel’s primary internet P2P and relay transport, pairing `Ticket` capability admission with **ephemeral follower `EndpointId`s** so co-authors dialing a public relay or Reading Room don’t expose a permanent linkable node ID.
* **Optional `arti` (Tor) Outbound Transport:** Behind an opt-in Cargo feature (`arti`), allowing writers in sensitive or censored environments to sync over Tor onion circuits.
* **Local Offline Nearby Sync (BLE / LAN):** Native **iOS/macOS CoreBluetooth + LAN `SyncCoordinator`** and **Android GATT BLE + local sockets** so writing clubs and classrooms can sync chapters, Plot Planners, and Character Travel Trunks face-to-face with zero internet access.
* **Portable Offline Drops & Constrained Radio Profiles:** `willow25::drop_format` file exchange (USB, AirDrop, email) plus Riot’s compact datagram framing pattern (e.g., LoRa/Meshtastic codec profile) for ultra-low-bandwidth resilience.

### 3.7 Native Cross-Platform Shells (iOS, macOS, Android) + Web Parity
* **Pattern:** Riot ships **Swift 6 / SwiftUI** (`apps/ios/`, compiled by reference for **`apps/macos/`**) and **Kotlin / Jetpack Compose** (`apps/android/`), both sharing the single Rust core via UniFFI/JNA and hosting sandboxed WebViews for interactive content.
* **InterNovel Adoption:** Keep our React/TypeScript UI as the primary web/PWA experience (and embeddable WebView UI where appropriate) while sharing the Rust Willow/SQLite/Crypto engine natively across iOS, macOS, and Android.

### 3.8 Stateless Public Web Gateway (Published Novel Renderer)
* **Pattern:** Riot’s `apps/gateway/` (`PublicGateway`) is a stateless HTTP renderer that ingests signed Willow space/site bundles, cryptographically verifies every entry’s signature, and serves fast, readable HTML with visible verification badges.
* **InterNovel Adoption:** Build an InterNovel **Public Gateway / Static Publisher** mode that verifies DID + Meadowcap signatures across a published Novel Share (excluding private Vertebrae structural metadata per `functional-spec.md`) and renders clean, verifiable HTML—including Remix/Fork attribution lineage badges—for readers who don’t have the app installed.

### 3.9 Signed Extensible Miniapps, Profile Cards, & Governance Conformance
* **Pattern:** Riot includes signed miniapp manifests/bundles with trust & endorsement directories, verifiable author profile cards, and deterministic CDDL/CBOR schema validation (`alert.cddl`, `governance-vectors.json`).
* **InterNovel Adoption:**
  * Cryptographically signed **Author Profile Cards** and **Character Travel Trunks** resolved via Willow paths rather than centralized databases.
  * Sandboxed, signed **Community Miniapp / Extension** support for the Casting Studio and Novel Café (e.g., character interview prompts, writing-group polls, shared lore wikis, world timelines).
  * **Structured Telemetry & CI Contract Enforcement:** Adopt Riot’s structured `tracing` spans (`internovel::sync`, `internovel::pio`, `internovel::admission`), lockstep contract tests, golden test vectors (`WILLIAM3` + Meadowcap + governance), and an `xtask validate-contracts` CI runner that blocks merges if dependency pins, wire codecs, or Wasm build boundaries regress.
