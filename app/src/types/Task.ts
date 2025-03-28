import moment from "moment";
import type { _Workspace } from "./Workspace";
import type { Tag } from "./Tag";

export interface _Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  sub_tasks: Array<_SubTask>;
  tags: Array<Tag>;
  completed: boolean;
  assign_to: string;
  last_update: Date;
  updated_by: string;
  is_private: boolean;
}

export class Task {
  id: string;
  name: string;
  description: string;
  workspace: string;
  sub_tasks: Array<_SubTask>;
  tags: Array<Tag>;
  completed: boolean;
  assign_to: string;
  last_update: Date;
  updated_by: string;
  is_private: boolean;

  constructor(
    id: string,
    name: string,
    title: string,
    workspace: string,
    completed: boolean,
    assign_to: string,
    last_update: Date,
    updated_by: string,
    is_private: boolean,
    sub_tasks: Array<_SubTask>,
    tags: Array<Tag>
  ) {
    this.id = id;
    this.name = name;
    this.description = title;
    this.workspace = workspace;
    this.sub_tasks = sub_tasks ? (sub_tasks.length > 0 ? sub_tasks : []) : [];
    this.tags = tags ? (tags.length > 0 ? tags : []) : [];
    this.completed = completed;
    this.assign_to = assign_to;
    this.last_update = last_update;
    this.updated_by = updated_by;
    this.is_private = is_private;
  }

  static createObject(obj: _Task) {
    return new Task(
      obj.id,
      obj.name,
      obj.description,
      obj.workspace,
      obj.completed,
      obj.assign_to,
      obj.last_update,
      obj.updated_by,
      obj.is_private,
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
        "",
        moment().toDate(),
        userId ? userId : "",
        false,
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
        false,
        [],
        []
      );
  }
}

export interface _SubTask {
  id: string;
  name: string;
  description: string;
  assign_to: string;
  completed: boolean;
}

export class SubTask {
  id: string;
  name: string;
  description: string;
  assign_to: string;
  completed: boolean;

  constructor(
    id: string,
    name: string,
    description: string,
    assign_to: string,
    completed: boolean
  ) {
    this.id = id;
    this.name = name;
    this.description = description;
    this.assign_to = assign_to;
    this.completed = completed;
  }

  static createObject(obj: _SubTask) {
    return new SubTask(
      obj.id,
      obj.name,
      obj.description,
      obj.assign_to,
      false
    );
  }

  static createEmptyObject() {
    return new SubTask("", "", "", "", false);
  }
}
