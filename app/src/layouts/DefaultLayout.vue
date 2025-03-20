<template>
    <div class="flex">
        <side-bar v-if="showSideBar" class="w-[75px] hover:w-[300px] border-r border-zinc-800 transition-all duration-300"/>
        <router-view class="p-4" :class="{ 'w-full': !showSideBar, 'w-auto': showSideBar }"></router-view>
    </div>
</template>

<script setup>
import { initFlowbite } from 'flowbite';
import { computed, onMounted } from 'vue';
import { useIndexStore } from '../store';
import { useRoute } from 'vue-router';

const indexStore = useIndexStore(),
    route = useRoute();

let showSideBar = computed(() => { return (!(route.path == "/" || route.path == "/workspaces") && indexStore.loggedIn) ? true : false });

onMounted(() => {
    initFlowbite();
});
</script>
