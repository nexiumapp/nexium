import { createSlice } from "@reduxjs/toolkit";

import { reducers } from "./reducers";
import { thunks } from "./thunks";

/**
 * State definition of the account slice.
 */
export interface State {
    loggedIn: boolean;
    user?: {
        id: string;
        fullName: string;
        username: string;
    };
}

/**
 * The account slice stores everything related to the current user account.
 */
export const sessionSlice = createSlice({
    name: "account",
    initialState: {
        loggedIn: false,
    } as State,
    reducers,
});

// Export all actions from the slice.
export const { setUser, clearUser } = sessionSlice.actions;
// Re-export the thunks.
export const { registerPassword, loginPassword, getUser, logout } = thunks;
// Export the reducer.
export default sessionSlice.reducer;
