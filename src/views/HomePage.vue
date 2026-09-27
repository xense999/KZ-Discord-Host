<script setup lang="ts">
import { open } from "@tauri-apps/plugin-dialog";
import { useBotsStore } from "../stores/bots";
import { emptySpec } from "../types";
import { useNow, stateText } from "../status";
import BotDetail from "../components/BotDetail.vue";

const store = useBotsStore();
const now = useNow();

function stateOf(id: string) {
  return store.statuses[id]?.state ?? { kind: "stopped" as const };
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
        <span>bot</span>
        <span class="count">{{ store.bots.length }}</span>
      </div>
      <div class="sidelist">
        <div
          v-for="bot in store.bots"
          :key="bot.id"
          class="sideitem"
          :class="{ on: bot.id === store.selectedId }"
          @click="store.select(bot.id)"
        >
          <span class="dot" :class="`dot-${stateOf(bot.id).kind}`"></span>
          <div class="sideitem-text">
            <span class="sideitem-main">{{ bot.name }}</span>
            <span class="sideitem-sub">{{ stateText(stateOf(bot.id), now) }}</span>
          </div>
        </div>
      </div>
      <div class="list-foot">
        <button class="primary" data-tip="手動填寫執行檔與 token" @click="addBot">新增</button>
        <button data-tip="選一個有 bot.toml 的資料夾，設定自動帶入" @click="importFolder">匯入</button>
      </div>
    </section>

    <BotDetail v-if="store.selected" :key="store.selected.id" :bot="store.selected" />
    <section v-else class="card empty">
      <p class="empty-title">還沒有任何 bot</p>
      <p class="empty-sub">有 bot.toml 的 bot 按「匯入」，其他的按「新增」手動填。</p>
      <div class="empty-ops">
        <button class="primary" @click="importFolder">匯入資料夾</button>
        <button @click="addBot">手動新增</button>
      </div>
    </section>
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
.list {
  width: 250px;
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
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: var(--sp-2);
  padding: var(--sp-6);
  text-align: center;
}
.empty-title {
  font-size: 18px;
  font-weight: 600;
  color: var(--text-strong);
}
.empty-sub {
  font-size: 15px;
  color: var(--text-dim);
}
.empty-ops {
  display: flex;
  gap: var(--sp-2);
  margin-top: var(--sp-3);
}
</style>
