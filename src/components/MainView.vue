<script setup lang="ts">
import { onMounted, watch } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { LogicalSize, getCurrentWindow } from "@tauri-apps/api/window";
import { useBotsStore } from "../stores/bots";
import TitleBar from "./TitleBar.vue";
import BotList from "./BotList.vue";
import BotPanel from "./BotPanel.vue";
import type { BotSpec } from "../types";

const store = useBotsStore();

onMounted(() => store.init());

// Fixed-size window: the list alone is COLLAPSED_WIDTH wide; selecting a bot
// grows the window to the right so the panel appears beside the list.
const COLLAPSED_WIDTH = 480;
const EXPANDED_WIDTH = 960;
const HEIGHT = 640;
watch(
  () => store.selected !== null,
  (expanded) => {
    void getCurrentWindow().setSize(new LogicalSize(expanded ? EXPANDED_WIDTH : COLLAPSED_WIDTH, HEIGHT));
  },
);

function selectBot(id: string) {
  store.select(store.selectedId === id ? null : id);
}

async function importFolder() {
  const dir = await open({ directory: true, multiple: false, title: "選擇含 bot.toml 的資料夾" });
  if (typeof dir !== "string") return;
  await store.openWindow("form", `import=${encodeURIComponent(dir)}`);
}

const newBot = () => store.openWindow("form");
const editBot = (bot: BotSpec) => store.openWindow("form", `id=${encodeURIComponent(bot.id)}`);
const openSettings = () => store.openWindow("settings");
</script>

<template>
  <TitleBar title="KZ Bot Host">
    <button class="btn btn-sm" @click="importFolder">匯入資料夾</button>
    <button class="btn btn-sm" @click="newBot">新增</button>
    <button class="btn btn-ghost btn-sm" @click="openSettings">設定</button>
  </TitleBar>

  <div v-if="store.startupNotice" class="banner banner-warn notice">
    設定檔讀取失敗，目前以空白設定啟動（原檔未被覆寫）：{{ store.startupNotice }}
  </div>
  <div v-if="store.error" class="banner banner-error notice" @click="store.error = null">
    {{ store.error }}
  </div>

  <main class="body" :class="{ expanded: store.selected }">
    <aside class="sidebar">
      <BotList :active-id="store.selectedId" @select="selectBot" />
      <div v-if="store.bots.length === 0" class="empty-list">
        還沒有任何 bot。按「匯入資料夾」或「新增」開始。
      </div>
    </aside>
    <section class="content">
      <BotPanel v-if="store.selected" :bot="store.selected" @edit="editBot" />
      <div v-else class="empty">
        <p v-if="store.bots.length > 0">選一隻 bot 展開它的 log。</p>
      </div>
    </section>
  </main>
</template>

<style scoped>
.notice {
  margin: 10px 14px 0;
  flex: none;
}

/* The list column is the whole collapsed window; the panel lives to its right. */
.body {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-rows: minmax(0, 1fr);
  grid-template-columns: 478px minmax(0, 1fr);
}

.sidebar {
  background: var(--canvas);
  min-height: 0;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
}

.body.expanded .sidebar {
  border-right: 1px solid var(--border);
}

.content {
  min-height: 0;
  min-width: 0;
  background: var(--surface);
  display: flex;
  flex-direction: column;
}

.body:not(.expanded) .content {
  display: none;
}

.empty,
.empty-list {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-faint);
  padding: 24px;
  text-align: center;
}
</style>
