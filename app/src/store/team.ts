import { defineStore } from "pinia";
import type { _User } from "../types/Auth";

export const useTeamStore = defineStore("team", {
  state: () => ({
    team: [] as _User[],
  }),
  actions: {
    async load(workspaceId: String) {
        const response = await fetch(`/api/v1/workspace/${workspaceId}/team`, { credentials: "include" });
        if (response.status == 200) {
            this.team = await response.json();
        }
    }
  },
  persist: [
    {
        key: 'team',
        storage: localStorage,
    }
]
});