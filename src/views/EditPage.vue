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
        </div>
        <div class="set-sep"></div>
        <div class="set-row">
          <span class="set-title fixed">執行檔</span>
          <input v-model="form.exe" type="text" class="path-input" placeholder="C:\bots\my-bot\bot.exe" spellcheck="false" />
          <button class="btn-browse" @click="pickExe">瀏覽</button>
        </div>
        <div class="set-sep"></div>
        <div class="set-row">
          <span class="set-title" data-tip="久世 Discord Host 一開就把這隻 bot 拉起來">開啟程式時自動啟動</span>
          <button class="pill-switch" role="switch" :class="{ on: form.autostart }" :aria-checked="form.autostart" @click="form.autostart = !form.autostart">
            <span class="pill-knob"></span>
          </button>
        </div>
      </div>

      <div class="set-card">
        <div class="set-row">
          <span class="set-title" data-tip="token 之類的設定放這裡；設成機密的值平常以圓點遮住">環境變數</span>
          <button class="btn-browse" @click="addEnv">新增一列</button>
        </div>
        <template v-for="(e, i) in form.env" :key="i">
          <div class="set-sep"></div>
          <div class="path-row env">
            <input v-model="e.name" type="text" class="path-input env-name" placeholder="名稱" spellcheck="false" />
            <input
              v-model="e.value"
              :type="e.secret && !revealed[i] ? 'password' : 'text'"
              class="path-input"
              :placeholder="e.description || '值'"
              autocomplete="off"
              spellcheck="false"
            />
            <button v-if="e.secret" class="btn-browse" @click="revealed[i] = !revealed[i]">{{ revealed[i] ? "隱藏" : "顯示" }}</button>
            <button class="btn-browse" :class="{ lock: e.secret }" data-tip="機密：值平常以圓點遮住" @click="e.secret = !e.secret">機密</button>
            <button class="btn-browse danger" data-tip="刪除這一列" @click="removeEnv(i)">刪除</button>
          </div>
        </template>
      </div>

      <div class="set-card" :class="{ unfolded: advanced }">
        <div class="set-row foldhead" @click="advanced = !advanced">
          <span class="set-title" data-tip="啟動參數、工作目錄；大部分 bot 不需要">進階</span>
          <svg class="foldchev" viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
            <path d="M2.5 4.5 6 8l3.5-3.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
          </svg>
        </div>
        <template v-if="advanced">
          <div class="set-sep"></div>
          <div class="set-row">
            <span class="set-title fixed" data-tip="一行一個">啟動參數</span>
            <textarea v-model="argsText" rows="2" class="path-input" spellcheck="false"></textarea>
          </div>
          <div class="set-sep"></div>
          <div class="set-row">
            <span class="set-title fixed">工作目錄</span>
            <input v-model="form.cwd" type="text" class="path-input" placeholder="留空＝執行檔所在的資料夾" spellcheck="false" />
            <button class="btn-browse" @click="pickCwd">瀏覽</button>
          </div>
        </template>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
}
.bar {
  height: 48px;
  flex: none;
  display: flex;
  align-items: center;
  gap: var(--sp-3);
}
.bar-title {
  font-size: 15px;
  font-weight: 600;
  color: var(--text-strong);
}
.page .set-scroll {
  padding: 0 0 12px;
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
.lock {
  color: var(--accent);
  border-color: var(--accent);
}
.btn-browse.danger:hover:not(:disabled) {
  color: var(--danger);
  background: var(--danger-soft);
}
</style>
