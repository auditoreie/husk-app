// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(default)]
    pub last_active_service_id: Option<String>,
    #[serde(default)]
    pub theme: Theme,
}

pub fn default_settings() -> Settings {
    Settings::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_to_system_theme_and_no_active_service() {
        let s = default_settings();
        assert_eq!(s.theme, Theme::System);
        assert!(s.last_active_service_id.is_none());
    }

    #[test]
    fn theme_serializes_lowercase() {
        let json = serde_json::to_string(&Theme::Dark).unwrap();
        assert_eq!(json, "\"dark\"");
    }
}
