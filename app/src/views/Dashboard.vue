<template>
    <div>
        Dashboard
    </div>
</template>

<script setup>
import { computed, onMounted } from 'vue';
import { useIndexStore } from '../store';

const indexStore = useIndexStore(),
    user = computed(() => { return indexStore.currentUser });
</script>