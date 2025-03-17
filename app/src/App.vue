<template>
  <div>
    <div class="flex">
      <side-bar v-if="showSideBar" class="w-[300px]"/>
      <router-view class="w-auto"/>
    </div>
  </div>
</template>

<script setup>
import { initFlowbite } from 'flowbite';
import { computed, onMounted } from 'vue';
import { useIndexStore } from './store';
import { useRoute } from 'vue-router';

const indexStore = useIndexStore(),
route = useRoute();

let showSideBar = computed(() => { return (!(route.path == "/" || route.path == "/workspaces") && indexStore.loggedIn) ? true : false });

onMounted(() => {
  initFlowbite();
});
</script>
