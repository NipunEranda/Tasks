import { defineStore } from "pinia";
import moment from "moment";
import type { _Tag, Tag } from "../models/Tag";
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
        async loadTags() {
            const tagsReq = await fetch(`/api/v1/tag`, {credentials: 'include'});
            if(tagsReq.status == 200)
                this.tags = await tagsReq.json();
        },
        async createTag(tag: Tag) {
            await fetch(`/api/v1/tag`, { method: 'POST', credentials: 'include', body: JSON.stringify({ "name": tag.name, "visibility": tag.visibility }) });
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