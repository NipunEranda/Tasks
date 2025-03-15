import type { RouteRecordRaw } from "vue-router";
import Index from "../views/Index.vue";
import Dashboard from "../views/Dashboard.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: "/",
        name: "index",
        component: Index,
    },
    {
        path: "/dashboard",
        name: "dashboard",
        component: Dashboard,
    }
];

export default routes;