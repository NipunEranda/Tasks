import { createApp } from 'vue';
import './assets/css/style.css';
import App from './App.vue';
import router from "./router";
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import 'flowbite';
import i18n from "./locales/i18n";

import WorkspaceModal from './components/modals/WorkspaceModal.vue';
import SideBar from './components/SideBar.vue';

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import {
    faArrowLeft,
    faArrowTrendUp,
    faGear,
    faHome,
    faLayerGroup,
    faList,
    faUserGroup
} from "@fortawesome/free-solid-svg-icons";

library.add(faHome, faLayerGroup, faArrowLeft, faList, faArrowTrendUp, faUserGroup, faGear);

createApp(App)
    .use(router)
    .use(pinia)
    .use(i18n)
    .component("fai", FontAwesomeIcon)
    .component("WorkspaceModal", WorkspaceModal)
    .component("SideBar", SideBar)
    .mount('#app')
