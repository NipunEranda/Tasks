<template>
  <div>
    <div class="dark:bg-zinc-800 p-3 rounded-md cursor-pointer">
      <div>
        <div class="w-full grid">
          <div
            class="h-full dark:bg-zinc-800 hover:dark:brightness-110 rounded-lg cursor-pointer items-center hidden sm:flex p-2"
          >
            <div class="text-3xl justify-end pr-2">
              <div
                class="border-3 border-zinc-700 rounded-full transition-all duration-300"
              >
                <img
                  class="h-9 w-9 lg:group-hover:w-14 rounded-full"
                  :class="{
                    invert: user ? (user.picture ? false : true) : true,
                  }"
                  :src="profileImage"
                  alt=""
                  referrerpolicy="no-referrer"
                />
              </div>
            </div>
            <div class="row text-xs place-content-center">
              <div class="font-bold lg:text-md xl:text-lg">{{ username }}</div>
              <div class="text-xs">
                {{ workspaceStore.getActiveWorkspace.name }}
              </div>
            </div>
          </div>
        </div>
        <div
          class="flex h-full p-2 dark:bg-purple-800 hover:dark:brightness-110 rounded-lg mt-2 cursor-pointer place-content-center items-center"
        >
          <div class="flex flex-grow">
            <div class="text-3xl justify-end pl-4 pr-4">
              {{ tasksStore.getSubTasksCount }}
            </div>
            <div class="row text-xs flex-grow place-content-center">
              <div class="font-bold">
                <span class="hidden md:inline">Active </span
                ><span>SubTasks</span>
              </div>
              <div class="text-xs">0 New</div>
            </div>
          </div>
          <div class="flex pl-2">
            <fai icon="fa-circle-check" class="px-1" />
          </div>
        </div>
        <div
          class="flex h-full p-2 dark:bg-teal-800 hover:dark:brightness-110 rounded-lg mt-2 cursor-pointer place-content-center items-center"
        >
          <div class="flex flex-grow">
            <div class="text-3xl justify-end pl-4 pr-4">
              {{ tasksStore.getTasksCount }}
            </div>
            <div class="row text-xs flex-grow place-content-center">
              <div class="font-bold">
                <span class="hidden md:inline">Active </span><span>Tasks</span>
              </div>
              <div class="text-xs">0 New</div>
            </div>
          </div>
          <div class="flex">
            <fai icon="fa-list" class="px-1" />
          </div>
        </div>
      </div>
    </div>
    <div
      id="section1"
      class="dark:bg-zinc-800 rounded-md mt-3 cursor-pointer"
      v-if="showSection1"
    >
      <div
        class="text-center font-bold text-xl p-3 dark:bg-teal-700 rounded-t-md"
      >
        {{ section1.title }}
      </div>
      <div class="w-full px-3">
        <div class="text-sm dark:text-zinc-300 font-bold py-2 pb-1">
          {{ tasksStore.updatedDate }}
        </div>
        <!-- List -->
        <div>
          <div
            class="border-b dark:bg-zinc-800 hover:dark:brightness-110 dark:border-zinc-600 flex items-center p-1 px-3 cursor-pointer"
          >
            <div
              class="flex border-2 dark:border-purple-600 items-center place-items-center p-1 rounded-sm mr-2"
            >
              <fai icon="fa-list" class="dark:text-purple-600" />
            </div>
            <div class="flex-grow text-sm">
              <div>Sub Task</div>
              <div>status</div>
            </div>
          </div>
          <div
            class="border-b dark:bg-zinc-800 hover:dark:brightness-110 dark:border-zinc-600 flex items-center p-1 px-3 cursor-pointer"
          >
            <div
              class="flex border-2 dark:border-teal-600 items-center place-items-center p-1 rounded-sm mr-2"
            >
              <fai icon="fa-list" class="dark:text-teal-600" />
            </div>
            <div class="flex-grow text-sm">
              <div>Task</div>
              <div>status</div>
            </div>
          </div>
        </div>
      </div>
      <div
        class="p-2 uppercase dark:text-teal-600 text-center items-center place-items-center cursor-pointer text-sm font-bold"
      >
        <div
          class="hover:dark:bg-teal-600/10 rounded-full w-fit p-2 px-4"
          @click="$router.push('/activity')"
        >
          Show More
        </div>
      </div>
    </div>
    <div
      id="section2"
      class="dark:bg-zinc-800 rounded-md mt-3"
      v-if="showSection2"
    >
      <div
        class="text-center font-bold text-xl p-3 dark:bg-theme-third dark:brightness-110 rounded-t-md"
      >
        {{ section2.title }}
      </div>
      <div class="p-3" v-if="tasksStore.getTagsCount > 0">
        <span
          id="badge-dismiss-default"
          class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium rounded-sm dark:bg-theme-third hover:dark:brightness-110"
          v-for="tag in tasksStore.getTags"
          :key="tag.id"
          >{{ tag.name }}
          <button
            type="button"
            class="inline-flex items-center p-1 ms-2 text-sm text-zinc-50 bg-transparent rounded-xs cursor-pointer"
            data-dismiss-target="#badge-dismiss-default"
            @click="openActionModal('tag', 'remove', tag)"
          >
            <svg
              class="w-2 h-2"
              xmlns="http://www.w3.org/2000/svg"
              fill="none"
              viewBox="0 0 14 14"
            >
              <path
                stroke="currentColor"
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="m1 1 6 6m0 0 6 6M7 7l6-6M7 7l-6 6"
              />
            </svg>
            <span class="sr-only">Remove badge</span>
          </button>
        </span>
      </div>
      <div class="px-3" :class="{ 'pt-2': tasksStore.getTagsCount == 0 }">
        <button
          ref="triggerButton"
          class="w-full border focus:outline-none font-medium rounded-lg text-sm px-5 py-2.5 text-center dark:border-theme-third dark:text-theme-third dark:hover:bg-theme-third/10 cursor-pointer"
          id="addTagButton"
          @click="openTagModal('add')"
        >
          Add Tag
        </button>
      </div>
      <div
        class="p-2 uppercase dark:text-theme-third text-center items-center place-items-center text-sm font-bold"
      >
        <div
          class="hover:dark:bg-theme-third/10 rounded-full w-fit p-2 px-4 cursor-pointer"
        >
          Show More
        </div>
      </div>
    </div>

    <TagsModal :modal="indexStore.tagModal" />
    <ActionModal :modal="indexStore.actionModal" :modalIcon="modalIcon" />
  </div>
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, onMounted, ref, type Ref } from "vue";
import { useIndexStore } from "../store";
import type { User } from "../types/Auth";
import { useWorkspaceStore } from "../store/workspace";
import { useTasksStore } from "../store/tasks";
import { useRoute } from "vue-router";
import { CustomModal } from "../types/Modal";
import { initFlowbite } from "flowbite";
import type { _Tag, Tag } from "../types/Tag";
import { initModal } from "@/utils";

