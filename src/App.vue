<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";
import MainView from "./components/MainView.vue";
import FormWindow from "./components/FormWindow.vue";
import SettingsWindow from "./components/SettingsWindow.vue";

// One bundle, several windows: each window shows the view named by its label
// (main / form / settings, see tray.rs).
const view = getCurrentWindow().label;
</script>

<template>
  <div class="shell">
    <FormWindow v-if="view === 'form'" />
    <SettingsWindow v-else-if="view === 'settings'" />
    <MainView v-else />
  </div>
</template>

<style scoped>
.shell {
  height: 100%;
  display: flex;
  flex-direction: column;
  border: 1px solid var(--border-strong);
  border-radius: var(--radius-window);
  overflow: hidden;
  background: var(--canvas);
  position: relative;
}
</style>
