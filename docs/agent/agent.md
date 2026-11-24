# AI Agent Instructions & Context

This document serves as the primary source of truth for all operational directives, architectural decisions, and specific instructions given to the AI agent for the InterNovel project.

## Core Directives

1. **Strict "Docs-First" Workflow:**
   - In every case, the first step is ALWAYS to update the documentation when an architectural decision or feature scope is established.
   - The docs must reflect the current state of the app at all times.
   - *Execution Rule:* Before executing major new architectural shifts without prior user confirmation, tell the user which docs will be updated next, why, and with what, and confirm alignment.

2. **No MVP or Versioning Terminology:**
   - Avoid all references in all docs (and code) to anything related to "MVP", "MVP1", "Post-MVP1", or arbitrary prototype phasing. We are building the real application.

## Architectural & UI Rules

3. **Leptos Full-Stack + Shared Rust Core Architecture:**
   - The application UI and server run on **Leptos (`0.8.23`)** + **Axum (`0.8.9`)** (`leptos_axum`), backed by **`willow_gap_core`** (`willow25` + `iroh` + PIO + WTP + `e2e`) and **`internovel_auth`**.
   - Keep filesystem crates (`fjall`, `tokio::fs`) strictly gated behind Cargo’s `persistent-storage` feature so core crates compile cleanly to `wasm32-unknown-unknown`.

4. **Dual Dynamic, Non-Recursive Documentation Menus:**
   - The UI must include two navigation menus: a "Docs" submenu for development specs and a "User" submenu for user-facing guides.
   - The "Docs" menu dynamically lists `.md` files found at the root of `/docs`.
   - The "User" menu dynamically lists `.md` files found in `/docs/user/`.
   - *Constraint:* Both menus must be non-recursive and built using a single, reusable Leptos component. Subdirectories (like `/docs/agent/`) must remain hidden from the UI navigation menus.

5. **Micro-Typography Design System:**
   - The UI (especially the Markdown viewer) must use an ultra-compact, micro-typography scale (`10px–12px` base, tight margins, snug line heights, and `25%–300%` text scaling) to prevent a "zoomed-in" feel and minimize scrolling.

6. **NextGraph Wallet + `did:peer:4` Cryptography:**
   - Follow `docs/identity-spec.md`: wrap a random 32-byte `master_key` via `Aes256GcmSiv` under Argon2id keys (`wallet_id` AD) for Password, BIP-39 Mnemonic + PIN, and Visual Pazzle + PIN slots, paired with `did:peer:4` (`did-peer 0.7.5`) long/short-form identifiers and `ZeroizeOnDrop` memory hygiene.
