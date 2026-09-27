<script setup lang="ts">
import { computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useBotsStore } from "../stores/bots";
import { emptySpec } from "../types";
import { useNow, stateText } from "../status";
import BotDetail from "../components/BotDetail.vue";
import EditPage from "./EditPage.vue";

const store = useBotsStore();
const now = useNow();

function stateOf(id: string) {
  return store.statuses[id]?.state ?? { kind: "stopped" as const };
}

// While editing, the row being edited is the active one (none for a new or imported bot).
const activeId = computed(() => (store.page.kind === "edit" ? store.page.draft.id : store.selectedId));

// Clicking the open bot again collapses the window. Picking a bot while the
// edit form is open leaves the form without saving, like 返回.
function pick(id: string) {
  const collapse = store.page.kind === "home" && store.selectedId === id;
  store.home();
  void store.select(collapse ? null : id);
}

function addBot() {
  store.edit("新增 bot", emptySpec());
}

async function importFolder() {
  const dir = await open({ directory: true, multiple: false, title: "選擇有 bot.toml 的資料夾" });
  if (typeof dir !== "string") return;
  const spec = await store.importFolder(dir);
  if (spec) store.edit(`匯入「${spec.name}」`, spec);
}
</script>

<template>
  <div class="home">
    <section class="card list">
      <div class="card-head">
        <span>BOT</span>
        <span class="count">{{ store.bots.length }}</span>
      </div>
      <div class="sidelist">
        <div
          v-for="bot in store.bots"
          :key="bot.id"
          class="sideitem"
          :class="{ on: bot.id === activeId }"
          @click="pick(bot.id)"
        >
          <span class="dot" :class="`dot-${stateOf(bot.id).kind}`"></span>
          <div class="sideitem-text">
            <span class="sideitem-main">{{ bot.name }}</span>
            <span class="sideitem-sub">{{ stateText(stateOf(bot.id), now) }}</span>
          </div>
        </div>
        <p v-if="store.bots.length === 0" class="empty">還沒有任何 bot。有 bot.toml 的按「匯入」，其他的按「新增」手動填。</p>
      </div>
      <div class="list-foot">
        <button class="primary" data-tip="手動填寫執行檔與 token" @click="addBot">新增</button>
        <button data-tip="選一個有 bot.toml 的資料夾，設定自動帶入" @click="importFolder">匯入</button>
      </div>
    </section>

    <EditPage v-if="store.page.kind === 'edit'" :key="store.page.draft.id || store.page.title" :page="store.page" />
    <BotDetail v-else-if="store.selected" :key="store.selected.id" :bot="store.selected" />
  </div>
</template>

<style scoped>
.home {
  flex: 1;
  min-height: 0;
  display: flex;
  gap: var(--sp-3);
  padding: var(--sp-3);
}
/* Same width collapsed or expanded (420 window - 2 x 12 padding), so the list
   does not jump when the right pane opens. */
.list {
  width: 396px;
  flex: none;
  display: flex;
  flex-direction: column;
}
.count {
  margin-left: auto;
  font-size: 15px;
  font-weight: 400;
  color: var(--text-faint);
}
.list-foot {
  flex: none;
  display: flex;
  gap: var(--sp-2);
  padding: var(--sp-3) var(--sp-4);
  border-top: 0.5px solid var(--border);
}
.list-foot button {
  flex: 1;
}

.empty {
  padding: var(--sp-6) var(--sp-4);
  font-size: 15px;
  text-align: center;
  color: var(--text-faint);
}
</style>
