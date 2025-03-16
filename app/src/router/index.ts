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
        //     if (to.path == "/") {
        //         if (workspaceStore.activeWorkspace != "")
        //             return "/dashboard";
        //         else
        //             return "/workspaces";
        //     }

        if (to.name != "index"){
            if(to.name != "workspaces")
                router.push("/workspaces");
            return;
        }else {
            if (workspaceStore.activeWorkspace != "")
                router.push("/dashboard");
            else
                router.push("/workspaces");
        }

        // if (to.name == "workspaces")
        //     if (workspaceStore.activeWorkspace != "")
        //         router.push("/dashboard");
        //     else
        //         return;
    } else {
        if(to.name != "index")
            router.push("/");
    }
});

export default router;