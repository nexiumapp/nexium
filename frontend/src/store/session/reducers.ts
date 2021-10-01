import { CaseReducer } from "@reduxjs/toolkit";

import { State } from "./";

/**
 * Set the currently logged in user.
 * @param state The current state of the slice.
 * @param action The action to process.
 */
const setUser: CaseReducer<State, SetUserAction> = (state, { payload }) => {
    state.loggedIn = true;
    state.tokens = payload.tokens;
    state.user = payload.user;
};

/**
 * Action for the `setUser` reducer.
 */
interface SetUserAction {
    type: "session/setUser";
    payload: {
        tokens: {
            access: string;
            refresh: string;
        };
        user: {
            id: string;
            fullName: string;
            username: string;
        };
    };
}

/**
 * Remove the user session.
 * @param state The current state of the slice.
 */
const removeUser: CaseReducer<State> = (state) => {
    state.loggedIn = false;
    state.tokens = undefined;
    state.user = undefined;
};

/**
 * Combined reducer object.
 */
export const reducers = {
    setUser,
    removeUser,
};
