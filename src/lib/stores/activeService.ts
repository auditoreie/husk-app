// SPDX-License-Identifier: Apache-2.0
// Copyright 2026 Auditore

import { writable } from "svelte/store";

export const activeServiceId = writable<string | null>(null);
