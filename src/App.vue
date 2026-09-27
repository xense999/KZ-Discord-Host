<script setup lang="ts">
import { onMounted, watch } from "vue";
import { LogicalSize, getCurrentWindow } from "@tauri-apps/api/window";
import AppTitlebar from "./components/AppTitlebar.vue";
import AppTooltip from "./components/AppTooltip.vue";
import ToastPop from "./components/ToastPop.vue";
import HomePage from "./views/HomePage.vue";
import { useBotsStore } from "./stores/bots";

const store = useBotsStore();

onMounted(() => store.init());

// Fixed-size window, as in the original design: the list alone is narrow;
// selecting a bot (or opening the add/edit form) grows it to the right.
// Collapsed width matches 久世登入器 (420).
const COLLAPSED_WIDTH = 420;
const EXPANDED_WIDTH = 960;
const HEIGHT = 640;
watch(
  () => store.expanded,
  (expanded) => {
    void getCurrentWindow().setSize(new LogicalSize(expanded ? EXPANDED_WIDTH : COLLAPSED_WIDTH, HEIGHT));
  },
);


</script>

<template>
  <div class="winframe">
    <AppTooltip />
    <ToastPop />
    <AppTitlebar title="久世 Discord Host" @open-settings="store.openSettings()" />

    <div v-if="store.startupNotice" class="notice">
      <span>設定檔讀取失敗，這次以空白設定啟動。{{ store.startupNotice }}</span>
      <button class="plain sm" @click="store.startupNotice = null">知道了</button>
    </div>

    <main class="content">
      <HomePage />
    </main>
  </div>
</template>

<style scoped>
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
