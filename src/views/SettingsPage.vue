<script setup lang="ts">
import { useBotsStore } from "../stores/bots";

const store = useBotsStore();
</script>

<template>
  <div class="settings">
    <h2>設定</h2>

    <div class="item">
      <div class="text">
        <div class="name">開機自動啟動</div>
        <div class="hint">登入 Windows 後靜默起在系統匣；有開「開啟程式時自動啟動」的 bot 會自動拉起。</div>
      </div>
      <button class="switch" :class="{ on: store.hostAutostart }" @click="store.setHostAutostart(!store.hostAutostart)"></button>
    </div>

    <div class="item">
      <div class="text">
        <div class="name">Log 資料夾</div>
        <div class="hint">每隻 bot 一個 .log，超過 5 MB 自動輪替、保留三份。</div>
      </div>
      <button @click="store.openLogsDir">開啟</button>
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
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: 20px 24px;
}

h2 {
  margin: 0 0 8px;
  font-size: 18px;
  font-weight: 600;
  color: var(--text-strong);
}

.item {
  max-width: 640px;
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
  color: var(--text-strong);
}

.hint {
  font-size: 14px;
  color: var(--text-faint);
}

.mono {
  user-select: text;
  word-break: break-all;
}
</style>
