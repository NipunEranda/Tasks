import moment from "moment";
import { Status } from "./enums/Status";
import { Visibility } from "./enums/Visibility";
import type { _Workspace } from "./Workspace";
import type { Tag } from "./Tag";

export interface _Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  subTasks: Array<_Task>;
  tags: Array<Tag>;
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
  subTasks: Array<_Task>;
  tags: Array<Tag>;
  status: Status;
  assignedTo: string;
  lastUpdate: Date;
  updatedBy: string;
  visibility: Visibility;

  constructor(
    id: string,
    name: string,
    title: string,
    workspace: string,
    status: Status,
    assignedTo: string,
    lastUpdate: Date,
    updatedBy: string,
    visibility: Visibility,
    subTasks: Array<_Task>,
    tags: Array<Tag>
  ) {
    this.id = id;
    this.name = name;
    this.description = title;
    this.workspace = workspace;
    this.subTasks = subTasks ? (subTasks.length > 0 ? subTasks : []) : [];
    this.tags = tags ? (tags.length > 0 ? tags : []) : [];
    this.status = status;
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
      obj.status,
      obj.assignedTo,
      obj.lastUpdate,
      obj.updatedBy,
      obj.visibility,
      obj.subTasks,
      obj.tags
    );
  }

  static createEmptyObject(workspace: _Workspace | undefined) {
    if (workspace)
      return new Task(
        "",
        "",
        "",
        workspace.id,
        Status.OPEN,
        "",
        moment().toDate(),
        "",
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
        Status.OPEN,
        "",
        moment().toDate(),
        "",
        Visibility.PUBLIC,
        [],
        []
      );
  }
}

export class SubTask {
  id: string;
  name: string;
  description: string;
  assignedTo: string;

  constructor(
    id: string,
    name: string,
    description: string,
    assignedTo: string
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.assignedTo = assignedTo;
  }

  static createObject(obj: _SubTask) {
    return new SubTask(obj.id, obj.name, obj.description, obj.assignedTo);
  }

  static createEmptyObject() {
    return new SubTask("", "", "", "");
  }
}
