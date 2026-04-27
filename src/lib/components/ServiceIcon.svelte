<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2026 Auditore -->

<script lang="ts">
  import type { Service } from "../types/service";

  interface Props {
    service: Service;
    active: boolean;
    onSelect: () => void;
  }

  let { service, active, onSelect }: Props = $props();

  function colorFromName(name: string): string {
    let hash = 0;
    for (let i = 0; i < name.length; i++) {
      hash = (hash * 31 + name.charCodeAt(i)) | 0;
    }
    const hue = Math.abs(hash) % 360;
    return `hsl(${hue} 55% 45%)`;
  }

  let initial = $derived(service.name.trim().charAt(0).toUpperCase() || "?");
  let fallbackColor = $derived(colorFromName(service.name));
</script>

<button
  type="button"
  class="icon"
  class:active
  aria-label={service.name}
  aria-pressed={active}
  title={service.name}
  onclick={onSelect}
>
  {#if service.iconUrl}
    <img src={service.iconUrl} alt="" />
  {:else}
    <span class="letter" style:background={fallbackColor}>{initial}</span>
  {/if}
  <span class="badge" aria-hidden="true"></span>
</button>

<style>
  .icon {
    position: relative;
    width: 40px;
    height: 40px;
    border-radius: 10px;
    border: none;
    padding: 0;
    background: transparent;
    cursor: pointer;
    overflow: visible;
    transition:
      transform 120ms ease,
      filter 120ms ease;
  }

  .icon:hover {
    transform: scale(1.05);
  }

  .icon:not(.active) {
    filter: saturate(0.85) brightness(0.95);
  }

  .icon.active::before {
    content: "";
    position: absolute;
    left: -10px;
    top: 4px;
    bottom: 4px;
    width: 3px;
    border-radius: 2px;
    background: var(--accent);
  }

  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: inherit;
    display: block;
  }

  .letter {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
    border-radius: inherit;
    color: white;
    font-weight: 600;
    font-size: 1.05rem;
    user-select: none;
  }

  .badge {
    display: none;
    position: absolute;
    top: -2px;
    right: -2px;
    min-width: 8px;
    height: 8px;
    border-radius: 50%;
    background: #ff5252;
    border: 2px solid var(--sidebar);
  }
</style>
