# InterNovel Platform: Technical Overview (Leptos + Willow + Iroh + `did:peer:4`)

## 1. System Overview & Stack

InterNovel is a local-first, peer-to-peer collaborative writing ecosystem written in **Rust**, targeting web browsers (Wasm), Linux/macOS/Windows desktop binaries, and iOS/Android mobile apps from a single shared core:

| Layer | Technology / Crate | Role |
| :--- | :--- | :--- |
| **Web Frontend & SSR** | **Leptos `0.8.23`** + `leptos_router 0.8.17` + `leptos_meta 0.8.7` + **Tailwind CSS v4** | Reactive isomorphic Rust/Wasm UI (`hydrate` + `ssr`) sharing types and crypto directly with the core engine. |
| **Web / Local Server** | **Axum `0.8.9`** + **`leptos_axum 0.8.10`** + `tokio 1.54.1` | Serves SSR HTML, hydrates Wasm bundles, exposes embedded documentation routes, and acts as the optional desktop/cloud server. |
| **P2P Sync & Confidentiality** | **`willow_gap_core`** (`willow25 0.7.12-alpha.1` + `iroh 1.5.0` + `bab_rs 0.8.2` + `ufotofu 0.12.7`) | Confidential Willow Transfer Protocol (WTP) over Iroh QUIC streams with PIO overlap gating, RBSR bisection, incremental `bab_rs` verification, and optional `specs/e2e` blind-relay encryption. See `/docs/willow-gap.md`. |
| **Identity & Wallet Auth** | **`internovel_auth`** (`did-peer 0.7.5` + `argon2 0.6` + `aes-gcm-siv 0.12` + `chacha20poly1305 0.11` + `bip39 3.0`) | NextGraph-inspired non-custodial local wallet (`Pazzle` + PIN, BIP-39 mnemonic, password, anti-phishing banner, QR/TextCode pairing) paired with **`did:peer:4`** long/short-form DIDs. See `/docs/identity-spec.md`. |
| **Mobile & Native FFI** | **`internovel_ffi`** (`uniffi 0.32.2`) | Riot-inspired panic-catching opaque-handle arbiter (`catch_unwind`) bridging the shared Rust engine to Swift 6 / SwiftUI (iOS/macOS) and Kotlin / Jetpack Compose (Android). |
| **Desktop Packaging** | **Tauri v2** + **`cargo-packager 0.11.8`** | Produces native installers for Linux (`.deb`, `.AppImage`, `.rpm`), macOS (`.dmg`, universal `.app`), and Windows (`.msi`, `.exe`). See `/docs/deployment-packaging.md`. |

---

## 2. Collaborative Writing & IP Architecture

* **Decentralized Identity (`did:peer:4`):** Every contribution, chapter edit, and character bio is signed by the author’s `did:peer:4` verification key for verifiable attribution and downstream Remix lineage.
* **The "Novel Spine" & "Vertebrae":** Structural outline nodes (`Ideas`, `Plot Points`, `Themes`) that organize manuscript documents in the Writing Studio (`Spine View` vs. `Flat List`). Vertebrae remain local/collaborator-private metadata and are excluded from published reader editions.
* **The "Character Travel Trunk":** Portable, Meadowcap-authorized character bundles (narrative sample + bio + provenance lineage) shared across stories and listed in the **Casting Studio**.
* **The "Novel Café":** Discovery space for featuring collaborative novels and open co-author invitations.
* **Willow Namespace & ACL Pattern:**
  * Each Novel lives in its own Willow `NamespaceId` with Meadowcap `ReadCapability` / `WriteCapability` delegation (`transitive` co-author grants).
  * Application-level ACL documents under `/acl/groups/<role>/members/<did>` enforce immediate cooperative revocation across synced peers even before capability expiry.
* **Staged Admission (`inspect → plan → commit`):** Adopted from `rabble/riot`, all incoming Willow sync batches, `drop_format` bundles, and Character Travel Trunks validate byte/entry/depth budgets in a staged transaction before touching live store state.

---

## 3. Search & Discovery Architecture

1. **Structured Metadata Directory:** Published novels emit compact structured metadata tags (`["Character:...", "Theme:...", "Location:..."]`, title, author `did:peer:4`, synopsis, namespace capability ticket) into contact-synced Directory subspaces.
2. **Local-First Full-Text Search:** Queries execute over locally synced `PersistentStore` (`fjall` on native / OPFS-backed store on Wasm) before hitting the network.
3. **Gossip Search (Query Routing):** Bounded-depth (`depth` TTL) query entries propagate through Contact outboxes and always-on Willow/Iroh relays, returning encrypted matches only when peers opt in.

---

## 4. UI & Documentation Rules

* **Dual Non-Recursive Documentation Menus:** A **Docs** menu listing `/docs/*.md` development specs and a **User** menu listing `/docs/user/*.md` guides, built from one reusable Leptos menu component while hiding `/docs/agent/`.
* **Micro-Typography Scale:** Compact `10px–12px` base typography with tight line heights and dynamic user scaling (`25%–300%` in `10%` steps) for dense technical specs and manuscripts with minimal scrolling.
