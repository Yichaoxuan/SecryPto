<script lang="ts">
  let { value }: { value: string } = $props();
</script>

<div class="search-container" data-no-drag>
  <svg class="search-icon" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
    <circle cx="11" cy="11" r="8"/>
    <path d="M21 21l-4.35-4.35"/>
  </svg>
  <input
    type="text"
    class="search-input"
    placeholder="搜索剪贴板历史..."
    bind:value
  />
  {#if value}
    <button class="clear-btn" onclick={() => value = ""}>
      <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
        <path d="M18 6L6 18M6 6l12 12"/>
      </svg>
    </button>
  {/if}
</div>

<style>
  .search-container {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 12px;
    background: var(--bg-secondary);
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    transition: all var(--transition-fast);
    -webkit-app-region: no-drag;
  }

  .search-container:focus-within {
    border-color: var(--accent);
    box-shadow: 0 0 0 1px var(--accent);
    background: var(--bg-input);
  }

  .search-icon {
    color: var(--text-tertiary);
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-family: var(--font-family);
    font-size: var(--font-size-md);
    outline: none;
  }

  .search-input::placeholder {
    color: var(--text-tertiary);
  }

  .clear-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 20px;
    height: 20px;
    border: none;
    border-radius: 50%;
    background: transparent;
    color: var(--text-tertiary);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .clear-btn:hover {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }
</style>
