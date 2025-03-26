<template>
  <div class="pr-4">
    <div v-for="member in teamStore.team" :key="member">
      <div
        class="flex bg-zinc-800 mb-2 rounded-lg justify-center place-items-center"
      >
        <div class="w-[10px] bg-teal-500 py-5 rounded-l-lg">a</div>
        <div class="flex-grow">
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
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import profile from "../assets/img/profile.png";
import { computed, onMounted, watch, ref, type Ref } from "vue";
import { useIndexStore } from "../store";
import { useTeamStore } from "../store/team";
import { useRoute } from "vue-router";
import { useWorkspaceStore } from "../store/workspace";

const indexStore = useIndexStore(),
  teamStore = useTeamStore(),
  workspaceStore = useWorkspaceStore(),
  user = computed(() => {
    return indexStore.currentUser;
  }),
  profileImage: Ref<string> = computed(() => {
    if (user.value) {
      if (user.value.picture) {
        return user.value.picture;
      }
      return profile;
    } else return profile;
  }),
  route = useRoute();

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
