import { CaseReducer } from "@reduxjs/toolkit";

import { State } from "./";

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
    type: "session/setUser";
    payload: {
        id: string;
        fullName: string;
        username: string;
    };
}

/**
 * Set the authentication tokens.
 * @param state The current state of the slice.
 * @param action The action including the tokens.
 */
const setToken: CaseReducer<State, SetTokenAction> = (state, { payload }) => {
    state.tokens = payload;
};

/**
 * Action for the `setToken` reducer.
 */
interface SetTokenAction {
    type: "session/setToken";
    payload: {
        access: string;
        refresh: string;
    };
}

/**
 * Remove the user session.
 * @param state The current state of the slice.
 */
const logoutUser: CaseReducer<State> = (state) => {
    state.loggedIn = false;
    state.tokens = undefined;
    state.user = undefined;
};

/**
 * Remove the user session.
 * @param state The current state of the slice.
 */
const setIntervalID: CaseReducer<State, SetIntervalIDAction> = (
    state,
    { payload },
) => {
    state.refreshIntervalID = payload.id;
};

/**
 * Action for the `setIntervalID` reducer.
 */
interface SetIntervalIDAction {
    type: "session/setIntervalID";
    payload: {
        id?: number;
    };
}

/**
 * Combined reducer object.
 */
export const reducers = {
    setUser,
    setToken,
    logoutUser,
    setIntervalID,
};
