import type { Status } from "./enums/Status";
import type { Visibility } from "./enums/Visibility";

export interface _Task {
    id: string;
    name: string;
    title: string;
    subTasks: Array<_Task>
    status: Status;
    assignedTo: string;
    lastUpdate: Date;
    updatedBy: string;
    visibility: Visibility;
}

export class Task {
    id: string;
    name: string;
    title: string;
    subTasks: Array<_Task>
    status: Status;
    assignedTo: string;
    lastUpdate: Date;
    updatedBy: string;
    visibility: Visibility;

    constructor(id: string, name: string, title: string, status: Status, assignedTo: string, lastUpdate: Date, updatedBy: string, visibility: Visibility, subTasks: Array<_Task>) {
        this.id = id;
        this.name = name;
        this.title = title;
        this.subTasks = subTasks ? subTasks.length > 0 ? subTasks : [] : [];
        this.status = status;
        this.assignedTo = assignedTo;
        this.lastUpdate = lastUpdate;
        this.updatedBy = updatedBy;
        this.visibility = visibility;
    }

    addSubTask(task: Task) {
        this.subTasks.push(task);
    }

    removeSubTaskById(taskId: string) {
        delete this.subTasks[this.subTasks.indexOf(this.subTasks.filter(st => st.id == taskId)[0])]
        this.subTasks = this.subTasks.map(st => st);
    }

    removeSubTaskByName(taskName: string) {
        delete this.subTasks[this.subTasks.indexOf(this.subTasks.filter(st => st.name == taskName)[0])]
        this.subTasks = this.subTasks.map(st => st);
    }
}