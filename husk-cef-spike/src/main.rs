// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

//! Husk CEF spike — experimental binary that will replace the Tauri/wry
//! webview pipeline with a Chromium Embedded Framework backend on macOS.
//!
//! This commit ships only the build skeleton (workspace member, setup script,
//! placeholder `main`). The actual CEF integration lands in the next sub-commit
//! once the `setup-cef.sh` flow has been validated end-to-end.

fn main() {
    eprintln!("husk-cef-spike skeleton — CEF integration pending in next commit.");
    eprintln!("Run ./scripts/setup-cef.sh first, then track progress in CHANGELOG.md.");
}
