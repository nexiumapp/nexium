import { CaseReducer } from "@reduxjs/toolkit";

import { Account } from "/src/models";

import { State } from ".";

/**
 * Set the currently logged in user.
 * @param state The current state of the slice.
 * @param action The action with the data about the user.
 */
const setUser: CaseReducer<State, SetUserAction> = (state, { payload }) => {
    state.loggedIn = true;
    state.user = payload;
};

/**
 * Action for the `setUser` reducer.
 */
interface SetUserAction {
    type: "account/setUser";
    payload: Account;
}

/**
 * Remove the user session.
 * @param state The current state of the slice.
 */
const clearUser: CaseReducer<State> = (state) => {
    state.loggedIn = false;
    state.user = undefined;
};

/**
 * Combined reducer object.
 */
export const reducers = {
    setUser,
    clearUser,
};
