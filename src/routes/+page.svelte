<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { listen } from "@tauri-apps/api/event";
  import type { ClipboardEntry } from "$lib/types";
  import SearchBar from "$lib/components/SearchBar.svelte";

  let entries = $state<ClipboardEntry[]>([]);
  let searchQuery = $state("");
  let loading = $state(true);
  let filterType = $state<string>("all");

  let unlisten: (() => void) | null = null;

  async function loadHistory() {
    try {
      loading = true;
      const data = await invoke<ClipboardEntry[]>("get_clipboard_history");
      entries = data;
    } catch (e) {
      console.error("Failed to load clipboard history:", e);
    } finally {
      loading = false;
    }
  }

  onMount(async () => {
    await loadHistory();

    // 监听后端发来的剪贴板更新事件
    unlisten = await listen("clipboard-updated", () => {
      loadHistory();
    });
  });

  onDestroy(() => {
    if (unlisten) unlisten();
  });

  async function handleDelete(id: string, event: Event) {
    event.stopPropagation();
    try {
      await invoke("delete_clipboard_entry", { id });
      entries = entries.filter(e => e.id !== id);
    } catch (e) {
      console.error("Delete failed:", e);
    }
  }

  async function handlePin(id: string, event: Event) {
    event.stopPropagation();
    try {
      await invoke("toggle_pin_entry", { id });
      await loadHistory();
    } catch (e) {
      console.error("Pin failed:", e);
    }
  }

  async function handleCopy(entry: ClipboardEntry) {
    if (entry.content_type === "text" && entry.text_content) {
      try {
        await navigator.clipboard.writeText(entry.text_content);
        // 简单反馈（可以用 toast）
      } catch (e) {
        console.error("Copy failed:", e);
      }
    }
  }

  function timeAgo(dateStr: string): string {
    const now = Date.now();
    const date = new Date(dateStr).getTime();
    const diff = now - date;
    const minutes = Math.floor(diff / 60000);
    if (minutes < 1) return "刚刚";
    if (minutes < 60) return `${minutes} 分钟前`;
    const hours = Math.floor(minutes / 60);
    if (hours < 24) return `${hours} 小时前`;
    const days = Math.floor(hours / 24);
    if (days < 30) return `${days} 天前`;
    return new Date(dateStr).toLocaleDateString("zh-CN");
  }

  // 过滤
  let filteredEntries = $derived.by(() => {
    let result = entries;
    if (searchQuery) {
      const q = searchQuery.toLowerCase();
      result = result.filter(e => e.text_content?.toLowerCase().includes(q));
    }
    if (filterType === "text") {
      result = result.filter(e => e.content_type === "text");
    } else if (filterType === "image") {
      result = result.filter(e => e.content_type === "image");
    } else if (filterType === "pinned") {
      result = result.filter(e => e.is_pinned);
    }
    return result;
  });
</script>

<div class="clipboard-page">
  <SearchBar bind:value={searchQuery} />

  <div class="filter-bar" data-no-drag>
    <button class="filter-btn" class:active={filterType === "all"} onclick={() => filterType = "all"}>全部</button>
    <button class="filter-btn" class:active={filterType === "text"} onclick={() => filterType = "text"}>文本</button>
    <button class="filter-btn" class:active={filterType === "image"} onclick={() => filterType = "image"}>图片</button>
    <button class="filter-btn" class:active={filterType === "pinned"} onclick={() => filterType = "pinned"}>已置顶</button>
  </div>

  {#if loading}
    <div class="loading-state">
      <p>加载中...</p>
    </div>
  {:else if filteredEntries.length === 0}
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
  {:else}
    <div class="entries-list">
      {#each filteredEntries as entry (entry.id)}
        <div class="entry-card card fade-in" role="button" tabindex="0" onclick={() => handleCopy(entry)} onkeypress={(e) => e.key === 'Enter' && handleCopy(entry)}>
          <div class="card-header">
            <span class="entry-time">{timeAgo(entry.created_at)}</span>
            <div class="card-actions">
              <button
                class="action-btn"
                class:pinned={entry.is_pinned}
                onclick={(e) => handlePin(entry.id, e)}
                title={entry.is_pinned ? "取消置顶" : "置顶"}
              >
                📌
              </button>
              <button class="action-btn" onclick={(e) => handleDelete(entry.id, e)} title="删除">
                🗑️
              </button>
            </div>
          </div>
          <div class="card-content">
            {#if entry.content_type === "text"}
              <p class="text-preview">{entry.text_content || ""}</p>
            {:else}
              <div class="image-preview">
                {#if entry.image_thumb}
                  <img src={entry.image_thumb} alt="剪贴板图片" />
                {:else}
                  <span>[图片]</span>
                {/if}
              </div>
            {/if}
          </div>
          {#if entry.is_pinned}
            <div class="pinned-badge">已置顶</div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .clipboard-page {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    height: 100%;
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

  .entries-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    overflow-y: auto;
    flex: 1;
    padding-bottom: var(--spacing-md);
  }

  .entry-card {
    display: flex;
    flex-direction: column;
    gap: 8px;
    width: 100%;
    text-align: left;
    cursor: pointer;
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    padding: var(--spacing-md);
    background: var(--bg-card);
    transition: all var(--transition-fast);
    position: relative;
    font-family: inherit;
  }

  .entry-card:hover {
    background: var(--bg-card-hover);
    border-color: var(--border-hover);
  }

  .entry-card:active {
    transform: scale(0.99);
  }

  .card-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .entry-time {
    font-size: var(--font-size-xs);
    color: var(--text-tertiary);
  }

  .card-actions {
    display: flex;
    gap: 4px;
  }

  .action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border: none;
    border-radius: var(--radius-sm);
    background: transparent;
    font-size: 14px;
    cursor: pointer;
    opacity: 0;
    transition: all var(--transition-fast);
  }

  .entry-card:hover .action-btn {
    opacity: 0.6;
  }

  .action-btn:hover {
    opacity: 1 !important;
    background: var(--bg-overlay);
  }

  .action-btn.pinned {
    opacity: 1;
  }

  .card-content {
    overflow: hidden;
  }

  .text-preview {
    font-size: var(--font-size-md);
    line-height: 1.5;
    color: var(--text-primary);
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    word-break: break-all;
    white-space: pre-wrap;
  }

  .image-preview {
    width: 100%;
    max-height: 120px;
    overflow: hidden;
    border-radius: var(--radius-sm);
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-secondary);
    color: var(--text-tertiary);
  }

  .image-preview img {
    max-width: 100%;
    max-height: 120px;
    object-fit: contain;
  }

  .pinned-badge {
    position: absolute;
    top: 8px;
    right: 8px;
    font-size: 10px;
    padding: 2px 6px;
    background: var(--accent-soft);
    color: var(--accent);
    border-radius: 8px;
  }

  .empty-state, .loading-state {
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
