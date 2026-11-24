# Closing the Confidential Sync & Transport Gap for Willow over Iroh (`willow-gap`)

This specification defines a complete, upstream-ready architecture for **Confidential Willow Transfer Protocol (WTP) over Iroh (`iroh`)** targeting the Rust Willow ecosystem (`codeberg.org/worm-blossom/willow_rs`). It closes the open gap left between the paused **Confidential Sync (WGPS)** specification and the unencrypted **Willow Transfer Protocol (WTP)** proposal—bringing together **Private Interest Overlap Detection (PIO)**, **RBSR reconciliation**, **verifiable `bab_rs` payload streaming**, optional **`specs/e2e` blind-relay storage encryption**, and production-hardened library engineering patterns distilled from auditing **`rabble/riot`**.

---

## 1. Verified Ecosystem Baseline & Crate Targets

| Component | Target Version / Source | Current Capabilities & Gap Status |
| :--- | :--- | :--- |
| **`willow25`** | `0.7.12-alpha.1` (`codeberg.org/worm-blossom/willow_rs`, `main` Nov 17, 2026 / `crates.io` Nov 18, 2026) | Pre-configured **Willow’25** parameters (Ed25519 + `WILLIAM3` + Meadowcap), `drop_format`, `MemoryStore`, `PersistentStore` (`fjall`), `PayloadPrefixStore`, `SummarisableStore`, `SplittableStore`, and the `reconciliation` module (`AreaSlice`, `ReconciliationRange`, `LengthyAuthorisedEntry`, `Fingerprint`, `Summary`, `Monoid`). |
| **`willow_transfer_protocol`** | `wtp` branch (`codeberg.org/worm-blossom/willow_rs`, commit `6f4b367`, Oct 12, 2026; WTP spec Oct 16, 2026) | Defines core WTP message types (`RequestEntries`, `RespondToRequestEntries`, `RequestPayloadSlice`, `RespondToRequestPayloadSlice`, `SendEntry`, `SendPayloadSlice`, `CancelOwnRequest`, `ResponseInformationProvenance`) matching the WTP specification (*Proposal*, Oct 16, 2026). Currently lacks an async session state machine, transport adapter, and confidentiality layer. |
| **`iroh`** | `1.4.2` (`crates.io` / `n0-computer/iroh`, Nov 19, 2026 UTC) | QUIC-based P2P transport providing `Endpoint`, Ed25519 `EndpointId`, TLS 1.3 authentication (`conn.remote_id()`), TLS keying material export (`export_keying_material`), `Router` + `ProtocolHandler` ALPN dispatch, hole-punching + relay fallback, and native stream multiplexing (`open_bi`, `accept_bi`, `open_uni`). |
| **`bab_rs`** | `0.8.2` (`crates.io` / Codeberg, Nov 17, 2026; `ufotofu-compat` feature) | Verifiable streaming `WILLIAM3` hashing (`PartialVerification`, `k`-grouped slice streams) for incremental payload verification. |
| **`ufotofu`** | `0.12.7` (`crates.io` / Codeberg, Sep 29, 2026) | Symmetric async `Producer`, `Consumer`, `BulkProducer`, and `BulkConsumer` stream traits used across `willow25` and `bab_rs`. |
| **Ecosystem Test & Interop Vectors** | `willow_test_vectors` (Sep 22, 2026), `Deln0r/willow-go` (`8c5b56c`, Nov 10, 2026), `sneakerweb` (Nov 18, 2026), `Pillow` (grant Aug 2026) | Byte-for-byte wire encoding fixtures (`willow_test_vectors`), zero-CGO Go/mobile bindings (`willow-go`), Drop Format `.snk` packaging (`sneakerweb`), and NLnet-funded WTP relay/support-node work (`Pillow` / `p2panda`). |

### Problem Statement: The Confidentiality & Session-Runner Gap
On `willowprotocol.org`, **Confidential Sync (WGPS)** is marked deprecated/paused while the Willow authors complete the **Willow Transfer Protocol (WTP)**. While WTP cleanly standardizes entry reconciliation and payload slicing, by design it transmits `namespace_id`, `grouping` (`AreaOfInterest` / `3dRange`), and `ReadCapability` tokens **in the open**. Without a confidentiality gate, any passive listener or peer without mutual interest can harvest which namespaces, subspaces, and path prefixes a node holds.

