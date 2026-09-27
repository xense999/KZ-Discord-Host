<script setup lang="ts">
import { onMounted } from "vue";
import AppTitlebar from "./components/AppTitlebar.vue";
import AppTooltip from "./components/AppTooltip.vue";
import ToastPop from "./components/ToastPop.vue";
import SettingsPage from "./views/SettingsPage.vue";
import { useBotsStore } from "./stores/bots";

const store = useBotsStore();
onMounted(() => store.loadSettings());
</script>

<template>
  <div class="winframe">
    <AppTooltip />
    <ToastPop />
    <AppTitlebar title="設定" child />
    <SettingsPage />
  </div>
</template>
