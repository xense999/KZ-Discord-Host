<script setup lang="ts">
import { onMounted } from "vue";
import { useBotsStore } from "../stores/bots";
import TitleBar from "./TitleBar.vue";
import SettingsPanel from "./SettingsPanel.vue";

const store = useBotsStore();
onMounted(() => store.loadSettings());
</script>

<template>
  <TitleBar title="設定" child />
  <div v-if="store.error" class="banner banner-error notice" @click="store.error = null">
    {{ store.error }}
  </div>
  <div class="scroll">
    <SettingsPanel />
  </div>
</template>

<style scoped>
.notice {
  margin: 10px 14px 0;
  flex: none;
}

.scroll {
  flex: 1;
  min-height: 0;
  overflow-y: auto;
  background: var(--surface);
}
</style>
