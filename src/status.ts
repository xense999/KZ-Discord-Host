import { onBeforeUnmount, onMounted, ref, type Ref } from "vue";
import type { BotState } from "./types";

/** A clock ticking once a second, for uptime and backoff countdowns. */
export function useNow(): Ref<number> {
  const now = ref(Date.now());
  let timer: number | undefined;
  onMounted(() => (timer = window.setInterval(() => (now.value = Date.now()), 1000)));
  onBeforeUnmount(() => window.clearInterval(timer));
  return now;
}

function duration(ms: number): string {
  const sec = Math.max(0, Math.floor(ms / 1000));
  const d = Math.floor(sec / 86400);
  const h = Math.floor((sec % 86400) / 3600);
  const m = Math.floor((sec % 3600) / 60);
  if (d > 0) return `${d} 天 ${h} 時`;
  if (h > 0) return `${h} 時 ${m} 分`;
  if (m > 0) return `${m} 分`;
  return `${sec} 秒`;
}

/** One short line per state; the list and the detail panel both use it. */
export function stateText(state: BotState, now: number): string {
  switch (state.kind) {
    case "running":
      return `執行中 · ${duration(now - state.since_ms)}`;
    case "starting":
      return "啟動中";
    case "stopping":
      return "停止中";
    case "backoff":
      return `${Math.max(0, Math.ceil((state.until_ms - now) / 1000))} 秒後自動重開`;
    default:
      return "已停止";
  }
}
