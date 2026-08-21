<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

defineProps<{
  title: string;
  /** Child windows have no minimise button. */
  child?: boolean;
}>();

const win = getCurrentWindow();

function minimize() {
  void win.minimize();
}

// Child windows hide too: the window object is reused on the next open.
function closeOrHide() {
  void win.hide();
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <img class="brand-mark" src="../assets/icon.png" alt="" data-tauri-drag-region />
      <span class="brand-name" data-tauri-drag-region>{{ title }}</span>
    </div>
    <div class="actions">
      <slot></slot>
    </div>
    <div class="wbtns">
      <button v-if="!child" class="wbtn" title="最小化" @click="minimize">
        <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1 5.5h8" stroke="currentColor" stroke-width="1.2" /></svg>
      </button>
      <button class="wbtn wbtn-close" :title="child ? '關閉' : '藏到系統匣'" @click="closeOrHide">
        <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1.5 1.5l7 7M8.5 1.5l-7 7" stroke="currentColor" stroke-width="1.2" /></svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.titlebar {
  height: 44px;
  flex: none;
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 0 0 0 14px;
  border-bottom: 1px solid var(--border);
  background: var(--surface);
}

.brand {
  display: flex;
  align-items: center;
  gap: 8px;
  flex: 1;
  min-width: 0;
}

.brand-mark {
  width: 18px;
  height: 18px;
  border-radius: 4px;
  display: block;
}

.brand-name {
  font-size: 13px;
  font-weight: 600;
  letter-spacing: -0.01em;
  white-space: nowrap;
}

.actions {
  display: flex;
  gap: 6px;
}

.wbtns {
  display: flex;
  height: 100%;
  margin-left: 4px;
}

.wbtn {
  width: 40px;
  height: 100%;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  color: var(--text-muted);
  transition: background 120ms, color 120ms;
}

.wbtn:hover {
  background: var(--hover);
  color: var(--text);
}

.wbtn-close:hover {
  background: var(--pale-red);
  color: var(--pale-red-text);
}
</style>
