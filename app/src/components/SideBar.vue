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
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'dashboard', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'dashboard' }"
                            @click="$router.push('/dashboard')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-home" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span
                                        class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{
                                            $t("app.sideBar.dashboard") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'templates', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'templates' }"
                            @click="$router.push('/templates')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-list" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span
                                        class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{
                                            $t("app.sideBar.templates") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'activity', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'activity' }"
                            @click="$router.push('/activity')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-arrow-trend-up" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span
                                        class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{
                                            $t("app.sideBar.activity") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'team', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'team' }"
                            @click="$router.push('/team')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-user-group" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span
                                        class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{
                                            $t("app.sideBar.team") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'settings', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'settings' }"
                            @click="$router.push('/settings')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-gear" />
                                </div>
                                <div class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll">
                                    <span
                                        class="hidden group-hover:inline-flex group-hover:pl-3 uppercase font-semibold">{{
                                            $t("app.sideBar.settings") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="h-17 grid w-full">
                    <div class="flex-1">
                        <div class="grid place-items-center items-center w-full p-2">
                            <div class=" dark:bg-teal-600 group-hover:dark:bg-zinc-800 dark:hover:brightness-125 cursor-pointer transition-all duration-300 rounded-lg"
                                id="profileMenu" data-dropdown-toggle="profileMenuContainer"
                                data-dropdown-trigger="hover" data-dropdown-placement="top" data-dropdown-delay="100">
                                <div class="flex w-full">
                                    <div>
                                        <img class="h-12 w-12 rounded-lg" :class="{
                                            invert: user ? (user.picture ? false : true) : true,
                                        }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                                    </div>
                                    <div
                                        class="w-0 group-hover:w-[225px] transition-all duration-300 overflow-scroll flex">
                                        <span
                                            class="hidden group-hover:inline-flex group-hover:pl-3 overflow-hidden text-ellipsis truncate items-center font-bold text-lg unselectable">{{
                                                username
                                            }}</span>
                                    </div>
                                </div>
                            </div>

                            <!-- Menu -->
                            <div id="profileMenuContainer"
                                class="z-10 hidden divide-zinc-100 rounded-lg shadow-sm w-full">
                                <ul class="text-sm px-3" aria-labelledby="profileMenu">
                                    <li class=" dark:bg-zinc-800 dark:hover:brightness-125 p-3 px-4 cursor-pointer uppercase font-semibold rounded-tl-md rounded-tr-md"
                                        @click="$router.push('/profile')">
                                        <fai icon="fa-user" class="mr-2" /> {{ $t("app.sideBar.profieMenu.profile") }}
                                    </li>
                                    <li class=" dark:bg-zinc-800 dark:hover:brightness-125 p-3 px-4 cursor-pointer uppercase font-semibold"
                                        @click="navigateToWorkspaces">
                                        <fai icon="fa-layer-group" class="mr-2" /> {{
                                            $t("app.sideBar.profieMenu.workspaces") }}
                                    </li>
                                    <li class=" dark:bg-zinc-800 dark:hover:bg-red-500 p-3 px-4 cursor-pointer uppercase font-semibold rounded-bl-md rounded-br-md"
                                        @click="indexStore.logout">
                                        <fai icon="fa-power-off" class="mr-2" /> {{ $t("app.sideBar.profieMenu.signOut")
                                        }}
                                    </li>
                                </ul>
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
import { computed, onMounted, ref, watch, type Ref } from 'vue';
import { useIndexStore } from '../store';
import type { User } from '../models/Auth';
// import { useI18n } from "vue-i18n";
import { initFlowbite } from "flowbite";
import { useRoute } from "vue-router";
import { useWorkspaceStore } from "../store/workspace";
import router from "../router";

const indexStore = useIndexStore(),
    workspaceStore = useWorkspaceStore(),
    // i18nLocale = useI18n(),
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

let activeTab = ref(route.name ? String(route.name) : "");

// function setLocalization(value) {
//     localStorage.setItem('locale', value);
//     i18nLocale.locale.value = value;
// }

// function isActiveLocalization(value) {
//     return i18nLocale.locale.value == value;
// }

function navigateToWorkspaces() {
    workspaceStore.setWorkspace("");
    router.go(0);
}

watch(() => route.path, () => {
    activeTab.value = route.name ? String(route.name) : "";
    console.log(activeTab.value);
});

onMounted(() => {
    initFlowbite();
});
</script>