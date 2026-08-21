<script setup lang="ts">
import { onMounted, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useBotsStore } from "./stores/bots";
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
  pane.value = { kind: "bot" };
  store.select(id);
}

function openSettings() {
  pane.value = { kind: "settings" };
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
</script>

<template>
  <div class="shell">
    <header class="topbar">
      <div class="brand">
        <span class="brand-mark" aria-hidden="true"></span>
        <span class="brand-name">KZ Bot Host</span>
      </div>
      <div class="actions">
        <button class="btn" @click="importFolder">匯入資料夾</button>
        <button class="btn" @click="newBot">新增</button>
        <button class="btn btn-ghost" :class="{ active: pane.kind === 'settings' }" @click="openSettings">設定</button>
      </div>
    </header>

    <div v-if="store.startupNotice" class="banner banner-warn notice">
      設定檔讀取失敗，目前以空白設定啟動（原檔未被覆寫）：{{ store.startupNotice }}
    </div>
    <div v-if="store.error" class="banner banner-error notice" @click="store.error = null">
      {{ store.error }}
    </div>

    <main class="body">
      <aside class="sidebar">
        <BotList :active-id="pane.kind === 'bot' ? store.selectedId : null" @select="selectBot" />
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
          <p v-if="store.bots.length === 0">還沒有任何 bot。按「匯入資料夾」或「新增」開始。</p>
          <p v-else>從左側選一隻 bot。</p>
        </div>
      </section>
    </main>
  </div>
</template>

<style scoped>
.shell {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.topbar {
  height: 52px;
  flex: none;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 20px;
  border-bottom: 1px solid var(--border);
  background: var(--surface);
}

.brand {
  display: flex;
  align-items: center;
  gap: 10px;
}

.brand-mark {
  width: 10px;
  height: 10px;
  border-radius: 2px;
  background: var(--ink);
}

.brand-name {
  font-size: 15px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.actions {
  display: flex;
  gap: 8px;
}

.actions .active {
  color: var(--text);
  background: rgba(0, 0, 0, 0.04);
}

.notice {
  margin: 12px 20px 0;
  flex: none;
}

.body {
  flex: 1;
  min-height: 0;
  display: grid;
  grid-template-columns: 260px 1fr;
}

.sidebar {
  border-right: 1px solid var(--border);
  background: var(--canvas);
  min-height: 0;
  overflow-y: auto;
}

.content {
  min-height: 0;
  min-width: 0;
  background: var(--surface);
  display: flex;
  flex-direction: column;
}

.empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--text-faint);
}
</style>
