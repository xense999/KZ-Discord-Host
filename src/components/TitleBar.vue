<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

const win = getCurrentWindow();

defineProps<{ settingsActive: boolean }>();
const emit = defineEmits<{ import: []; new: []; settings: [] }>();

function minimize() {
  void win.minimize();
}

// Close only hides: the app lives in the tray until "結束" from the tray menu.
function hide() {
  void win.hide();
}
</script>

<template>
  <header class="titlebar" data-tauri-drag-region>
    <div class="brand" data-tauri-drag-region>
      <span class="brand-mark" data-tauri-drag-region></span>
      <span class="brand-name" data-tauri-drag-region>KZ Bot Host</span>
    </div>
    <div class="actions">
      <button class="btn btn-sm" @click="emit('import')">匯入資料夾</button>
      <button class="btn btn-sm" @click="emit('new')">新增</button>
      <button class="btn btn-ghost btn-sm" :class="{ active: settingsActive }" @click="emit('settings')">設定</button>
    </div>
    <div class="wbtns">
      <button class="wbtn" title="最小化" @click="minimize">
        <svg width="10" height="10" viewBox="0 0 10 10"><path d="M1 5.5h8" stroke="currentColor" stroke-width="1.2" /></svg>
      </button>
      <button class="wbtn wbtn-close" title="藏到系統匣" @click="hide">
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
  width: 9px;
  height: 9px;
  border-radius: 2px;
  background: var(--ink);
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

.actions .active {
  color: var(--text);
  background: var(--hover);
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
