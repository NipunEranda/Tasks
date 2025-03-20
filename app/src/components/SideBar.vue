<template>
    <div class="h-screen dark:bg-zinc-900 dark:brightness-110">
        <div class="w-full">
            <div class="group flex flex-col h-screen">
                <div class="h-12 my-5">
                    <div class="grid place-items-center items-center w-full transition-all duration-300 overflow-auto">
                        <div class="hidden group-hover:flex font-extrabold text-6xl mb-8 font-serif tracking-widest">
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">T</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">A</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">K</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                        </div>
                        <div class="flex group-hover:hidden font-extrabold text-6xl mb-8 font-serif tracking-widest">
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">T</span>
                        </div>
                    </div>
                </div>
                <div class="flex-1 transition-all duration-300">
                    <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                        <div
                            class="dark:bg-zinc-800 dark:hover:brightness-125 dark:text-zinc-400 hover:dark:text-zinc-50 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-home" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{ $t("app.sideBar.dashboard") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                        <div
                            class="dark:bg-zinc-800 dark:hover:brightness-125 dark:text-zinc-400 hover:dark:text-zinc-50 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-list" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{ $t("app.sideBar.templates") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                        <div
                            class="dark:bg-zinc-800 dark:hover:brightness-125 dark:text-zinc-400 hover:dark:text-zinc-50 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-arrow-trend-up" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{ $t("app.sideBar.activity") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                        <div
                            class="dark:bg-zinc-800 dark:hover:brightness-125 dark:text-zinc-400 hover:dark:text-zinc-50 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-user-group" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{ $t("app.sideBar.team") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                        <div
                            class="dark:bg-zinc-800 dark:hover:brightness-125 dark:text-zinc-400 hover:dark:text-zinc-50 w-full p-4 rounded-lg cursor-pointer">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-gear" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{ $t("app.sideBar.settings") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="h-17 grid w-full">
                    <div class="flex-1">
                        <div class="grid place-items-center items-center w-full p-2 transition-all duration-300">
                            <div
                                class=" dark:bg-teal-600 group-hover:dark:bg-zinc-800 dark:hover:brightness-125 cursor-pointer transition-all duration-300 rounded-lg">
                                <div class="flex w-full">
                                    <div>
                                        <img class="h-12 w-12 rounded-lg" :class="{
                                            invert: user ? (user.picture ? false : true) : true,
                                        }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                                    </div>
                                    <div
                                        class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll flex">
                                        <span
                                            class="hidden group-hover:inline-flex group-hover:pl-3 overflow-hidden text-ellipsis truncate items-center font-bold text-lg">{{
                                                username
                                            }}</span>
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
import { useI18n } from "vue-i18n";

const indexStore = useIndexStore(),
    i18nLocale = useI18n(),
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

onMounted(() => {
    console.log(user.value);
});
</script>