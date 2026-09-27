<script setup lang="ts">
import { computed, nextTick, onMounted, ref, watch } from "vue";
import { useBotsStore } from "../stores/bots";
import { stateText, useNow } from "../status";
import type { BotSpec, BotState } from "../types";

const props = defineProps<{ bot: BotSpec }>();
const store = useBotsStore();
const now = useNow();

const status = computed(() => store.statuses[props.bot.id]);
const state = computed<BotState>(() => status.value?.state ?? { kind: "stopped" });
const lines = computed(() => store.logs[props.bot.id] ?? []);
const stopped = computed(() => state.value.kind === "stopped");
const busy = computed(() => state.value.kind === "stopping");
const pidTip = computed(() => (state.value.kind === "running" ? `pid ${state.value.pid}` : ""));

function edit() {
  store.edit(`編輯「${props.bot.name}」`, JSON.parse(JSON.stringify(props.bot)));
}

// Removing needs a second click within 3 s; no native confirm dialog.
const armed = ref(false);
let disarm: number | undefined;
function remove() {
  if (!armed.value) {
    armed.value = true;
    disarm = window.setTimeout(() => (armed.value = false), 3000);
    return;
  }
  window.clearTimeout(disarm);
  void store.remove(props.bot.id);
}

// Follow new lines unless the user scrolled up to read.
const logEl = ref<HTMLElement | null>(null);
const follow = ref(true);
function onScroll() {
  const el = logEl.value;
  if (el) follow.value = el.scrollHeight - el.scrollTop - el.clientHeight < 24;
}
async function toBottom() {
  await nextTick();
  logEl.value?.scrollTo({ top: logEl.value.scrollHeight });
}
watch(
  () => lines.value.length,
  () => follow.value && toBottom(),
);
onMounted(toBottom);

function timeOnly(ts: string): string {
  return ts.length >= 19 ? ts.slice(11, 19) : ts;
}
</script>

<template>
  <section class="card detail">
    <div class="head">
      <span class="dot" :class="`dot-${state.kind}`"></span>
      <div class="title">
        <span class="name">{{ bot.name }}</span>
        <span class="state" :data-tip="pidTip || undefined">
          {{ stateText(state, now) }}<template v-if="status && status.restarts > 0"> · 已自動重開 {{ status.restarts }} 次</template>
        </span>
      </div>
      <div class="spacer"></div>
      <button v-if="stopped" class="primary" @click="store.start(bot.id)">啟動</button>
      <template v-else>
        <button :disabled="busy" @click="store.stop(bot.id)">停止</button>
        <button :disabled="busy" @click="store.restart(bot.id)">重開</button>
      </template>
    </div>

    <div class="row">
      <span class="row-title">
        開啟程式時自動啟動
        <span class="row-sub mono" :data-tip="bot.exe">{{ bot.exe }}</span>
      </span>
      <div class="ops">
        <button class="switch" role="switch" :class="{ on: bot.autostart }" :aria-checked="bot.autostart" @click="store.setAutostart(bot, !bot.autostart)"></button>
        <span class="sep"></span>
        <button class="plain sm" @click="edit">編輯</button>
        <button class="plain sm danger" :class="{ armed }" @click="remove">{{ armed ? "再按一次移除" : "移除" }}</button>
      </div>
    </div>

    <div ref="logEl" class="log mono" @scroll="onScroll">
      <p v-if="lines.length === 0" class="log-empty">還沒有輸出。</p>
      <div v-for="(l, i) in lines" :key="i" class="line" :class="`s-${l.stream}`">
        <span class="ts">{{ timeOnly(l.ts) }}</span>
        <span class="txt">{{ l.line }}</span>
      </div>
    </div>
    <button v-if="!follow" class="plain sm jump" @click="(follow = true), toBottom()">跳到最新</button>
  </section>
</template>

<style scoped>
.detail {
  position: relative;
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.head {
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: var(--sp-3) var(--sp-4);
  border-bottom: 0.5px solid var(--border);
}
.head .dot {
  width: 11px;
  height: 11px;
}
.title {
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.name {
  font-size: 18px;
  font-weight: 700;
  color: var(--text-strong);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.state {
  font-size: 14px;
  color: var(--text-dim);
}

.row {
  flex: none;
  border-bottom: 0.5px solid var(--border);
}
.row-sub {
  max-width: 100%;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
.ops {
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-2);
}
.sep {
  width: 1px;
  height: 22px;
  margin: 0 var(--sp-1);
  background: var(--border-strong);
}

.log {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: var(--sp-3) var(--sp-4);
  font-size: 13px;
  line-height: 1.6;
  background: var(--bg-2);
  user-select: text;
  cursor: text;
}
.log-empty {
  color: var(--text-faint);
}
.line {
  display: flex;
  gap: var(--sp-3);
}
.ts {
  flex: none;
  color: var(--text-faint);
}
.txt {
  min-width: 0;
  white-space: pre-wrap;
  word-break: break-all;
  color: var(--text);
}
.s-stderr .txt {
  color: var(--danger);
}
.s-system .txt {
  color: var(--text-faint);
}
.jump {
  position: absolute;
  right: var(--sp-4);
  bottom: var(--sp-3);
  background: var(--float);
  border: 1px solid var(--control-border);
}
</style>
