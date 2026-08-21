<script setup lang="ts">
import { onMounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useBotsStore } from "./stores/bots";
import TitleBar from "./components/TitleBar.vue";
import BotList from "./components/BotList.vue";
import BotPanel from "./components/BotPanel.vue";
import BotForm from "./components/BotForm.vue";
import SettingsPanel from "./components/SettingsPanel.vue";
import type { BotSpec } from "./types";
import { emptySpec } from "./types";

const store = useBotsStore();

type Pane = { kind: "bot" } | { kind: "settings" } | { kind: "form"; draft: BotSpec; title: string };
const pane = ref<Pane>({ kind: "bot" });

onMounted(() => store.init());

function selectBot(id: string) {
  const alreadyShowing = pane.value.kind === "bot" && store.selectedId === id;
  pane.value = { kind: "bot" };
  store.select(alreadyShowing ? null : id);
}

function openSettings() {
  pane.value = pane.value.kind === "settings" ? { kind: "bot" } : { kind: "settings" };
}

function newBot() {
  pane.value = { kind: "form", draft: emptySpec(), title: "新增 bot" };
}

function editBot(bot: BotSpec) {
  pane.value = { kind: "form", draft: JSON.parse(JSON.stringify(bot)), title: `編輯「${bot.name}」` };
}

async function importFolder() {
  const dir = await open({ directory: true, multiple: false, title: "選擇含 bot.toml 的資料夾" });
  if (typeof dir !== "string") return;
  const draft = await store.importFolder(dir);
  if (draft) pane.value = { kind: "form", draft, title: `匯入「${draft.name}」` };
}

async function saveForm(spec: BotSpec) {
  const saved = await store.upsert(spec);
  if (saved) {
    pane.value = { kind: "bot" };
    store.select(saved.id);
  }
}

function cancelForm() {
  pane.value = { kind: "bot" };
}

// Undecorated windows on Windows have no native resize border; this grip
// hands the drag to the OS so the window still resizes.
function startResize(e: MouseEvent) {
  if (e.button !== 0) return;
  void getCurrentWindow().startResizeDragging("SouthEast");
}
</script>

<template>
  <div class="shell">
    <TitleBar
      :settings-active="pane.kind === 'settings'"
      @import="importFolder"
      @new="newBot"
      @settings="openSettings"
    />

    <div v-if="store.startupNotice" class="banner banner-warn notice">
      設定檔讀取失敗，目前以空白設定啟動（原檔未被覆寫）：{{ store.startupNotice }}
    </div>
    <div v-if="store.error" class="banner banner-error notice" @click="store.error = null">
      {{ store.error }}
    </div>

    <main class="body" :class="{ expanded: pane.kind !== 'bot' || store.selected }">
      <aside class="sidebar">
        <BotList :active-id="pane.kind === 'bot' ? store.selectedId : null" @select="selectBot" />
        <div v-if="store.bots.length === 0" class="empty-list">
          還沒有任何 bot。按「匯入資料夾」或「新增」開始。
        </div>
      </aside>
      <section class="content">
        <SettingsPanel v-if="pane.kind === 'settings'" />
        <BotForm
          v-else-if="pane.kind === 'form'"
          :draft="pane.draft"
          :title="pane.title"
          @save="saveForm"
          @cancel="cancelForm"
        />
        <BotPanel v-else-if="store.selected" :bot="store.selected" @edit="editBot" />
        <div v-else class="empty">
          <p v-if="store.bots.length > 0">選一隻 bot 展開它的 log。</p>
        </div>
      </section>
    </main>

    <div class="grip" @mousedown="startResize" title="拖曳調整大小"></div>
  </div>
</template>

<style scoped>
.shell {
  height: 100%;
  display: flex;
  flex-direction: column;
  border: 1px solid #dcdad5;
  background: var(--canvas);
  position: relative;
}

.notice {
  margin: 10px 14px 0;
  flex: none;
}

/* Narrow (default) layout: list on top, selected bot expands below. */
.body {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-rows: auto minmax(0, 1fr);
  grid-template-columns: 1fr;
}

.sidebar {
  background: var(--canvas);
  min-height: 0;
  overflow-y: auto;
  max-height: 40vh;
  border-bottom: 1px solid var(--border);
}

.body:not(.expanded) .sidebar {
  max-height: none;
  border-bottom: none;
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

/* Wide layout: list beside the panel. */
@media (min-width: 700px) {
  .body {
    grid-template-rows: 1fr;
    grid-template-columns: 240px 1fr;
  }

  .sidebar,
  .body:not(.expanded) .sidebar {
    max-height: none;
    border-bottom: none;
    border-right: 1px solid var(--border);
  }

  .body:not(.expanded) .content {
    display: flex;
  }
}

.grip {
  position: absolute;
  right: 0;
  bottom: 0;
  width: 16px;
  height: 16px;
  cursor: nwse-resize;
  background-image: url("data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' width='16' height='16'><path d='M15 5L5 15M15 10l-5 5' stroke='%23cfcdc8' stroke-width='1.5'/></svg>");
  background-repeat: no-repeat;
}
</style>
