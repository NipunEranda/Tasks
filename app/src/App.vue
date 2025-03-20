<template>
  <div>
    <RouterView />
  </div>
</template>

<script setup>
import { initFlowbite } from 'flowbite';
import { onMounted } from 'vue';

onMounted(() => {
  initFlowbite();
});
</script>
