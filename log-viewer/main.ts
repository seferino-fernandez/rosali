import "primeflex/primeflex.css";
import "primeicons/primeicons.css";
import "primevue/resources/themes/mdc-dark-indigo/theme.css";
import "primevue/resources/primevue.min.css";
import "@/styles.css";

import { createApp } from "vue";
import router from "../src/router";
import App from "./App.vue";
import PrimeVue from "primevue/config";
import i18n from "../src/i18n";
import Tooltip from "primevue/tooltip";

const app = createApp(App);
app.use(i18n);
app.use(PrimeVue, { ripple: true });
app.directive("tooltip", Tooltip);
app.use(router);
app.mount("#app");