This specification closes that gap cleanly for **Rust + Iroh**: combining a sans-I/O **`willow_transfer_protocol`** session runner, a general-purpose **`willow_pio`** confidentiality gate, an **`iroh 1.4.2`** QUIC transport binding (`willow_iroh`), and an optional **`willow_e2e`** storage codec for untrusted always-on relays.

---

## 2. Layered Architecture (`willow-iroh-wtp`)

### 2.1 What Iroh Eliminates from the Standalone Willow Transport Stack
Willow’s transport-neutral specifications define a custom Noise XX handshake (**`Handshake and Encryption Scheme`**) and a stream multiplexer (**`LCMUX`**) to compensate for raw byte pipes (TCP, Unix sockets) that lack encryption and independent channels. When targeting **Iroh `1.4.2`**, both layers are redundant and deliberately omitted:

1. **No Noise XX Inside TLS 1.3:** Iroh establishes TLS 1.3 over QUIC authenticated to each peer’s Ed25519 `EndpointId`. Double-encrypting frames adds CPU overhead, latency, and redundant replay-window state.
2. **No `LCMUX` Framing:** QUIC supplies native, independently flow-controlled streams without head-of-line blocking:
   * **Stream `0` (Bidirectional Control Stream):** Session receiver-key binding proof, PIO salted interest negotiation, capability reveal, and `RequestEntries` / `RespondToRequestEntries` / `CancelOwnRequest` / RBSR control flow.
   * **Parallel Bidirectional / Unidirectional Streams:** Dedicated QUIC streams for `SendEntry` eager subscriptions and `RequestPayloadSlice` / `RespondToRequestPayloadSlice` / `SendPayloadSlice` `bab_rs` chunk transfers so large binary payloads never stall metadata reconciliation.
3. **ALPN Service Dispatch:** A single `iroh::protocol::ProtocolHandler` registered on Iroh’s `Router` under a versioned ALPN (e.g., `b"willow/wtp-pio/1"`).

```
┌──────────────────────────────────────────────────────────────────────┐
│ Caller / Host Runtime (CLI, Daemon, Relay, FFI Host, Browser/Wasm)   │
│  • Generic over NamespaceScheme, AuthorisationToken, and Store       │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 4: Encrypted Willow (willow_e2e) [Optional Blind-Relay Wrapper]│
│  • Per-namespace / subspace / path-component HKDF + ChaCha20Poly1305 │
│  • Preserves timestamps & prefix-pruning order for untrusted stores  │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 3: Sans-I/O WTP + RBSR + Bab Payload Verification              │
│  • RequestEntries / SendEntry / RequestPayloadSlice / CancelRequest  │
│  • Generic over SplittableStore + SummarisableStore + PayloadPrefix  │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 2: General-Purpose PIO Gate & Resource Handle Lifecycle        │
│  • TLS Exporter proof binds Willow Receiver <-> Iroh EndpointId      │
│  • Exact + relaxed parent Private Interests + EnumerationCapability  │
│  • Refcounted ResourceHandle (u64) & SessionCapabilityHandle tables  │
├──────────────────────────────────────────────────────────────────────┤
│ Layer 1: Iroh 1.4.2 QUIC Transport (willow_iroh)                     │
│  • Router + ProtocolHandler (ALPN), remote_id(), NAT traversal       │
│  • TokioStreamBridge (tokio::io <-> ufotofu::BulkProducer/Consumer)  │
└──────────────────────────────────────────────────────────────────────┘
```

---

### 2.2 Layer 1 & 2: Session Key Binding, Anti-MITM Invariant & Directional Salts

Willow’s **Read Access and Confidentiality (PIO)** specification enforces a strict anti-MITM invariant:
> *All `ReadCapability` tokens presented by a peer within a session must share a single `Receiver` public key, and the peer must prove possession of the corresponding `Receiver` secret key bound cryptographically to that exact encrypted session.*

`willow_iroh` supports two binding modes without performing a separate Noise handshake:

