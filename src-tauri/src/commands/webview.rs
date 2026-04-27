// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use tauri::{AppHandle, Runtime};

use crate::infra::webview;

#[tauri::command]
pub fn activate_service<R: Runtime>(app: AppHandle<R>, id: String) -> Result<(), String> {
    webview::activate_service(&app, &id).map_err(|e| e.to_string())
}
