import { Visibility } from "./enums/Visibility";

export interface _Workspace {
    id: string;
    name: string;
    owner: string;
    visibility: Visibility,
    created: Date;
    deleted: Boolean;
}

export class Workspace {
    id: string;
    name: string;
    owner: string;
    visibility: Visibility;
    created: Date;
    deleted: Boolean;

    constructor(id: string, name: string, owner: string, visibility: Visibility, created: Date, deleted: Boolean) {
        this.id = id;
        this.name = name;
        this.owner = owner;
        this.visibility = visibility;
        this.created = created;
        this.deleted = deleted;
    }

    static createObject(obj: _Workspace) {
        return new Workspace(obj.id, obj.name, "", Visibility.PRIVATE, obj.created, obj.deleted);
    }

    static createEmptyObject() {
        return new Workspace("", "", "", Visibility.PRIVATE,new Date(), false);
    }
}