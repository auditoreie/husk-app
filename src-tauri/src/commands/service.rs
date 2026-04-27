// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use tauri::{AppHandle, Runtime};

use crate::domain::Service;
use crate::infra::storage;

#[tauri::command]
pub fn list_services<R: Runtime>(app: AppHandle<R>) -> Result<Vec<Service>, String> {
    storage::load_services(&app).map_err(|e| e.to_string())
}
