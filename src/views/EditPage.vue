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
const argsText = ref(draft.args.join("\n"));
const revealed = ref<boolean[]>(draft.env.map(() => false));
// Arguments and working directory are rarely needed; keep them folded unless already set.
const advanced = ref(draft.args.length > 0 || !!draft.cwd);

const missingSecrets = computed(() => form.env.filter((e) => e.secret && !e.value).map((e) => e.name));

async function pickExe() {
  const file = await open({ multiple: false, filters: [{ name: "執行檔", extensions: ["exe", "bat", "cmd"] }], title: "選擇 bot 執行檔" });
  if (typeof file === "string") form.exe = file;
}

async function pickCwd() {
  const dir = await open({ directory: true, multiple: false, title: "選擇工作目錄" });
  if (typeof dir === "string") form.cwd = dir;
}

function addEnv() {
  form.env.push({ name: "", value: "", secret: false });
  revealed.value.push(false);
}

function removeEnv(i: number) {
  form.env.splice(i, 1);
  revealed.value.splice(i, 1);
}

async function save() {
  if (!form.name.trim()) return void (store.error = "名稱不可為空");
  if (!form.exe.trim()) return void (store.error = "執行檔不可為空");
  const env: EnvVar[] = form.env.filter((e) => e.name.trim()).map((e) => ({ ...e, name: e.name.trim() }));
  const spec: BotSpec = {
    ...form,
    name: form.name.trim(),
    exe: form.exe.trim(),
    cwd: form.cwd?.trim() ? form.cwd.trim() : undefined,
    args: argsText.value
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean),
    env,
  };
  const saved = await store.upsert(spec);
  if (!saved) return;
  await store.select(saved.id);
  store.home();
}
</script>

<template>
  <div class="page">
    <div class="bar">
      <button class="plain back" data-tip="不儲存，回到清單" @click="store.home()">
        <svg viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path d="M7.5 2.5 4 6l3.5 3.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
        返回
      </button>
      <span class="bar-title">{{ page.title }}</span>
      <div class="spacer"></div>
      <button class="primary" @click="save">儲存</button>
    </div>

    <div class="body">
      <p v-if="missingSecrets.length" class="warn">還要填：{{ missingSecrets.join("、") }}</p>

      <section class="card">
        <div class="field">
          <label>名稱</label>
          <input v-model="form.name" type="text" placeholder="顯示在清單上的名字" />
        </div>
        <div class="field">
          <label>執行檔</label>
          <div class="inline">
            <input v-model="form.exe" type="text" class="mono" placeholder="C:\bots\my-bot\bot.exe" />
            <button @click="pickExe">瀏覽</button>
          </div>
        </div>
        <div class="row">
          <span class="row-title">
            開啟程式時自動啟動
            <span class="row-sub">久世 Discord Host 一開就把這隻 bot 拉起來</span>
          </span>
          <button class="switch" role="switch" :class="{ on: form.autostart }" :aria-checked="form.autostart" @click="form.autostart = !form.autostart"></button>
        </div>
      </section>

      <section class="card">
        <div class="card-head">
          <span>環境變數</span>
          <span class="head-sub">token 之類的設定放這裡</span>
          <div class="spacer"></div>
          <button class="plain sm" @click="addEnv">新增一列</button>
        </div>
        <p v-if="form.env.length === 0" class="none">沒有環境變數。</p>
        <div v-for="(e, i) in form.env" :key="i" class="env">
          <input v-model="e.name" type="text" class="mono env-name" placeholder="名稱，例如 DISCORD_TOKEN" />
          <input
            v-model="e.value"
            :type="e.secret && !revealed[i] ? 'password' : 'text'"
            class="mono env-value"
            :placeholder="e.description || '值'"
            autocomplete="off"
          />
          <button v-if="e.secret" class="plain sm" @click="revealed[i] = !revealed[i]">{{ revealed[i] ? "隱藏" : "顯示" }}</button>
          <button
            class="plain sm"
            :class="{ lock: e.secret }"
            :data-tip="e.secret ? '機密：值平常以圓點遮住' : '設為機密：值平常以圓點遮住'"
            @click="e.secret = !e.secret"
          >
            機密
          </button>
          <button class="plain sm danger" data-tip="刪除這一列" @click="removeEnv(i)">刪除</button>
        </div>
      </section>

      <section class="card">
        <button class="fold" :aria-expanded="advanced" @click="advanced = !advanced">
          <span>進階</span>
          <span class="head-sub">啟動參數、工作目錄</span>
          <div class="spacer"></div>
          <svg :class="{ open: advanced }" viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
            <path d="M2.5 4.5 6 8l3.5-3.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </button>
        <template v-if="advanced">
          <div class="field">
            <label>啟動參數（一行一個）</label>
            <textarea v-model="argsText" rows="2" class="mono"></textarea>
          </div>
          <div class="field">
            <label>工作目錄</label>
            <div class="inline">
              <input v-model="form.cwd" type="text" class="mono" placeholder="留空＝執行檔所在的資料夾" />
              <button @click="pickCwd">瀏覽</button>
            </div>
          </div>
        </template>
      </section>
    </div>
  </div>
</template>

<style scoped>
.page {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
}
.bar {
  height: 52px;
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: 0 var(--sp-3);
  border-bottom: 0.5px solid var(--border);
}
.back {
  padding: 0 10px;
}
.bar-title {
  font-size: 17px;
  font-weight: 700;
  color: var(--text-strong);
}
.body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}
.warn {
  font-size: 15px;
  font-weight: 600;
  color: var(--warn);
}
.card {
  flex: none;
}

.field {
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: var(--sp-3) var(--sp-4);
}
.field + .field,
.field + .row {
  border-top: 0.5px solid var(--border);
}
.field label {
  font-size: 14px;
  font-weight: 600;
  color: var(--text-dim);
}
.inline {
  display: flex;
  gap: var(--sp-2);
}
.inline input {
  flex: 1;
  min-width: 0;
}

.head-sub {
  font-size: 13px;
  font-weight: 500;
  color: var(--text-faint);
}
.none {
  padding: var(--sp-3) var(--sp-4);
  font-size: 15px;
  color: var(--text-faint);
}
.env {
  display: flex;
  align-items: center;
  gap: var(--sp-2);
  padding: var(--sp-2) var(--sp-4);
}
.env + .env {
  border-top: 0.5px solid var(--border);
}
.env-name {
  width: 220px;
  flex: none;
}
.env-value {
  flex: 1;
  min-width: 0;
}
.lock {
  color: var(--accent);
  background: var(--accent-soft);
}

.fold {
  width: 100%;
  height: 48px;
  justify-content: flex-start;
  gap: var(--sp-2);
  padding: 0 var(--sp-4);
  font-size: 16px;
  font-weight: 600;
  color: var(--text-strong);
  background: transparent;
  border: none;
  border-radius: 0;
}
.fold:hover:not(:disabled) {
  background: var(--hover);
}
.fold svg {
  color: var(--text-dim);
  transition: transform 0.15s ease;
}
.fold svg.open {
  transform: rotate(180deg);
}
.fold + .field {
  border-top: 0.5px solid var(--border);
}
</style>
