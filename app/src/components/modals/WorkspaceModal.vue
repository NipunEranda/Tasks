<template>
    <div :id="props.modal?.id" tabindex="-1"
        class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
        <div class="relative p-4 w-full max-w-2xl max-h-full">
            <!-- Modal content -->
            <div class="relative rounded-lg shadow-sm dark:bg-theme-primary dark:border-theme-primary-border">
                <!-- Modal header -->
                <div
                    class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:bg-theme-primary dark:brightness-120 dark:border-theme-primary-border">
                    <h3 class="text-xl font-semibold dark:text-theme-primary-text">
                        {{ props.modal?.title }}
                    </h3>
                    <!-- @vue-ignore -->
                    <button type="button"
                        class="hover:dark:bg-theme-primary hover:dark:brightness-150 dark:text-theme-primary-text-secondary hover:dark:text-theme-primary-text  bg-transparent rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center cursor-pointer"
                        @click="props.modal?.modalEl ? props.modal?.modalEl.hide() : null">
                        <fai icon="fa-xmark"/>
                        <span class="sr-only">Close modal</span>
                    </button>
                </div>
                <!-- Modal body -->
                <div class="p-4 md:p-5 space-y-4 dark:bg-theme-primary">
                    <div class="flex group">
                        <span
                            class="inline-flex items-center px-3 text-sm border-1 rounded-s-md dark:bg-theme-primary dark:brightness-150 dark:text-theme-primary-text-secondary dark:border-theme-primary-border/40 dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-150">
                            <fai icon="fa-layer-group" />
                        </span>
                        <input type="text" id="website-admin"
                            class="rounded-none rounded-e-lg border block flex-1 min-w-0 w-full text-sm p-2.5 dark:bg-theme-primary dark:brightness-150 dark:border-theme-primary-border/40 dark:placeholder-theme-primary-placeholder/40 dark:text-theme-primary-text dark:group-focus-within:ring-theme-first dark:group-focus-within:border-theme-first focus:outline-none cursor-pointer"
                            placeholder="Workspace Name" v-model="workspace.name" autocomplete="off">
                    </div>
                    <div class="flex group">
                        <span
                        class="inline-flex items-center px-3 text-sm border-1 rounded-s-md dark:bg-theme-primary dark:brightness-150 dark:text-theme-primary-text-secondary dark:border-theme-primary-border/40 dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-150">
                            <fai :icon="workspace.is_private == 'true' ? 'fa-eye-slash' : 'fa-eye'" />
                        </span>
                        <select id="small"
                        class="rounded-none rounded-e-lg border block flex-1 min-w-0 w-full text-sm p-2.5 dark:bg-theme-primary dark:brightness-150 dark:border-theme-primary-border/40 dark:text-theme-primary-text dark:focus:ring-theme-first dark:focus:border-theme-first dark:group-focus-within:border-theme-first  focus:outline-none cursor-pointer" v-model="workspace.is_private">
                            <option :value="false">Public</option>
                            <option :value="true">Private</option>
                        </select>
                    </div>
                </div>
                <!-- Footer -->
                <div
                    class="dark:bg-theme-primary dark:brightness-120 dark:border-theme-primary-border border-t-1 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 rounded-b-md">
                    <button @click="props.modal?.process(workspace)" type="button"
                        class="inline-flex w-full justify-center rounded-md px-5 py-2 text-sm font-semibold shadow-sm sm:w-auto dark:bg-theme-first dark:hover:brightness-110 mb-2 sm:mb-0 sm:ms-2 cursor-pointer">
                        {{ props.modal?.processName }}
                    </button>
                    <!-- @vue-ignore -->
                    <button @click="props.modal?.modalEl.hide()" type="button"
                        class="inline-flex w-full justify-center rounded-md px-3 py-2 text-sm font-semibold shadow-sm sm:w-auto mb-2 sm:mb-0 sm:ms-2 dark:border-theme-primary-border/80 dark:text-theme-primary-text dark:bg-theme-primary dark:brightness-110 hover:dark:brightness-150 border cursor-pointer">
                        Cancel
                    </button>
                </div>
                <!-- Footer -->
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { CustomModal } from "../../types/Modal";
import { Workspace, type _Workspace } from "../../types/Workspace";
const props = defineProps({
    modal: CustomModal,
});

let workspace = ref(Workspace.createEmptyObject());
</script>