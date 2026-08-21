import { emit, listen } from "@tauri-apps/api/event";

export type Theme = "dark" | "light";
const EVENT_THEME = "theme-changed";

const STORAGE_KEY = "kz-bot-host.theme";
const DEFAULT_THEME: Theme = "dark";

export function currentTheme(): Theme {
  try {
    return localStorage.getItem(STORAGE_KEY) === "light" ? "light" : DEFAULT_THEME;
  } catch {
    return DEFAULT_THEME;
  }
}

export function applyTheme(theme: Theme) {
  document.documentElement.setAttribute("data-theme", theme);
  try {
    localStorage.setItem(STORAGE_KEY, theme);
  } catch {
    // storage unavailable: theme still applies for this session
  }
}

/** Apply here and tell every other window. */
export function setTheme(theme: Theme) {
  applyTheme(theme);
  void emit(EVENT_THEME, theme);
}

export function followThemeChanges() {
  void listen<Theme>(EVENT_THEME, (e) => applyTheme(e.payload));
}
