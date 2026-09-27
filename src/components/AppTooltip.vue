<script setup lang="ts">
import { nextTick, onMounted, onUnmounted, ref } from "vue";

/**
 * App-wide hover tip for any element with `data-tip` (same as 久世管理器).
 * A fixed singleton rather than CSS ::after, so no ancestor's overflow can clip
 * it and it can keep itself inside the window edges.
 */
const GAP = 8;
const EDGE = 8;

const text = ref("");
const x = ref(0);
const y = ref(0);
const box = ref<HTMLElement | null>(null);

function place(target: HTMLElement) {
  const r = target.getBoundingClientRect();
  const b = box.value?.getBoundingClientRect();
  if (!b) return;
  const half = b.width / 2;
  const center = r.left + r.width / 2;
  x.value = Math.min(Math.max(center, EDGE + half), window.innerWidth - EDGE - half);
  const above = r.top - GAP - b.height;
  y.value = above >= EDGE ? above : r.bottom + GAP;
}

function onOver(e: MouseEvent) {
  const el = (e.target as HTMLElement | null)?.closest?.("[data-tip]") as HTMLElement | null;
  const tip = el?.dataset.tip;
  if (!el || !tip) {
    text.value = "";
    return;
  }
  text.value = tip;
  void nextTick(() => place(el));
}

function onOut(e: MouseEvent) {
  const from = (e.target as HTMLElement | null)?.closest?.("[data-tip]");
  const to = (e.relatedTarget as HTMLElement | null)?.closest?.("[data-tip]");
  if (from && from !== to) text.value = "";
}

function onScroll() {
  text.value = "";
}

onMounted(() => {
  document.addEventListener("mouseover", onOver);
  document.addEventListener("mouseout", onOut);
  document.addEventListener("scroll", onScroll, true);
});
onUnmounted(() => {
  document.removeEventListener("mouseover", onOver);
  document.removeEventListener("mouseout", onOut);
  document.removeEventListener("scroll", onScroll, true);
});
</script>

<template>
  <div v-show="text" ref="box" class="tip" :style="{ left: `${x}px`, top: `${y}px` }" aria-hidden="true">
    {{ text }}
  </div>
</template>

<style scoped>
.tip {
  position: fixed;
  z-index: 200;
  transform: translateX(-50%);
  padding: 7px 13px;
  font-size: 13px;
  font-weight: 600;
  line-height: 1.45;
  /* Short tips stay on one line; long explanations wrap inside the window. */
  width: max-content;
  max-width: min(340px, calc(100vw - 16px));
  white-space: normal;
  text-align: center;
  color: var(--text-on-accent);
  background: var(--accent);
  border-radius: var(--radius);
  pointer-events: none;
}
</style>
