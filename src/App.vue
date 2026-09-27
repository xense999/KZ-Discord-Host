<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import AppTitlebar from "./components/AppTitlebar.vue";
import AppTooltip from "./components/AppTooltip.vue";
import ToastPop from "./components/ToastPop.vue";
import HomePage from "./views/HomePage.vue";
import EditPage from "./views/EditPage.vue";
import SettingsPage from "./views/SettingsPage.vue";
import { useBotsStore } from "./stores/bots";

const store = useBotsStore();

// Maximized: square corners and no outline, or the desktop shows through the screen corners.
const maximized = ref(false);
const win = getCurrentWindow();
async function syncMaximized() {
  maximized.value = await win.isMaximized();
}

onMounted(async () => {
  void store.init();
  await syncMaximized();
  await win.onResized(syncMaximized);
});

// Settings is a separate screen, not an overlay: the gear swaps the whole content area.
const showSettings = ref(false);
const view = computed(() => {
  if (showSettings.value) return "settings";
  return store.page.kind;
});
</script>

<template>
  <div class="app" :class="{ maxed: maximized }">
    <AppTooltip />
    <ToastPop />
    <AppTitlebar title="久世 Discord Host" :settings-open="showSettings" @toggle-settings="showSettings = !showSettings" />

    <div v-if="store.startupNotice" class="notice">
      <span>設定檔讀取失敗，這次以空白設定啟動。{{ store.startupNotice }}</span>
      <button class="plain sm" @click="store.startupNotice = null">知道了</button>
    </div>

    <main class="content">
      <SettingsPage v-if="view === 'settings'" />
      <EditPage v-else-if="store.page.kind === 'edit'" :key="store.page.draft.id || store.page.title" :page="store.page" />
      <HomePage v-else />
    </main>
  </div>
</template>

<style scoped>
/* Rounded window drawn here: Windows 10 has no DWM rounding for an undecorated transparent window. */
.app {
  position: relative;
  height: 100%;
  display: flex;
  flex-direction: column;
  background: var(--bg-0);
  border-radius: var(--radius-window);
  overflow: hidden;
}
.app.maxed {
  border-radius: 0;
}
.app.maxed::after {
  display: none;
}
/* The outline sits on top of everything, otherwise the title bar's own background hides it. */
.app::after {
  content: "";
  position: absolute;
  inset: 0;
  z-index: 100;
  pointer-events: none;
  border: 1px solid var(--window-edge);
  border-radius: inherit;
  box-shadow: inset 0 1px 0 var(--window-highlight);
}

.notice {
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: 8px var(--sp-4);
  font-size: 14px;
  color: var(--warn);
  background: var(--bg-2);
  border-bottom: 0.5px solid var(--border);
}
.notice span {
  flex: 1;
  min-width: 0;
}

.content {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
</style>
