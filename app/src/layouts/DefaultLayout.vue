<template>
    <div class="flex">
        <Header v-if="showSideBar"/>
        <side-bar id="sideBar" v-if="showSideBar"
            class="w-0 lg:w-[75px] hover:w-[256px] border-r border-zinc-800 transition-all duration-300 z-20" />
        <div class="flex ml-1 lg:ml-20 mr-1 w-full">
            <router-view class="flex-grow px-4 pr-0 pt-[150px] lg:pt-[73px] h-screen"></router-view>
            <div v-if="showSideBar" class="pt-[150px] lg:pt-[73px] pr-2 hidden lg:inline lg:w-3/12">
                <info-side-bar />
            </div>
        </div>
    </div>
</template>

<script setup>
import { initFlowbite } from 'flowbite';
import { computed, onMounted, watch } from 'vue';
import { useIndexStore } from '../store';
import { useRoute } from 'vue-router';

const indexStore = useIndexStore(),
    route = useRoute();

let showSideBar = computed(() => { return (!(route.path == "/" || route.path == "/workspaces") && indexStore.loggedIn) ? true : false });

watch(() => route.path, () => {
    initFlowbite();
});

onMounted(() => {
    initFlowbite();
});
</script>
