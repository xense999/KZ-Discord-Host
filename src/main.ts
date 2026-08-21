import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./styles.css";
import { applyTheme, currentTheme, followThemeChanges } from "./theme";

applyTheme(currentTheme());
followThemeChanges();

createApp(App).use(createPinia()).mount("#app");