1. **Direct Ephemeral Binding (`Receiver == EndpointId`):**
   * An Iroh `EndpointId` is an Ed25519 verifying key authenticated during the QUIC TLS 1.3 handshake (`conn.remote_id()`).
   * A peer may delegate an ephemeral session `McCapability` (or custom `ReadCapability`) directly to its `EndpointId`.
   * Zero additional signature round-trips are required, and using ephemeral per-session `EndpointId`s for outbound initiators prevents cross-session social-graph linkability across relays or public endpoints.
2. **Decoupled Identity Binding (TLS Exporter Proof):**
   * When a peer separates its networking `EndpointId` from its Willow `Receiver` identity key (e.g., long-lived relay endpoints or multi-identity daemons), both sides invoke `conn.export_keying_material(...)` immediately after QUIC handshake completion using a fixed domain-separation label (e.g., `b"willow-iroh-pio-v1"`):
     * `session_binding_digest` (32 bytes, binding both peers' `EndpointId`s, ALPN, and TLS exporter output)
     * `ini_salt` (32 bytes) and `res_salt` (32 bytes) — fresh directional PIO salts derived deterministically per session so stale salted hashes cannot be replayed across connections.
   * Each peer transmits `(receiver_pk, sig(receiver_sk, session_binding_digest))` as Frame `0` on Control Stream `0`. Verification failure triggers an immediate fail-closed `abort_session`.

---

### 2.3 Layer 2: General-Purpose Private Interest Overlap Detection (`willow_pio`)

To uphold **Private Interest Overlap** for arbitrary Willow namespaces and capability topologies before any WTP `RequestEntries` or `ReadCapability` crosses the wire:

1. **Full Interest Normalization (Exact, Relaxed & Enumeration Capabilities):**
   * Callers supply local `(NamespaceId, AreaOfInterest)` subscriptions and available `ReadCapability` / `EnumerationCapability` tokens.
   * Because peers frequently hold intersecting areas at different granularities (e.g., Peer A holds `(/docs, Subspace::Any, /2026)` while Peer B holds `(/docs, Subspace::Id(k), /2026/q4/spec)`), `willow_pio` expands every local interest into its canonical **Private Interest** set plus **all relaxed ancestor/subspace variants** defined in `specs/pio`.
   * When a peer wishes to keep its queried `SubspaceId`s confidential inside a shared namespace, `willow_pio` executes the spec’s **`EnumerationCapability` negotiation** flow rather than broadcasting raw `SubspaceId`s.
2. **Salted Digest Exchange:**
   * Initiator and Responder exchange compact frames carrying `WILLIAM3(own_salt || canonical_encode(private_interest))` digests—never unhashed `NamespaceId`, `SubspaceId`, or `Path` components.
3. **Local Intersection Verification:**
   * Each peer evaluates `WILLIAM3(peer_salt || canonical_encode(local_interest))` against the peer’s advertised digest set. Only colliding digests unlock an overlap; non-overlapping namespaces, subspaces, and paths remain cryptographically indistinguishable.
4. **Scoped `ResourceHandle` (`u64`) & Admission Boundary:**
   * Confirmed overlaps unlock transmission of the bound `ReadCapability` (along with optional `AreaOfInterest` / grouping narrowing), registering each verified capability under a compact `u64` **`SessionCapabilityHandle`**.
   * `willow_pio` maintains explicit reference-counted `ResourceHandle` tables (for granted capabilities, active area handles, and payload handles) with configurable per-session slot caps and deterministic eviction/freeing so untrusted peers cannot exhaust session memory.
   * Every downstream WTP frame referencing a handle is checked at the admission boundary to enforce `requested_grouping ⊆ granted_area(read_capability)` strictly.

---

### 2.4 Layer 3: Sans-I/O Async WTP + RBSR State Machine (`willow_transfer_protocol`)

Building directly on the message types in `willow_rs` (`wtp` branch) and the reconciliation traits in `willow25` (`main` branch):

* **Trait-Generic Core (Zero Iroh/Tokio Coupling):**
  Parameterize `WtpSession<S, N, ...>` strictly over Willow traits and `ufotofu` streams:
  ```rust
  S: SplittableStore + SummarisableStore + PayloadPrefixStore + Clone + Send + Sync + 'static
  ```
  with zero dependency on `tokio` or `iroh` inside `willow_transfer_protocol`. Every wire decode, signature check, capability check, or budget violation returns a typed `SessionAbort` error—never panicking.
* **Request & RBSR Flow (`RequestEntries` / `RespondToRequestEntries`):**
  1. **Fingerprint Short-Circuit:** When `own_fingerprint == store.summarise(&area_slice)`, respond with `ResponseForRequestEntries::Metadata` (zero entries sent).
  2. **Caller-Configurable Threshold Evaluation:** If entry count `< threshold_count` and total claimed payload bytes `< threshold_size`, stream `ImmediateEntries` (`LengthyAuthorisedEntry`) respecting `preferred_ordering` (`NewestThenSubspaceThenPath` or `SubspaceThenPathThenNewest`).
  3. **Symmetric / Asymmetric RBSR Bisection:** When thresholds are exceeded:
     * If `allow_symmetric_rbsr == true`, split `AreaSlice` via `SplittableStore` and emit recursive child `RequestEntries` stamped with `ResponseInformationProvenance { parent, is_final_response, root, is_root_by_sender }`, enabling $O(1)$-memory completion tracking across peers.
     * If `allow_symmetric_rbsr == false`, emit `Metadata { fingerprint, count, size }` so the requester drives bisection.
* **Live Subscriptions & Pipelined Cancellation:**
  * Support `is_oneshot = false` subscriptions registered against store mutation streams (`watch_area`), pushing incremental `SendEntry` frames until `CancelOwnRequest` or stream termination.
  * Support both stateless fire-and-forget queries (`request_id: None`) and pipelined stateful queries keyed by monotonic `u64` `WtpMessageId` counters.

---

### 2.5 Layer 4: Verifiable `bab_rs` Payload Streaming & `TokioStreamBridge`

1. **`TokioStreamBridge` (`tokio::io` ↔ `ufotofu`):**
   * Provided in `willow_iroh`: a lightweight, zero-copy adapter wrapping `(iroh::endpoint::SendStream, iroh::endpoint::RecvStream)` into `ufotofu::BulkProducer<Item = u8>` and `ufotofu::BulkConsumer<Item = u8>`. Canonical `compact_u64` codecs and `bab_rs` incremental verifiers stream directly over QUIC without intermediate whole-buffer copies.
2. **Configurable `payload_lazyness` Cutoff:**
   * Entries whose payload length `<= payload_lazyness` may travel inline with `SendEntry` on the active stream.
   * Larger payloads are requested lazily via `RequestPayloadSlice` / `RespondToRequestPayloadSlice` (`Found` / `Done` / `Nope { retry }`) over dedicated parallel QUIC streams.
3. **Incremental `bab_rs` Slice Verification:**
   * Senders stream `k`-grouped `bab_rs::PartialVerification` proofs alongside raw slice bytes read from `PayloadPrefixStore`.
   * Receivers verify each incoming chunk against the entry’s `WILLIAM3` `PayloadDigest` *as bytes arrive*, committing verified prefixes atomically into `PayloadPrefixStore` and aborting immediately on any hash mismatch.

---

### 2.6 Layer 5: Optional `willow_e2e` Codec for Untrusted Always-On Storage Relays

PIO + Iroh TLS secures sync *between mutual peers* and blinds passive Iroh network relays (which forward opaque QUIC packets). However, an **always-on Willow storage relay**—a third-party node executing `SplittableStore` / `SummarisableStore` RBSR while authors are offline—must hold a `ReadCapability` to participate in WTP.

To keep untrusted storage relays blind to entry paths and payload contents:
* Implement **`specs/e2e` (Encrypted Willow)** as a generic `Store` / entry codec wrapper (`EncryptedStore<S>`):
  * Derive symmetric keys hierarchically via `HKDF` over `(root_or_subspace_key, path_component)` so read keys can be scoped cleanly to arbitrary subtrees.
  * Encrypt each `Path` component individually—preserving hierarchical prefix structure and pruning semantics for authorized readers while exposing only pseudorandom ciphertext components to the relay.
  * Encrypt payloads with AEAD (`ChaCha20-Poly1305` + random nonce) **before** computing `WILLIAM3` `PayloadDigest`s.
  * Keep `Timestamp` and Willow entry framing intact so blind relays run full RBSR `Summary` fingerprinting, `AreaSlice` splitting, and prefix pruning without ever possessing decryption keys.

---

## 3. Upstream Crate Split & Hardening Patterns (Adapted from `rabble/riot`)

Auditing `github.com/rabble/riot` surfaced several library-level engineering safeguards that are directly applicable to an upstream `worm-blossom/willow_rs` submission—independent of any specific host application:

### 3.1 Clean Workspace Crate Separation
* **`willow_transfer_protocol`**: Pure sans-I/O protocol codec + `WtpSession` state machine parameterized over `SplittableStore + SummarisableStore + PayloadPrefixStore` and `ufotofu` streams (zero `tokio` or `iroh` dependencies).
* **`willow_pio`**: Pure Private Interest normalization, `EnumerationCapability` handling, salted overlap checks, and `ResourceHandle` table logic composable with WTP.
* **`willow_e2e`**: Optional `Store` wrapper implementing hierarchical path-component encryption and AEAD payload sealing per `specs/e2e`.
* **`willow_iroh`**: Thin `iroh 1.4.2` transport binding (`Router`, `ProtocolHandler`, `TokioStreamBridge`, `EndpointId` / TLS exporter binding, multi-stream WTP/Bab dispatch).
* **`willow_ffi`** *(optional)*: Panic-catching opaque handle + arbiter wrapper for official C, UniFFI, JNA, or Wasm host integrations.

### 3.2 Feature-Gated `willow25` & Wasm Dependency-Graph Enforcement
* **Gap in `willow25 0.7.12-alpha.1`:** Pulling filesystem/LSM dependencies (`fjall`, `lsm-tree`, `async-fs`, `bab_rs/storage-fs`) unconditionally breaks compilation on `wasm32-unknown-unknown` and embedded targets.
* **Upstream Fix (per Riot’s patch):** Gate all filesystem storage crates behind an opt-in `persistent-storage` Cargo feature and add a CI graph check (`check-wasm-graph`) ensuring core traits (`willow25`, `willow_transfer_protocol`, `willow_pio`, `willow_e2e`) always compile cleanly to Wasm and `no_std`+`alloc`.

### 3.3 Copy-on-Write Staged Admission (`inspect → plan → commit`)
* Incoming WTP frames, `ReadCapability` delegations, and `drop_format` imports must never mutate a live `Store` piecemeal as bytes arrive from untrusted peers.
* Adopt Riot’s deterministic 3-phase admission boundary:
  1. **`inspect()`**: Canonical-decode and verify signatures/digests against strict caller-configured resource budgets (`max_entry_bytes`, `max_capability_bytes`, `max_path_components`, `max_entries_per_batch`, duplicate ID rejection).
  2. **`plan()`**: Stage entry insertion, prefix pruning, and capability scope validation against an isolated snapshot/transaction without mutating visible store state.
  3. **`commit()`**: Apply validated mutations atomically (or roll back cleanly on budget overflow or abort).

### 3.4 Panic-Catching Handle + Arbiter FFI Boundary (`willow_ffi`)
* For hosts embedding Rust via C-ABI, UniFFI, JNA, or Wasm (mirroring `willow-go`’s zero-CGO mobile bindings), expose opaque session handles backed by `Arc<Mutex<SessionState>>` arbiters.
* Wrap every FFI entry point in `std::panic::catch_unwind` (compiled with `panic = "unwind"` where supported) so an unexpected fault triggered by adversarial peer input quarantines the affected session handle rather than crashing the host process.

### 3.5 Verification, Fuzzing & CI Contract Enforcement
* **Wire & Golden Vector Parity:** Run automated byte-for-byte parity tests against `willow_test_vectors` and cross-implementation suites (`Deln0r/willow-go`, `sneakerweb`) covering `compact_u64`, `Path`, `Entry`, `McCapability`, `PrivateInterest`, and WTP frames.
* **Adversarial Fuzzing (`willow_rs/fuzz/`):** Fuzz every `ufotofu` decoder, PIO overlap state machine, RBSR `AreaSlice` splitter, and `bab_rs` `PartialVerification` reader to guarantee zero panics and bounded allocations on arbitrary byte streams.
* **Structured `tracing` & Lockstep Contract CI:** Instrument session boundaries with structured `tracing` spans (`willow::iroh`, `willow::pio`, `willow::wtp`, `willow::admission`) and enforce dependency-pin, Wasm-graph, and wire-codec contract checks in CI.
