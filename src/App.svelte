<!-- SPDX-License-Identifier: Apache-2.0 -->
<!-- Copyright 2026 Auditore -->

<script lang="ts">
  const tagline = "Lightweight service aggregator. No Electron, no bloat.";
</script>

<main>
  <h1>Husk</h1>
  <p>{tagline}</p>
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100vh;
    gap: 0.5rem;
    color: var(--fg);
    background: var(--bg);
  }

  h1 {
    margin: 0;
    font-size: 2.5rem;
    letter-spacing: 0.02em;
  }

  p {
    margin: 0;
    opacity: 0.7;
    font-size: 0.95rem;
  }
</style>
