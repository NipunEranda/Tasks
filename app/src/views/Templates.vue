<template>
    <div>
        <div v-if="!isNewTemplate">
            Template List
        </div>
        <NewTemplate v-if="isNewTemplate"></NewTemplate>
    </div>
</template>

<script setup>
import { computed, onMounted, watch } from 'vue';
import { useIndexStore } from '../store';
import { useRoute } from 'vue-router';

const indexStore = useIndexStore(),
    user = computed(() => { return indexStore.currentUser }),
    route = useRoute(),
    isNewTemplate = computed(() => route.query.type === 'new');

watch(() => route.path, () => {
    initFlowbite();
    console.log(route.query.type);
});

onMounted(async () => {
    initFlowbite();
});
</script>