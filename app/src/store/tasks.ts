import { defineStore } from "pinia";
import moment from "moment";
import type { _Tag, Tag } from "../types/Tag";
import router from "../router";
import type { _Modal } from "@/types/Modal";

export const useTasksStore = defineStore("tasks", {
  state: () => ({
    tags: [] as Array<_Tag>,
    tasks: ["test"],
    subTasks: ["test sub task"],
    updatedDate: moment().format("MMMM DD, YYYY"),
    selectedTag: undefined as _Tag | undefined,
  }),
  getters: {
    getTags: (state) => state.tags,
    getTagsCount: (state) => state.tags.length,
    getTasks: (state) => state.tasks,
    getSubTasks: (state) => state.subTasks,
    getTasksCount: (state) => state.tasks.length,
    getSubTasksCount: (state) => state.subTasks.length,
  },
  actions: {
    async loadTags(workspaceId: string) {
      const tagsReq = await fetch(`/api/v1/tag/${workspaceId}`, {
        credentials: "include",
      });
      if (tagsReq.status == 200) this.tags = await tagsReq.json();
    },
    async createTag(tag: Tag, modal: _Modal) {
      await fetch(`/api/v1/tag`, {
        method: "POST",
        credentials: "include",
        body: JSON.stringify({
          name: tag.name,
          visibility: tag.visibility,
          workspace: tag.workspace,
        }),
      });
      this.loadTags(tag.workspace);
      modal.modalEl?.hide();
    },
    async deleteTag(tag: Tag, modal: _Modal) {
      await fetch(`/api/v1/tag/${tag.id}`, {
        method: "DELETE",
        credentials: "include",
      });

      this.loadTags(tag.workspace);
      modal.modalEl?.hide();
    },
  },
  persist: [
    {
      key: "tasks",
      storage: localStorage,
    },
  ],
});
