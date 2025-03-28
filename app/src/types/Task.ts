import moment from "moment";
import { Visibility } from "./enums/Visibility";
import type { _Workspace } from "./Workspace";
import type { Tag } from "./Tag";

export interface _Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  subTasks: Array<_SubTask>;
  tags: Array<Tag>;
  completed: boolean;
  assignedTo: string;
  lastUpdate: Date;
  updatedBy: string;
  visibility: Visibility;
}

export class Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  subTasks: Array<_SubTask>;
  tags: Array<Tag>;
  completed: boolean;
  assignedTo: string;
  lastUpdate: Date;
  updatedBy: string;
  visibility: Visibility;

  constructor(
    id: string,
    name: string,
    title: string,
    workspace: string,
    completed: boolean,
    assignedTo: string,
    lastUpdate: Date,
    updatedBy: string,
    visibility: Visibility,
    subTasks: Array<_SubTask>,
    tags: Array<Tag>
  ) {
    this.id = id;
    this.name = name;
    this.description = title;
    this.workspace = workspace;
    this.subTasks = subTasks ? (subTasks.length > 0 ? subTasks : []) : [];
    this.tags = tags ? (tags.length > 0 ? tags : []) : [];
    this.completed = completed;
    this.assignedTo = assignedTo;
    this.lastUpdate = lastUpdate;
    this.updatedBy = updatedBy;
    this.visibility = visibility;
  }

  static createObject(obj: _Task) {
    return new Task(
      obj.id,
      obj.name,
      obj.description,
      obj.workspace,
      obj.completed,
      obj.assignedTo,
      obj.lastUpdate,
      obj.updatedBy,
      obj.visibility,
      obj.subTasks,
      obj.tags
    );
  }

  static createEmptyObject(workspaceId: string, userId: string | undefined,) {
    if (workspaceId != "")
      return new Task(
        "",
        "",
        "",
        workspaceId,
        false,
        "",
        moment().toDate(),
        userId ? userId : "",
        Visibility.PUBLIC,
        [],
        []
      );
    else
      return new Task(
        "",
        "",
        "",
        "",
        false,
        "",
        moment().toDate(),
        userId ? userId : "",
        Visibility.PUBLIC,
        [],
        []
      );
  }
}

export interface _SubTask {
  id: string;
  name: string;
  description: string;
  assignedTo: string;
  completed: boolean;
}

export class SubTask {
  id: string;
  name: string;
  description: string;
  assignedTo: string;
  completed: boolean;

  constructor(
    id: string,
    name: string,
    description: string,
    assignedTo: string,
    completed: boolean
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.assignedTo = assignedTo;
    this.completed = completed;
  }

  static createObject(obj: _SubTask) {
    return new SubTask(obj.id, obj.name, obj.description, obj.assignedTo, false);
  }

  static createEmptyObject() {
    return new SubTask("", "", "", "", false);
  }
}
