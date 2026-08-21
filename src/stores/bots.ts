import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import type { BotSpec, LogEvent, LogLine, StateEvent } from "../types";

export const LOG_RING = 500;

export const useBotsStore = defineStore("bots", () => {
  const bots = ref<BotSpec[]>([]);
  const statuses = ref<Record<string, StateEvent>>({});
  const logs = ref<Record<string, LogLine[]>>({});
  const selectedId = ref<string | null>(null);
  const startupNotice = ref<string | null>(null);
  const error = ref<string | null>(null);
  const hostAutostart = ref(false);
  const configPath = ref("");

  const selected = computed(() => bots.value.find((b) => b.id === selectedId.value) ?? null);

  let pending: LogEvent[] = [];
  let flushScheduled = false;
  function flushLogs() {
    flushScheduled = false;
    const batch = pending;
    pending = [];
    for (const ev of batch) {
      const list = logs.value[ev.id] ?? (logs.value[ev.id] = []);
      list.push({ ts: ev.ts, stream: ev.stream, line: ev.line });
      if (list.length > LOG_RING) list.splice(0, list.length - LOG_RING);
    }
  }
  function queueLog(ev: LogEvent) {
    pending.push(ev);
    if (!flushScheduled) {
      flushScheduled = true;
      requestAnimationFrame(flushLogs);
    }
  }

  async function run<T>(task: () => Promise<T>): Promise<T | undefined> {
    try {
      error.value = null;
      return await task();
    } catch (e) {
      error.value = typeof e === "string" ? e : String(e);
      return undefined;
    }
  }

  async function refresh() {
    bots.value = await invoke<BotSpec[]>("list_bots");
    const list = await invoke<StateEvent[]>("list_status");
    statuses.value = Object.fromEntries(list.map((s) => [s.id, s]));
  }

  let listening = false;
  async function init() {
    await run(async () => {
      await refresh();
      startupNotice.value = await invoke<string | null>("startup_notice");
      configPath.value = await invoke<string>("config_path");
      hostAutostart.value = await invoke<boolean>("get_host_autostart");
    });
    if (listening) return;
    listening = true;
    await listen<StateEvent>("bot-state", (e) => {
      statuses.value[e.payload.id] = e.payload;
    });
    await listen<LogEvent>("bot-log", (e) => queueLog(e.payload));
  }

  async function select(id: string | null) {
    selectedId.value = id;
    if (id && !logs.value[id]) {
      const tail = await run(() => invoke<LogLine[]>("get_log_tail", { id }));
      if (tail) logs.value[id] = tail;
    }
  }

  const start = (id: string) => run(() => invoke("start_bot", { id }));
  const stop = (id: string) => run(() => invoke("stop_bot", { id }));
  const restart = (id: string) => run(() => invoke("restart_bot", { id }));

  async function upsert(spec: BotSpec): Promise<BotSpec | undefined> {
    const saved = await run(() => invoke<BotSpec>("upsert_bot", { spec }));
    if (saved) await refresh();
    return saved;
  }

  async function remove(id: string) {
    await run(() => invoke("remove_bot", { id }));
    delete logs.value[id];
    if (selectedId.value === id) selectedId.value = null;
    await refresh();
  }

  const importFolder = (dir: string) => run(() => invoke<BotSpec>("import_bot_folder", { dir }));

  async function setAutostart(bot: BotSpec, on: boolean) {
    await upsert({ ...bot, autostart: on });
  }

  async function setHostAutostart(on: boolean) {
    await run(() => invoke("set_host_autostart", { enabled: on }));
    hostAutostart.value = (await run(() => invoke<boolean>("get_host_autostart"))) ?? hostAutostart.value;
  }

  const openLogsDir = () => run(() => invoke("open_logs_dir"));

  function clearLogView(id: string) {
    logs.value[id] = [];
  }

  return {
    bots,
    statuses,
    logs,
    selectedId,
    selected,
    startupNotice,
    error,
    hostAutostart,
    configPath,
    init,
    refresh,
    select,
    start,
    stop,
    restart,
    upsert,
    remove,
    importFolder,
    setAutostart,
    setHostAutostart,
    openLogsDir,
    clearLogView,
  };
});
