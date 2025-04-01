<template>
  <div>
    <!-- Mobile View -->
    <nav
      class="absolute top-0 flex w-full z-10 dark:bg-theme-primary dark:brightness-125 lg:hidden"
    >
      <div class="px-3 py-3 lg:px-3 lg:pl-3 w-full">
        <div class="flex items-center justify-between">
          <button
            class="p-1 pl-3 rounded-lg cursor-pointer text-xl hover:dark:text-theme-first"
            data-drawer-target="sideBar"
            data-drawer-toggle="sideBar"
          >
            <fai icon="fa-bars" />
          </button>
          <div class="flex-grow pl-4 pr-2">
            <div class="grid grid-cols-2 sm:grid-cols-3">
              <div
                class="h-full p-2 dark:bg-theme-primary dark:hover:brightness-110 rounded-lg ml-2 cursor-pointer items-center hidden sm:flex"
              >
                <div class="text-3xl justify-end pl-2 pr-2">
                  <div
                    class="border-3 border-theme-primary-border rounded-full transition-all duration-300"
                  >
                    <img
                      class="h-7 w-7 lg:group-hover:w-14 rounded-full"
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
                  <div class="font-bold text-md">{{ username }}</div>
                  <div class="text-xs">
                    {{ workspaceStore.getActiveWorkspace.name }}
                  </div>
                </div>
              </div>
              <div
                class="flex h-full p-2 dark:bg-theme-second hover:dark:brightness-110 rounded-lg ml-2 cursor-pointer place-content-center items-center"
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
                <div class="flex">
                  <fai icon="fa-circle-check" class="px-1" />
                </div>
              </div>
              <div
                class="flex h-full p-2 dark:bg-theme-first hover:dark:brightness-110 rounded-lg ml-2 cursor-pointer place-content-center items-center"
              >
                <div class="flex flex-grow">
                  <div class="text-3xl justify-end pl-4 pr-4">
                    {{ tasksStore.getTasksCount }}
                  </div>
                  <div class="row text-xs flex-grow place-content-center">
                    <div class="font-bold">
                      <span class="hidden md:inline">Active </span
                      ><span>Tasks</span>
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
        </div>
      </div>
    </nav>

    <nav
      class="absolute top-0 w-full flex dark:bg-theme-primary mt-[73px] lg:m-0"
    >
      <div class="px-3 py-3 lg:px-3 lg:pl-3 w-full">
        <div class="flex items-center justify-between">
          <div
            class="hidden lg:flex items-center justify-start rtl:justify-end"
          ></div>
          <div class="flex items-center w-full lg:w-fit">
            <div class="flex items-center ms-3 w-full lg:w-fit">
              <div class="flex mr-5 lg:mr-2 items-center">
                <button
                  id="tasksTypeButton"
                  data-dropdown-toggle="taskTypesDropdownIcon"
                  data-dropdown-offset-distance="10"
                  data-dropdown-offset-skidding="60"
                  data-dropdown-placement="bottom"
                  class="cursor-pointer dark:text-theme-primary-text flex lg:hidden"
                >
                  <fai icon="fa-filter" class="text-2xl" />
                </button>

                <button
                  id="tasksTypeButton"
                  data-dropdown-toggle="taskTypesDropdown"
                  class="font-medium text-sm px-5 py-2 text-center lg:inline-flex items-center border rounded-md dark:bg-theme-primary dark:brightness-125 dark:border-theme-primary-border dark:placeholder-theme-primary-border dark:text-theme-primary-text focus:outline-none cursor-pointer dark:focus:border-theme-first dark:focus:bg-theme-first ml-2 hidden"
                  type="button"
                >
                  {{ taskTypes[selectedTasksType] }}
                  <svg
                    class="w-2.5 h-2.5 ms-3"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 10 6"
                  >
                    <path
                      stroke="currentColor"
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="m1 1 4 4 4-4"
                    />
                  </svg>
                </button>

                <div
                  id="taskTypesDropdown"
                  class="z-10 hidden divide-y rounded-md shadow-sm w-44 dark:bg-theme-primary-secondary"
                >
                  <ul class="py-2 text-sm dark:text-theme-primary-text-secondary">
                    <li v-for="(type, t) in taskTypes">
                      <a
                        class="block px-4 py-2 dark:bg-theme-primary-secondary hover:dark:brightness-150 cursor-pointer"
                        @click="selectedTasksType = t"
                        >{{ type }}</a
                      >
                    </li>
                  </ul>
                </div>

                <div
                  id="taskTypesDropdownIcon"
                  class="z-10 hidden divide-y rounded-md shadow-sm w-44  dark:bg-theme-primary-secondary"
                >
                  <ul class="py-2 text-sm dark:text-theme-primary-text-secondary">
                    <li v-for="(type, t) in taskTypes">
                      <a
                        class="block px-4 py-2 dark:bg-theme-primary-secondary hover:dark:brightness-150 dark:hover:text-theme-primary-text cursor-pointer"
                        @click="selectedTasksType = t"
                        >{{ type }}</a
                      >
                    </li>
                  </ul>
                </div>
              </div>
              <div class="hidden lg:flex mr-2" v-if="showMainActionModalButton">
                <button
                  type="button"
                  class="inline-flex w-full justify-center px-5 py-2 text-sm font-semibold border rounded-md dark:bg-theme-first dark:border-theme-first dark:hover:brightness-110 dark:placeholder-theme-first dark:text-theme-primary-text focus:outline-none cursor-pointer items-center"
                  @click="headerButtonOperation()"
                >
                  <fai :icon="mainActionModalButtonContent[0]" class="mr-2 cursor-pointer" />
                  <span class="cursor-pointer">{{ mainActionModalButtonContent[1] }}</span>
                </button>
              </div>
              <div class="flex-grow pr-2 lg:pr-0">
                <label
                  for="search"
                  class="mb-2 text-sm font-medium sr-only dark:text-theme-primary-text"
                  >Search</label
                >
                <div class="relative dark:brightness-110">
                  <div
                    class="absolute inset-y-0 start-0 flex items-center ps-3"
                  >
                    <fai icon="fa-search" class="dark:text-theme-primary-placeholder"/>
                  </div>
                  <input
                    type="text"
                    id="search"
                    class="block w-full p-2 ps-10 text-sm border rounded-md dark:bg-theme-primary dark:border-theme-primary-border dark:placeholder-theme-primary-placeholder dark:text-theme-primary-text dark:focus:ring-theme-first dark:focus:border-theme-first focus:outline-none"
                    placeholder="Search"
                    autocomplete="off"
                  />
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </nav>
  </div>
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { initFlowbite } from "flowbite";
import { computed, onMounted, ref, watch, type Ref } from "vue";
import { useRoute } from "vue-router";
import type { User } from "../types/Auth";
import { useIndexStore } from "../store";
import { useWorkspaceStore } from "../store/workspace";
import { useTasksStore } from "../store/tasks";
import router from "@/router";

