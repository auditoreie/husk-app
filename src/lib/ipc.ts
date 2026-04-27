// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

import { invoke } from "@tauri-apps/api/core";
import type { Service } from "./types/service";
import type { Settings } from "./types/settings";

export async function listServices(): Promise<Service[]> {
  return invoke<Service[]>("list_services");
}

export async function getSettings(): Promise<Settings> {
  return invoke<Settings>("get_settings");
}
