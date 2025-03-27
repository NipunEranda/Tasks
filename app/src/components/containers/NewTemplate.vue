<template>
  <div class="pr-5 overflow-hidden">
    <!-- <div
      id="vertical-line"
      class="fixed bg-teal-400 w-[2px] left-[44px] lg:left-[120px] z-0 h-full"
    ></div> -->
    <TaskCard />
    <ul>
      <VueDraggableNext
        class="dragArea list-group w-full"
        v-model="subTasks"
      >
        <div v-for="element in subTasks" :key="element.id">
          <SubTaskCard />
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
import { SubTask, type _SubTask } from "@/types/Task";
import { ref, type Ref } from "vue";
import { VueDraggableNext } from "vue-draggable-next";

let subTasks: Ref<_SubTask[]> = ref([]),
count = ref(0);

function addTask() {
  count.value++;
  subTasks.value.push(new SubTask((++count.value).toString(), "", "", ""));
}

function log(event: Event) {
  console.log(event);
}
</script>
