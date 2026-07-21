import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import "virtual:uno.css";
import "./styles/tokens.css";
import "./styles/global.css";

createApp(App).use(createPinia()).mount("#app");
