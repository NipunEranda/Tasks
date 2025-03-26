import type { Modal } from "flowbite";

export interface _Modal {
    id: string;
    title: string;
    message: string;
    type: string;
    processName: string;
    process: Function;
    modalEl: Modal | undefined;
}

export class CustomModal {
    id: string;
    title: string;
    message: string;
    type: string;
    processName: string;
    process: Function;
    modalEl: Modal | undefined;

    constructor(obj: _Modal){
        this.id = obj.id;
        this.title = obj.title;
        this.message = obj.message;
        this.type = obj.type;
        this.processName = obj.processName;
        this.process = obj.process;
        this.modalEl = obj.modalEl;
    }

    static createObj(id: string, title: string, type: string, processName: string, process:Function, modalEl: Modal | undefined) {
        return new CustomModal({ id, title, message: "", type, processName, process, modalEl });
    }
}