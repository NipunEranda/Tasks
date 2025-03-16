export interface _User {
    id: string;
    name: String;
    email: string;
    picture: String;
    type: String;
    workspaces: string[];
}

export class User {
    id: string;
    name: String;
    email: string;
    picture: String;
    type: String;
    workspaces: string[];

    constructor(
        id: string,
        name: String,
        email: string,
        picture: String,
        type: String,
        workspaces: string[]
    ) {
        this.id = id;
        this.name = name;
        this.email = email;
        this.picture = picture;
        this.type = type;
        this.workspaces = workspaces;
    }

    static createObject(obj: _User) {
        return new User(
            obj.id,
            obj.name,
            obj.email,
            obj.picture,
            obj.type,
            obj.workspaces
        );
    }

    static createEmptyObject() {
        return new User(
            "",
            "",
            "",
            "",
            "",
            []
        );
    }
}