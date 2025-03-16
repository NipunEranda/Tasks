import { defineStore } from "pinia";

export const useWorkspaceStore = defineStore('workspace', {
    state: () => ({
        workspaces: [],
        activeWorkspace: ""
    }),
    getters: {
        getWorkspaces: (state) => state.workspaces,
        getActiveWorkspace: (state) => state.activeWorkspace,
    },
    actions: {
        async load(){

        }
    },
    persist: [
        {
            key: 'workspaces',
            storage: localStorage,
        }
    ]
});