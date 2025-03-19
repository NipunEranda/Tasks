<template>
    <div class="h-screen dark:bg-zinc-900 dark:brightness-110">
        <div class="w-full">
            <div class="flex flex-col h-screen">
                <div class="h-12 my-5">
                    <div class="grid place-items-center items-center justify-center">
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
                            <fai icon="fa-home" class="mr-3" /> Home
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <fai icon="fa-list" class="mr-3" /> Templates
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <fai icon="fa-arrow-trend-up" class="mr-3" /> Activity
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <fai icon="fa-user-group" class="mr-3" /> Team
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer">
                            <fai icon="fa-gear" class="mr-3" /> Settings
                        </div>
                    </div>
                </div>
                <div class="h-20 grid place-items-center items-center w-full p-2">
                    <div class="w-full">
                        <div
                            class="flex dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer items-center">
                            <img class="mr-2 h-7 w-7 rounded-full" :class="{
                                invert: user ? (user.picture ? false : true) : true,
                            }" :src="getProfileImage()" alt="" referrerpolicy="no-referrer" />
                            {{ user ? user.name : "" }}
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

function getProfileImage() {
    if (user.value) {
        if (user.value.picture) {
            return user.value.picture;
        }
        return profile;
    } else return profile;
}

onMounted(() => {
    console.log(user.value);
});
</script>