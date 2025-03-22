<template>
    <div>
        <div class="dark:bg-zinc-800 p-3 rounded-md cursor-pointer">
            <div>
                <div class="w-full grid">
                    <div
                        class="h-full dark:bg-zinc-800 hover:dark:brightness-110 rounded-lg cursor-pointer items-center hidden sm:flex p-2">
                        <div class="text-3xl justify-end pr-2">
                            <div class="border-3 border-zinc-700 rounded-full transition-all duration-300">
                                <img class="h-9 w-9 lg:group-hover:w-14 rounded-full" :class="{
                                    invert: user ? (user.picture ? false : true) : true,
                                }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                            </div>
                        </div>
                        <div class="row text-xs place-content-center">
                            <div class="font-bold lg:text-md xl:text-lg">{{ username }}</div>
                            <div class="text-xs">{{ workspaceStore.getActiveWorkspace.name }}</div>
                        </div>
                    </div>
                </div>
                <div
                    class="flex h-full p-2 dark:bg-purple-800 hover:dark:brightness-110 rounded-lg mt-2 cursor-pointer place-content-center items-center">
                    <div class="flex flex-grow">
                        <div class="text-3xl justify-end pl-4 pr-4">{{ tasksStore.getSubTasksCount }}</div>
                        <div class="row text-xs flex-grow place-content-center">
                            <div class="font-bold"><span class="hidden md:inline">Active
                                </span><span>SubTasks</span></div>
                            <div class="text-xs">0 New</div>
                        </div>
                    </div>
                    <div class="flex pl-2">
                        <fai icon="fa-circle-check" class="px-1" />
                    </div>
                </div>
                <div
                    class="flex h-full p-2 dark:bg-teal-800 hover:dark:brightness-110 rounded-lg mt-2 cursor-pointer place-content-center items-center">
                    <div class="flex flex-grow">
                        <div class="text-3xl justify-end pl-4 pr-4">{{ tasksStore.getTasksCount }}</div>
                        <div class="row text-xs flex-grow place-content-center">
                            <div class="font-bold"><span class="hidden md:inline">Active
                                </span><span>Tasks</span></div>
                            <div class="text-xs">0 New</div>
                        </div>
                    </div>
                    <div class="flex">
                        <fai icon="fa-list" class="px-1" />
                    </div>
                </div>
            </div>
        </div>
        <div id="section" class="dark:bg-zinc-800 rounded-md mt-3 cursor-pointer" v-if="showSection">
            <div class="text-center font-bold text-xl p-3 dark:bg-teal-700 rounded-t-md">{{ section.title }}</div>
            <div class="w-full px-3">
                <div class="text-sm dark:text-zinc-300 font-bold py-2 pb-1">
                    {{ tasksStore.updatedDate }}
                </div>
                <!-- List -->
                <div>
                    <div
                        class="border-b dark:bg-zinc-800 hover:dark:brightness-110 dark:border-zinc-600 flex items-center p-1 px-3 cursor-pointer">
                        <div
                            class="flex border-2 dark:border-purple-600 items-center place-items-center p-1 rounded-sm mr-2">
                            <fai icon="fa-list" class="dark:text-purple-600" />
                        </div>
                        <div class="flex-grow text-sm">
                            <div>Sub Task</div>
                            <div>status</div>
                        </div>
                    </div>
                    <div
                        class="border-b dark:bg-zinc-800 hover:dark:brightness-110 dark:border-zinc-600 flex items-center p-1 px-3 cursor-pointer">
                        <div
                            class="flex border-2 dark:border-teal-600 items-center place-items-center p-1 rounded-sm mr-2">
                            <fai icon="fa-list" class="dark:text-teal-600" />
                        </div>
                        <div class="flex-grow text-sm">
                            <div>Task</div>
                            <div>status</div>
                        </div>
                    </div>
                </div>
            </div>
            <div class="p-2 uppercase dark:text-teal-600 text-center items-center place-items-center cursor-pointer">
                <div class="hover:dark:bg-teal-600/10 rounded-full w-fit p-2 px-4" @click="$router.push('/activity')">Show More</div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, ref, watch, type Ref } from 'vue';
import { useIndexStore } from '../store';
import type { User } from '../models/Auth';
import { useWorkspaceStore } from "../store/workspace";
import { useTasksStore } from "../store/tasks";
import { useRoute } from "vue-router";

const indexStore = useIndexStore(),
    workspaceStore = useWorkspaceStore(),
    tasksStore = useTasksStore(),
    user: Ref<User | null> = computed(() => { return indexStore.currentUser }),
    profileImage: Ref<string> = computed(() => {
        if (user.value) {
            if (user.value.picture) {
                return user.value.picture;
            }
            return profile;
        } else return profile;
    }),
    username = computed(() => { return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "" }),
    section = ref({
        title: "Recent Changes"
    }),
    route = useRoute();

let showSection = computed(() => {
    switch(route.path) {
        case '/dashboard':
            return true;
        case '/templates':
            return true;
        case '/activity':
            return false;
        case '/team':
            return true;
        case '/settings':
            return false;
        default:
            return false;
    }
});
</script>