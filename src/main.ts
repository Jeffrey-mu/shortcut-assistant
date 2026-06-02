import { createApp } from "vue";
import { createPinia } from "pinia";
import "./style.css";
import App from "./App.vue";
import { installBrowserGuards } from "./utils/browserGuards";

const app = createApp(App);
app.use(createPinia());
app.mount("#app");

installBrowserGuards();
