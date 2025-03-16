import type { RouteRecordRaw } from "vue-router";
import Index from "../views/Index.vue";
import Dashboard from "../views/Dashboard.vue";
import Workspaces from "../views/Workspaces.vue";
import Templates from "../views/Templates.vue";

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
    },
    {
        path: "/templates",
        name: "templates",
        component: Templates
    }
];

export default routes;