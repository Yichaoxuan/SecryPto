<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";

  let expiryDays = $state(3);
  let theme = $state("system");
  let autoStart = $state(true);
  let saving = $state(false);
  let saved = $state(false);

  onMount(async () => {
    try {
      // 将来可通过 invoke("get_settings") 获取
      // 当前使用默认值
    } catch (e) {
      console.error(e);
    }
  });

  async function saveSettings() {
    saving = true;
    saved = false;
    try {
      // TODO: 实现 settings 持久化 Tauri 命令
      await new Promise(r => setTimeout(r, 300));
      saved = true;
      setTimeout(() => saved = false, 2000);
    } catch (e) {
      console.error(e);
    } finally {
      saving = false;
    }
  }
</script>

<div class="settings-page">
  <h2 class="section-title">剪贴板设置</h2>

  <div class="setting-group">
    <label class="setting-label">默认存储期限</label>
    <div class="expiry-options" data-no-drag>
      {#each [["1 天", 1], ["3 天", 3], ["5 天", 5], ["永久", 0]] as [label, val]}
        <button
          class="expiry-btn"
          class:active={expiryDays === val}
          onclick={() => expiryDays = val as number}
        >{label}</button>
      {/each}
    </div>
    <p class="setting-desc">超过期限的剪贴记录将被自动清理</p>
  </div>

  <div class="divider"></div>

  <h2 class="section-title">外观</h2>

  <div class="setting-group">
    <label class="setting-label">主题模式</label>
    <div class="theme-options" data-no-drag>
      {#each [["🌓 跟随系统", "system"], ["☀️ 浅色", "light"], ["🌙 深色", "dark"]] as [label, val]}
        <button
          class="theme-btn"
          class:active={theme === val}
          onclick={() => theme = val}
        >{label}</button>
      {/each}
    </div>
  </div>

  <div class="divider"></div>

  <h2 class="section-title">系统</h2>

  <div class="setting-group">
    <div class="toggle-row" data-no-drag>
      <div>
        <label class="setting-label">开机自启</label>
        <p class="setting-desc">系统启动时自动运行 Secrypto</p>
      </div>
      <button
        class="toggle-switch"
        class:active={autoStart}
        onclick={() => autoStart = !autoStart}
        role="switch"
        aria-checked={autoStart}
      >
        <span class="toggle-knob"></span>
      </button>
    </div>
  </div>

  <div class="divider"></div>

  <button class="btn btn-primary save-btn" onclick={saveSettings} disabled={saving} data-no-drag>
    {saving ? "保存中..." : saved ? "✓ 已保存" : "保存设置"}
  </button>
</div>

<style>
  .settings-page {
    max-width: 480px;
    margin: 0 auto;
    padding: var(--spacing-lg) 0;
  }

  .section-title {
    font-size: var(--font-size-sm);
    font-weight: 600;
    color: var(--text-secondary);
    text-transform: uppercase;
    letter-spacing: 0.5px;
    margin-bottom: var(--spacing-md);
  }

  .setting-group {
    margin-bottom: var(--spacing-lg);
  }

  .setting-label {
    font-size: var(--font-size-md);
    font-weight: 500;
    color: var(--text-primary);
    margin-bottom: var(--spacing-sm);
    display: block;
  }

  .setting-desc {
    font-size: var(--font-size-sm);
    color: var(--text-tertiary);
    margin-top: var(--spacing-xs);
  }

  .expiry-options, .theme-options {
    display: flex;
    gap: 8px;
    -webkit-app-region: no-drag;
  }

  .expiry-btn, .theme-btn {
    padding: 8px 16px;
    border: 1px solid var(--border-default);
    border-radius: var(--radius-md);
    background: var(--bg-card);
    color: var(--text-primary);
    font-family: var(--font-family);
    font-size: var(--font-size-sm);
    cursor: pointer;
    transition: all var(--transition-fast);
  }

  .expiry-btn:hover, .theme-btn:hover {
    background: var(--bg-card-hover);
    border-color: var(--border-hover);
  }

  .expiry-btn.active, .theme-btn.active {
    background: var(--accent-soft);
    border-color: var(--accent);
    color: var(--accent);
  }

  .toggle-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: var(--spacing-lg);
  }

  .toggle-switch {
    position: relative;
    width: 44px;
    height: 24px;
    border: none;
    border-radius: 12px;
    background: var(--border-default);
    cursor: pointer;
    transition: background var(--transition-fast);
    flex-shrink: 0;
    padding: 0;
  }

  .toggle-switch.active {
    background: var(--accent);
  }

  .toggle-knob {
    position: absolute;
    top: 2px;
    left: 2px;
    width: 20px;
    height: 20px;
    border-radius: 50%;
    background: white;
    box-shadow: 0 1px 3px rgba(0,0,0,0.2);
    transition: transform var(--transition-fast);
  }

  .toggle-switch.active .toggle-knob {
    transform: translateX(20px);
  }

  .divider {
    height: 1px;
    background: var(--border-default);
    margin: var(--spacing-lg) 0;
  }

  .save-btn {
    width: 100%;
    margin-top: var(--spacing-lg);
    padding: 10px;
  }
</style>
