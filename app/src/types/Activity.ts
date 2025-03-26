import type { ActivityType } from "./enums/ActivityType";

export interface _Activity {
    id: string,
    type: ActivityType,
    message: string,
    user: string;
    created: Date;
}

export class Activity {
    id: string;
    type: ActivityType;
    message: string;
    user: string;
    created: Date;

    constructor(id: string, type: ActivityType, message: string, user: string, created: Date){
        this.id = id;
        this.type = type;
        this.message = message;
        this.user = user;
        this.created = created;
    }
}