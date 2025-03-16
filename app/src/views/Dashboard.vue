<template>
    <div>
        Dashboard
    </div>
</template>

<script setup>
import { onMounted } from 'vue';

onMounted(async () => {
    fetch("/api/v1/user", { credentials: 'include' }).then(async response => {
        console.log(await response.json());
    });
});
</script>