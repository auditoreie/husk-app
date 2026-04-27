// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

export interface Service {
  id: string;
  name: string;
  url: string;
  iconUrl?: string;
  enabled: boolean;
  position: number;
  notificationsMuted: boolean;
  customUserAgent?: string;
}

export interface ServiceInput {
  name: string;
  url: string;
  iconUrl?: string;
  customUserAgent?: string;
}