const indexStore = useIndexStore(),
  workspaceStore = useWorkspaceStore(),
  tasksStore = useTasksStore(),
  user: Ref<User | null> = computed(() => {
    return indexStore.currentUser || null;
  }),
  profileImage: Ref<string> = computed(() => {
    if (user.value) {
      if (user.value.picture) {
        return user.value.picture;
      }
      return profile;
    } else return profile;
  }),
  username = computed(() => {
    return user.value ? user.value.name.split(" ").splice(0, 2).join(" ") : "";
  }),
  section1 = ref({
    title: "Recent Changes",
  }),
  section2 = ref({
    title: "Tags",
  }),
  route = useRoute(),
  tagsModal = computed(() => {
    const modal: CustomModal = CustomModal.createObj(
      "tagModal",
      "New Tag",
      "add",
      "Save",
      tagModalProcess,
      undefined
    );

    indexStore.tagModal = modal;
    return indexStore.tagModal;
  }),
  actionModal = computed(() => {
    const modal: CustomModal = CustomModal.createObj(
      "actionModal",
      "Remove Modal",
      "remove",
      "Remove",
      actionModalProcess,
      undefined
    );

    indexStore.actionModal = modal;
    return indexStore.actionModal;
  }),
  modalIcon = ref("fa-tag");

async function loadData() {
  await tasksStore.loadTags(workspaceStore.activeWorkspace);
}

function openTagModal(type: string) {
  tagsModal.value.type = type;
  // @ts-ignore
  tagsModal.value.modalEl.show();
}

function openActionModal(type: string, operation: string, tag: Tag) {
  tasksStore.selectedTag = tag;
  initModal(actionModal.value, "actionModal", "addTagButton");

  switch (type) {
    case "tag":
      if (operation == "remove") {
        actionModal.value.message = `<span class='font-bold text-lg'>Do you want to remove <b class='text-red-600'>${tag.name}</b> tag ?</span>`;
        actionModal.value.processName = "Remove";
        actionModal.value.title = "Remove Tag";
        actionModal.value.type = operation;
      }
      break;
    default:
      break;
  }

  actionModal.value.modalEl?.show();
}

async function tagModalProcess(tag: Tag) {
  await tasksStore.createTag(tag, tagsModal.value);
}

async function actionModalProcess() {
  if (tasksStore.selectedTag) {
    tasksStore.selectedTag.workspace = workspaceStore.activeWorkspace;
    await tasksStore.deleteTag(tasksStore.selectedTag, actionModal.value);
  }
}

let showSection1 = computed(() => {
  if (
    route.path == "/dashboard" ||
    route.path == "/templates" ||
    route.path == "/team"
  ) {
    return true;
  }
  return false;
});

let showSection2 = computed(() => {
  if (
    route.path == "/dashboard" ||
    route.path == "/templates" ||
    route.path == "/team"
  ) {
    return true;
  }
  return false;
});

onMounted(async () => {
  initFlowbite();
  initModal(tagsModal.value, "tagModal", "addTagButton");
  initModal(actionModal.value, "actionModal", "addTagButton");
  await loadData();
});
</script>
