# InterNovel Feature Roadmap

This document provides a high-level roadmap of the key features planned for the InterNovel platform, intended for project management and tracking development progress.

## Local-First Data Synchronization (Willow P2P)

- The platform utilizes the Willow P2P protocol for a multi-user environment with personalized and isolated data stacks per user.
- Data is stored locally first, ensuring offline capabilities, and synchronized peer-to-peer when online.
- Fine-grained access control is managed via Willow's capabilities.

## Startup and Authentication

- The startup process initializes the local data store and Willow P2P synchronization engine.
- The authentication system is entirely decentralized: true DID-based verification (`did:peer`) for user identity, generated and managed per device.
- Route protection middleware ensures users have generated or imported a valid `did:peer` before accessing collaborative features.

## Foundation & Developer Experience

*   [x] **Dynamic In-App Documentation Viewer:** Auto-detecting Markdown renderer that dynamically lists and displays all `.md` files from the `/docs` folder in a navigation submenu.

## Core Features

*   [x] **Identity Module (Active):** Implement `did:peer` based user identities using a hybrid approach (`@noble/curves` and Web Crypto API). Build an IndexedDB "Keychain" for multi-account storage (keys, display name, avatar), an Account Switcher Chip UI, and an Identity Export/Import mechanism for cross-device portability.
  *   [x] Add Profile and Settings icons to Navbar (enabled when identity is active).
  *   [x] Build Profile Page for Avatar and Display Name updating.
  *   [x] Build Settings Dropdown Panel with tri-state theme toggle (Dark/Light/System) and Markdown text scaling slider (25% to 300%, 10% steps).
*   [ ] **Local Database Migration:** Replace `idb-keyval` with WebAssembly SQLite (`wa-sqlite`) for robust, cross-platform SQL storage and OPFS performance.
*   [ ] **Dual-Editor Integration:**
    *   [ ] Implement **TipTap** + **Yjs** for the main Novel Editor to support simultaneous, real-time co-editing.
    *   [ ] Implement **BlockNote** for the "Plot Planner" attached to Novels, Vertebrae, and Chapters for block-based brainstorming.
*   [ ] **Local Data Safety, Backups & Restore:** 
    *   [ ] Implement manual export to physical `.sqlite` or `.zip` files via the File System Access API.
    *   [ ] Add Backup Scheduling to the user Settings panel.
    *   [ ] Implement Multi-Platform Automation (auto-save on Desktop Chrome, scheduled download reminders on Safari/Mobile).
    *   [ ] Implement a robust Restore mechanism to recover local database state from backup files.
*   [ ] **Willow P2P Integration:** Set up the local data store and peer-to-peer synchronization engine.
*   [x] **User Profile Management:** Users can manage their profiles, linked to their local DID.
*   [x] **Collaborative Writing Environment:** Establish the basic real-time collaboration features, including presence indicators, updates, notifications, and core writing tools via Willow P2P. (Editor, Markdown, Auto-save implemented; P2P sync pending).
*   [ ] **Authorization and Moderation:** Set up the tiered authorization system and moderation process, particularly for student accounts.
*   [x] **Novel Spine:** Build the core structure for non-linear idea development, including Vertebrae, time stamping, and tagging.
*   [ ] **Character Travel Trunk:** Develop the functionality for authors to propose and integrate characters into other novels.
*   [ ] **Casting Studio:** Create the platform for showcasing and discovering characters for collaboration.
*   [ ] **Novel Café:** Develop the space for showcasing novels and facilitating collaboration calls.

## Future Vision

*   [ ] Enhanced DID Management (WebAuthn, external wallets)
*   [ ] Advanced Collaborative Tools (version control, branching, co-editing, analytics)
*   [ ] Expanded Community & Discovery (advanced filtering, recommendations, writing groups, events)
*   [ ] Monetization & Licensing (DID-based frameworks)
*   [ ] Secure Messaging & Key Exchange (**DIDComm**): Introduce DIDComm v2 for end-to-end encrypted peer-to-peer channels.
