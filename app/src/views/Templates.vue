<template>
  <div>
    <div v-if="!isNewTemplate">Template List</div>
    <NewTemplate v-if="isNewTemplate"></NewTemplate>
  </div>
</template>

<script setup>
import { computed, onMounted, watch, ref } from "vue";
import { useIndexStore } from "../store";
import { useRoute } from "vue-router";
import { useTasksStore } from "@/store/tasks";
import { useWorkspaceStore } from "@/store/workspace";

const indexStore = useIndexStore(),
  tasksStore = useTasksStore(),
  workspaceStore = useWorkspaceStore(),
  user = computed(() => {
    return indexStore.currentUser;
  }),
  route = useRoute(),
  templates = ref([]),
  isNewTemplate = computed(() => route.query.type === "new");

async function getTemplates() {
  await tasksStore.loadTemplates(workspaceStore.activeWorkspace);
  templates.value = tasksStore.templates;
}

watch(
  () => route.path,
  () => {
    initFlowbite();
  }
);

onMounted(async () => {
  initFlowbite();
  await getTemplates();
});
</script>
