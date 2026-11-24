# Mobile, Desktop Packaging & Ubuntu Local Deployment (`deployment-packaging`)

InterNovel targets **5 native builds (3 Desktop + 2 Mobile)** plus **Web/Wasm (`internovel.exosystems.net`)** from one shared Rust + Leptos workspace:

1. **Linux Desktop** (`x86_64` / `aarch64` — `.deb`, `.AppImage`, `.rpm`)
2. **macOS Desktop** (Universal `aarch64` + `x86_64` — `.dmg`, `.app`)
3. **Windows Desktop** (`x86_64` — `.msi`, NSIS `.exe`)
4. **iOS Mobile** (`aarch64-apple-ios` / `aarch64-apple-ios-sim` — `libinternovel_ffi.xcframework` + Swift 6/SwiftUI shell)
5. **Android Mobile** (`aarch64-linux-android` / `armv7-linux-androideabi` / `x86_64-linux-android` — JNI `.so` + Kotlin/Jetpack Compose shell)

---

## 1. Riot-Inspired Mobile Deployment (Builds 4 & 5: iOS & Android)

Authors write on phones offline, on cellular, and face-to-face in writing groups. InterNovel adopts `rabble/riot`’s native-core + thin-shell deployment model:

1. **Single Rust Core via `internovel_ffi` (`uniffi 0.32.2`):**
   * Exposes opaque `SessionHandle` IDs backed by `Arc<Mutex<SessionArbiter>>`.
   * Compiled with `panic = "unwind"` and `std::panic::catch_unwind` across every FFI entry point so malformed peer frames or storage errors quarantine a single session rather than crashing the mobile app.
2. **Native Mobile Shells:**
   * **iOS (`apps/ios` — Swift 6 / SwiftUI):** Links `libinternovel_ffi.xcframework` (compiled for `aarch64-apple-ios` & `aarch64-apple-ios-sim`), binds device keys to **Apple Keychain**, embeds the Leptos Wasm/WebView writing surface inside `WKWebView`, and drives **CoreBluetooth BLE + Bonjour LAN** offline nearby sync.
   * **Android (`apps/android` — Kotlin / Jetpack Compose):** Links JNI `.so` libraries (`arm64-v8a`, `armeabi-v7a`, `x86_64`) compiled via `cargo-ndk` with UniFFI Kotlin bindings, stores wrapping keys in **Android Keystore**, and drives **Android GATT BLE + mDNS LAN** sync.
3. **Wasm & Mobile Graph Guard (`scripts/check-wasm-graph.mjs`):**
   * CI validates that `cargo check -p willow_gap_core --target wasm32-unknown-unknown --no-default-features` and `cargo check -p internovel_auth --target wasm32-unknown-unknown` never pull `fjall`, `lsm-tree`, `async-fs`, or OS-only networking crates.
   * Argon2id parameters automatically select the mobile-safe profile (`KdfProfile::MobileConstrained`) on iOS so total process memory stays well below Apple’s 120 MiB budget.

---

## 2. Cross-Platform Rust Desktop Binary Packaging (Linux, macOS, Windows)

InterNovel provides two complementary desktop release targets from the same workspace using **`Tauri v2`** (matching NextGraph’s `app/shell/src-tauri/` desktop wrapper) and **`cargo-packager 0.11.8`**:

### Target A: Native Desktop Window App (`Tauri v2` + `internovel_app` + `willow_gap_core`)
Embeds the compiled Leptos frontend inside the OS native WebView (`WebKitGTK` on Linux, `WKWebView` on macOS, `WebView2` on Windows) with direct Rust IPC commands to `internovel_auth`, `willow_gap_core`, and `fjall` local storage—shipping as a single self-contained native binary.

### Target B: Standalone Local Server Binary (`internovel_server`)
Packages `internovel_server` (Axum + Leptos SSR + embedded static/Wasm assets) as a lightweight native executable that binds `127.0.0.1` on startup and opens the user’s system browser—ideal for headless relays, USB portable installs, or users preferring their own browser.

### Per-OS Package Formats (`packaging/CargoPackager.toml` & `.github/workflows/ci-and-release.yml`)

| Platform | Architectures | Generated Installer / Package Formats | Notes |
| :--- | :--- | :--- | :--- |
| **Linux** | `x86_64`, `aarch64` | **`.deb`** (Ubuntu/Debian), **`.rpm`** (Fedora/RHEL), **`.AppImage`** (universal), **`.tar.gz`** | Includes `packaging/internovel.desktop` freedesktop launcher, `hicolor` PNG/SVG icons, and optional user `systemd` relay unit. |
| **macOS** | `aarch64-apple-darwin`, `x86_64-apple-darwin` (Universal `.app`) | **`.dmg`** disk image & signed **`.app`** bundle | Built on `macos-latest` CI runner (`lipo` universal binary, code-signed & notarized via Apple Developer ID). |
| **Windows** | `x86_64-pc-windows-msvc`, `aarch64-pc-windows-msvc` | **`.msi`** (WiX Toolset) & **`.exe`** (NSIS setup wizard) | Registers `.ngw` / `.snk` (Willow Drop) file associations and installs Start Menu shortcuts. |

---

## 3. Local Testing on Ubuntu (22.04 / 24.04)

Run these commands after cloning `exonomyapp/internovel_g` on your Ubuntu laptop:

```bash
# 1. Install Rust toolchain & Wasm target
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup target add wasm32-unknown-unknown

# 2. Install system build deps (Ubuntu/Debian) for native GUI / crypto / OpenSSL
sudo apt-get update && sudo apt-get install -y \
  build-essential pkg-config libssl-dev \
  libwebkit2gtk-4.1-dev libayatana-appindicator3-dev librsvg2-dev patchelf

# 3. Install cargo-leptos (0.3.12+) & cargo-packager (0.11.8+)
cargo install --locked cargo-leptos
cargo install --locked cargo-packager

# 4. Run unit & integration tests across core crates
cargo test --workspace

# 5. Verify browser Wasm dependency isolation (Riot check)
cargo check -p willow_gap_core --target wasm32-unknown-unknown --no-default-features
cargo check -p internovel_auth --target wasm32-unknown-unknown

# 6. Run the Leptos web app locally with live reload (http://127.0.0.1:3000)
cargo leptos watch

# 7. Build a local Ubuntu .deb / AppImage package
cargo build --release -p internovel_server
cargo packager --release --config packaging/CargoPackager.toml
```

---

## 4. Web Production Deployment (`internovel.exosystems.net` on Tencent Lighthouse)

On the Tencent Lighthouse instance alongside `exosystems.net`:

1. Build release server + Wasm assets in GitHub Actions (`.github/workflows/ci-and-release.yml`):
   ```bash
   cargo leptos build --release
   ```
2. Run `target/release/internovel_server` on `127.0.0.1:3100` managed by `systemd` (`LEPTOS_SITE_ADDR=127.0.0.1:3100`, `LEPTOS_SITE_ROOT=target/site`).
3. Route `internovel.exosystems.net` via Caddy or Nginx:
   ```nginx
   server {
       server_name internovel.exosystems.net;
       location / {
           proxy_pass http://127.0.0.1:3100;
           proxy_http_version 1.1;
           proxy_set_header Upgrade $http_upgrade;
           proxy_set_header Connection "upgrade";
           proxy_set_header Host $host;
       }
   }
   ```
4. Expose an optional UDP QUIC port (`4433/udp` or `11204/udp`) in the Tencent Lighthouse firewall if co-locating an always-on Iroh + Willow storage relay (`willow_gap_core`).
