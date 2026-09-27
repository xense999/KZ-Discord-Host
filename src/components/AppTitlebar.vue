<script setup lang="ts">
import { onMounted, onUnmounted } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";

/** `child`: the settings window, which has no gear and no minimise button. */
defineProps<{ title: string; child?: boolean }>();
const emit = defineEmits<{ openSettings: [] }>();

const appWin = getCurrentWindow();

/**
 * Drag starts only after the pointer moves DRAG_SLOP px (as in 久世管理器):
 * the click that brings the window to the front loses its mouseup to the
 * system, and a drag started on mousedown would then stick to the cursor.
 */
const DRAG_SLOP = 4;
let origin: { x: number; y: number } | null = null;

function onDown(e: MouseEvent) {
  if (e.button !== 0 || (e.target as HTMLElement | null)?.closest("button")) return;
  origin = { x: e.clientX, y: e.clientY };
}

function onMove(e: MouseEvent) {
  if (!origin) return;
  if (!(e.buttons & 1)) {
    origin = null;
    return;
  }
  if (Math.abs(e.clientX - origin.x) + Math.abs(e.clientY - origin.y) < DRAG_SLOP) return;
  origin = null;
  void appWin.startDragging();
}

function onUp() {
  origin = null;
}

onMounted(() => {
  window.addEventListener("mousemove", onMove);
  window.addEventListener("mouseup", onUp);
});
onUnmounted(() => {
  window.removeEventListener("mousemove", onMove);
  window.removeEventListener("mouseup", onUp);
});
</script>

<template>
  <div class="titlebar" :class="{ child }" @mousedown="onDown">
    <button v-if="!child" class="gear" data-tip="設定" @click="emit('openSettings')">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.9" stroke-linecap="round" stroke-linejoin="round">
        <path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z" />
        <circle cx="12" cy="12" r="3" />
      </svg>
    </button>

    <span class="brand">{{ title }}</span>
    <div class="spacer"></div>

    <div class="win-controls">
      <button v-if="!child" class="wbtn" data-tip="最小化" @click="appWin.minimize()">
        <svg class="wico" viewBox="0 0 10 10"><rect x="1.5" y="4.7" width="7" height="0.8" rx="0.4" /></svg>
      </button>
      <button class="wbtn close" :data-tip="child ? '關閉' : '收進系統匣（bot 繼續跑）'" @click="appWin.hide()">
        <svg class="wico" viewBox="0 0 10 10">
          <path d="M2.4 2.4 7.6 7.6M7.6 2.4 2.4 7.6" stroke="currentColor" stroke-width="1" stroke-linecap="round" />
        </svg>
      </button>
    </div>
  </div>
</template>

<style scoped>
.titlebar {
  position: relative;
  z-index: 20;
  height: 44px;
  flex: none;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 0 8px;
  background: var(--toolbar);
  border-bottom: 0.5px solid var(--border);
}
.titlebar.child {
  padding-left: var(--sp-4);
}
/* line-height 1 so the name centres on the same line as the gear and window buttons. */
.brand {
  font-size: 16px;
  font-weight: 700;
  line-height: 1;
  letter-spacing: 0.02em;
  color: var(--text-strong);
}
.spacer {
  height: 100%;
}

.gear,
.wbtn {
  border: none;
  padding: 0;
  flex: none;
  background: transparent;
  border-radius: var(--radius-xs);
  color: var(--text-dim);
}
.gear {
  width: 32px;
  height: 32px;
}
.gear:hover:not(:disabled),
.wbtn:hover:not(:disabled) {
  background: var(--hover);
  color: var(--text);
}
.gear.on {
  background: var(--accent-soft);
  color: var(--accent);
}
.gear svg {
  width: 18px;
  height: 18px;
  display: block;
}

.win-controls {
  display: flex;
  gap: 2px;
}
.wbtn {
  width: 42px;
  height: 30px;
}
.wbtn.close:hover:not(:disabled) {
  background: var(--danger);
  color: #fff;
}
.wico {
  width: 11px;
  height: 11px;
  fill: currentColor;
  display: block;
}
</style>
