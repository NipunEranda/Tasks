<template>
  <div class="pr-5">
    <div v-for="member in workspaceStore.team" :key="member.id">
      <div
        class="group flex bg-zinc-800 mb-1 rounded-lg justify-center place-items-center cursor-pointer transition-all duration-300 hover:brightness-125"
      >
        <div class="w-[10px] dark:bg-theme-first rounded-l-lg py-10"></div>
        <div class="flex-grow flex items-center">
          <div class="w-20 place-items-center">
            <img
              class="h-14 w-14 lg:group-hover:w-14 rounded-full border-2 border-zinc-400 dark:group-hover:border-theme-first"
              :class="{
                invert: member ? (member.picture ? false : true) : true,
              }"
              :src="profileImage(member)"
              alt=""
              referrerpolicy="no-referrer"
            />
          </div>
          <div class="flex-grow">
            <span class="text-xl font-bold text-zinc-400">
              {{ member.name }} </span
            ><br />
            <span class="text-sm font-extralight text-zinc-100">
              {{ member.email }}
            </span>
          </div>
          <div class="w-fit px-5 font-bold text-lg dark:text-theme-first">
            <fai icon="fa-circle-check" class="mr-2" /><span>Online</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import profile from "@/assets/img/profile.png";
import { onMounted, watch } from "vue";
import { useRoute } from "vue-router";
import { useWorkspaceStore } from "../store/workspace";
import { initFlowbite } from "flowbite";
import type { _User } from "@/types/Auth";

const workspaceStore = useWorkspaceStore(),
  route = useRoute();

function profileImage(member: _User) {
  if (member) {
    if (member.picture) {
      return member.picture;
    }
    return profile;
  } else return profile;
}

watch(
  () => route.path,
  () => {
    initFlowbite();
  }
);

onMounted(async () => {
  initFlowbite();
  await workspaceStore.loadTeam();
});
</script>
