<template>
  <div>
    <div>
      <router-view />
    </div>
  </div>
</template>

<script setup>
import { initFlowbite } from 'flowbite';
import { onMounted } from 'vue';

onMounted(() => {
  initFlowbite();
});
</script>
