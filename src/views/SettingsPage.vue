<script setup lang="ts">
import { ref } from "vue";
import { useBotsStore } from "../stores/bots";

const store = useBotsStore();
// Set-and-forget items stay folded to their title until clicked, as in 久世登入器.
const configOpen = ref(false);
</script>

<template>
  <div class="set-scroll">
    <div class="set-card">
      <div class="set-row">
        <span class="set-title" data-tip="登入 Windows 後靜默起在系統匣；有開「開啟程式時自動啟動」的 bot 會自動拉起。">開機自動啟動</span>
        <button
          class="pill-switch"
          role="switch"
          :class="{ on: store.hostAutostart }"
          :aria-checked="store.hostAutostart"
          @click="store.setHostAutostart(!store.hostAutostart)"
        >
          <span class="pill-knob"></span>
        </button>
      </div>
    </div>

    <div class="set-card">
      <div class="set-row">
        <span class="set-title" data-tip="每隻 bot 一個 .log，超過 5 MB 自動輪替、保留三份。">Log 資料夾</span>
        <button class="btn-browse" @click="store.openLogsDir">開啟</button>
      </div>
    </div>

    <div class="set-card" :class="{ unfolded: configOpen }">
      <div class="set-row foldhead" @click="configOpen = !configOpen">
        <span class="set-title" data-tip="所有 bot 的設定（含 token）都存在這個檔案，換電腦時整個複製過去即可。">設定檔</span>
        <svg class="foldchev" viewBox="0 0 12 12" width="12" height="12" aria-hidden="true">
          <path d="M2.5 4.5 6 8l3.5-3.5" fill="none" stroke="currentColor" stroke-width="1.6" stroke-linecap="round" stroke-linejoin="round" />
        </svg>
      </div>
      <template v-if="configOpen">
        <div class="set-sep"></div>
        <div class="path-row">
          <input class="path-input" :value="store.configPath" readonly spellcheck="false" />
        </div>
      </template>
    </div>
  </div>
</template>
