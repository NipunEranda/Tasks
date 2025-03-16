export interface _Modal {
    id: string;
    title: string;
    type: string;
    processName: string;
    process: Function;
    modalEl: Element | undefined;
}

export class CustomModal {
    id: string;
    title: string;
    type: string;
    processName: string;
    process: Function;
    modalEl: Element | undefined;

    constructor(obj: _Modal){
        this.id = obj.id;
        this.title = obj.title;
        this.type = obj.type;
        this.processName = obj.processName;
        this.process = obj.process;
        this.modalEl = obj.modalEl;
    }

    static createObj(id: string, title: string, type: string, processName: string, process:Function, modalEl: Element | undefined) {
        return new CustomModal({ id, title, type, processName, process, modalEl });
    }
}