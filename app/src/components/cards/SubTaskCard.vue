<template>
  <div
    class="flex items-center space-x-2 mt-4"
    :name="'subtask-' + props.subTask.id + '-name'"
    :id="'subtask-' + props.subTask.id + '-id'"
  >
    <button
      class="rounded-full dark:text-theme-primary-text dark:bg-theme-danger brightness-110 hover:brightness-125 hover:cursor-pointer text-xl flex p-4"
      @click="removeSubTask(props.subTask.id)"
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
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Sub Task Name"
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
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Sub Task Description"
                required
                autocomplete="off"
                v-model="subTask.description"
              />
            </div>
          </span>
          <div class="flex mb-2 group">
            <div
              class="w-7 mr-2 place-content-center text-center dark:group-focus-within:text-theme-first dark:group-focus-within:brightness-200"
            >
              <fai icon="fa-circle-user" />
            </div>
            <div
              class="flex-grow relative"
              :id="'subtask-assigneeDropdown-' + props.subTask.id + '-container'"
            >
              <input
                type="text"
                :name="'subtask-assignee-' + props.subTask.id + '-name'"
                :id="'subtask-assignee-' + props.subTask.id + '-name'"
                class="block px-0 w-full text-md bg-transparent border-0 border-b-2 appearance-none dark:text-theme-primary-text-secondary dark:border-theme-primary-border dark:group-focus:brightness-200 dark:focus:border-theme-first focus:outline-none focus:ring-0 dark:group-focus-within:border-theme-first dark:group-focus-within:brightness-200"
                placeholder="Assignees"
                required
                autocomplete="off"
                @click="
                  toggleElement('subtask-assigneeDropdown-' + props.subTask.id, true)
                "
              />

              <div
                :id="'subtask-assigneeDropdown-' + props.subTask.id"
                class="subtask-assigneeDropdown p-3 dropdown hidden absolute w-full border dark:bg-theme-primary-secondary dark:border-theme-primary-border shadow-md mt-1 rounded-md z-20"
              >
                <button
                  id="member"
                  class="inline-flex items-center me-2 text-sm font-medium rounded-sm dark:bg-theme-primary-secondary dark:brightness-120 hover:dark:brightness-150 cursor-pointer border-2 border-theme-primary-border/20"
                  :class="{ 'opacity-50': props.subTask.assignees.includes(member.id) }"
                  v-for="member in workspaceStore.team"
                  :key="member.id"
                  @click="addToAssignees(props.subTask.id, member.id)"
                  :disabled="props.subTask.assignees.includes(member.id)"
                >
                  <img
                    class="h-9 w-9 dark:brightness-90 group-hover:dark:brightness-70 rounded-sm"
                    :class="{
                      invert: member ? (member.picture ? false : true) : true,
                    }"
                    :src="profileImage(member)"
                    alt=""
                    referrerpolicy="no-referrer"
                  />
                  <span class="text-sm m-1 mx-3">{{ member.name }}</span>
                </button>
              </div>
            </div>
          </div>

          <div class="ml-8 mt-5">
            <button
              class="inline-flex items-center me-2 text-sm font-medium rounded-sm dark:bg-theme-primary-secondary dark:brightness-120 hover:dark:brightness-150 cursor-pointer border-2 border-theme-primary-border/20 pr-2"
              v-for="member in workspaceStore.team.filter((t: _User) => props.subTask.assignees.includes(t.id))"
              :key="member.id"
            >
              <img
                class="h-9 w-9 dark:brightness-90 group-hover:dark:brightness-70 rounded-sm"
                :class="{
                  invert: member ? (member.picture ? false : true) : true,
                }"
                :src="profileImage(member)"
                alt=""
                referrerpolicy="no-referrer"
              />
              <span class="text-sm m-1 ml-3">{{ member.name }}</span>
              <fai icon="fa-xmark" class="ml-2 self-center dark:text-theme-primary-text-secondary" @click="removeAssignee(member.id)"/>
            </button>
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
import { type _SubTask } from "@/types/Task";
import { toggleElement } from "@/utils";

const workspaceStore = useWorkspaceStore();

const props = defineProps<{
  index: number;
  subTask: _SubTask;
  removeSubTask: Function;
}>();

function profileImage(member: _User) {
  if (member) {
    if (member.picture) {
      return member.picture;
    }
    return profile;
  } else return profile;
}

function addToAssignees(taskId: string, mid: string) {
  if (!props.subTask.assignees.includes(mid)) props.subTask.assignees.push(mid);
  toggleElement("subtask-assigneeDropdown-" + taskId, true);
}

function removeAssignee(id: string) {
  delete props.subTask.assignees[props.subTask.assignees.indexOf(id)];
  props.subTask.assignees = props.subTask.assignees.filter(a => a);
}

// Close assignee dropdowns on outer click
document.body.addEventListener("click", function (event) {
  if (event.target) {
    if (!(event.target as HTMLElement).id.includes("subtask-")) {
      Array.from(document.getElementsByClassName("subtask-assigneeDropdown")).forEach((element) => {
        if (!(event.target as HTMLElement).id.includes("subtask-assignee-"))
          toggleElement(element.id, false);
      });
    }
  }
});
</script>
