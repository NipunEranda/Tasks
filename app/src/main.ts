import { createApp } from "vue";
import "./assets/css/style.css";
import App from "./App.vue";
import router from "./router";
import { createPinia } from "pinia";
import piniaPluginPersistedstate from "pinia-plugin-persistedstate";
import "flowbite";
import i18n from "./locales/i18n";

import WorkspaceModal from "./components/modals/WorkspaceModal.vue";
import TagsModal from "./components/modals/TagsModal.vue";
import ActionModal from "./components/modals/ActionModal.vue";
import SideBar from "./components/SideBar.vue";
import Header from "./components/Header.vue";
import InfoSideBar from "./components/InfoSideBar.vue";

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";

import {
  faArrowLeft,
  faArrowTrendUp,
  faBars,
  faCircleCheck,
  faCircleInfo,
  faCircleUser,
  faEye,
  faEyeSlash,
  faFilter,
  faGear,
  faHome,
  faLayerGroup,
  faList,
  faLock,
  faLockOpen,
  faPen,
  faPlus,
  faPowerOff,
  faSave,
  faSearch,
  faTag,
  faTags,
  faTrashCan,
  faUser,
  faUserGroup,
  faXmark,
} from "@fortawesome/free-solid-svg-icons";
import NewTemplate from "./components/containers/NewTemplate.vue";
import TaskCard from "./components/cards/TaskCard.vue";
import SubTaskCard from "./components/cards/SubTaskCard.vue";

library.add(
  faHome,
  faLayerGroup,
  faArrowLeft,
  faList,
  faArrowTrendUp,
  faUserGroup,
  faGear,
  faUser,
  faPowerOff,
  faTags,
  faTag,
  faXmark,
  faFilter,
  faBars,
  faCircleCheck,
  faPlus,
  faEye,
  faEyeSlash,
  faPen,
  faCircleInfo,
  faTrashCan,
  faCircleUser,
  faSave,
  faSearch,
  faLock,
  faLockOpen
);

createApp(App)
  .use(router)
  .use(pinia)
  .use(i18n)
  .component("fai", FontAwesomeIcon)
  .component("WorkspaceModal", WorkspaceModal)
  .component("TagsModal", TagsModal)
  .component("ActionModal", ActionModal)
  .component("SideBar", SideBar)
  .component("Header", Header)
  .component("InfoSideBar", InfoSideBar)
  .component("NewTemplate", NewTemplate)
  .component("TaskCard", TaskCard)
  .component("SubTaskCard", SubTaskCard)
  .mount("#app");