const indexStore = useIndexStore(),
  workspaceStore = useWorkspaceStore(),
  tasksStore = useTasksStore(),
  taskTypes = computed(() => {
    return ["My Tasks", "My Active Tasks", "All Tasks"];
  }),
  route = useRoute(),
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
  mainActionModalButtonContent = computed(() => {
    if (route.name == "dashboard") return ["fa-plus", "Start New Task"];
    else if (route.name == "templates" && route.query.type == "new")
      return ["fa-save", "Save"];
    else if (route.name == "templates")
      return ["fa-plus", "Start New Template"];
    else if (route.name == "team") return ["fa-plus", "Invite User"];
    else return "Start New Task";
  }),
  showMainActionModalButton = computed(() => {
    return (
      route.name == "dashboard" ||
      route.name == "templates" ||
      route.name == "team"
    );
  });

let selectedTasksType = ref(1);

async function headerButtonOperation() {
  if (route.name == "dashboard") {
    router.push("/templates");
  } else if (route.name == "templates" && !route.query.type) {
    router.push("/templates?type=new");
  } else if (route.name == "templates" && route.query.type == "new") {
    await tasksStore.createTemplate();
    await tasksStore.loadTemplates(workspaceStore.activeWorkspace);
    router.push("/templates");
  }
}

onMounted(() => {
  initFlowbite();
});

watch(
  () => route.path,
  () => {
    initFlowbite();
  }
);
</script>
