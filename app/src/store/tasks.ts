import { defineStore } from "pinia";
import moment from "moment";
import type { _Tag } from "../models/Tag";

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

    },
    persist: [
        {
            key: 'tasks',
            storage: localStorage,
        }
    ]
});