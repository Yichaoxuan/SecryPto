<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  // 状态
  let isInitialized = $state(false);
  let isUnlocked = $state(false);
  let masterPassword = $state("");
  let confirmPassword = $state("");
  let setupMode = $state(false);
  let errorMsg = $state("");
  let loading = $state(false);

  // 条目列表
  let entries = $state<import("$lib/types").PasswordEntry[]>([]);
  let searchQuery = $state("");

  // 表单
  let showForm = $state(false);
  let editingId = $state<string | null>(null);
  let formName = $state("");
  let formUrl = $state("");
  let formUsername = $state("");
  let formPassword = $state("");
  let formNotes = $state("");
  let formTags = $state("");
  let showPassword = $state(false);

  // 密码生成器
  let showGenerator = $state(false);
  let genLength = $state(16);
  let genUpper = $state(true);
  let genLower = $state(true);
  let genDigits = $state(true);
  let genSymbols = $state(false);
  let genResult = $state("");

  onMount(async () => {
    try {
      const initialized = await invoke<boolean>("vault_is_initialized");
      isInitialized = initialized;
      if (initialized) {
        const unlocked = await invoke<boolean>("vault_is_unlocked");
        isUnlocked = unlocked;
        if (unlocked) await loadEntries();
      } else {
        setupMode = true;
      }
    } catch (e) {
      console.error(e);
    }
  });

  async function handleUnlock() {
    if (!masterPassword) return;
    loading = true;
    errorMsg = "";
    try {
      const ok = await invoke<boolean>("vault_unlock", { masterPassword });
      if (ok) {
        isUnlocked = true;
        masterPassword = "";
        await loadEntries();
      } else {
        errorMsg = "主密码错误";
      }
    } catch (e) {
      errorMsg = String(e);
    } finally {
      loading = false;
    }
  }

  async function handleSetup() {
    if (!masterPassword || masterPassword.length < 4) {
      errorMsg = "主密码至少 4 位";
      return;
    }
    if (masterPassword !== confirmPassword) {
      errorMsg = "两次密码不一致";
      return;
    }
    loading = true;
    errorMsg = "";
    try {
      await invoke("vault_initialize", { masterPassword });
      isInitialized = true;
      isUnlocked = true;
      setupMode = false;
      masterPassword = "";
      confirmPassword = "";
      await loadEntries();
    } catch (e) {
      errorMsg = String(e);
    } finally {
      loading = false;
    }
  }

  async function lock() {
    await invoke("vault_lock");
    isUnlocked = false;
    entries = [];
  }

  async function loadEntries() {
    try {
      entries = await invoke<import("$lib/types").PasswordEntry[]>("vault_get_entries");
    } catch (e) {
      console.error(e);
    }
  }

  function openAddForm() {
    editingId = null;
    formName = "";
    formUrl = "";
    formUsername = "";
    formPassword = "";
    formNotes = "";
    formTags = "";
    showPassword = false;
    showForm = true;
  }

  function openEditForm(entry: import("$lib/types").PasswordEntry) {
    editingId = entry.id;
    formName = entry.name;
    formUrl = entry.url;
    formUsername = entry.username;
    formPassword = "********";
    formNotes = entry.notes;
    formTags = entry.tags.join(", ");
    showForm = true;
  }

  async function saveEntry() {
    if (!formName || !formPassword) return;
    loading = true;
    try {
      const tags = formTags.split(",").map(t => t.trim()).filter(Boolean);
      if (editingId) {
        await invoke("vault_update_entry", {
          id: editingId, name: formName, url: formUrl,
          username: formUsername, password: formPassword,
          notes: formNotes, tags
        });
      } else {
        await invoke("vault_add_entry", {
          name: formName, url: formUrl,
          username: formUsername, password: formPassword,
          notes: formNotes, tags
        });
      }
      showForm = false;
      await loadEntries();
    } catch (e) {
      errorMsg = String(e);
    } finally {
      loading = false;
    }
  }

  async function deleteEntry(id: string) {
    if (!confirm("确定删除此条目？")) return;
    try {
      await invoke("vault_delete_entry", { id });
      await loadEntries();
    } catch (e) {
      errorMsg = String(e);
    }
  }

  async function copyPassword(id: string) {
    try {
      const pwd = await invoke<string>("vault_get_password", { id });
      await navigator.clipboard.writeText(pwd);
    } catch (e) {
      errorMsg = String(e);
    }
  }

  function generatePassword() {
    invoke<string>("vault_generate_password", {
      length: genLength, useUpper: genUpper,
      useLower: genLower, useDigits: genDigits, useSymbols: genSymbols
    }).then(r => genResult = r).catch(e => errorMsg = String(e));
  }

  function useGeneratedPw() {
    formPassword = genResult;
    showGenerator = false;
  }

  let filteredEntries = $derived.by(() => {
    if (!searchQuery) return entries;
    const q = searchQuery.toLowerCase();
    return entries.filter(e =>
      e.name.toLowerCase().includes(q) ||
      e.url.toLowerCase().includes(q) ||
      e.username.toLowerCase().includes(q) ||
      e.tags.some(t => t.toLowerCase().includes(q))
    );
  });
