<script lang="ts">
  import SearchBar from "$lib/components/SearchBar.svelte";

  let searchQuery = $state("");
</script>

<div class="clipboard-page">
  <!-- 搜索栏 -->
  <SearchBar bind:value={searchQuery} />

  <!-- 过滤标签 -->
  <div class="filter-bar" data-no-drag>
    {#each ["全部", "文本", "图片", "已置顶"] as filter}
      <button class="filter-btn active">
        {filter}
      </button>
    {/each}
  </div>

  <!-- 空状态 -->
  <div class="empty-state">
    <div class="empty-icon">
      <svg width="64" height="64" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2"/>
        <rect x="9" y="3" width="6" height="4" rx="1"/>
        <path d="M9 12l2 2 4-4"/>
      </svg>
    </div>
    <p class="empty-title">暂无剪贴板历史</p>
    <p class="empty-desc">复制文字或图片后，内容将自动显示在此处</p>
  </div>
</div>

<style>
  .clipboard-page {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
  }

  .filter-bar {
    display: flex;
    gap: 4px;
    padding: 2px 0;
    -webkit-app-region: no-drag;
  }

  .filter-btn {
    padding: 4px 12px;
    border: 1px solid var(--border-default);
    border-radius: 14px;
    background: transparent;
    color: var(--text-secondary);
    font-family: var(--font-family);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .filter-btn:hover {
    background: var(--bg-overlay);
    color: var(--text-primary);
  }

  .filter-btn.active {
    background: var(--accent-soft);
    border-color: var(--accent);
    color: var(--accent);
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 80px 20px;
    color: var(--text-tertiary);
  }

  .empty-icon {
    margin-bottom: var(--spacing-lg);
    opacity: 0.4;
  }

  .empty-title {
    font-size: var(--font-size-lg);
    font-weight: 600;
    color: var(--text-secondary);
    margin-bottom: var(--spacing-sm);
  }

  .empty-desc {
    font-size: var(--font-size-sm);
    text-align: center;
    max-width: 280px;
    line-height: 1.5;
  }
</style>
