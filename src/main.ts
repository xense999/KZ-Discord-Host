import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "./styles.css";
import { applyTheme, currentTheme } from "./theme";

applyTheme(currentTheme());

createApp(App).use(createPinia()).mount("#app");
