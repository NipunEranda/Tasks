import { defineStore } from "pinia";
import moment from "moment";

export const useTasksStore = defineStore('tasks', {
    state: () => ({
        tasks: ["test"],
        subTasks: ["test sub task"],
        updatedDate: moment().format("MMMM DD, YYYY"),
    }),
    getters: {
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