import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./styles.css";
import { applyTheme } from "./theme";
import { lockScale } from "./scale";

applyTheme();

// No browser context menu in a desktop app, except in text fields (right-click paste).
document.addEventListener("contextmenu", (e) => {
  if ((e.target as HTMLElement | null)?.closest("input, textarea")) return;
  e.preventDefault();
});

// Block WebView2's browser shortcuts (F5, Ctrl+F, Ctrl+P, Alt+Left...) by
// allow-list, as 久世管理器 does: a deny-list misses whatever a new WebView2 adds.
// Only preventDefault, so the app's own key handling still sees the keys.
const EDIT_KEYS = new Set(["c", "v", "x", "a", "z", "y"]);
const CARET_KEYS = new Set(["ArrowLeft", "ArrowRight", "ArrowUp", "ArrowDown", "Home", "End", "Backspace", "Delete", "Insert"]);
document.addEventListener(
  "keydown",
  (e) => {
    if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return;
    if (e.getModifierState("AltGraph")) return;
    const fkey = /^F\d{1,2}$/.test(e.key);
    if (!fkey && !e.ctrlKey && !e.altKey && !e.metaKey) return;
    if (e.key === "F10" && e.shiftKey && !e.ctrlKey && !e.altKey) return;
    if (import.meta.env.DEV && (e.key === "F12" || (e.ctrlKey && e.shiftKey && e.key.toLowerCase() === "i"))) return;
    if (e.altKey && !e.ctrlKey && !e.metaKey) {
      if (e.key === "F4" || e.code === "Space" || /^Numpad\d$/.test(e.code)) return;
    }
    if (e.ctrlKey && !e.altKey && !e.metaKey) {
      const k = e.key.toLowerCase();
      if (CARET_KEYS.has(e.key)) return;
      if (EDIT_KEYS.has(k) && (!e.shiftKey || k === "z" || k === "v")) return;
    }
    e.preventDefault();
  },
  true,
);

createApp(App).use(createPinia()).mount("#app");
void lockScale();
