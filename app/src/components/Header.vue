<template>
    <div>
        <!-- Mobile View -->
        <nav
            class="fixed top-0 z-10 w-full bg-white border-zinc-200 dark:bg-zinc-900 dark:brightness-125 inline md:hidden">
            <div class="px-3 py-3 lg:px-3 lg:pl-3">
                <div class="flex items-center justify-between">
                    <button class="p-1 pl-3 rounded-lg cursor-pointer text-xl hover:dark:text-teal-600"
                        data-drawer-target="sideBar" data-drawer-toggle="sideBar">
                        <fai icon="fa-bars" />
                    </button>
                    <div class="flex-grow pl-4 pr-2">
                        <div class="grid grid-cols-2">
                            <div class="flex h-full p-2 dark:bg-purple-800 rounded-lg ml-2 cursor-pointer">
                                <div class="text-3xl justify-end pl-5 pr-5">25</div>
                                <div class="row text-xs place-content-center">
                                    <div class="font-bold">Sub Tasks</div>
                                    <div class="text-xs">0 New</div>
                                </div>
                            </div>
                            <div class="flex h-full p-2 dark:bg-teal-800 rounded-lg ml-2 cursor-pointer">
                                <div class="text-3xl justify-end pl-5 pr-5">0</div>
                                <div class="row text-xs place-content-center">
                                    <div class="font-bold">Tasks</div>
                                    <div class="text-xs">0 New</div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>

        <nav class="fixed top-0 w-full bg-white border-zinc-200 dark:bg-zinc-900 mt-[73px] md:m-0">
            <div class="px-3 py-3 lg:px-3 lg:pl-3">
                <div class="flex items-center justify-between">
                    <div class="hidden md:flex items-center justify-start rtl:justify-end">
                    </div>
                    <div class="flex items-center w-full md:w-fit">
                        <div class="flex items-center ms-3 w-full md:w-fit">
                            <div class="flex mr-5 md:mr-2 items-center">
                                <button id="tasksTypeButton" data-dropdown-toggle="taskTypesDropdownIcon"
                                    data-dropdown-offset-distance="10" data-dropdown-offset-skidding="60"
                                    data-dropdown-placement="bottom" class="cursor-pointer">
                                    <fai icon="fa-filter" class="text-2xl" />
                                </button>

                                <button id="tasksTypeButton" data-dropdown-toggle="taskTypesDropdown"
                                    class="font-medium text-sm px-5 py-2 text-center md:inline-flex items-center border rounded-md bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white focus:outline-none cursor-pointer dark:focus:border-teal-600 focus:ring-teal-600 dark:focus:bg-teal-700 ml-2 hidden"
                                    type="button"> {{ taskTypes[selectedTasksType] }} <svg class="w-2.5 h-2.5 ms-3"
                                        aria-hidden="true" xmlns="http://www.w3.org/2000/svg" fill="none"
                                        viewBox="0 0 10 6">
                                        <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                                            stroke-width="2" d="m1 1 4 4 4-4" />
                                    </svg>
                                </button>

                                <!-- Dropdown menu -->
                                <div id="taskTypesDropdown"
                                    class="z-10 hidden bg-white divide-y divide-zinc-100 rounded-md shadow-sm w-44 dark:bg-zinc-700">
                                    <ul class="py-2 text-sm text-zinc-700 dark:text-zinc-200"
                                        aria-labelledby="tasksTypeButton">
                                        <li v-for="(type, t) in taskTypes">
                                            <a class="block px-4 py-2 hover:bg-zinc-100 dark:hover:bg-zinc-600 dark:hover:text-white cursor-pointer"
                                                @click="selectedTasksType = t">{{
                                                    type }}</a>
                                        </li>
                                    </ul>
                                </div>

                                <div id="taskTypesDropdownIcon"
                                    class="ml-10 z-10 hidden bg-white divide-y divide-zinc-100 rounded-md shadow-sm w-44 dark:bg-zinc-700">
                                    <ul class="py-2 text-sm text-zinc-700 dark:text-zinc-200"
                                        aria-labelledby="taskTypesDropdownIcon">
                                        <li v-for="(type, t) in taskTypes">
                                            <a class="block px-4 py-2 hover:bg-zinc-100 dark:hover:bg-zinc-600 dark:hover:text-white cursor-pointer"
                                                @click="selectedTasksType = t">{{
                                                    type }}</a>
                                        </li>
                                    </ul>
                                </div>
                            </div>
                            <div class="hidden md:flex mr-2">
                                <button type="button"
                                    class="inline-flex w-full justify-center px-5 py-2 text-sm font-semibold border rounded-md bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-teal-700 dark:border-teal-600 dark:hover:bg-teal-600 dark:placeholder-teal-400 dark:text-white focus:ring-teal-600 focus:outline-none cursor-pointer">
                                    Start New
                                </button>
                            </div>
                            <div class="flex-grow pr-2 md:pr-0">
                                <label for="search"
                                    class="mb-2 text-sm font-medium text-zinc-900 sr-only dark:text-white">Search</label>
                                <div class="relative">
                                    <div class="absolute inset-y-0 start-0 flex items-center ps-3 pointer-events-none">
                                        <svg class="w-4 h-4 text-zinc-500 dark:text-zinc-400" aria-hidden="true"
                                            xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 20 20">
                                            <path stroke="currentColor" stroke-linecap="round" stroke-linejoin="round"
                                                stroke-width="2" d="m19 19-4-4m0-7A7 7 0 1 1 1 8a7 7 0 0 1 14 0Z" />
                                        </svg>
                                    </div>
                                    <input type="text" id="search"
                                        class="block w-full p-2 ps-10 text-sm border rounded-md bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white focus:ring-teal-600 focus:border-teal-600 dark:focus:ring-teal-500 dark:focus:border-teal-500 focus:outline-none"
                                        placeholder="Search" autocomplete="none" />
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    </div>
</template>

<script setup lang="ts">
import { initFlowbite } from 'flowbite';
import { computed, onMounted, ref, watch } from 'vue';
import { useRoute } from 'vue-router';

const taskTypes = computed(() => { return ["My Tasks", "My Active Tasks", "All Tasks"] }),
    route = useRoute();

let selectedTasksType = ref(1);

onMounted(() => {
    initFlowbite();
});


watch(() => route.path, () => {
    initFlowbite();
});
</script>