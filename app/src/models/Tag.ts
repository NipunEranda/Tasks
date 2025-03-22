import type { Visibility } from "./enums/Visibility";

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
}