import { createRouter, createWebHistory } from "vue-router";
import routes from "./routes";
import { useIndexStore } from "../store";
import { useWorkspaceStore } from "../store/workspace";

const router = createRouter({
    history: createWebHistory(),
    routes,
});

router.beforeEach((to /*, from*/) => {
    const indexStore = useIndexStore();
    const workspaceStore = useWorkspaceStore();

    if (indexStore.loggedIn && indexStore.currentUser) {
        if (to.name != "index") {
            if (workspaceStore.activeWorkspace == "") {
                if (to.name != "workspaces")
                    router.push("/workspaces");

                return;
            } else {
                if (to.name == "workspaces")
                    router.push("/dashboard");
            }
        } else {
            if (workspaceStore.activeWorkspace != "")
                router.push("/dashboard");
            else
                router.push("/workspaces");
        }
    } else {
        if (to.name != "index")
            router.push("/");
    }
});

export default router;