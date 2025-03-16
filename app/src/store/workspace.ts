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
        async load() {
            const response = await fetch(`/api/v1/workspace`, { credentials: 'include' });
            if (response.status == 200) {
                return await response.json();
            }

            return [];
        },
        setWorkspace(workspace: string) {
            this.activeWorkspace = workspace;
        }
    },
    persist: [
        {
            key: 'workspace',
            storage: localStorage,
        }
    ]
});