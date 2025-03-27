<template>
  <div class="pr-5 overflow-hidden pb-20">
    <!-- <div
      id="vertical-line"
      class="fixed bg-teal-400 w-[2px] left-[44px] lg:left-[120px] z-0 h-full"
    ></div> -->
    <TaskCard />
    <ul>
      <VueDraggableNext class="dragArea list-group w-full" v-model="subTasks">
        <div v-for="(subTask, e) in subTasks" :key="e">
          <SubTaskCard :subTask="subTask" :removeSubTask="removeSubTask" />
        </div>
      </VueDraggableNext>
    </ul>

    <div class="flex items-center space-x-2 mt-4 group">
      <button
        class="rounded-full bg-teal-600 text-white group-hover:brightness-110 hover:cursor-pointer text-xl flex p-4"
      >
        <fai icon="fa-plus" />
      </button>
      <button
        class="flex items-center p-6 py-4 bg-teal-600 text-white font-semibold rounded-lg shadow-md group-hover:brightness-110 hover:cursor-pointer w-full text-xl"
        @click="addTask"
      >
        + ADD NEW TASK
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTasksStore } from "@/store/tasks";
import { useWorkspaceStore } from "@/store/workspace";
import { SubTask, type _SubTask } from "@/types/Task";
import { onMounted, ref, type Ref } from "vue";
import { VueDraggableNext } from "vue-draggable-next";

let subTasks: Ref<_SubTask[]> = ref([]),
  taskStore = useTasksStore(),
  workspaceStore = useWorkspaceStore(),
  count = ref(0);

function addTask() {
  subTasks.value.push(new SubTask((count.value++).toString(), "", "", ""));
}

function removeSubTask(id: string) {
  const subTask = subTasks.value.find((subTask) => subTask.id === id);
  if (subTask) {
    delete subTasks.value[subTasks.value.indexOf(subTask)];
    subTasks.value = subTasks.value.filter((subTask) => subTask);
  }
}

onMounted(async () => {
  await Promise.all([taskStore.loadTags(workspaceStore.activeWorkspace), workspaceStore.loadTeam()]);
});
</script>
