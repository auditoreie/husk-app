<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2026 Auditore -->

<script lang="ts">
  import { onMount } from "svelte";
  import Sidebar from "./lib/components/Sidebar.svelte";
  import { activateService, getSettings } from "./lib/ipc";
  import { activeServiceId } from "./lib/stores/activeService";
  import { loadServices, services } from "./lib/stores/services";

  let loadError = $state<string | null>(null);
  let loading = $state(true);

  onMount(async () => {
    try {
      const list = await loadServices();
      const settings = await getSettings();

      const enabledSorted = list
        .filter((s) => s.enabled)
        .slice()
        .sort((a, b) => a.position - b.position);

      const fromSettings = settings.lastActiveServiceId
        ? enabledSorted.find((s) => s.id === settings.lastActiveServiceId)?.id
        : undefined;

      const initial = fromSettings ?? enabledSorted[0]?.id ?? null;
      activeServiceId.set(initial);
    } catch (err) {
      loadError = err instanceof Error ? err.message : String(err);
    } finally {
      loading = false;
    }
  });

  async function selectService(id: string) {
    activeServiceId.set(id);
    try {
      await activateService(id);
    } catch (err) {
      console.error("activate_service failed", err);
    }
  }

  let active = $derived(
    $services.find((s) => s.id === $activeServiceId) ?? null,
  );
</script>

<div class="shell">
  <Sidebar
    services={$services}
    activeId={$activeServiceId}
    onSelect={selectService}
  />
  <main class="content">
    {#if loadError}
      <div class="error">
        <strong>Failed to load services</strong>
        <code>{loadError}</code>
      </div>
    {:else if loading}
      <p class="hint">Loading…</p>
    {:else if active}
      <header class="active" aria-hidden="true">
        <h1>{active.name}</h1>
        <code>{active.url}</code>
      </header>
    {:else}
      <div class="empty">
        <h1>No services yet</h1>
        <p>Add a service to get started.</p>
      </div>
    {/if}
  </main>
</div>

<style>
  .shell {
    display: flex;
    height: 100vh;
    width: 100vw;
    background: var(--bg);
    color: var(--fg);
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 2rem;
    gap: 0.5rem;
  }

  .active {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.4rem;
  }

  h1 {
    margin: 0;
    font-size: 2rem;
    letter-spacing: 0.01em;
  }

  code {
    font-family:
      ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, monospace;
    font-size: 0.9rem;
    opacity: 0.7;
  }

  .hint {
    margin: 0;
    opacity: 0.55;
    font-size: 0.85rem;
  }

  .empty {
    text-align: center;
    opacity: 0.65;
  }

  .error {
    max-width: 480px;
    padding: 1rem 1.25rem;
    border: 1px solid #ff5252;
    border-radius: 8px;
    background: rgba(255, 82, 82, 0.08);
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .error strong {
    color: #ff7777;
  }
</style>
