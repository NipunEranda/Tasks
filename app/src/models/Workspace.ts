export interface _Workspace {
    id: string;
    name: string;
    created: Date;
    deleted: Boolean;
}

export class Workspace {
    id: string;
    name: string;
    created: Date;
    deleted: Boolean;

    constructor(id: string, name: string, created: Date, deleted: Boolean) {
        this.id = id;
        this.name = name;
        this.created = created;
        this.deleted = deleted;
    }

    static createObject(obj: _Workspace) {
        return new Workspace(obj.id, obj.name, obj.created, obj.deleted);
    }

    static createEmptyObject() {
        return new Workspace("", "", new Date(), false);
    }
}