<template>
  <div
    class="relative flex dark:bg-theme-primary-secondary/40 rounded-lg cursor-pointer transition-all duration-300 hover:brightness-110 mb-4 z-10"
  >
    <div class="w-[10px] dark:bg-theme-first rounded-l-lg py-10"></div>
    <div class="flex-grow flex">
      <div class="flex-grow p-3">
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
          >
            <fai icon="fa-list" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                name="name"
                id="name"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Task Name"
                required
                autocomplete="off"
                v-model="props.task.name"
              />
            </div>
          </div>
        </span>
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
          >
            <fai icon="fa-circle-info" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                name="description"
                id="description"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Task Description"
                required
                autocomplete="off"
                v-model="props.task.description"
              />
            </div>
          </div>
        </span>
        <div class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
          >
            <fai icon="fa-tags" />
          </div>
          <div class="flex-grow group">
            <div class="relative" id="task-tagDropdown-container">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Select Tags"
                required
                autocomplete="off"
                name="task-tag-name"
                id="task-tag-id"
                @click="toggleElement('task-tagDropdown', true)"
              />

              <div
                id="task-tagDropdown"
                class="task-tagDropdown p-3 dropdown hidden absolute w-full border dark:bg-theme-primary-secondary dark:border-theme-primary-border shadow-md mt-1 rounded-md z-20"
              >
                <div class="w-full" v-if="taskStore.getTagsCount > 0">
                  <button
                    id="badge-dismiss-default"
                    class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium rounded-sm dark:bg-theme-third hover:dark:brightness-110 cursor-pointer"
                    :class="{ 'opacity-50': props.task.tags.includes(tag.id) }"
                    v-for="tag in taskStore.getTags"
                    :key="tag.id"
                    @click="addTagToTask(tag)"
                    :disabled="props.task.tags.includes(tag.id)"
                  >
                    <fai
                      icon="fa-lock"
                      class="mr-2 self-center text-xs"
                      v-show="tag.is_private"
                    />
                    <span class="text-sm m-1">{{ tag.name }}</span>
                  </button>
                </div>

                <div class="mt-5">
                  <button
                    type="button"
                    class="inline-flex w-full rounded-md px-4 py-2 text-sm font-semibold text-white shadow-sm sm:w-auto dark:bg-theme-third dark:hover:brightness-110 cursor-pointer"
                    @click="openTagModal('add')"
                  >
                    <fai icon="fa-plus" class="mr-1 self-center" />
                    Create New Tag
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="ml-8 mt-3">
          <span
            v-for="tag in taskStore.getTags.filter((t: _Tag) => props.task.tags.includes(t.id))"
            class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium rounded-sm dark:bg-theme-third/90 dark:hover:brightness-110 dark:text-theme-primary-text"
          >
            <fai
              icon="fa-lock"
              class="mr-1 self-center text-xs"
              v-show="tag.is_private"
            />
            <span class="text-sm m-1">{{ tag.name }}</span>
            <button
              type="button"
              class="inline-flex items-center p-1 ms-2 text-sm bg-transparent rounded-xs dark:text-theme-primary-text cursor-pointer"
              @click="removeTag(tag)"
            >
              <fai icon="fa-xmark" class="w-2 h-2" />
              <span class="sr-only">Remove badge</span>
            </button>
          </span>
        </div>
        <span class="flex group">
          <div class="w-7 mr-2 place-content-center text-center"></div>
          <div class="flex-grow">
            <label class="inline-flex items-center cursor-pointer mt-5">
              <input
                type="checkbox"
                v-model="task.is_private"
                class="sr-only peer"
              />
              <div
                class="relative w-11 h-6 peer-focus:outline-none peer-focus:ring-0 rounded-full peer dark:bg-theme-primary-border peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-theme-primary-border after:border after:rounded-full after:w-5 after:h-5 after:transition-all dark:border-theme-primary-border dark:peer-checked:bg-theme-first dark:peer-checked:brightness-150"
              ></div>
              <span
                class="ms-3 text-sm font-medium dark:text-theme-primary-text-secondary"
                >Private</span
              >
            </label>
          </div>
        </span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useTasksStore } from "@/store/tasks";
import { useIndexStore } from "@/store";
import type { _Tag, Tag } from "@/types/Tag";
import { type Task } from "@/types/Task";
import { onMounted } from "vue";
import { toggleElement } from "@/utils";

const props = defineProps<{
  task: Task;
}>();

const taskStore = useTasksStore(),
  indexStore = useIndexStore();

function openTagModal(type: string) {
  if (indexStore.tagModal) {
    indexStore.tagModal.type = type;
    indexStore.tagModal.modalEl?.show();
  }
}

function addTagToTask(tag: Tag) {
  if (!props.task.tags.includes(tag.id)) props.task.tags.push(tag.id);
  toggleElement("task-tagDropdown", true);
}

function removeTag(tag: Tag) {
  delete props.task.tags[props.task.tags.indexOf(tag.id)];
  props.task.tags = props.task.tags.map((t) => t);
}

// Close assignee dropdowns on outer click
document.body.addEventListener("click", function (event) {
  if (event.target) {
    if (!(event.target as HTMLElement).id.includes("task-")) {
      toggleElement("task-tagDropdown", false);
    }
  }
});
</script>
