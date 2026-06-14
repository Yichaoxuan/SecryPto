<script lang="ts">
  import "../app.css";
  import TabNav from "$lib/components/TabNav.svelte";
  import VaultPage from "./vault/+page.svelte";
  import SettingsPage from "./settings/+page.svelte";

  let { children }: { children: import("svelte").Snippet } = $props();
  let activeTab = $state<"clipboard" | "vault" | "settings">("clipboard");
</script>

<div class="app-container">
  <header class="title-bar" data-no-drag>
    <span class="app-title">Secrypto</span>
  </header>

  <TabNav bind:activeTab />

  <main class="main-content">
    {#if activeTab === "clipboard"}
      {@render children()}
    {:else if activeTab === "vault"}
      <VaultPage />
    {:else if activeTab === "settings"}
      <SettingsPage />
    {/if}
  </main>
</div>

<style>
  .app-container {
    display: flex;
    flex-direction: column;
    height: 100vh;
    overflow: hidden;
    background: var(--mica-tint);
  }

  .title-bar {
    display: flex;
    align-items: center;
    padding: var(--spacing-sm) var(--spacing-lg);
    height: 36px;
    -webkit-app-region: drag;
  }

  .app-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-secondary);
    letter-spacing: 0.5px;
  }

  .main-content {
    flex: 1;
    overflow-y: auto;
    padding: var(--spacing-sm) var(--spacing-lg) var(--spacing-lg);
  }
</style>
