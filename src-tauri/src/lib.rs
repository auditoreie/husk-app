// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

#![warn(clippy::all)]

pub mod commands;
pub mod domain;
pub mod infra;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::service::list_services,
            commands::settings::get_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
