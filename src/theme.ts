export type Theme = "dark" | "light";

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
