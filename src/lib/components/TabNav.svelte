<script lang="ts">
  let { activeTab }: { activeTab: "clipboard" | "vault" | "settings" } = $props();

  const tabs = [
    { id: "clipboard", label: "剪贴板", icon: "📋" },
    { id: "vault", label: "密码本", icon: "🔒" },
    { id: "settings", label: "设置", icon: "⚙️" },
  ] as const;
</script>

<nav class="tab-nav" data-no-drag>
  {#each tabs as tab}
    <button
      class="tab-btn"
      class:active={activeTab === tab.id}
      onclick={() => activeTab = tab.id}
    >
      <span class="tab-icon">{tab.icon}</span>
      <span class="tab-label">{tab.label}</span>
    </button>
  {/each}
</nav>

<style>
  .tab-nav {
    display: flex;
    gap: 2px;
    padding: 0 var(--spacing-lg);
    border-bottom: 1px solid var(--border-default);
    -webkit-app-region: no-drag;
  }

  .tab-btn {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-family: var(--font-family);
    font-size: var(--font-size-sm);
    cursor: pointer;
    position: relative;
    transition: color var(--transition-fast);
    -webkit-app-region: no-drag;
  }

  .tab-btn:hover {
    color: var(--text-primary);
    background: var(--bg-overlay);
  }

  .tab-btn.active {
    color: var(--accent);
  }

  .tab-btn.active::after {
    content: '';
    position: absolute;
    bottom: -1px;
    left: 8px;
    right: 8px;
    height: 2px;
    background: var(--accent);
    border-radius: 1px 1px 0 0;
  }

  .tab-icon {
    font-size: 14px;
  }

  .tab-label {
    font-weight: 500;
  }
</style>
