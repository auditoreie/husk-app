# Changelog

All notable changes to Husk will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial Tauri 2 + Svelte 5 + TypeScript + Vite scaffold (Bun-managed).
- Apache 2.0 license, NOTICE, SPDX headers across source files.
- `Service` and `Settings` domain types in Rust with validation
  (HTTPS-only URLs, name length cap, optional icon scheme allowlist).
- Storage adapter on top of `tauri-plugin-store`, with seed-on-first-load
  for the three default services and default settings.
- Read-only IPC commands `list_services` and `get_settings`.
- Matching TypeScript types and typed `ipc.ts` wrapper.
