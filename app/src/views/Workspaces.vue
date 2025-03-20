<template>
    <div class="w-full flex h-screen">
        <div class="container mx-auto pt-40 px-10">
            <div class="w-full flex">
                <h1 class="text-4xl mx-auto">Select a Workspace</h1>
            </div>
            <div class="pt-7 w-full">
                <!-- Workspaces list -->
                <div v-for="workspace in workspaces" class="flex">
                    <div class="mx-auto border-t border-b dark:border-zinc-700 w-full md:w-1/2 lg:w-1/2 text-center p-2 cursor-pointer"
                        @click="selectWorkspace(workspace)">
                        <div class="py-2 dark:hover:bg-zinc-800 rounded-xl font-semibold text-xl">{{ workspace.name }}
                        </div>
                    </div>
                </div>
            </div>
            <div class="p-2 pt-10 w-full grid place-items-center">
                <button type="button"
                    class="text-white dark:bg-teal-800 dark:hover:bg-teal-700 bg-teal-700 hover:bg-green-800 font-medium rounded-lg text-sm px-8 py-3 text-center uppercase cursor-pointer"
                    @click="openWorkspaceModal('add')">Create
                    Workspace</button>
            </div>
        </div>

        <workspace-modal :modal="modal" />
    </div>
</template>

<script setup lang="ts">
import { Modal } from 'flowbite';
import { CustomModal } from '../models/Modal';
import { onMounted, ref } from 'vue';
import type { _Workspace } from '../models/Workspace';
import { useWorkspaceStore } from '../store/workspace';
import { useRouter } from 'vue-router';

const modal = ref(CustomModal.createObj("workspaceModal", "Add Workspace", "add", "Save", processModal, undefined)),
    workspaceStore = useWorkspaceStore(),
    router = useRouter();

let workspaces = ref<_Workspace[]>([]);

async function load() {
    workspaces.value = await workspaceStore.load();
}

function openWorkspaceModal(type: string) {
    modal.value.type = type;
    // @ts-ignore
    modal.value.modalEl.show();
}

function processModal(workspace: _Workspace) {
    if (modal.value.type == 'add') {
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

function initModal() {
    const modalId = "workspaceModal";
    const modalOptions = {
        backdrop: "dynamic",
        backdropClasses: "bg-zinc-900/50 dark:bg-zinc-900/80 fixed inset-0 z-40",
        closable: true,
    };

    // instance options object
    const instanceOptions = {
        id: modalId,
        override: true,
    };

    // @ts-ignore
    modal.value.modalEl = new Modal(document.getElementById(modalId), modalOptions, instanceOptions);
}

onMounted(async () => {
    initModal();
    await load();
});
</script>