</script>

<div class="vault-page">
  <!-- 首次设置 -->
  {#if setupMode && !isInitialized}
    <div class="vault-lock">
      <div class="lock-icon">🔒</div>
      <h2 class="lock-title">初始化密码本</h2>
      <p class="lock-desc">设置主密码来保护你的密码条目</p>
      <div class="lock-form" data-no-drag>
        <input class="input" type="password" placeholder="设置主密码" bind:value={masterPassword} onkeypress={(e) => e.key === 'Enter' && handleSetup()} />
        <input class="input" type="password" placeholder="确认主密码" bind:value={confirmPassword} onkeypress={(e) => e.key === 'Enter' && handleSetup()} />
        {#if errorMsg}<p class="error-text">{errorMsg}</p>{/if}
        <button class="btn btn-primary" onclick={handleSetup} disabled={loading}>
          {loading ? "初始化中..." : "初始化"}
        </button>
      </div>
    </div>

  <!-- 锁屏状态 -->
  {:else if !isUnlocked}
    <div class="vault-lock">
      <div class="lock-icon">🔒</div>
      <h2 class="lock-title">密码本已锁定</h2>
      <p class="lock-desc">输入主密码解锁</p>
      <div class="lock-form" data-no-drag>
        <input class="input" type="password" placeholder="主密码" bind:value={masterPassword} onkeypress={(e) => e.key === 'Enter' && handleUnlock()} />
        {#if errorMsg}<p class="error-text">{errorMsg}</p>{/if}
        <button class="btn btn-primary" onclick={handleUnlock} disabled={loading}>
          {loading ? "验证中..." : "解锁"}
        </button>
      </div>
    </div>

  <!-- 已解锁 - 条目管理 -->
  {:else}
    <div class="vault-header">
      <button class="btn btn-ghost lock-btn" onclick={lock} data-no-drag>🔒 锁定</button>
      <button class="btn btn-primary" onclick={openAddForm} data-no-drag>+ 新增</button>
    </div>

    <div class="search-bar" data-no-drag>
      <input class="input" placeholder="搜索密码条目..." bind:value={searchQuery} />
    </div>

    {#if showForm}
      <div class="form-overlay" onclick={() => showForm = false} data-no-drag></div>
      <div class="entry-form card" data-no-drag>
        <h3>{editingId ? "编辑条目" : "新增条目"}</h3>
        <input class="input" placeholder="名称 *" bind:value={formName} />
        <input class="input" placeholder="网站 URL" bind:value={formUrl} />
        <input class="input" placeholder="用户名" bind:value={formUsername} />
        <div class="password-row">
          <input class="input" type={showPassword ? "text" : "password"} placeholder="密码 *" bind:value={formPassword} />
          <button class="btn btn-ghost" onclick={() => showPassword = !showPassword}>{showPassword ? "🙈" : "👁️"}</button>
          <button class="btn btn-ghost" onclick={() => showGenerator = !showGenerator}>🎲</button>
        </div>
        {#if showGenerator}
          <div class="generator card">
            <div class="gen-row">
              <label>长度: {genLength}</label>
              <input type="range" min={4} max={64} bind:value={genLength} />
            </div>
            <label><input type="checkbox" bind:checked={genUpper} /> 大写</label>
            <label><input type="checkbox" bind:checked={genLower} /> 小写</label>
            <label><input type="checkbox" bind:checked={genDigits} /> 数字</label>
            <label><input type="checkbox" bind:checked={genSymbols} /> 符号</label>
            <button class="btn" onclick={generatePassword}>生成密码</button>
            {#if genResult}
              <div class="gen-result">
                <code>{genResult}</code>
                <button class="btn btn-primary" onclick={useGeneratedPw}>使用</button>
              </div>
            {/if}
          </div>
        {/if}
        <input class="input" placeholder="备注" bind:value={formNotes} />
        <input class="input" placeholder="标签（逗号分隔）" bind:value={formTags} />
        {#if errorMsg}<p class="error-text">{errorMsg}</p>{/if}
        <div class="form-actions">
          <button class="btn" onclick={() => showForm = false}>取消</button>
          <button class="btn btn-primary" onclick={saveEntry} disabled={loading}>{loading ? "保存中..." : "保存"}</button>
        </div>
      </div>
    {/if}

    <div class="entries-list">
      {#each filteredEntries as entry (entry.id)}
        <div class="entry-card card" data-no-drag>
          <div class="entry-header">
            <div>
              <strong>{entry.name}</strong>
              {#if entry.url}
                <span class="entry-url">{entry.url}</span>
              {/if}
            </div>
            <div class="entry-actions">
              <button class="btn-ghost action-btn" onclick={() => copyPassword(entry.id)} title="复制密码">📋</button>
              <button class="btn-ghost action-btn" onclick={() => openEditForm(entry)} title="编辑">✏️</button>
              <button class="btn-ghost action-btn" onclick={() => deleteEntry(entry.id)} title="删除">🗑️</button>
            </div>
          </div>
          <div class="entry-detail">
            <span class="detail-label">用户名:</span>
            <span>{entry.username}</span>
          </div>
          {#if entry.tags.length > 0}
            <div class="tags">
              {#each entry.tags as tag}
                <span class="tag">{tag}</span>
              {/each}
            </div>
          {/if}
        </div>
      {:else}
        <div class="empty-state">
          <p>暂无密码条目，点击"+ 新增"添加</p>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style>
  .vault-page {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-md);
    height: 100%;
  }

  /* 锁定界面 */
  .vault-lock {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: 60px 20px;
    text-align: center;
  }

  .lock-icon {
    font-size: 48px;
    margin-bottom: var(--spacing-md);
  }

  .lock-title {
    font-size: var(--font-size-xl);
    font-weight: 600;
    margin-bottom: var(--spacing-sm);
  }

  .lock-desc {
    color: var(--text-secondary);
    margin-bottom: var(--spacing-lg);
  }

  .lock-form {
    display: flex;
    flex-direction: column;
    gap: var(--spacing-sm);
    width: 100%;
    max-width: 320px;
  }

  .error-text {
    color: #e81123;
    font-size: var(--font-size-sm);
  }

  /* 已解锁头部 */
  .vault-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .lock-btn {
    font-size: var(--font-size-sm);
  }

  /* 表单弹窗 */
  .form-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0,0,0,0.3);
    z-index: 10;
  }

  .entry-form {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: 90%;
    max-width: 400px;
    max-height: 80vh;
    overflow-y: auto;
    display: flex;
    flex-direction: column;
    gap: 8px;
    z-index: 11;
    padding: var(--spacing-lg);
  }

  .password-row {
    display: flex;
    gap: 4px;
  }

  .password-row .input {
    flex: 1;
  }

  .form-actions {
    display: flex;
    gap: 8px;
    justify-content: flex-end;
    margin-top: var(--spacing-sm);
  }

  /* 生成器 */
  .generator {
    display: flex;
    flex-direction: column;
    gap: 6px;
    padding: var(--spacing-md);
  }

  .gen-row {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .gen-row input[type="range"] {
    flex: 1;
  }

  .gen-result {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: var(--spacing-sm);
    background: var(--bg-secondary);
    border-radius: var(--radius-sm);
  }

  .gen-result code {
    flex: 1;
    font-size: var(--font-size-sm);
    word-break: break-all;
  }

  /* 条目列表 */
  .entries-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    overflow-y: auto;
    flex: 1;
  }

  .entry-card {
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .entry-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
  }

  .entry-url {
    display: block;
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    font-weight: 400;
  }

  .entry-actions {
    display: flex;
    gap: 2px;
    opacity: 0;
    transition: opacity var(--transition-fast);
  }

  .entry-card:hover .entry-actions {
    opacity: 1;
  }

  .action-btn {
    font-size: 14px;
    padding: 4px 6px;
  }

  .entry-detail {
    font-size: var(--font-size-sm);
    color: var(--text-secondary);
  }

  .detail-label {
    color: var(--text-tertiary);
    margin-right: 4px;
  }

  .tags {
    display: flex;
    gap: 4px;
    flex-wrap: wrap;
  }

  .tag {
    font-size: 11px;
    padding: 2px 8px;
    background: var(--accent-soft);
    color: var(--accent);
    border-radius: 8px;
  }

  .empty-state {
    text-align: center;
    padding: 40px 20px;
    color: var(--text-tertiary);
  }
</style>
