import 'primeicons/primeicons.css';
import "primevue/resources/themes/tailwind-light/theme.css";
import "primevue/resources/primevue.min.css";

import { createApp } from "vue";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import i18n from "./i18n";
import router from './router';
import "@/styles.css";
import hljs from 'highlight.js/lib/common';
import hljsVuePlugin from "@highlightjs/vue-plugin";

const app = createApp(App);
app.use(hljsVuePlugin)
app.use(i18n);
app.use(PrimeVue, { ripple: true });
app.use(router);
app.mount("#app");
