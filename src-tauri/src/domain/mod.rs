// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

pub mod service;
pub mod settings;

pub use service::{default_services, Service, ServiceInput, ServiceValidationError};
pub use settings::{default_settings, Settings, Theme};
