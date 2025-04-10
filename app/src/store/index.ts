import { defineStore } from "pinia"
import router from "../router";
import type { _User } from "../types/Auth";
import type { CustomModal } from "@/types/Modal";

export const useIndexStore = defineStore('index', {
    state: () => ({
        currentUser: undefined as _User | undefined,
        loggedIn: false,
        tagModal: undefined as CustomModal | undefined,
        actionModal: undefined as CustomModal | undefined,
    }),
    getters: {
        getCurrentUser: (state) => state.currentUser,
        getLoggedIn: (state) => state.loggedIn,
    },
    actions: {
        async login(code: string) {
            const response = await fetch(`/api/v1/user/login/${code}`, { method: 'POST' });
            if (response.status == 200 || response.status == 201) {
                const userResponse = await fetch("/api/v1/user", { credentials: 'include' });

                if (userResponse.status == 200){
                    this.currentUser = await userResponse.json();
                    this.loggedIn = true;
                }

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