import { defineStore } from "pinia";
import type { _Workspace } from "../types/Workspace";
import type { _User } from "@/types/Auth";

export const useWorkspaceStore = defineStore("workspace", {
  state: () => ({
    workspaces: [] as _Workspace[],
    activeWorkspace: "",
    team: [] as _User[],
  }),
  getters: {
    getWorkspaces: (state) => state.workspaces,
    getActiveWorkspace: (state) =>
      state.workspaces.filter((w) => w.id == state.activeWorkspace)[0],
  },
  actions: {
    async load() {
      const response = await fetch(`/api/v1/workspace`, {
        credentials: "include",
      });
      if (response.status == 200) {
        this.workspaces = await response.json();
      }
    },
    async loadTeam() {
      if (this.activeWorkspace != "") {
        const response = await fetch(`/api/v1/workspace/${this.activeWorkspace}/team`, {
          credentials: "include",
        });
        if (response.status == 200) {
          this.team = await response.json();
        }
      }
    },
    setWorkspace(workspace: string) {
      this.activeWorkspace = workspace;
    },
    async create(workspace: _Workspace) {
      const response = await fetch(`/api/v1/workspace`, {
        method: "POST",
        body: JSON.stringify(workspace),
        credentials: "include",
      });
      console.log(response);
    },
  },
  persist: [
    {
      key: "workspace",
      storage: localStorage,
    },
  ],
});
