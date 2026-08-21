<script setup lang="ts">
import { useBotsStore } from "../stores/bots";
import type { BotSpec, BotState } from "../types";

defineProps<{ activeId: string | null }>();
const emit = defineEmits<{ select: [id: string] }>();
const store = useBotsStore();

function stateOf(id: string): BotState {
  return store.statuses[id]?.state ?? { kind: "stopped" };
}

async function toggleAutostart(bot: BotSpec, ev: Event) {
  ev.stopPropagation();
  await store.setAutostart(bot, !bot.autostart);
}
</script>

<template>
  <ul class="list">
    <li
      v-for="bot in store.bots"
      :key="bot.id"
      class="row"
      :class="{ active: bot.id === activeId }"
      @click="emit('select', bot.id)"
    >
      <span class="dot" :class="`dot-${stateOf(bot.id).kind}`"></span>
      <span class="name" :title="bot.exe">{{ bot.name }}</span>
      <button
        class="switch"
        :class="{ on: bot.autostart }"
        :title="bot.autostart ? '隨管家啟動：開' : '隨管家啟動：關'"
        @click="toggleAutostart(bot, $event)"
      ></button>
    </li>
  </ul>
</template>

<style scoped>
.list {
  list-style: none;
  margin: 0;
  padding: 12px 0;
}

.row {
  display: flex;
  align-items: center;
  gap: 10px;
  height: 40px;
  padding: 0 12px 0 20px;
  cursor: pointer;
  transition: background 120ms;
}

.row:hover {
  background: rgba(0, 0, 0, 0.03);
}

.row.active {
  background: var(--surface);
  box-shadow: inset 2px 0 0 var(--ink);
}

.name {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
