// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use tauri::{AppHandle, Runtime};

use crate::domain::Settings;
use crate::infra::storage;

#[tauri::command]
pub fn get_settings<R: Runtime>(app: AppHandle<R>) -> Result<Settings, String> {
    storage::load_settings(&app).map_err(|e| e.to_string())
}
