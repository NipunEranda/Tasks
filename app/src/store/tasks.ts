import { defineStore } from "pinia";
import moment from "moment";
import type { _Tag, Tag } from "../types/Tag";
import type { _Modal } from "@/types/Modal";
import { Task, type _Task } from "@/types/Task";
import type { _Workspace } from "@/types/Workspace";

export const useTasksStore = defineStore("tasks", {
  state: () => ({
    tags: [] as Array<_Tag>,
    tasks: ["test"],
    sub_tasks: ["test sub task"],
    updatedDate: moment().format("MMMM DD, YYYY"),
    selectedTag: undefined as _Tag | undefined,
    newTask: undefined as unknown as Task,
  }),
  getters: {
    getTags: (state) => state.tags,
    getTagsCount: (state) => state.tags.length,
    getTasks: (state) => state.tasks,
    getSubTasks: (state) => state.sub_tasks,
    getTasksCount: (state) => state.tasks.length,
    getSubTasksCount: (state) => state.sub_tasks.length,
  },
  actions: {
    initializeNewTemplate(task: _Task) {
      this.newTask = task;
    },
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
          is_private: tag.is_private,
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
    async createTemplate() {
      await fetch(`/api/v1/task/template`, {
        method: "POST",
        credentials: "include",
        body: JSON.stringify(this.newTask),
      });
    },
  },
  persist: [
    {
      key: "tasks",
      storage: localStorage,
    },
  ],
});
