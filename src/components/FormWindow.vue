<script setup lang="ts">
import { onMounted, ref } from "vue";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { listen } from "@tauri-apps/api/event";
import { useBotsStore } from "../stores/bots";
import TitleBar from "./TitleBar.vue";
import BotForm from "./BotForm.vue";
import type { BotSpec } from "../types";
import { emptySpec } from "../types";

const store = useBotsStore();

const draft = ref<BotSpec | null>(null);
const title = ref("新增 bot");
const loadError = ref<string | null>(null);
// Re-keyed on every route so BotForm re-initialises from the new draft.
const formKey = ref(0);

async function route(query: URLSearchParams) {
  draft.value = null;
  loadError.value = null;
  const id = query.get("id");
  const importDir = query.get("import");
  if (id) {
    await store.refresh();
    const bot = store.bots.find((b) => b.id === id);
    if (!bot) {
      loadError.value = "找不到這隻 bot（可能已被移除）。";
      return;
    }
    title.value = `編輯「${bot.name}」`;
    draft.value = JSON.parse(JSON.stringify(bot));
  } else if (importDir) {
    const imported = await store.importFolder(importDir);
    if (!imported) {
      loadError.value = store.error ?? "匯入失敗。";
      return;
    }
    title.value = `匯入「${imported.name}」`;
    draft.value = imported;
  } else {
    title.value = "新增 bot";
    draft.value = emptySpec();
  }
  formKey.value += 1;
}

onMounted(async () => {
  // Main window re-routes an already-open form instead of opening another.
  await listen<string>("route", (e) => route(new URLSearchParams(e.payload)));
  await route(new URLSearchParams((await store.takeRoute()) ?? ""));
});

async function save(spec: BotSpec) {
  const saved = await store.upsert(spec);
  if (saved) void getCurrentWindow().hide();
}

function cancel() {
  void getCurrentWindow().hide();
}
</script>

<template>
  <TitleBar :title="title" child />
  <div v-if="store.error && !loadError" class="banner banner-error notice" @click="store.error = null">
    {{ store.error }}
  </div>
  <div v-if="loadError" class="load-error">
    <p class="banner banner-error">{{ loadError }}</p>
    <button class="btn" @click="cancel">關閉</button>
  </div>
  <BotForm v-else-if="draft" :key="formKey" :draft="draft" :title="title" @save="save" @cancel="cancel" />
</template>

<style scoped>
.notice {
  margin: 10px 14px 0;
  flex: none;
}

.load-error {
  padding: 24px;
  display: flex;
  flex-direction: column;
  gap: 12px;
  align-items: flex-start;
}
</style>
