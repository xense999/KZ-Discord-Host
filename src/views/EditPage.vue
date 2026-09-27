<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import { useBotsStore } from "../stores/bots";
import type { Page } from "../stores/bots";
import type { BotSpec, EnvVar } from "../types";

const props = defineProps<{ page: Extract<Page, { kind: "edit" }> }>();
const store = useBotsStore();

const draft = props.page.draft;
const form = reactive<BotSpec>({ ...draft, env: draft.env.map((e) => ({ ...e })) });
const revealed = ref<boolean[]>(draft.env.map(() => false));

const missingSecrets = computed(() => form.env.filter((e) => e.name.trim() && !e.value).map((e) => e.name.trim()));

async function pickExe() {
  const file = await open({ multiple: false, filters: [{ name: "執行檔", extensions: ["exe"] }], title: "選擇 bot 執行檔" });
  if (typeof file === "string") form.exe = file;
}

function addEnv() {
  form.env.push({ name: "", value: "", secret: true });
  revealed.value.push(false);
}

function removeEnv(i: number) {
  form.env.splice(i, 1);
  revealed.value.splice(i, 1);
}

async function save() {
  if (!form.name.trim()) return void (store.error = "名稱不可為空");
  if (!form.exe.trim()) return void (store.error = "執行檔不可為空");
  // Every value is treated as secret (masked in the UI); there is no per-row switch.
  const env: EnvVar[] = form.env.filter((e) => e.name.trim()).map((e) => ({ ...e, name: e.name.trim(), secret: true }));
  // Arguments and working directory are not editable here (exe bots only);
  // values that came from bot.toml ride along unchanged in `form`.
  const spec: BotSpec = { ...form, name: form.name.trim(), exe: form.exe.trim(), env };
  const saved = await store.upsert(spec);
  if (!saved) return;
  await store.select(saved.id);
  store.home();
}
</script>

<template>
  <section class="card page">
    <div class="bar">
      <button class="btn-browse" data-tip="不儲存，回到清單" @click="store.home()">返回</button>
      <span class="bar-title">{{ page.title }}</span>
      <div class="spacer"></div>
      <button class="primary sm" @click="save">儲存</button>
    </div>

    <div class="set-scroll">
      <p v-if="missingSecrets.length" class="warn">還要填：{{ missingSecrets.join("、") }}</p>

      <div class="set-card">
        <div class="set-row">
          <span class="set-title fixed">名稱</span>
          <input v-model="form.name" type="text" class="path-input" placeholder="顯示在清單上的名字" spellcheck="false" />
          <span class="set-title">開啟時自啟</span>
          <button
            class="pill-switch"
            role="switch"
            :class="{ on: form.autostart }"
            :aria-checked="form.autostart"
            :data-tip="form.autostart ? '開啟程式時自動啟動：開' : '開啟程式時自動啟動：關'"
            @click="form.autostart = !form.autostart"
          >
            <span class="pill-knob"></span>
          </button>
        </div>
        <div class="set-sep"></div>
        <div class="set-row">
          <span class="set-title fixed">執行檔</span>
          <input v-model="form.exe" type="text" class="path-input" placeholder="C:\bots\my-bot\bot.exe" spellcheck="false" />
          <button class="btn-browse" @click="pickExe">瀏覽</button>
        </div>
      </div>

      <div class="set-card">
        <div class="set-row">
          <span class="set-title" data-tip="token 之類的設定放這裡；值平常以圓點遮住，按「顯示」才看得到">環境變數</span>
          <button class="btn-browse" @click="addEnv">新增一列</button>
        </div>
        <template v-for="(e, i) in form.env" :key="i">
          <div class="set-sep"></div>
          <div class="path-row env">
            <input v-model="e.name" type="text" class="path-input env-name" placeholder="名稱" spellcheck="false" />
            <input
              v-model="e.value"
              :type="revealed[i] ? 'text' : 'password'"
              class="path-input"
              :placeholder="e.description || '值'"
              autocomplete="off"
              spellcheck="false"
            />
            <button class="btn-browse" @click="revealed[i] = !revealed[i]">{{ revealed[i] ? "隱藏" : "顯示" }}</button>
            <button class="btn-browse danger" data-tip="刪除這一列" @click="removeEnv(i)">刪除</button>
          </div>
        </template>
      </div>
    </div>
  </section>
</template>

<style scoped>
.page {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.bar {
  height: 56px;
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-3);
  padding: 0 var(--sp-4);
  border-bottom: 0.5px solid var(--border);
}
/* The inner cards sit one step darker so they still read as cards inside the outer one. */
.page .set-card {
  background: var(--bg-2);
}
.bar-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-strong);
}
/* Breathing room between the bar and the first card. */
.page .set-scroll {
  padding: 16px;
}
.warn {
  font-size: 14px;
  font-weight: 600;
  color: var(--warn);
}
.env {
  padding-top: 10px;
}
.env-name {
  flex: 0 0 120px;
}
.btn-browse.danger:hover:not(:disabled) {
  color: var(--danger);
  background: var(--danger-soft);
}
</style>
