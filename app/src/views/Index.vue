<template>
    <div class="grid place-items-center items-center justify-center h-screen">
        <div class="grid place-items-center -translate-y-16">
            <div class="flex font-extrabold text-9xl mb-8 font-serif tracking-widest">
                <span class="dark:text-theme-first dark:brightness-150" style="font-family: 'Faith Hope', sans-serif;">T</span>
                <span class="dark:text-theme-first dark:brightness-50" style="font-family: 'Faith Hope', sans-serif;">A</span>
                <span class="dark:text-theme-first dark:brightness-150" style="font-family: 'Faith Hope', sans-serif;">S</span>
                <span class="dark:text-theme-first dark:brightness-50" style="font-family: 'Faith Hope', sans-serif;">K</span>
                <span class="dark:text-theme-first dark:brightness-150" style="font-family: 'Faith Hope', sans-serif;">S</span>
            </div>
            <button
                class="text-white dark:bg-theme-first dark:hover:brightness-110 p-3 px-5 rounded-lg mb-2 cursor-pointer"
                @click="client.requestCode();">
                <img src="../assets/img/google.png" width="25" class="inline-flex" />
                <span class="ms-3 font-bold">Continue with Google</span>
            </button>
        </div>
    </div>
</template>

<script setup>
import { onMounted } from "vue";
import { useRoute } from "vue-router";
import $ from "jquery";
import router from "../router";
import { ref, watch } from "vue";
import { useIndexStore } from "../store";
import { initFlowbite } from "flowbite";

const indexStore = useIndexStore();

let client = ref(null),
    route = useRoute();

watch(() => route.path, () => {
    initFlowbite();
});

onMounted(async () => {
    initFlowbite();
    if (route.query.code) {
        await indexStore.login(btoa(route.query.code));
    } else {
        client.value = google.accounts.oauth2.initCodeClient({
            client_id: import.meta.env.VITE_GOOGLE_CLIENT_ID,
            scope: 'openid email https://www.googleapis.com/auth/userinfo.profile',
            ux_mode: 'redirect',
            redirect_uri: import.meta.env.VITE_GOOGLE_REDIRECT_URI,
            state: "1234"
        });
    }
})
</script>