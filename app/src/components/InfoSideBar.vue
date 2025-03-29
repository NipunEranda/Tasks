<template>
  <div>
    <div
      class="dark:bg-theme-primary-secondary/60 p-3 rounded-md cursor-pointer"
    >
      <div>
        <div class="w-full grid">
          <div
            class="h-full dark:bg-theme-primary-secondary hover:dark:brightness-130 rounded-lg cursor-pointer items-center hidden sm:flex p-2 group"
          >
            <div class="text-3xl justify-end pr-2">
              <div
                class="border-3 dark:border-theme-primary-border rounded-full transition-all duration-300"
              >
                <img
                  class="h-9 w-9 rounded-full"
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
          class="flex h-full p-2 dark:bg-theme-second dark:brightness-90 hover:dark:brightness-100 rounded-lg mt-2 cursor-pointer place-content-center items-center"
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
          class="flex h-full p-2 dark:bg-theme-first dark:brightness-90 hover:dark:brightness-100 rounded-lg mt-2 cursor-pointer place-content-center items-center"
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
      class="dark:bg-theme-primary-secondary/60 rounded-md mt-3 cursor-pointer"
      v-if="showSection1"
    >
      <div
        class="text-center font-bold text-xl p-3 dark:bg-theme-first rounded-t-md"
      >
        {{ section1.title }}
      </div>
      <div class="w-full px-3">
        <div
          class="text-sm dark:text-theme-primary-text-secondary font-bold py-4"
        >
          {{ tasksStore.updatedDate }}
        </div>
        <!-- List -->
        <div>
          <div
            class="border-b hover:dark:bg-theme-primary-secondary hover:dark:brightness-125 dark:border-theme-primary-border flex items-center p-1 px-3 cursor-pointer"
          >
            <div
              class="flex border-2 dark:border-theme-second items-center place-items-center p-1 rounded-sm mr-2"
            >
              <fai icon="fa-list" class="dark:text-theme-second" />
            </div>
            <div class="flex-grow text-sm">
              <div>Sub Task</div>
              <div>status</div>
            </div>
          </div>
          <div
            class="border-b hover:dark:bg-theme-primary-secondary hover:dark:brightness-125 dark:border-theme-primary-border flex items-center p-1 px-3 cursor-pointer"
          >
            <div
              class="flex border-2 dark:border-theme-first items-center place-items-center p-1 rounded-sm mr-2"
            >
              <fai icon="fa-list" class="dark:text-theme-first" />
            </div>
            <div class="flex-grow text-sm">
              <div>Task</div>
              <div>status</div>
            </div>
          </div>
        </div>
      </div>
      <div
        class="p-2 uppercase dark:text-theme-first text-center items-center place-items-center cursor-pointer text-sm font-bold"
      >
        <div
          class="hover:dark:bg-theme-first/20 rounded-full w-fit p-2 px-4"
          @click="$router.push('/activity')"
        >
          Show More
        </div>
      </div>
    </div>
    <div
      id="section2"
      class="dark:bg-theme-primary-secondary/60 rounded-md mt-3"
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
          class="inline-flex items-center px-2 py-1 me-2 text-sm font-medium rounded-sm dark:bg-theme-third dark:brightness-100 hover:dark:brightness-115 cursor-pointer"
          v-for="tag in tasksStore.getTags"
          :key="tag.id"
        >
          <fai
            icon="fa-lock"
            class="mr-1 self-center text-xs"
            v-show="tag.is_private"
          />
          <span class="text-sm m-1">{{ tag.name }}</span>
          <button
            type="button"
            class="inline-flex items-center p-1 ms-2 text-sm dark:text-theme-primary-text bg-transparent rounded-xs cursor-pointer"
            data-dismiss-target="#badge-dismiss-default"
            @click="openActionModal('tag', 'remove', tag)"
          >
            <fai icon="fa-xmark" />
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
        actionModal.value.message = `<span class='font-bold text-lg'>Do you want to remove <b class='dark:text-theme-danger dark:brightness-130'>${tag.name}</b> tag ?</span>`;
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
