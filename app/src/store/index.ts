import { defineStore } from "pinia"
import router from "../router";
import type { _User } from "../models/Auth";

export const useIndexStore = defineStore('index', {
    state: () => ({
        currentUser: undefined as _User | undefined,
        loggedIn: false
    }),
    getters: {
        getCurrentUser: (state) => state.currentUser,
        getLoggedIn: (state) => state.loggedIn,
    },
    actions: {
        async login(code: string) {
            const response = await fetch(`/api/v1/user/login/${code}`, { method: 'POST' });
            if (response.status == 200) {
                const userResponse = await fetch("/api/v1/user", { credentials: 'include' });

                if (userResponse.status == 200)
                    this.currentUser = await userResponse.json();

                this.loggedIn = (await response.json()) == "true";
                router.push("/dashboard");
            }
        },
        async logout() {
            localStorage.clear();
            await fetch(`/api/v1/user/logout`, { credentials: 'include' });
            router.go(0);
        }
    },
    persist: [
        {
            key: 'auth',
            storage: localStorage,
        }
    ]
})