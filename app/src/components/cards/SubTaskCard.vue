<template>
  <div
    class="flex items-center space-x-2 mt-4"
    :name="'subtask-' + props.subTask.id + '-name'"
    :id="'subtask-' + props.subTask.id + '-id'"
  >
    <button
      class="rounded-full dark:text-theme-primary-text dark:bg-theme-danger brightness-110 hover:brightness-125 hover:cursor-pointer text-xl flex p-4"
      @click="removeSubTask(subTask.id)"
    >
      <fai icon="fa-trash-can" />
    </button>
    <div
      class="flex bg-theme-primary-secondary/80 rounded-lg cursor-pointer transition-all duration-300 hover:brightness-110 mb-4 w-full"
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
              <input
                type="text"
                :name="'subtask-name-' + props.subTask.id + '-name'"
                :id="'subtask-name-' + props.subTask.id + '-id'"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Task Name"
                required
                autocomplete="off"
                v-model="subTask.name"
              />
            </div>
          </span>
          <span class="flex mb-2 group">
            <div
              class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
            >
              <fai icon="fa-circle-info" />
            </div>
            <div class="flex-grow">
              <input
                type="text"
                :name="'subtask-description-' + props.subTask.id + '-name'"
                :id="'subtask-description-' + props.subTask.id + '-name'"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Task Description"
                required
                autocomplete="off"
                v-model="subTask.description"
              />
            </div>
          </span>
          <div class="flex mb-2 group">
            <div class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200">
              <fai icon="fa-circle-user" />
            </div>
            <div class="flex-grow">
              <input
                type="text"
                :name="'subtask-assignee-' + props.subTask.id + '-name'"
                :id="'subtask-assignee-' + props.subTask.id + '-name'"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Assignees"
                required
                autocomplete="off"
              />

              <div
                id="dropdown"
                class="dropdown hidden absolute border dark:bg-theme-primary-secondary dark:border-theme-primary-border shadow-md mt-1 rounded-md z-20 group-focus-within:flex"
              >
                <div class="min-h-30 max-h-42 overflow-scroll group" v-if="workspaceStore.team.length > 0">
                  <div
                    class="flex w-full p-2 hover:dark:bg-theme-primary-secondary hover:dark:brightness-125 pr-10"
                    v-for="member in workspaceStore.team"
                    :key="member.id"
                  >
                    <div class="border-3 dark:border-theme-primary-border group-hover:dark:border-theme-first rounded-full transition-all duration-300 mr-2">
                      <img
                        class="h-9 w-9 rounded-full dark:brightness-90"
                        :class="{
                          invert: member
                            ? member.picture
                              ? false
                              : true
                            : true,
                        }"
                        :src="profileImage(member)"
                        alt=""
                        referrerpolicy="no-referrer"
                      />
                    </div>
                    <div class="self-center">{{ member.name }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import profile from "@/assets/img/profile.png";
import { useWorkspaceStore } from "@/store/workspace";
import type { _User } from "@/types/Auth";
import type { _SubTask } from "@/types/Task";

const workspaceStore = useWorkspaceStore();

const props = defineProps<{
  subTask: _SubTask,
  removeSubTask: Function
}>();

function profileImage(member: _User) {
  if (member) {
    if (member.picture) {
      return member.picture;
    }
    return profile;
  } else return profile;
}
</script>
