import type { RouteRecordRaw } from "vue-router";
import Index from "../views/Index.vue";
import Dashboard from "../views/Dashboard.vue";
import Workspaces from "../views/Workspaces.vue";

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
    },
    {
        path: "/workspaces",
        name: "workspaces",
        component: Workspaces
    }
];

export default routes;