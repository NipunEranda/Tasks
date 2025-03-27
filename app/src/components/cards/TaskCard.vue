<template>
  <div
    class="relative flex bg-zinc-800 rounded-lg cursor-pointer transition-all duration-300 hover:brightness-110 mb-4 z-10"
  >
    <div class="w-[10px] bg-teal-500 rounded-l-lg py-10"></div>
    <div class="flex-grow flex">
      <div class="flex-grow p-3">
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          >
            <fai icon="fa-list" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-teal-500"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                name="name"
                id="name"
                class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 dark:focus:border-teal-500 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500 peer"
                placeholder="Template Name"
                required
                autocomplete="off"
                v-model="props.task.name"
              />
            </div>
          </div>
        </span>
        <span class="flex mb-2 group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          >
            <fai icon="fa-circle-info" />
          </div>
          <div class="flex-grow">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-teal-500"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                name="description"
                id="description"
                class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500"
                placeholder="Template Description"
                required
                autocomplete="off"
                v-model="props.task.description"
              />
            </div>
          </div>
        </span>
        <div class="flex mb-2">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          >
            <fai icon="fa-tags" />
          </div>
          <div class="flex-grow group">
            <div class="relative">
              <div
                class="absolute inset-y-0 end-0 flex items-center pr-3.5 pointer-events-none dark:group-focus-within:text-teal-500"
              >
                <fai icon="fa-pen" />
              </div>
              <input
                type="text"
                name="floating_email"
                id="floating_email"
                class="block px-0 w-full text-md text-gray-900 bg-transparent border-0 border-b-2 border-gray-300 appearance-none dark:text-white dark:border-gray-600 focus:outline-none focus:ring-0 dark:group-focus-within:border-teal-500 cursor-pointer"
                placeholder="Tags"
                required
                autocomplete="off"
              />

              <div
                id="dropdown"
                class="p-3 dropdown hidden absolute w-full border dark:bg-zinc-800 dark:border-zinc-700 shadow-md mt-1 rounded-md z-20 group-focus-within:block"
              >
                <div class="w-full" v-if="taskStore.getTagsCount > 0">
                  <button
                    id="badge-dismiss-default"
                    class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium rounded-sm dark:bg-[#ee855be1] hover:dark:brightness-110 cursor-pointer"
                    v-for="tag in taskStore.getTags"
                    :key="tag.id"
                    @click="addTagToTask(tag)"
                  >
                    {{ tag.name }}
                    <span
                      class="inline-flex items-center p-1 ms-2 text-sm text-zinc-50 bg-transparent rounded-xs cursor-pointer"
                      data-dismiss-target="#badge-dismiss-default"
                      aria-label="Remove"
                      @click="openActionModal('tag', 'remove', tag)"
                    >
                      <fai icon="fa-xmark" />
                      <span class="sr-only">Remove badge</span>
                    </span>
                  </button>
                </div>

                <div class="mt-5">
                  <button
                    type="button"
                    class="inline-flex w-full rounded-md px-4 py-2 text-sm font-semibold text-white shadow-sm sm:w-auto dark:bg-[#ee855bd1] dark:hover:brightness-110 cursor-pointer"
                    @click="openTagModal('add')"
                  >
                    <fai icon="fa-plus" class="mr-2 self-center" />
                    Create New Tag
                  </button>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="ml-8 mt-3">
          <span
            v-for="tag in props.task.tags.filter(tag => tag)"
            class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium text-yellow-800 bg-yellow-100 rounded-sm dark:bg-yellow-900 dark:text-yellow-300 z-50"
          >
          {{ tag.name }}
            <button
              type="button"
              class="inline-flex items-center p-1 ms-2 text-sm text-yellow-400 bg-transparent rounded-xs hover:bg-yellow-200 hover:text-yellow-900 dark:hover:bg-yellow-800 dark:hover:text-yellow-300 cursor-pointer"
              aria-label="Remove"
              @click="removeTag(tag)"
            >
              <fai icon="fa-xmark" class="w-2 h-2" />
              <span class="sr-only">Remove badge</span>
            </button>
          </span>
        </div>
        <span class="flex group">
          <div
            class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-teal-500"
          ></div>
          <div class="flex-grow">
            <label class="inline-flex items-center cursor-pointer mt-5">
              <input type="checkbox" v-model="task.visibility" class="sr-only peer" />
              <div
                class="relative w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-0 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:w-5 after:h-5 after:transition-all dark:border-gray-600 peer-checked:bg-teal-600 dark:peer-checked:bg-teal-600"
              ></div>
              <span class="ms-3 text-sm font-medium dark:text-zinc-400"
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
import type { Task } from "@/types/Task";

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

function openActionModal(type: string, operation: string, tag: Tag) {
  taskStore.selectedTag = tag;

  switch (type) {
    case "tag":
      if (operation == "remove") {
        if (indexStore.actionModal) {
          indexStore.actionModal.message = `<span class='font-bold text-lg'>Do you want to remove <b class='text-red-600'>${tag.name}</b> tag ?</span>`;
          indexStore.actionModal.processName = "Remove";
          indexStore.actionModal.title = "Remove Tag";
          indexStore.actionModal.type = operation;
          indexStore.actionModal.modalEl?.show();
        }
      }
      break;
    default:
      break;
  }
}

function addTagToTask(tag: Tag) {
  props.task.tags.push(tag);
}

function removeTag(tag: Tag) {
  delete props.task.tags[
    props.task.tags.indexOf(props.task.tags.filter((t) => t == tag)[0])
  ];
  props.task.tags = props.task.tags.map((t) => t);
}
</script>
