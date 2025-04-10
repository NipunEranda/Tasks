<template>
  <div class="w-full flex h-screen">
    <div class="container mx-auto pt-40 px-10">
      <div class="w-full flex">
        <h1 class="text-4xl mx-auto">Select a Workspace</h1>
      </div>
      <div class="pt-7 w-full">
        <!-- Workspaces list -->
        <div
          v-for="workspace in workspaceStore.workspaces"
          class="flex"
          :key="workspace.id"
        >
          <div
            class="mx-auto border-t border-b dark:border-theme-primary-border w-full md:w-1/2 lg:w-1/2 text-center p-2 cursor-pointer"
            @click="selectWorkspace(workspace)"
          >
            <div
              class="py-2 dark:bg-theme-primary hover:dark:brightness-150 rounded-xl font-semibold text-xl"
            >
              {{ workspace.name }}
            </div>
          </div>
        </div>
      </div>
      <div class="p-2 pt-10 w-full grid place-items-center">
        <button
          id="workspaceModalTrigger"
          type="button"
          class="dark:text-theme-primary-text dark:bg-theme-first dark:hover:brightness-110 font-medium rounded-lg text-sm px-8 py-3 text-center uppercase cursor-pointer"
          @click="openWorkspaceModal('add')"
        >
          Create Workspace
        </button>
      </div>
    </div>

    <workspace-modal :modal="modal" />
  </div>
</template>

<script setup lang="ts">
import { initFlowbite } from "flowbite";
import { CustomModal } from "../types/Modal";
import { onMounted, ref, watch } from "vue";
import type { _Workspace } from "../types/Workspace";
import { useWorkspaceStore } from "../store/workspace";
import { useRoute, useRouter } from "vue-router";
import { initModal } from "@/utils";

const modal = ref(
    CustomModal.createObj(
      "workspaceModal",
      "Add Workspace",
      "add",
      "Save",
      processModal,
      undefined
    )
  ),
  workspaceStore = useWorkspaceStore(),
  router = useRouter(),
  route = useRoute();

async function load() {
  await workspaceStore.load();
}

function openWorkspaceModal(type: string) {
  modal.value.type = type;
  // @ts-ignore
  modal.value.modalEl.show();
}

function processModal(workspace: _Workspace) {
  if (modal.value.type == "add") {
    workspaceStore.create(workspace);
  }
  // @ts-ignore
  modal.value.modalEl?.hide();
  router.go(0);
}

function selectWorkspace(workspace: _Workspace) {
  workspaceStore.setWorkspace(workspace.id);
  router.push("/dashboard");
}

watch(
  () => route.path,
  () => {
    initFlowbite();
  }
);

onMounted(async () => {
  initFlowbite();
  initModal(modal.value, "workspaceModal", "workspaceModalTrigger");
  await load();
});
</script>
