// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use std::path::PathBuf;
use std::sync::Mutex;

use tauri::{
    AppHandle, LogicalPosition, LogicalSize, Manager, Runtime, WebviewBuilder, WebviewUrl,
    WindowEvent,
};
use thiserror::Error;

use crate::domain::Service;

pub const SHELL_WEBVIEW_LABEL: &str = "main";
const SIDEBAR_WIDTH: f64 = 52.0;
const SERVICE_LABEL_PREFIX: &str = "svc-";
const SESSIONS_DIR: &str = "sessions";

const DEFAULT_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 \
                                  (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";

const INIT_SCRIPT: &str = r#"
;(function() {
  try {
    Object.defineProperty(navigator, 'webdriver', {
      get: () => false,
      configurable: true,
    });
  } catch (_) {}
})();
"#;

#[derive(Default)]
pub struct ServiceWebviews {
    labels: Mutex<Vec<String>>,
}

impl ServiceWebviews {
    fn snapshot(&self) -> Vec<String> {
        self.labels
            .lock()
            .map(|guard| guard.clone())
            .unwrap_or_default()
    }

    fn set(&self, labels: Vec<String>) {
        if let Ok(mut guard) = self.labels.lock() {
            *guard = labels;
        }
    }
}

#[derive(Debug, Error)]
pub enum WebviewError {
    #[error("main window `{0}` not available")]
    MainWindowMissing(String),
    #[error("invalid service url `{url}`: {source}")]
    InvalidUrl {
        url: String,
        #[source]
        source: url::ParseError,
    },
    #[error("no webview found for service `{0}`")]
    UnknownService(String),
    #[error("could not prepare session directory `{path}`: {source}")]
    SessionDir {
        path: String,
        #[source]
        source: std::io::Error,
    },
    #[error("tauri runtime error: {0}")]
    Runtime(String),
}

impl From<tauri::Error> for WebviewError {
    fn from(e: tauri::Error) -> Self {
        Self::Runtime(e.to_string())
    }
}

fn label_for(id: &str) -> String {
    format!("{SERVICE_LABEL_PREFIX}{id}")
}

fn session_dir<R: Runtime>(app: &AppHandle<R>, service_id: &str) -> Result<PathBuf, WebviewError> {
    let base = app
        .path()
        .app_data_dir()
        .map_err(|e| WebviewError::Runtime(e.to_string()))?;
    let dir = base.join(SESSIONS_DIR).join(service_id);
    std::fs::create_dir_all(&dir).map_err(|e| WebviewError::SessionDir {
        path: dir.display().to_string(),
        source: e,
    })?;
    Ok(dir)
}

pub fn setup_service_webviews<R: Runtime>(
    app: &AppHandle<R>,
    services: &[Service],
    initial_active: Option<&str>,
) -> Result<(), WebviewError> {
    let shell_window = app
        .get_window(SHELL_WEBVIEW_LABEL)
        .ok_or_else(|| WebviewError::MainWindowMissing(SHELL_WEBVIEW_LABEL.to_string()))?;

    let (content_width, content_height) = compute_content_size(app)?;

    let mut created_labels = Vec::new();
    for service in services.iter().filter(|s| s.enabled) {
        let parsed = service.url.parse().map_err(|e| WebviewError::InvalidUrl {
            url: service.url.clone(),
            source: e,
        })?;
        let label = label_for(&service.id);
        let session = session_dir(app, &service.id)?;
        let user_agent = service
            .custom_user_agent
            .as_deref()
            .unwrap_or(DEFAULT_USER_AGENT);
        let identifier = uuid::Uuid::parse_str(&service.id)
            .map(|u| *u.as_bytes())
            .map_err(|e| WebviewError::Runtime(format!("invalid service uuid: {e}")))?;

        let builder = WebviewBuilder::<R>::new(&label, WebviewUrl::External(parsed))
            .data_directory(session)
            .data_store_identifier(identifier)
            .user_agent(user_agent)
            .initialization_script(INIT_SCRIPT);
        let webview = shell_window.add_child(
            builder,
            LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
            LogicalSize::new(content_width, content_height),
        )?;

        let is_active = initial_active == Some(service.id.as_str());
        if !is_active {
            webview.hide()?;
        } else {
            let _ = webview.set_focus();
        }

        created_labels.push(label);
    }

    app.state::<ServiceWebviews>().set(created_labels);

    let app_handle = app.clone();
    shell_window.on_window_event(move |event| {
        if matches!(
            event,
            WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. }
        ) {
            let _ = relayout(&app_handle);
        }
    });

    Ok(())
}

pub fn activate_service<R: Runtime>(
    app: &AppHandle<R>,
    service_id: &str,
) -> Result<(), WebviewError> {
    let target_label = label_for(service_id);
    let labels = app.state::<ServiceWebviews>().snapshot();

    let mut found = false;
    for label in labels {
        let Some(webview) = app.get_webview(&label) else {
            continue;
        };
        if label == target_label {
            webview.show()?;
            let _ = webview.set_focus();
            found = true;
        } else {
            webview.hide()?;
        }
    }

    if !found {
        return Err(WebviewError::UnknownService(service_id.to_string()));
    }
    Ok(())
}

fn compute_content_size<R: Runtime>(app: &AppHandle<R>) -> Result<(f64, f64), WebviewError> {
    let window = app
        .get_webview_window(SHELL_WEBVIEW_LABEL)
        .ok_or_else(|| WebviewError::MainWindowMissing(SHELL_WEBVIEW_LABEL.to_string()))?;

    let physical = window.inner_size()?;
    let scale = window.scale_factor()?;
    let logical = physical.to_logical::<f64>(scale);

    let width = (logical.width - SIDEBAR_WIDTH).max(0.0);
    let height = logical.height.max(0.0);
    Ok((width, height))
}

fn relayout<R: Runtime>(app: &AppHandle<R>) -> Result<(), WebviewError> {
    let (content_width, content_height) = compute_content_size(app)?;
    let labels = app.state::<ServiceWebviews>().snapshot();
    for label in labels {
        let Some(webview) = app.get_webview(&label) else {
            continue;
        };
        webview.set_position(LogicalPosition::new(SIDEBAR_WIDTH, 0.0))?;
        webview.set_size(LogicalSize::new(content_width, content_height))?;
    }
    Ok(())
}
