<template>
    <div class="dark:bg-zinc-800 p-3 rounded-md">
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
        <div class="w-full">
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
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, type Ref } from 'vue';
import { useIndexStore } from '../store';
import type { User } from '../models/Auth';
import { useWorkspaceStore } from "../store/workspace";
import { useTasksStore } from "../store/tasks";

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
    username = computed(() => { return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "" });
</script>