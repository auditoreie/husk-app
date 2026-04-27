<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2026 Auditore -->

<script lang="ts">
  import type { Service } from "../types/service";
  import ServiceIcon from "./ServiceIcon.svelte";

  interface Props {
    services: Service[];
    activeId: string | null;
    onSelect: (id: string) => void;
    onAddService?: () => void;
    onOpenSettings?: () => void;
  }

  let { services, activeId, onSelect, onAddService, onOpenSettings }: Props =
    $props();

  let visible = $derived(
    services
      .filter((s) => s.enabled)
      .slice()
      .sort((a, b) => a.position - b.position),
  );
</script>

<aside class="sidebar" aria-label="Services">
  <ul class="list">
    {#each visible as svc (svc.id)}
      <li>
        <ServiceIcon
          service={svc}
          active={svc.id === activeId}
          onSelect={() => onSelect(svc.id)}
        />
      </li>
    {/each}
    <li class="ghost">
      <button
        type="button"
        class="ghost-btn"
        aria-label="Add service"
        title="Add service"
        onclick={() => onAddService?.()}
      >
        +
      </button>
    </li>
  </ul>
  <button
    type="button"
    class="settings"
    aria-label="Open settings"
    title="Settings"
    onclick={() => onOpenSettings?.()}
  >
    ⚙
  </button>
</aside>

<style>
  .sidebar {
    width: 52px;
    flex-shrink: 0;
    height: 100%;
    background: var(--sidebar);
    border-right: 1px solid var(--border);
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 10px 0;
    gap: 10px;
  }

  .list {
    list-style: none;
    margin: 0;
    padding: 0;
    display: flex;
    flex-direction: column;
    gap: 10px;
    flex: 1;
    width: 100%;
    align-items: center;
  }

  .ghost-btn {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    border: 1px dashed var(--border);
    background: transparent;
    color: var(--fg);
    font-size: 1.25rem;
    line-height: 1;
    cursor: pointer;
    opacity: 0.55;
    transition: opacity 120ms ease;
  }

  .ghost-btn:hover {
    opacity: 1;
  }

  .settings {
    width: 40px;
    height: 40px;
    border-radius: 10px;
    border: none;
    background: transparent;
    color: var(--fg);
    font-size: 1.2rem;
    cursor: pointer;
    opacity: 0.6;
    transition: opacity 120ms ease;
  }

  .settings:hover {
    opacity: 1;
  }
</style>
