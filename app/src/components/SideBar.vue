<template>
    <aside id="sideBar"
        class="fixed top-0 left-0 z-40 w-64 h-screen dark:bg-zinc-900 dark:brightness-110 transition-all -translate-x-full lg:translate-x-0 duration-300"
        aria-label="Sidebar">
        <div class="w-full">
            <div class="group flex flex-col h-screen">
                <div class="h-12 my-5">
                    <div class="grid place-items-center items-center w-full transition-all duration-300 overflow-auto">
                        <div
                            class="flex lg:hidden lg:group-hover:flex font-extrabold text-6xl mb-8 font-serif tracking-widest">
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">T</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">A</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                            <span class="text-teal-700" style="font-family: 'Faith Hope', sans-serif;">K</span>
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">S</span>
                        </div>
                        <div
                            class="hidden lg:flex group-hover:hidden font-extrabold text-6xl mb-8 font-serif tracking-widest">
                            <span class="text-teal-500" style="font-family: 'Faith Hope', sans-serif;">T</span>
                        </div>
                    </div>
                </div>
                <div class="flex-1 transition-all duration-300 overflow-hidden">
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'dashboard', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'dashboard' }"
                            data-drawer-toggle="sideBar" @click="navigation('/dashboard')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-home" />
                                </div>
                                <!-- group-hover:w-[200px]  -->
                                <div class="transition-all duration-300 overflow-scroll">
                                    <span class="lg:hidden group-hover:inline-flex pl-3 uppercase font-semibold">{{
                                        $t("app.sideBar.dashboard") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'templates', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'templates' }"
                            data-drawer-toggle="sideBar" @click="navigation('/templates')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-list" />
                                </div>
                                <div class="transition-all duration-300 overflow-scroll">
                                    <span class="lg:hidden group-hover:inline-flex pl-3 uppercase font-semibold">{{
                                        $t("app.sideBar.templates") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'activity', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'activity' }"
                            data-drawer-toggle="sideBar" @click="navigation('/activity')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-arrow-trend-up" />
                                </div>
                                <div class="transition-all duration-300 overflow-scroll">
                                    <span class="lg:hidden group-hover:inline-flex pl-3 uppercase font-semibold">{{
                                        $t("app.sideBar.activity") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'team', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'team' }"
                            data-drawer-toggle="sideBar" @click="navigation('/team')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-user-group" />
                                </div>
                                <div class="transition-all duration-300 overflow-scroll">
                                    <span class="lg:hidden group-hover:inline-flex pl-3 uppercase font-semibold">{{
                                        $t("app.sideBar.team") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="grid place-items-center items-center w-full p-2">
                        <div class="dark:bg-zinc-800 dark:hover:brightness-125 w-full p-4 rounded-lg cursor-pointer"
                            :class="{ 'dark:text-teal-500 hover:dark:text-teal-400': activeTab == 'settings', 'dark:text-zinc-400 hover:dark:text-zinc-50': activeTab != 'settings' }"
                            data-drawer-toggle="sideBar" @click="navigation('/settings')">
                            <div class="flex w-full">
                                <div class="ml-1">
                                    <fai icon="fa-gear" />
                                </div>
                                <div class="transition-all duration-300 overflow-scroll">
                                    <span class="lg:hidden group-hover:inline-flex pl-3 uppercase font-semibold">{{
                                        $t("app.sideBar.settings") }}</span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="h-17 grid w-full px-2">
                    <div class="flex-1">
                        <div class="grid place-items-center items-center w-full">
                            <div class="dark:bg-zinc-800 group-hover:dark:bg-zinc-800 dark:hover:brightness-125 cursor-pointer transition-all duration-300 rounded-lg"
                                id="profileMenu" data-dropdown-toggle="profileMenuContainer"
                                data-dropdown-trigger="hover" data-dropdown-placement="top" data-dropdown-delay="100">
                                <div class="flex w-full group-hover:w-full">
                                    <div class="border-3 border-zinc-700 rounded-lg transition-all duration-300">
                                        <img class="h-11 lg:w-11 w-14 lg:group-hover:w-14 rounded-lg" :class="{
                                            invert: user ? (user.picture ? false : true) : true,
                                        }" :src="profileImage" alt="" referrerpolicy="no-referrer" />
                                    </div>
                                    <div
                                        class="w-[225px] lg:w-0 lg:group-hover:w-[225px] overflow-scroll flex transition-all duration-300">
                                        <span
                                            class="flex pl-3 overflow-hidden text-ellipsis truncate items-center font-bold text-lg unselectable">{{
                                                username
                                            }}</span>
                                    </div>
                                </div>
                            </div>

                            <div id="profileMenuContainer"
                                class="z-10 hidden divide-zinc-100 rounded-lg shadow-sm w-full">
                                <ul class="text-sm px-3" aria-labelledby="profileMenu">
                                    <li class=" dark:bg-zinc-800 dark:hover:brightness-125 p-3 px-4 cursor-pointer uppercase font-semibold rounded-tl-md rounded-tr-md" data-drawer-toggle="sideBar"
                                        @click="navigation('/profile')">
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
    </aside>

</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, onMounted, ref, watch, type Ref } from 'vue';
import { useIndexStore } from '../store';
import type { User } from '../models/Auth';
// import { useI18n } from "vue-i18n";
import { Dropdown, initFlowbite, type DropdownInterface, type DropdownOptions } from "flowbite";
import { useRoute } from "vue-router";
import { useWorkspaceStore } from "../store/workspace";
import router from "../router";
import { sleep } from "../utils";
import $ from "jquery";

const indexStore = useIndexStore(),
    workspaceStore = useWorkspaceStore(),
    // i18nLocale = useI18n(),
    route = useRoute(),
    user: Ref<User | null> = computed(() => { return indexStore.currentUser || null }),
    profileImage: Ref<string> = computed(() => {
        if (user.value) {
            if (user.value.picture) {
                return user.value.picture;
            }
            return profile;
        } else return profile;
    }),
    username = computed(() => { return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "" });

let activeTab = ref(route.name ? String(route.name) : ""),
    subMenu: Ref<DropdownInterface | null> = ref(null);

// function setLocalization(value) {
//     localStorage.setItem('locale', value);
//     i18nLocale.locale.value = value;
// }

// function isActiveLocalization(value) {
//     return i18nLocale.locale.value == value;
// }

function initSubMenu() {
    const subMenuEl = document.getElementById('profileMenuContainer');

    // set the element that trigger the dropdown menu on click
    const subMenuButtonEl = document.getElementById('profileMenu');

    // options with default values
    const options: DropdownOptions = {
        placement: 'top',
        triggerType: 'click',
        offsetSkidding: 0,
        offsetDistance: 10,
        delay: 300,
        ignoreClickOutsideClass: false
    };

    // instance options object
    const instanceOptions = {
        id: 'profileMenuContainer',
        override: true
    };

    subMenu.value = new Dropdown(
        subMenuEl,
        subMenuButtonEl,
        options,
        instanceOptions
    );
}

async function navigation(path: string) {
    await sleep(10);
    subMenu.value?.hide();
    $('#sideBar').removeClass('transition: none').addClass('-translate-x-full lg:translate-x-0');
    router.push(path);
}

function navigateToWorkspaces() {
    workspaceStore.setWorkspace("");
    router.go(0);
}

watch(() => route.path, () => {
    initFlowbite();
    initSubMenu();
    activeTab.value = route.name ? String(route.name) : "";
});

onMounted(() => {
    initFlowbite();
    initSubMenu();
});
</script>