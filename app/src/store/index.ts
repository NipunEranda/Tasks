// store.ts
// store.ts
import type { InjectionKey } from "vue";
import { createStore, Store, ActionContext } from 'vuex';

import AuthModule, { type AuthState } from "./Auth";

// define your typings for the store state
export interface State {
    auth: AuthState;
    loggedIn: boolean;
}

// define injection key
export const key: InjectionKey<Store<State>> = Symbol();

export const store = createStore<State>({
    state: { loggedIn: false } as State,
    modules: {
      auth: AuthModule,
    },
    mutations: {
      setLoggedIn(state: State, data: boolean) {
        state.loggedIn = data;
      },
    },
    actions: {
      handleRequestErrors(_context: ActionContext<State, State>, _data:any) {
        // if (data.response.status == 401) {
        //   this.dispatch("logout");
        // }
      },
    //   logout() {
    //     this.commit("auth/resetState");
    //     this.commit("workspace/resetState");
    //     this.commit("repository/resetState");
    //     this.commit("file/resetState");
    //     this.commit("language/resetState");
    //     this.commit("setLoggedIn", false);
    //     this.state.loggedIn = false;
    //     location.href = "/";
    //   },
    },
  });