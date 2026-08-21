<script setup lang="ts">
import { computed, reactive, ref } from "vue";
import { open } from "@tauri-apps/plugin-dialog";
import type { BotSpec, EnvVar } from "../types";

const props = defineProps<{ draft: BotSpec; title: string }>();
const emit = defineEmits<{ save: [spec: BotSpec]; cancel: [] }>();

const form = reactive<BotSpec>({ ...props.draft, env: props.draft.env.map((e) => ({ ...e })) });
const argsText = ref(props.draft.args.join("\n"));
const revealed = ref<boolean[]>(props.draft.env.map(() => false));
const localError = ref<string | null>(null);

const missingSecrets = computed(() => form.env.filter((e) => e.secret && !e.value).map((e) => e.name));

async function pickExe() {
  const file = await open({
    multiple: false,
    filters: [{ name: "執行檔", extensions: ["exe"] }],
    title: "選擇 bot 執行檔",
  });
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

function submit() {
  localError.value = null;
  if (!form.name.trim()) return (localError.value = "名稱不可為空");
  if (!form.exe.trim()) return (localError.value = "執行檔路徑不可為空");
  const env: EnvVar[] = form.env
    .filter((e) => e.name.trim())
    .map((e) => ({ ...e, name: e.name.trim() }));
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
  emit("save", spec);
}
</script>

<template>
  <form class="form" @submit.prevent="submit">
    <header class="head">
      <h2>{{ title }}</h2>
      <div class="ops">
        <button type="button" class="btn btn-ghost" @click="emit('cancel')">取消</button>
        <button type="submit" class="btn btn-primary">儲存</button>
      </div>
    </header>

    <div v-if="localError" class="banner banner-error">{{ localError }}</div>
    <div v-else-if="missingSecrets.length" class="banner banner-warn">
      還要補填：{{ missingSecrets.join("、") }}
    </div>

    <div class="grid">
      <div class="field">
        <label>名稱</label>
        <input v-model="form.name" class="input" placeholder="顯示在清單的名字" />
      </div>

      <div class="field">
        <label>執行檔</label>
        <div class="row">
          <input v-model="form.exe" class="input mono" placeholder="C:\path\to\bot.exe" />
          <button type="button" class="btn" @click="pickExe">瀏覽</button>
        </div>
      </div>

      <div class="field">
        <label>參數（每行一個）</label>
        <textarea v-model="argsText" class="textarea" rows="2"></textarea>
      </div>

      <div class="field">
        <label>工作目錄</label>
        <div class="row">
          <input v-model="form.cwd" class="input mono" placeholder="留空＝執行檔所在資料夾" />
          <button type="button" class="btn" @click="pickCwd">瀏覽</button>
        </div>
      </div>

      <div class="field">
        <div class="env-head">
          <label>環境變數</label>
          <button type="button" class="btn btn-ghost btn-sm" @click="addEnv">新增一列</button>
        </div>
        <div v-if="form.env.length === 0" class="hint">沒有環境變數。</div>
        <div v-for="(e, i) in form.env" :key="i" class="env-row">
          <input v-model="e.name" class="input mono" placeholder="NAME" />
          <div class="value">
            <input
              v-model="e.value"
              class="input mono"
              :type="e.secret && !revealed[i] ? 'password' : 'text'"
              :placeholder="e.description || 'value'"
              autocomplete="off"
            />
            <button
              v-if="e.secret"
              type="button"
              class="btn btn-ghost btn-sm"
              @click="revealed[i] = !revealed[i]"
            >
              {{ revealed[i] ? "隱藏" : "顯示" }}
            </button>
          </div>
          <label class="check" :title="'秘密：UI 遮罩顯示'">
            <input type="checkbox" v-model="e.secret" /> 秘密
          </label>
          <button type="button" class="btn btn-ghost btn-sm btn-danger" @click="removeEnv(i)">刪</button>
          <div v-if="e.description" class="hint env-hint">{{ e.description }}</div>
        </div>
      </div>

      <label class="check autostart">
        <input type="checkbox" v-model="form.autostart" /> 隨管家啟動
      </label>
    </div>
  </form>
</template>

<style scoped>
.form {
  flex: 1;
  min-height: 0;
  display: flex;
  flex-direction: column;
  overflow-y: auto;
}

.head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 20px 24px 12px;
}

h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.ops {
  display: flex;
  gap: 8px;
}

.banner {
  margin: 0 24px 8px;
}

.grid {
  display: flex;
  flex-direction: column;
  gap: 18px;
  padding: 8px 24px 32px;
  max-width: 720px;
}

.row {
  display: flex;
  gap: 8px;
}

.env-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.env-row {
  display: grid;
  grid-template-columns: 180px 1fr auto auto;
  gap: 8px;
  align-items: center;
  padding: 8px 0;
  border-bottom: 1px solid var(--border);
}

.env-row .value {
  display: flex;
  gap: 6px;
  min-width: 0;
}

.env-hint {
  grid-column: 1 / -1;
}

.check {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  font-size: 13px;
  color: var(--text-muted);
  white-space: nowrap;
}

.autostart {
  font-size: 14px;
  color: var(--text);
}
</style>
