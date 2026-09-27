<script setup lang="ts">
import { useBotsStore } from "../stores/bots";
import { setTheme, theme } from "../theme";

const store = useBotsStore();
</script>

<template>
  <div class="body">
    <section class="card">
      <div class="row">
        <span class="row-title">主題</span>
        <div class="seg">
          <button :class="{ on: theme === 'light' }" @click="setTheme('light')">淺色</button>
          <button :class="{ on: theme === 'dark' }" @click="setTheme('dark')">深色</button>
        </div>
      </div>
      <div class="row">
        <span class="row-title">
          開機自動啟動
          <span class="row-sub">登入 Windows 後直接收在系統匣，有開「自動啟動」的 bot 會一起拉起來</span>
        </span>
        <button
          class="switch"
          role="switch"
          :class="{ on: store.hostAutostart }"
          :aria-checked="store.hostAutostart"
          @click="store.setHostAutostart(!store.hostAutostart)"
        ></button>
      </div>
      <div class="row">
        <span class="row-title">
          關閉視窗
          <span class="row-sub">按 X 只是收進系統匣，bot 會繼續跑；對系統匣圖示按右鍵 →「結束」才會全部停止</span>
        </span>
      </div>
    </section>

    <section class="card">
      <div class="row">
        <span class="row-title">
          紀錄資料夾
          <span class="row-sub">每隻 bot 一個檔，超過 5 MB 自動換新檔、保留三份</span>
        </span>
        <button @click="store.openLogsDir">開啟</button>
      </div>
      <div class="row">
        <span class="row-title">
          設定檔
          <span class="row-sub mono path" :data-tip="store.configPath">{{ store.configPath }}</span>
        </span>
      </div>
    </section>
  </div>
</template>

<style scoped>
.body {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  padding: var(--sp-4);
  display: flex;
  flex-direction: column;
  gap: var(--sp-3);
}
.card {
  flex: none;
}
.path {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}
</style>
