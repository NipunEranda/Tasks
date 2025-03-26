<template>
  <div class="pr-5">
    <div v-for="(member, m) in teamStore.team" :key="m">
      <div
        class="group flex bg-zinc-800 mb-1 rounded-lg justify-center place-items-center cursor-pointer transition-all duration-300 hover:brightness-125"
      >
        <div class="w-[10px] bg-teal-500 rounded-l-lg py-10"></div>
        <div class="flex-grow flex items-center">
          <div class="w-20 place-items-center">
            <img
              class="h-14 w-14 lg:group-hover:w-14 rounded-full border-2 border-zinc-400 group-hover:border-teal-500"
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
              {{ member.name }}
            </span><br/>
            <span class="text-sm font-extralight text-zinc-100">
              {{ member.email }}
            </span>
          </div>
          <div class="w-fit px-5 font-bold text-lg text-teal-500">
            <fai icon="fa-circle-check" class="mr-2"/><span>Online</span>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import profile from "@/assets/img/profile.png";
import { computed, onMounted, watch } from "vue";
import { useIndexStore } from "../store";
import { useTeamStore } from "../store/team";
import { useRoute } from "vue-router";
import { useWorkspaceStore } from "../store/workspace";
import { initFlowbite } from "flowbite";
import type { _User } from "@/types/Auth";

const indexStore = useIndexStore(),
  teamStore = useTeamStore(),
  workspaceStore = useWorkspaceStore(),
  user = computed(() => {
    return indexStore.currentUser;
  }),
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
  await teamStore.load(workspaceStore.activeWorkspace);
});
</script>
