import { createSlice } from "@reduxjs/toolkit";

import { reducers } from "./reducers";
import { thunks } from "./thunks";

/**
 * State definition when the user is logged in.
 */
export interface State {
    loggedIn: boolean;
    refreshIntervalID?: number;
    tokens?: {
        access: string;
        refresh: string;
    };
    user?: {
        id: string;
        fullName: string;
        username: string;
    };
}

/**
 * The session slice stores everything related to the current session.
 */
export const sessionSlice = createSlice({
    name: "session",
    initialState: {
        loggedIn: false,
    } as State,
    reducers,
});

// Export all actions from the slice.
export const { setUser, setToken, logoutUser, setIntervalID } =
    sessionSlice.actions;
// Re-export the thunks.
export const {
    registerPassword,
    loginPassword,
    refreshAccessToken,
    enableSessionRefresh,
    disableSessionRefresh,
} = thunks;
// Export the reducer.
export default sessionSlice.reducer;
