import { ref } from "vue";

/** Light / dark. CSS only reads <html data-theme>. First run is light, like the other 久世 apps. */
export type Theme = "light" | "dark";

const STORAGE_KEY = "kz-discord-host:theme";

function initial(): Theme {
  try {
    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved === "light" || saved === "dark") return saved;
  } catch {
    // storage unavailable: fall back to the default
  }
  return "light";
}

export const theme = ref<Theme>(initial());

export function applyTheme() {
  document.documentElement.dataset.theme = theme.value;
}

export function setTheme(t: Theme) {
  theme.value = t;
  try {
    localStorage.setItem(STORAGE_KEY, t);
  } catch {
    // storage unavailable: the choice lasts for this run only
  }
  applyTheme();
}
