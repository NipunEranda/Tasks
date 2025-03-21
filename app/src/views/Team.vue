<template>
    <div>Team</div>
</template>


<script setup>
import { computed, onMounted, watch } from 'vue';
import { useIndexStore } from '../store';
import { useRoute } from 'vue-router';

const indexStore = useIndexStore(),
    user = computed(() => { return indexStore.currentUser }),
    route = useRoute();

watch(() => route.path, () => {
    initFlowbite();
});

onMounted(async () => {
    initFlowbite();
});
</script>