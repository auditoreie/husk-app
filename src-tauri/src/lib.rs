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
        .manage(infra::webview::ServiceWebviews::default())
        .invoke_handler(tauri::generate_handler![
            commands::service::list_services,
            commands::settings::get_settings,
            commands::webview::activate_service,
        ])
        .setup(|app| {
            let handle = app.handle();
            let services = infra::storage::load_services(handle)?;
            let settings = infra::storage::load_settings(handle)?;

            let initial_active = settings.last_active_service_id.as_deref().and_then(|id| {
                services
                    .iter()
                    .find(|s| s.enabled && s.id == id)
                    .map(|s| s.id.as_str())
            });
            let initial_active = initial_active.or_else(|| {
                services
                    .iter()
                    .filter(|s| s.enabled)
                    .min_by_key(|s| s.position)
                    .map(|s| s.id.as_str())
            });

            infra::webview::setup_service_webviews(handle, &services, initial_active)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
