<template>
    <div class="flex">
        <Header class="z-40" v-if="showSideBar"/>
        <side-bar id="sideBar" v-if="showSideBar"
            class="w-0 md:w-[75px] hover:w-[256px] border-r border-zinc-800 transition-all duration-300 z-50" />
        <div class="ml-1 md:ml-20 mr-1 w-full">
            <router-view class="w-full px-4 pt-[150px] md:pt-[73px] h-screen"></router-view>
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
