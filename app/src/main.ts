import { createApp } from 'vue';
import './assets/css/style.css';
import App from './App.vue';
import router from "./router";
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import 'flowbite';

import WorkspaceModal from './components/modals/WorkspaceModal.vue';

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import {
    faHome,
    faLayerGroup
} from "@fortawesome/free-solid-svg-icons";

library.add(faHome, faLayerGroup);

createApp(App)
    .use(router)
    .use(pinia)
    .component("fai", FontAwesomeIcon)
    .component("WorkspaceModal", WorkspaceModal)
    .mount('#app')
