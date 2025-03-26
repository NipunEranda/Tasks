import { defineStore } from "pinia";
import moment from "moment";
import type { _Tag, Tag } from "../types/Tag";
import router from "../router";

export const useTasksStore = defineStore('tasks', {
    state: () => ({
        tags: [] as Array<_Tag>,
        tasks: ["test"],
        subTasks: ["test sub task"],
        updatedDate: moment().format("MMMM DD, YYYY"),
    }),
    getters: {
        getTags: (state) => state.tags,
        getTagsCount: (state) => state.tags.length,
        getTasks: (state) => state.tasks,
        getSubTasks: (state) => state.subTasks,
        getTasksCount: (state) => state.tasks.length,
        getSubTasksCount: (state) => state.subTasks.length
    },
    actions: {
        async loadTags(workspaceId: string) {
            const tagsReq = await fetch(`/api/v1/tag/${workspaceId}`, {credentials: 'include'});
            if(tagsReq.status == 200)
                this.tags = await tagsReq.json();
        },
        async createTag(tag: Tag) {
            await fetch(`/api/v1/tag`, { method: 'POST', credentials: 'include', body: JSON.stringify({ name: tag.name, visibility: tag.visibility, workspace: tag.workspace }) });
            router.go(0);
        },
        async deleteTag(tag: Tag) {
            await fetch(`/api/v1/tag/${tag.id}`, { method: 'DELETE', credentials: 'include' });
            router.go(0);
        }
    },
    persist: [
        {
            key: 'tasks',
            storage: localStorage,
        }
    ]
});