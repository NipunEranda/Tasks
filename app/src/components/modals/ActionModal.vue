<template>
  <div
    :id="props.modal?.id"
    tabindex="-1"
    class="hidden overflow-y-auto overflow-x-hidden fixed top-0 right-0 left-0 z-50 justify-center items-center w-full md:inset-0 h-[calc(100%-1rem)] max-h-full"
  >
    <div class="relative p-4 w-full max-w-2xl max-h-full">
      <!-- Modal content -->
      <div class="relative rounded-lg shadow-sm dark:bg-theme-primary dark:text-theme-primary-text dark:border-theme-primary-border dark:brightness-125">
        <!-- Modal header -->
        <div
          class="flex items-center justify-between p-4 md:p-5 border-b rounded-t dark:border-theme-primary-border"
        >
          <div
            class="text-xl font-semibold dark:text-theme-primary-text"
          >
            <fai :icon="modalIcon" class="mr-3 text-2xl" />{{
              props.modal?.title
            }}
          </div>
          <!-- @vue-ignore -->
          <button
            type="button"
            class="hover:dark:bg-theme-primary hover:dark:brightness-150 dark:text-theme-primary-text-secondary hover:dark:text-theme-primary-text bg-transparent rounded-lg text-sm w-8 h-8 ms-auto inline-flex justify-center items-center cursor-pointer"
            @click="props.modal?.modalEl ? props.modal?.modalEl.hide() : null"
          >
          <fai icon="fa-xmark"/>
            <span class="sr-only">Close modal</span>
          </button>
        </div>
        <!-- Modal body -->
        <div
          class="p-4 md:p-5 space-y-4 dark:bg-theme-primary text-center"
          v-html="props.modal?.message"
        ></div>
        <!-- Footer -->
        <div
          class="dark:bg-theme-primary dark:border-theme-primary-border border-t-1 px-4 py-3 sm:flex sm:flex-row-reverse sm:px-6 rounded-b-lg"
        >
          <button
            @click="props.modal?.process()"
            type="button"
            class="inline-flex w-full justify-center rounded-md px-5 py-2 text-sm font-semibold shadow-sm sm:w-auto dark:hover:brightness-110 mb-2 sm:mb-0 sm:ms-2 cursor-pointer"
            :class="{ 'dark:bg-theme-danger': props.modal?.type == 'remove' }"
          >
            <fai icon="fa-trash-can" class="mr-2 self-center"/>{{ props.modal?.processName }}
          </button>
          <!-- @vue-ignore -->
          <button
            @click="props.modal?.modalEl.hide()"
            type="button"
            class="inline-flex w-full justify-center rounded-md px-3 py-2 text-sm font-semibold shadow-sm sm:w-auto mb-2 sm:mb-0 sm:ms-2 dark:border-theme-primary-border/80 dark:text-theme-primary-text dark:bg-theme-primary dark:brightness-110 hover:dark:brightness-150 border cursor-pointer"
          >
            Cancel
          </button>
        </div>
        <!-- Footer -->
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import type { _Modal } from '@/types/Modal';
import type { PropType } from 'vue';

const props = defineProps({
    modal: {
        type: Object as PropType<_Modal>
    },
    modalIcon: String,
});
</script>
