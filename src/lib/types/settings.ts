// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

export type Theme = "light" | "dark" | "system";

export interface Settings {
  lastActiveServiceId: string | null;
  theme: Theme;
}
