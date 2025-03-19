<template>
    <div class="h-screen dark:bg-zinc-900 dark:brightness-110">
        <div class="w-full">
            <div class="group flex flex-col h-screen">
                <div class="h-12 my-5">
                    <div class="place-items-center items-center justify-center hidden group-hover:inline-flex">
                        <div class="flex font-extrabold text-6xl mb-8 font-serif tracking-widest">
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">T</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">A</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">K</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                        </div>
                    </div>
                </div>
                <div class="flex-1">
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1 group-hover:mr-3">
                                    <fai icon="fa-home" />
                                </div>
                                <div class="hidden group-hover:inline">
                                    Home
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1 group-hover:mr-3">
                                    <fai icon="fa-list" />
                                </div>
                                <div class="hidden group-hover:inline">
                                    Templates
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1 group-hover:mr-3">
                                    <fai icon="fa-arrow-trend-up" />
                                </div>
                                <div class="hidden group-hover:inline">
                                    Activity
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1 group-hover:mr-3">
                                    <fai icon="fa-user-group" />
                                </div>
                                <div class="hidden group-hover:inline">
                                    Team
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1 group-hover:mr-3">
                                    <fai icon="fa-gear" />
                                </div>
                                <div class="hidden group-hover:inline">
                                    Settings
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <!-- <div class="h-20 grid w-full p-2">
                    <div class="w-full">
                        <div
                            class="flex dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer text-lg">
                            <img class="h-6 w-6 rounded-full" :class="{
                                invert: user ? (user.picture ? false : true) : true,
                            }" :src="getProfileImage()" alt="" referrerpolicy="no-referrer" />
                            <div class="ml-3 font-extrabold w-full hidden group-hover:inline-flex">{{ user ? user.name
                                : ""
                            }}</div>
                        </div>
                    </div>
                </div> -->
                <div class="h-20 grid w-full p-2">
                    <div class="flex-1">
                        <div class="grid place-items-center items-center w-full">
                            <div class="dark:bg-zinc-800 dark:hover:brightness-125 rounded-full cursor-pointer group-hover:w-full">
                                <div class="flex w-full">
                                    <div>
                                        <img class="h-12 w-12 rounded-full group-hover:mr-4" :class="{
                                            invert: user ? (user.picture ? false : true) : true,
                                        }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                                    </div>
                                    <div
                                        class="hidden group-hover:inline-flex max-w-64 items-center overflow-hidden text-ellipsis truncate">
                                        {{username}}
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, onMounted, ref, type Ref } from 'vue';
import { useIndexStore } from '../store';
import type { User } from '../models/Auth';

const indexStore = useIndexStore();

const user: Ref<User | null> = computed(() => { return indexStore.currentUser });

const profileImage: Ref<string> = computed(() => {
    if (user.value) {
        if (user.value.picture) {
            return user.value.picture;
        }
        return profile;
    } else return profile;
})

const username = computed(() => { return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "" });

onMounted(() => {
    console.log(user.value);
});
</script>