# Changelog

All notable changes to Husk will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed (architecture pivot in progress)

- Repo root is now a Cargo workspace; `src-tauri/` remains the default member
  (Husk shell, unchanged) and a new sibling member `husk-cef-spike/` hosts
  the experimental CEF backend. `cargo check` / `cargo build` from the root
  default to `src-tauri` so the existing flow is undisturbed.
- Added `scripts/setup-cef.sh`: clones `tauri-apps/cef-rs` to a tempdir and
  runs `export-cef-dir` to land CEF binaries in `$HOME/.local/share/cef`.
  Network-heavy (~1.2 GB extracted); idempotent unless `FORCE=1` is set.
- `husk-cef-spike` ships only a placeholder `main()` in this commit. CEF
  integration lands incrementally — next commit pulls the `cef` crate and
  opens a browser window.

### Added

- Initial Tauri 2 + Svelte 5 + TypeScript + Vite scaffold (Bun-managed).
- Apache 2.0 license, NOTICE, SPDX headers across source files.
- `Service` and `Settings` domain types in Rust with validation
  (HTTPS-only URLs, name length cap, optional icon scheme allowlist).
- Storage adapter on top of `tauri-plugin-store`, with seed-on-first-load
  for the three default services and default settings.
- Read-only IPC commands `list_services` and `get_settings`.
- Matching TypeScript types and typed `ipc.ts` wrapper.
- Static sidebar (52px) with per-service icons, active indicator, and
  ghost slots for "add service" and settings; fed by a Svelte
  `services` store that hydrates from `list_services` on app mount.
  Initial active service is restored from `settings.lastActiveServiceId`
  when present, else falls back to the first enabled service by
  `position`.
- Webview switching: one child webview per enabled service, created
  on startup as a child of the main window (offset by sidebar width).
  All hidden except the initial active one. New `activate_service`
  IPC command swaps visibility and focus. Window resize / scale
  changes relayout all service webviews automatically.
- Per-service session isolation: each service webview gets its own
  `data_directory` under `$APP_DATA/sessions/{service_id}` (used on
  Windows / Linux) and a `data_store_identifier` derived from the
  service UUID (used on macOS 14+ / iOS 17+, where WKWebView ignores
  `data_directory` and the only working primitive is
  `WKWebsiteDataStore.dataStoreForIdentifier`). Cookies, IndexedDB,
  and localStorage are now isolated across all three engines.
- Default user-agent override (Chrome 120 on Windows) applied to
  every service webview, with per-service `customUserAgent`
  overriding it when set.
- Init script injected at document start spoofs
  `navigator.webdriver` to `false` to avoid trivial bot detection.

### Changed

- Default capability now scoped to `webviews: ["main"]` so service
  webviews cannot reach the IPC handler.
- `tauri` crate enabled with `unstable` feature for child-webview
  enumeration via `Manager::webviews()` / `get_webview()`.
