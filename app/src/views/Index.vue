<template>
    <div class="grid place-items-center items-center justify-center h-screen">
        <div class="grid place-items-center -translate-y-16">
            <button class="bg-zinc-800 hover:bg-zinc-700 bg-opacity-90 p-3 px-5 rounded-lg text-white w-full mb-2 cursor-pointer"
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

let client = ref(null),
    route = useRoute();

onMounted(async () => {
    if (route.query.code) {
        await fetch(`/api/v1/user/login/${btoa(route.query.code)}`, { method: 'POST' });
        router.push('/dashboard');
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