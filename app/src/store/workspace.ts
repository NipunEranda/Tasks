import { defineStore } from "pinia";
import type { _Workspace } from "../models/Workspace";

export const useWorkspaceStore = defineStore('workspace', {
    state: () => ({
        workspaces: [] as _Workspace[],
        activeWorkspace: ""
    }),
    getters: {
        getWorkspaces: (state) => state.workspaces,
        getActiveWorkspace: (state) => state.workspaces.filter(w => w.id == state.activeWorkspace)[0],
    },
    actions: {
        async load() {
            const response = await fetch(`/api/v1/workspace`, { credentials: 'include' });
            if (response.status == 200) {
                this.workspaces = await response.json();
                return this.workspaces;
            }

            return [];
        },
        setWorkspace(workspace: string) {
            this.activeWorkspace = workspace;
        },
        async create(workspace: _Workspace){
            const response = await fetch(`/api/v1/workspace`, { method: 'POST', body: JSON.stringify(workspace), credentials: 'include' });
            console.log(response);
        }
    },
    persist: [
        {
            key: 'workspace',
            storage: localStorage,
        }
    ]
});