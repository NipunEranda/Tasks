import type { CustomModal } from "@/types/Modal";
import $ from "jquery";

export const sleep = async (ms: number) => {
  return new Promise((resolve) => setTimeout(resolve, ms));
};

export const initModal = (
  modal: CustomModal,
  modalId: string,
  triggerName: string
) => {
  const modalOptions = {
    backdrop: "dynamic" as "dynamic",
    backdropClasses:
      "bg-theme-primary/50 dark:bg-theme-primary/80 fixed inset-0 z-40",
    closable: true,
    onHide: () => {
      const triggerButton = document.getElementById(triggerName);
      triggerButton?.focus();
    },
  };

  // instance options object
  const instanceOptions = {
    id: modalId,
    override: true,
  };

  // @ts-ignore
  modal.modalEl = new Modal(
    document.getElementById(modalId) as HTMLElement,
    modalOptions,
    instanceOptions
  ) as any;
};

export const toggleElement = (elementName: string, hideOverride: boolean) => {
    !hideOverride
      ? $(`#${elementName}`).removeClass("hidden").addClass("hidden")
      : $(`#${elementName}`).removeClass("hidden");
};
