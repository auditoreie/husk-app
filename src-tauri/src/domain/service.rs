// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;
use uuid::Uuid;

const NAME_MAX_LEN: usize = 64;
const USER_AGENT_MAX_LEN: usize = 512;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Service {
    pub id: String,
    pub name: String,
    pub url: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    pub enabled: bool,
    pub position: u32,
    pub notifications_muted: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInput {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub icon_url: Option<String>,
    #[serde(default)]
    pub custom_user_agent: Option<String>,
}

#[derive(Debug, Error)]
pub enum ServiceValidationError {
    #[error("name must not be empty")]
    NameEmpty,
    #[error("name exceeds {NAME_MAX_LEN} characters")]
    NameTooLong,
    #[error("url is not a valid URL: {0}")]
    UrlMalformed(String),
    #[error("url scheme must be https (got `{0}`)")]
    UrlInsecure(String),
    #[error("url must include a host")]
    UrlNoHost,
    #[error("custom user agent exceeds {USER_AGENT_MAX_LEN} characters")]
    UserAgentTooLong,
    #[error("icon url scheme must be https or data: (got `{0}`)")]
    IconUrlInsecure(String),
}

impl ServiceInput {
    pub fn validate(&self) -> Result<(), ServiceValidationError> {
        let trimmed_name = self.name.trim();
        if trimmed_name.is_empty() {
            return Err(ServiceValidationError::NameEmpty);
        }
        if trimmed_name.chars().count() > NAME_MAX_LEN {
            return Err(ServiceValidationError::NameTooLong);
        }

        let parsed = Url::parse(&self.url)
            .map_err(|e| ServiceValidationError::UrlMalformed(e.to_string()))?;
        if parsed.scheme() != "https" {
            return Err(ServiceValidationError::UrlInsecure(
                parsed.scheme().to_string(),
            ));
        }
        if parsed.host().is_none() {
            return Err(ServiceValidationError::UrlNoHost);
        }

        if let Some(icon) = &self.icon_url {
            let icon_parsed = Url::parse(icon)
                .map_err(|e| ServiceValidationError::UrlMalformed(e.to_string()))?;
            let scheme = icon_parsed.scheme();
            if scheme != "https" && scheme != "data" {
                return Err(ServiceValidationError::IconUrlInsecure(scheme.to_string()));
            }
        }

        if let Some(ua) = &self.custom_user_agent {
            if ua.chars().count() > USER_AGENT_MAX_LEN {
                return Err(ServiceValidationError::UserAgentTooLong);
            }
        }

        Ok(())
    }

    pub fn into_service(self, position: u32) -> Result<Service, ServiceValidationError> {
        self.validate()?;
        Ok(Service {
            id: Uuid::new_v4().to_string(),
            name: self.name.trim().to_string(),
            url: self.url,
            icon_url: self.icon_url,
            enabled: true,
            position,
            notifications_muted: false,
            custom_user_agent: self.custom_user_agent,
        })
    }
}

pub fn default_services() -> Vec<Service> {
    [
        ("WhatsApp Personal", "https://web.whatsapp.com"),
        ("WhatsApp Business", "https://web.whatsapp.com"),
        ("Gmail", "https://mail.google.com"),
    ]
    .into_iter()
    .enumerate()
    .map(|(idx, (name, url))| Service {
        id: Uuid::new_v4().to_string(),
        name: name.to_string(),
        url: url.to_string(),
        icon_url: None,
        enabled: true,
        position: u32::try_from(idx).unwrap_or(0),
        notifications_muted: false,
        custom_user_agent: None,
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input(name: &str, url: &str) -> ServiceInput {
        ServiceInput {
            name: name.to_string(),
            url: url.to_string(),
            icon_url: None,
            custom_user_agent: None,
        }
    }

    #[test]
    fn accepts_https_url() {
        assert!(input("Gmail", "https://mail.google.com").validate().is_ok());
    }

    #[test]
    fn rejects_http() {
        let err = input("X", "http://example.com").validate().unwrap_err();
        assert!(matches!(err, ServiceValidationError::UrlInsecure(_)));
    }

    #[test]
    fn rejects_javascript_url() {
        let err = input("X", "javascript:alert(1)").validate().unwrap_err();
        assert!(matches!(err, ServiceValidationError::UrlInsecure(_)));
    }

    #[test]
    fn rejects_file_url() {
        let err = input("X", "file:///etc/passwd").validate().unwrap_err();
        assert!(matches!(err, ServiceValidationError::UrlInsecure(_)));
    }

    #[test]
    fn rejects_empty_name() {
        let err = input("   ", "https://example.com").validate().unwrap_err();
        assert!(matches!(err, ServiceValidationError::NameEmpty));
    }

    #[test]
    fn rejects_malformed_url() {
        let err = input("X", "not a url").validate().unwrap_err();
        assert!(matches!(err, ServiceValidationError::UrlMalformed(_)));
    }

    #[test]
    fn defaults_have_three_services() {
        let services = default_services();
        assert_eq!(services.len(), 3);
        assert_eq!(services[0].position, 0);
        assert_eq!(services[2].position, 2);
        assert!(services.iter().all(|s| s.enabled));
        assert!(services.iter().all(|s| s.url.starts_with("https://")));
    }

    #[test]
    fn into_service_assigns_uuid_and_position() {
        let svc = input("My Service", "https://example.com")
            .into_service(7)
            .unwrap();
        assert_eq!(svc.position, 7);
        assert!(svc.enabled);
        assert!(!svc.notifications_muted);
        assert!(Uuid::parse_str(&svc.id).is_ok());
    }
}
