<script setup lang="ts">
import { watch } from "vue";
import { useBotsStore } from "../stores/bots";

/** Errors from any action, shown top-centre instead of a native dialog; click or wait to dismiss. */
const store = useBotsStore();
const SHOW_MS = 5000;
let timer: number | undefined;

watch(
  () => store.error,
  (msg) => {
    window.clearTimeout(timer);
    if (msg) timer = window.setTimeout(() => (store.error = null), SHOW_MS);
  },
);
</script>

<template>
  <Transition name="toast">
    <div v-if="store.error" class="toast" @click="store.error = null">{{ store.error }}</div>
  </Transition>
</template>

<style scoped>
.toast {
  position: fixed;
  top: 56px;
  left: 50%;
  z-index: 150;
  transform: translateX(-50%);
  max-width: calc(100% - 48px);
  padding: 9px 18px;
  font-size: 15px;
  font-weight: 600;
  color: #fff;
  background: var(--danger);
  border-radius: var(--radius-pill);
  cursor: pointer;
}
.toast-enter-active,
.toast-leave-active {
  transition: opacity 0.18s ease, transform 0.18s ease;
}
.toast-enter-from,
.toast-leave-to {
  opacity: 0;
  transform: translate(-50%, -6px);
}
</style>
