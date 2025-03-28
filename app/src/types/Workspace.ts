export interface _Workspace {
  id: string;
  name: string;
  owner: string;
  is_private: boolean;
  created: Date;
  deleted: Boolean;
}

export class Workspace {
  id: string;
  name: string;
  owner: string;
  is_private: boolean;
  created: Date;
  deleted: Boolean;

  constructor(
    id: string,
    name: string,
    owner: string,
    is_private: boolean,
    created: Date,
    deleted: Boolean
  ) {
    this.id = id;
    this.name = name;
    this.owner = owner;
    this.is_private = is_private;
    this.created = created;
    this.deleted = deleted;
  }

  static createObject(obj: _Workspace) {
    return new Workspace(obj.id, obj.name, "", obj.is_private, obj.created, obj.deleted);
  }

  static createEmptyObject() {
    return new Workspace("", "", "", false, new Date(), false);
  }
}
