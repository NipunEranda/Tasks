import { Visibility } from "./enums/Visibility";

export interface _Tag {
    id: string;
    name: string;
    createdBy: string;
    visibility: Visibility;
    workspace: string;
}

export class Tag {
    id: string;
    name: string;
    createdBy: string;
    visibility: Visibility;
    workspace: string;

    constructor(id: string, name: string, createdBy: string, visibility: Visibility, workspace: string){
        this.id = id;
        this.name = name;
        this.createdBy = createdBy;
        this.visibility = visibility;
        this.workspace = workspace;
    }

    static createObject(obj: _Tag) {
            return new Tag(obj.id, obj.name, obj.createdBy, obj.visibility, obj.workspace);
        }
    
        static createEmptyObject(workspace: string) {
            return new Tag("", "", "", Visibility.PUBLIC, workspace);
        }
}