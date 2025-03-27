import moment from "moment";
import { Status } from "./enums/Status";
import { Visibility } from "./enums/Visibility";
import type { _Workspace } from "./Workspace";

export interface _Task {
    id: string;
    name: string;
    description: string;
    workspace: string;
    subTasks: Array<_Task>
    status: Status;
    assignedTo: string;
    lastUpdate: Date;
    updatedBy: string;
    visibility: Visibility;
}

export interface _SubTask {
    id: string;
    name: string;
    description: string;
    assignedTo: string;
}

export class Task {
    id: string;
    name: string;
    description: string;
    workspace: string;
    subTasks: Array<_Task>
    status: Status;
    assignedTo: string;
    lastUpdate: Date;
    updatedBy: string;
    visibility: Visibility;

    constructor(id: string, name: string, title: string, workspace: string, status: Status, assignedTo: string, lastUpdate: Date, updatedBy: string, visibility: Visibility, subTasks: Array<_Task>) {
        this.id = id;
        this.name = name;
        this.description = title;
        this.workspace = workspace;
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

    static createObject(obj: _Task) {
        return new Task(obj.id, obj.name, obj.description, obj.workspace, obj.status, obj.assignedTo, obj.lastUpdate, obj.updatedBy, obj.visibility, obj.subTasks);
    }

    static createEmptyObject(workspace: _Workspace | undefined) {
        if (workspace)
            return new Task("", "", "", workspace.id, Status.OPEN, "", moment().toDate(), "", Visibility.PUBLIC, []);
        else
            return new Task("", "", "", "", Status.OPEN, "", moment().toDate(), "", Visibility.PUBLIC, []);
    }
}

export class SubTask {
    id: string;
    name: string;
    description: string;
    assignedTo: string;

    constructor(id: string, name: string, description: string, assignedTo: string){
        this.id = id;
        this.name = name;
        this.description = description;
        this.assignedTo = assignedTo;
    }
}