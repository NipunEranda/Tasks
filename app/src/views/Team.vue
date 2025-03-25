<template>
    <div>Team</div>
</template>


<script setup>
import { computed, onMounted, watch } from 'vue';
import { useIndexStore } from '../store';
import { useTeamStore } from "../store/team";
import { useRoute } from 'vue-router';
import { useWorkspaceStore } from '../store/workspace';

const indexStore = useIndexStore(),
    teamStore = useTeamStore(),
    workspaceStore = useWorkspaceStore(),
    user = computed(() => { return indexStore.currentUser }),
    route = useRoute();

watch(() => route.path, () => {
    initFlowbite();
});

onMounted(async () => {
    initFlowbite();
    await teamStore.load(workspaceStore.activeWorkspace);
});
</script>