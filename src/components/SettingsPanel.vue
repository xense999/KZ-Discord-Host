<script setup lang="ts">
import { ref } from "vue";
import { useBotsStore } from "../stores/bots";
import { applyTheme, currentTheme } from "../theme";

const store = useBotsStore();
const dark = ref(currentTheme() === "dark");

function toggleTheme() {
  dark.value = !dark.value;
  applyTheme(dark.value ? "dark" : "light");
}
</script>

<template>
  <div class="settings">
    <h2>設定</h2>

    <div class="item">
      <div class="text">
        <div class="name">開機自動啟動</div>
        <div class="hint">登入 Windows 後管家靜默起在系統匣；有勾「隨管家啟動」的 bot 會自動拉起。</div>
      </div>
      <button class="switch" :class="{ on: store.hostAutostart }" @click="store.setHostAutostart(!store.hostAutostart)"></button>
    </div>

    <div class="item">
      <div class="text">
        <div class="name">暗色主題</div>
        <div class="hint">關掉就是暖色淺色版。</div>
      </div>
      <button class="switch" :class="{ on: dark }" @click="toggleTheme"></button>
    </div>

    <div class="item">
      <div class="text">
        <div class="name">Log 資料夾</div>
        <div class="hint">每隻 bot 一個 .log，超過 5 MB 自動輪替、保留三份。</div>
      </div>
      <button class="btn" @click="store.openLogsDir">開啟</button>
    </div>

    <div class="item">
      <div class="text">
        <div class="name">設定檔</div>
        <div class="hint mono">{{ store.configPath }}</div>
      </div>
    </div>

    <div class="item">
      <div class="text">
        <div class="name">關閉視窗</div>
        <div class="hint">按 X 只是藏到系統匣；要真的結束請在系統匣圖示按右鍵 → 結束（所有 bot 一起停止）。</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings {
  padding: 20px 24px;
  max-width: 640px;
}

h2 {
  margin: 0 0 8px;
  font-size: 18px;
  font-weight: 600;
  letter-spacing: -0.01em;
}

.item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 24px;
  padding: 16px 0;
  border-bottom: 1px solid var(--border);
}

.text {
  min-width: 0;
}

.name {
  font-weight: 500;
}

.mono {
  font-family: var(--font-mono);
  user-select: text;
  word-break: break-all;
}
</style>
