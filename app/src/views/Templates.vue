<template>
  <div class="pr-5">
    <div v-if="!isNewTemplate">
      <div v-for="template in templates" :key="template.id" class="group border border-theme-primary-border/50 rounded-lg">
        <div
          class="flex bg-theme-primary dark:brightness-125 group-hover:dark:brightness-150 rounded-lg cursor-pointer transition-all duration-300"
        >
          <div class="w-[10px] dark:bg-theme-second rounded-l-lg py-10"></div>
          <div class="flex-grow p-4">
            <div class="flex w-full">
              <span class="dark:text-theme-primary-text-secondary">
                <fai icon="fa-list" class=" text-xl"/>
                <span class="ml-2 font-bold text-xl">{{ template.name }}</span
                ><br />
                <small class="mt-2">{{ template.description }}</small>
              </span>
            </div>
            <div class="py-4 pt-6 flex">
              <div v-for="(subTask, s) in template.sub_tasks">
                <div class="flex">
                  <div
                    class="w-9 h-9 border-3 border-theme-first dark:brightness-200 rounded-full"
                  ></div>
                  <div
                    class="w-[20px] border-2 border-theme-first dark:brightness-200 self-center"
                    v-if="s < template.sub_tasks.length - 1"
                  ></div>
                </div>
              </div>
            </div>
            <div class="flex w-full">
              <div class="flex-grow flex">
                <span
                  class="dark:text-theme-primary-text-secondary border-1 border-theme-primary-border px-2 rounded-md flex items-center"
                >
                  <fai
                    :icon="template.is_private ? 'fa-lock' : 'fa-lock-open'"
                    class="text-xs mr-1"
                  />
                  <span>{{ template.is_private ? "private" : "public" }}</span>
                </span>

                <span
                  v-for="tag in tasksStore.tags.filter((t) =>
                    template.tags.includes(t.id)
                  )"
                  :key="tag.id"
                  class="ml-2 flex"
                >
                  <span
                    class="dark:text-theme-second border-1 border-theme-second px-2 rounded-md flex items-center"
                  >
                    <fai icon="fa-tag" class="text-xs mr-1" />
                    <span>{{ tag.name }}</span>
                  </span>
                </span>
              </div>
              <span
                class="flex dark:text-theme-primary-text-secondary items-center"
                >Created By
                <img
                  :src="
                    profileImage(
                      workspaceStore.team.filter(
                        (t) => t.id == template.created_by
                      )[0]
                    )
                  "
                  class="w-7 h-7 rounded-full ml-2"
              /></span>
            </div>
          </div>
        </div>
      </div>
    </div>
    <NewTemplate v-if="isNewTemplate"></NewTemplate>
  </div>
</template>

<script setup lang="ts">
import profile from "@/assets/img/profile.png";
import { computed, onMounted, watch, ref, type Ref } from "vue";
import { useIndexStore } from "../store";
import { useRoute } from "vue-router";
import { useTasksStore } from "@/store/tasks";
import { useWorkspaceStore } from "@/store/workspace";
import type { _Task } from "@/types/Task";
import { initFlowbite } from "flowbite";
import type { _User } from "@/types/Auth";

const indexStore = useIndexStore(),
  tasksStore = useTasksStore(),
  workspaceStore = useWorkspaceStore(),
  user = computed(() => {
    return indexStore.currentUser;
  }),
  route = useRoute(),
  templates: Ref<_Task[]> = ref([]),
  isNewTemplate = computed(() => route.query.type === "new");

function profileImage(user: _User) {
  if (user) {
    if (user.picture) {
      return user.picture;
    }
    return profile;
  } else return profile;
}

async function getTemplates() {
  await tasksStore.loadTemplates(workspaceStore.activeWorkspace);
  templates.value = tasksStore.templates;
}

watch(
  () => route.path,
  async () => {
    initFlowbite();
    await getTemplates();
  }
);

onMounted(async () => {
  initFlowbite();
  await getTemplates();
});
</script>
