<template>
    <div :id="props.modal?.id" tabindex="-1"
        class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full">
        <div class="relative p-4 w-full max-w-2xl max-h-full">
            <!-- Modal content -->
            <div class="relative bg-white rounded-lg shadow-sm dark:bg-zinc-800">
                <!-- Modal header -->
                <div
                    class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-zinc-700 border-zinc-100">
                    <h3 class="text-xl font-semibold text-zinc-900 dark:text-white">
                        {{ props.modal?.title }}
                    </h3>
                    <!-- @vue-ignore -->
                    <button type="button"
                        class="text-zinc-400 dark:hover:bg-zinc-700 dark:hover:text-white hover:bg-zinc-200 hover:text-zinc-900  bg-transparent rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center cursor-pointer"
                        @click="props.modal?.modalEl ? props.modal?.modalEl.hide() : null">
                        <svg class="w-3 h-3" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 14 14">
                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6" />
                        </svg>
                        <span class="sr-only">Close modal</span>
                    </button>
                </div>
                <!-- Modal body -->
                <div class="p-4 md:p-5 space-y-4 bg-zinc-50 dark:bg-zinc-800">
                    <div class="flex">
                        <span
                            class="inline-flex items-center px-3 text-sm text-zinc-900 bg-zinc-200 border border-e-0 border-zinc-300 rounded-s-md dark:bg-zinc-600 dark:text-zinc-400 dark:border-zinc-600">
                            <fai icon="fa-layer-group" />
                        </span>
                        <input type="text" id="website-admin"
                            class="rounded-none rounded-e-lg border block flex-1 min-w-0 w-full text-sm p-2.5 bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white focus:ring-teal-600 focus:border-teal-600 dark:focus:ring-teal-500 dark:focus:border-teal-500 focus:outline-none"
                            placeholder="Workspace Name" v-model="workspace.name">
                    </div>
                </div>
                <!-- Footer -->
                <div
                    class="bg-zinc-100 bg-opacity-50 dark:bg-zinc-800 border-zinc-200 dark:border-zinc-700 border-t-1 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 rounded-b-full">
                    <button @click="props.modal?.process(workspace)" type="button"
                        class="inline-flex w-full justify-center rounded-md px-5 py-2 text-sm font-semibold text-white shadow-sm sm:w-auto bg-teal-600 hover:bg-teal-700 dark:bg-teal-800 dark:hover:bg-teal-700 mb-2 sm:mb-0 sm:ms-2 cursor-pointer">
                        {{ props.modal?.processName }}
                    </button>
                    <!-- @vue-ignore -->
                    <button @click="props.modal?.modalEl.hide()" type="button"
                        class="inline-flex w-full justify-center rounded-md px-3 py-2 text-sm font-semibold shadow-sm sm:w-auto mb-2 sm:mb-0 sm:ms-2 dark:border-zinc-600 dark:text-white dark:hover:bg-zinc-700 text-black hover:bg-zinc-200 border border-zinc-300 cursor-pointer">
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
import { CustomModal } from "../../models/Modal";
import { Workspace, type _Workspace } from "../../models/Workspace";
const props = defineProps({
    modal: CustomModal,
});

let workspace = ref(Workspace.createEmptyObject());
</script>