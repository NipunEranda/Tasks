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
import Header from './components/Header.vue';

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import {
    faArrowLeft,
    faArrowTrendUp,
    faBars,
    faCircleCheck,
    faFilter,
    faGear,
    faHome,
    faLayerGroup,
    faList,
    faPlus,
    faPowerOff,
    faTags,
    faUser,
    faUserGroup,
    faXmark
} from "@fortawesome/free-solid-svg-icons";
import InfoSideBar from './components/InfoSideBar.vue';

library.add(faHome, faLayerGroup, faArrowLeft, faList, faArrowTrendUp, faUserGroup, faGear, faUser, faPowerOff, faTags, faXmark, faFilter, faBars, faCircleCheck, faPlus);

createApp(App)
    .use(router)
    .use(pinia)
    .use(i18n)
    .component("fai", FontAwesomeIcon)
    .component("WorkspaceModal", WorkspaceModal)
    .component("SideBar", SideBar)
    .component("Header", Header)
    .component("InfoSideBar", InfoSideBar)
    .mount('#app')
