<template>
    <div class="grid place-items-center items-center justify-center h-screen">
        <div class="grid place-items-center -translate-y-16">
            <button
                class="text-white dark:bg-teal-800 dark:hover:bg-teal-700 bg-teal-700 hover:bg-green-800 focus:ring-teal-300 p-3 px-5 rounded-lg w-full mb-2 cursor-pointer"
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
import { ref } from "vue";
import { useIndexStore } from "../store";

const indexStore = useIndexStore();

let client = ref(null),
    route = useRoute();

onMounted(async () => {
    if (route.query.code) {
        await indexStore.login(btoa(route.query.code));
    } else {
        client.value = google.accounts.oauth2.initCodeClient({
            client_id: import.meta.env.VITE_GOOGLE_CLIENT_ID,
            scope: 'openid https://www.googleapis.com/auth/userinfo.profile',
            ux_mode: 'redirect',
            redirect_uri: import.meta.env.VITE_GOOGLE_REDIRECT_URI,
            state: "1234"
        });
    }
})
</script>