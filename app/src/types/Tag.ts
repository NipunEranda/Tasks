export interface _Tag {
    id: string;
    name: string;
    created_by: string;
    is_private: boolean;
    workspace: string;
}

export class Tag {
    id: string;
    name: string;
    created_by: string;
    is_private: boolean;
    workspace: string;

    constructor(id: string, name: string, created_by: string, is_private: boolean, workspace: string) {
        this.id = id;
        this.name = name;
        this.created_by = created_by;
        this.is_private = is_private;
        this.workspace = workspace;
    }

    static createObject(obj: _Tag) {
        return new Tag(obj.id, obj.name, obj.created_by, obj.is_private, obj.workspace);
    }

    static createEmptyObject(workspace: string) {
        return new Tag("", "", "", false, workspace);
    }
}