import { Visibility } from "./enums/Visibility";

export interface _Tag {
    id: string;
    name: string;
    createdBy: string;
    visibility: Visibility;
}

export class Tag {
    id: string;
    name: string;
    createdBy: string;
    visibility: Visibility;

    constructor(id: string, name: string, createdBy: string, visibility: Visibility){
        this.id = id;
        this.name = name;
        this.createdBy = createdBy;
        this.visibility = visibility;
    }

    static createObject(obj: _Tag) {
            return new Tag(obj.id, obj.name, obj.createdBy, obj.visibility);
        }
    
        static createEmptyObject() {
            return new Tag("", "", "", Visibility.PUBLIC);
        }
}