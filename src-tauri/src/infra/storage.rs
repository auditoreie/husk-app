// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use serde_json::{from_value, to_value};
use tauri::{AppHandle, Runtime};
use tauri_plugin_store::StoreExt;
use thiserror::Error;

use crate::domain::{default_services, default_settings, Service, Settings};

const SERVICES_FILE: &str = "services.json";
const SERVICES_KEY: &str = "services";

const SETTINGS_FILE: &str = "settings.json";
const SETTINGS_KEY: &str = "settings";

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("store backend error: {0}")]
    Backend(String),
    #[error("failed to decode persisted state: {0}")]
    Decode(String),
    #[error("failed to encode state for persistence: {0}")]
    Encode(String),
}

pub fn load_services<R: Runtime>(app: &AppHandle<R>) -> Result<Vec<Service>, StorageError> {
    let store = app
        .store(SERVICES_FILE)
        .map_err(|e| StorageError::Backend(e.to_string()))?;

    match store.get(SERVICES_KEY) {
        Some(value) => from_value(value).map_err(|e| StorageError::Decode(e.to_string())),
        None => {
            let defaults = default_services();
            persist_services(app, &defaults)?;
            Ok(defaults)
        }
    }
}

pub fn save_services<R: Runtime>(
    app: &AppHandle<R>,
    services: &[Service],
) -> Result<(), StorageError> {
    persist_services(app, services)
}

fn persist_services<R: Runtime>(
    app: &AppHandle<R>,
    services: &[Service],
) -> Result<(), StorageError> {
    let store = app
        .store(SERVICES_FILE)
        .map_err(|e| StorageError::Backend(e.to_string()))?;
    let value = to_value(services).map_err(|e| StorageError::Encode(e.to_string()))?;
    store.set(SERVICES_KEY, value);
    store
        .save()
        .map_err(|e| StorageError::Backend(e.to_string()))
}

pub fn load_settings<R: Runtime>(app: &AppHandle<R>) -> Result<Settings, StorageError> {
    let store = app
        .store(SETTINGS_FILE)
        .map_err(|e| StorageError::Backend(e.to_string()))?;

    match store.get(SETTINGS_KEY) {
        Some(value) => from_value(value).map_err(|e| StorageError::Decode(e.to_string())),
        None => {
            let defaults = default_settings();
            persist_settings(app, &defaults)?;
            Ok(defaults)
        }
    }
}

pub fn save_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &Settings,
) -> Result<(), StorageError> {
    persist_settings(app, settings)
}

fn persist_settings<R: Runtime>(
    app: &AppHandle<R>,
    settings: &Settings,
) -> Result<(), StorageError> {
    let store = app
        .store(SETTINGS_FILE)
        .map_err(|e| StorageError::Backend(e.to_string()))?;
    let value = to_value(settings).map_err(|e| StorageError::Encode(e.to_string()))?;
    store.set(SETTINGS_KEY, value);
    store
        .save()
        .map_err(|e| StorageError::Backend(e.to_string()))
}
