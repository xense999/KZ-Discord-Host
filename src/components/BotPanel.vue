<script setup lang="ts">
import { computed, nextTick, onBeforeUnmount, onMounted, ref, watch } from "vue";
import { useBotsStore } from "../stores/bots";
import type { BotSpec, BotState } from "../types";

const props = defineProps<{ bot: BotSpec }>();
const emit = defineEmits<{ edit: [bot: BotSpec] }>();
const store = useBotsStore();

const status = computed(() => store.statuses[props.bot.id]);
const state = computed<BotState>(() => status.value?.state ?? { kind: "stopped" });
const lines = computed(() => store.logs[props.bot.id] ?? []);

const now = ref(Date.now());
let ticker: number | undefined;
onMounted(() => {
  ticker = window.setInterval(() => (now.value = Date.now()), 1000);
});
onBeforeUnmount(() => window.clearInterval(ticker));

const stateLabel = computed(() => {
  const s = state.value;
  switch (s.kind) {
    case "running":
      return `執行中 · pid ${s.pid} · ${uptime(s.since_ms)}`;
    case "starting":
      return "啟動中";
    case "backoff": {
      const left = Math.max(0, Math.ceil((s.until_ms - now.value) / 1000));
      return `等待重啟 · ${left} 秒後（第 ${s.attempt + 1} 次）`;
    }
    default:
      return "已停止";
  }
});

function uptime(since: number): string {
  const sec = Math.max(0, Math.floor((now.value - since) / 1000));
  const h = Math.floor(sec / 3600);
  const m = Math.floor((sec % 3600) / 60);
  const s = sec % 60;
  return h > 0 ? `${h} 時 ${m} 分` : m > 0 ? `${m} 分 ${s} 秒` : `${s} 秒`;
}

const isActive = computed(() => state.value.kind !== "stopped");

const logEl = ref<HTMLElement | null>(null);
const autoScroll = ref(true);
function onScroll() {
  const el = logEl.value;
  if (!el) return;
  autoScroll.value = el.scrollHeight - el.scrollTop - el.clientHeight < 24;
}
watch(
  () => lines.value.length,
  async () => {
    if (!autoScroll.value) return;
    await nextTick();
    logEl.value?.scrollTo({ top: logEl.value.scrollHeight });
  },
);
watch(
  () => props.bot.id,
  async () => {
    autoScroll.value = true;
    await nextTick();
    logEl.value?.scrollTo({ top: logEl.value.scrollHeight });
  },
);

const confirmRemove = ref(false);
async function remove() {
  if (!confirmRemove.value) {
    confirmRemove.value = true;
    window.setTimeout(() => (confirmRemove.value = false), 3000);
    return;
  }
  await store.remove(props.bot.id);
}

function timeOnly(ts: string): string {
  return ts.length >= 19 ? ts.slice(11, 19) : ts;
}
</script>

<template>
  <div class="panel">
    <header class="head">
      <div class="title">
        <h2>{{ bot.name }}</h2>
        <span class="tag" :class="`tag-${state.kind}`">{{ stateLabel }}</span>
      </div>
      <div class="ops">
        <button class="btn btn-primary" :disabled="isActive" @click="store.start(bot.id)">啟動</button>
        <button class="btn" :disabled="!isActive" @click="store.stop(bot.id)">停止</button>
        <button class="btn" :disabled="!isActive" @click="store.restart(bot.id)">重啟</button>
        <span class="spacer"></span>
        <button class="btn btn-ghost" @click="emit('edit', bot)">編輯</button>
        <button class="btn btn-ghost btn-danger" @click="remove">
          {{ confirmRemove ? "再按一次確認移除" : "移除" }}
        </button>
      </div>
      <dl class="meta">
        <dt>執行檔</dt>
        <dd class="mono" :title="bot.exe">{{ bot.exe }}</dd>
        <template v-if="bot.args.length">
          <dt>參數</dt>
          <dd class="mono">{{ bot.args.join(" ") }}</dd>
        </template>
        <template v-if="status && status.restarts > 0">
          <dt>自動重啟</dt>
          <dd>{{ status.restarts }} 次</dd>
        </template>
      </dl>
    </header>

    <div class="divider"></div>

    <div class="logbar">
      <span class="label">Log</span>
      <span class="hint" v-if="!autoScroll">已暫停自動捲動（捲到底恢復）</span>
      <span class="spacer"></span>
      <button class="btn btn-ghost small" @click="store.clearLogView(bot.id)">清除畫面</button>
    </div>
    <div ref="logEl" class="log" @scroll="onScroll">
      <div v-if="lines.length === 0" class="log-empty">尚無輸出。</div>
      <div v-for="(l, i) in lines" :key="i" class="line" :class="`s-${l.stream}`">
        <span class="ts">{{ timeOnly(l.ts) }}</span>
        <span class="txt">{{ l.line }}</span>
      </div>
    </div>
  </div>
</template>

<style scoped>
.panel {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}

.head {
  padding: 20px 24px 16px;
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.title {
  display: flex;
  align-items: center;
  gap: 12px;
  min-width: 0;
}

h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.01em;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.ops {
  display: flex;
  gap: 8px;
  align-items: center;
}

.spacer {
  flex: 1;
}

.meta {
  display: grid;
  grid-template-columns: auto 1fr;
  gap: 2px 16px;
  margin: 0;
  font-size: 12px;
}

.meta dt {
  color: var(--text-faint);
}

.meta dd {
  margin: 0;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  color: var(--text-muted);
}

.mono {
  font-family: var(--font-mono);
}

.logbar {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 8px 24px 8px;
}

.label {
  font-size: 11px;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-faint);
}

.small {
  height: 24px;
  padding: 0 8px;
  font-size: 12px;
}

.log {
  flex: 1;
  min-height: 0;
  overflow: auto;
  padding: 4px 24px 20px;
  font-family: var(--font-mono);
  font-size: 12.5px;
  line-height: 1.55;
  user-select: text;
  background: var(--surface-muted);
  border-top: 1px solid var(--border);
}

.log-empty {
  color: var(--text-faint);
  padding-top: 12px;
}

.line {
  display: grid;
  grid-template-columns: 64px 1fr;
  gap: 12px;
  white-space: pre-wrap;
  word-break: break-all;
}

.ts {
  color: var(--text-faint);
}

.s-stderr .txt {
  color: var(--pale-red-text);
}

.s-system .txt {
  color: var(--pale-blue-text);
}
</style>
