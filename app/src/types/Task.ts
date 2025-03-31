import moment from "moment";
import type { _Workspace } from "./Workspace";

export interface _Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  sub_tasks: Array<_SubTask>;
  tags: Array<string>;
  completed: boolean;
  created_by: string;
  last_update: Date;
  updated_by: string;
  is_private: boolean;
  deleted: boolean;
}

export class Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  sub_tasks: Array<_SubTask>;
  tags: Array<string>;
  completed: boolean;
  created_by: string;
  last_update: Date;
  updated_by: string;
  is_private: boolean;
  deleted: boolean;

  constructor(
    id: string,
    name: string,
    title: string,
    workspace: string,
    completed: boolean,
    created_by: string,
    last_update: Date,
    updated_by: string,
    sub_tasks: Array<_SubTask>,
    tags: Array<string>
  ) {
    this.id = id;
    this.name = name;
    this.description = title;
    this.workspace = workspace;
    this.sub_tasks = sub_tasks ? (sub_tasks.length > 0 ? sub_tasks : []) : [];
    this.tags = tags ? (tags.length > 0 ? tags : []) : [];
    this.completed = completed;
    this.created_by = created_by;
    this.last_update = last_update;
    this.updated_by = updated_by;
    this.is_private = false;
    this.deleted = false;
  }

  static createObject(obj: _Task) {
    return new Task(
      obj.id,
      obj.name,
      obj.description,
      obj.workspace,
      obj.completed,
      obj.created_by,
      obj.last_update,
      obj.updated_by,
      obj.sub_tasks,
      obj.tags
    );
  }

  static createEmptyObject(workspaceId: string, userId: string | undefined) {
    if (workspaceId != "")
      return new Task(
        "",
        "",
        "",
        workspaceId,
        false,
        userId ? userId : "",
        moment().toDate(),
        userId ? userId : "",
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
        [],
        []
      );
  }
}

export interface _SubTask {
  id: string;
  name: string;
  description: string;
  assignees: Array<string>;
  completed: boolean;
  deleted: boolean;
}

export class SubTask {
  id: string;
  name: string;
  description: string;
  assignees: Array<string>;
  completed: boolean;
  deleted: boolean;

  constructor(
    id: string,
    name: string,
    description: string,
    assignees: Array<string>,
    completed: boolean,
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.assignees = assignees;
    this.completed = completed;
    this.deleted = false;
  }

  static createObject(obj: _SubTask) {
    return new SubTask(
      obj.id,
      obj.name,
      obj.description,
      obj.assignees,
      false
    );
  }

  static createEmptyObject() {
    return new SubTask("", "", "", [], false);
  }
}
