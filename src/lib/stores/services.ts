// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

import { writable } from "svelte/store";
import { listServices } from "../ipc";
import type { Service } from "../types/service";

export const services = writable<Service[]>([]);

export async function loadServices(): Promise<Service[]> {
  const list = await listServices();
  services.set(list);
  return list;
}
