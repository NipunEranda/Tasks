<template>
    <div>
        <!-- Mobile View -->
        <nav
            class="fixed top-0 z-10 w-full bg-white border-zinc-200 dark:bg-zinc-900 dark:brightness-125 inline lg:hidden">
            <div class="px-3 py-3 lg:px-3 lg:pl-3">
                <div class="flex items-center justify-between">
                    <button class="p-1 pl-3 rounded-lg cursor-pointer text-xl hover:dark:text-teal-600"
                        data-drawer-target="sideBar" data-drawer-toggle="sideBar">
                        <fai icon="fa-bars" />
                    </button>
                    <div class="flex-grow pl-4 pr-2">
                        <div class="grid grid-cols-2 sm:grid-cols-3">
                            <div
                                class="h-full p-2 dark:bg-zinc-800 hover:dark:brightness-110 rounded-lg ml-2 cursor-pointer items-center hidden sm:flex">
                                <div class="text-3xl justify-end pl-2 pr-2">
                                    <div class="border-3 border-zinc-700 rounded-full transition-all duration-300">
                                        <img class="h-7 w-7 lg:group-hover:w-14 rounded-full" :class="{
                                            invert: user ? (user.picture ? false : true) : true,
                                        }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                                    </div>
                                </div>
                                <div class="row text-xs place-content-center">
                                    <div class="font-bold text-md">{{ username }}</div>
                                    <div class="text-xs">{{ workspaceStore.getActiveWorkspace.name }}</div>
                                </div>
                            </div>
                            <div
                                class="flex h-full p-2 dark:bg-purple-800 hover:dark:brightness-110 rounded-lg ml-2 cursor-pointer place-content-center items-center">
                                <div class="flex flex-grow">
                                    <div class="text-3xl justify-end pl-4 pr-4">{{ tasksStore.getSubTasksCount }}</div>
                                    <div class="row text-xs flex-grow place-content-center">
                                        <div class="font-bold"><span class="hidden md:inline">Active
                                            </span><span>SubTasks</span></div>
                                        <div class="text-xs">0 New</div>
                                    </div>
                                </div>
                                <div class="flex">
                                    <fai icon="fa-circle-check" class="px-1"/>
                                </div>
                            </div>
                            <div
                                class="flex h-full p-2 dark:bg-teal-800 hover:dark:brightness-110 rounded-lg ml-2 cursor-pointer place-content-center items-center">
                                <div class="flex flex-grow">
                                    <div class="text-3xl justify-end pl-4 pr-4">{{ tasksStore.getTasksCount }}</div>
                                    <div class="row text-xs flex-grow place-content-center">
                                        <div class="font-bold"><span class="hidden md:inline">Active
                                            </span><span>Tasks</span></div>
                                        <div class="text-xs">0 New</div>
                                    </div>
                                </div>
                                <div class="flex">
                                    <fai icon="fa-list" class="px-1"/>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>

        <nav class="fixed top-0 w-full bg-white border-zinc-200 dark:bg-zinc-900 mt-[73px] lg:m-0">
            <div class="px-3 py-3 lg:px-3 lg:pl-3">
                <div class="flex items-center justify-between">
                    <div class="hidden lg:flex items-center justify-start rtl:justify-end">
                    </div>
                    <div class="flex items-center w-full lg:w-fit">
                        <div class="flex items-center ms-3 w-full lg:w-fit">
                            <div class="flex mr-5 lg:mr-2 items-center">
                                <button id="tasksTypeButton" data-dropdown-toggle="taskTypesDropdownIcon"
                                    data-dropdown-offset-distance="10" data-dropdown-offset-skidding="60"
                                    data-dropdown-placement="bottom" class="cursor-pointer">
                                    <fai icon="fa-filter" class="text-2xl" />
                                </button>

                                <button id="tasksTypeButton" data-dropdown-toggle="taskTypesDropdown"
                                    class="font-medium text-sm px-5 py-2 text-center lg:inline-flex items-center border rounded-md bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-zinc-700 dark:border-zinc-600 dark:placeholder-zinc-400 dark:text-white focus:outline-none cursor-pointer dark:focus:border-teal-600 focus:ring-teal-600 dark:focus:bg-teal-700 ml-2 hidden"
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
                            <div class="hidden lg:flex mr-2">
                                <button type="button"
                                    class="inline-flex w-full justify-center px-5 py-2 text-sm font-semibold border rounded-md bg-zinc-50 border-zinc-300 text-zinc-900 dark:bg-teal-700 dark:border-teal-600 dark:hover:bg-teal-600 dark:placeholder-teal-400 dark:text-white focus:ring-teal-600 focus:outline-none cursor-pointer">
                                    Start New
                                </button>
                            </div>
                            <div class="flex-grow pr-2 lg:pr-0">
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
import profile from "../assets/img/profile.png";
import { initFlowbite } from 'flowbite';
import { computed, onMounted, ref, watch, type Ref } from 'vue';
import { useRoute } from 'vue-router';
import type { User } from '../models/Auth';
import { useIndexStore } from '../store';
import { useWorkspaceStore } from "../store/workspace";
import { useTasksStore } from "../store/tasks";

const indexStore = useIndexStore(),
    workspaceStore = useWorkspaceStore(),
    tasksStore = useTasksStore(),
    taskTypes = computed(() => { return ["My Tasks", "My Active Tasks", "All Tasks"] }),
    route = useRoute(),
    user: Ref<User | null> = computed(() => { return indexStore.currentUser }),
    profileImage: Ref<string> = computed(() => {
        if (user.value) {
            if (user.value.picture) {
                return user.value.picture;
            }
            return profile;
        } else return profile;
    }),
    username = computed(() => { return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "" });

let selectedTasksType = ref(1);

onMounted(() => {
    initFlowbite();
});


watch(() => route.path, () => {
    initFlowbite();
});
</script>