import { createAsyncThunk } from "@reduxjs/toolkit";
import { err, ok, Result } from "neverthrow";

import { ApiError, FetchError } from "/src/api";
import { account } from "/src/logic";
import { Account } from "/src/models";

import { ThunkOptions } from "../";
import { clearUser, setUser } from "./index";

/**
 * Register using password authentication.
 */
const registerPassword = createAsyncThunk<
    Result<Account, ApiError<account.CreateError> | FetchError>,
    RegisterPasswordAction,
    ThunkOptions
>(
    "account/registerPassword",
    async (
        action,
        { dispatch },
    ): Promise<Result<Account, ApiError<account.CreateError> | FetchError>> => {
        // Create the account.
        const result = await account.create(action.username, {
            type: "password",
            password: action.password,
        });

        // Return the error if the creation failed.
        if (result.isErr()) {
            return err(result.error);
        }

        // Dispatch an user update to the store.
        dispatch(setUser(result.value));

        // Return the created account.
        return ok(result.value);
    },
);

/**
 * Registration data required for the corresponding thunk.
 */
interface RegisterPasswordAction {
    username: string;
    password: string;
}

/**
 * Login using password authentication.
 */
const loginPassword = createAsyncThunk<
    Result<Account, ApiError<account.LoginError> | FetchError>,
    LoginPasswordAction,
    ThunkOptions
>(
    "account/loginPassword",
    async (
        action,
        { dispatch },
    ): Promise<Result<Account, ApiError<account.LoginError> | FetchError>> => {
        // Run the login logic.
        const result = await account.login(action.username, {
            type: "password",
            password: action.password,
        });

        // Return the error if logging in failed.
        if (result.isErr()) {
            return err(result.error);
        }

        // Dispatch an user update to the store.
        dispatch(setUser(result.value));

        // Return the account to the caller.
        return ok(result.value);
    },
);

/**
 * Login data required for the corresponding thunk.
 */
interface LoginPasswordAction {
    username: string;
    password: string;
}

/**
 * Get the currently logged in user.
 */
const getUser = createAsyncThunk<
    Result<Account, FetchError>,
    undefined,
    ThunkOptions
>(
    "account/getUser",
    async (_, { dispatch }): Promise<Result<Account, FetchError>> => {
        // Get the user from the API.
        const result = await account.get();

        // Dispatch an user update if the request succeeded.
        if (result.isOk()) {
            dispatch(setUser(result.value));
        }

        // return the result.
        return result;
    },
);

/**
 * Logout the current user.
 */
const logout = createAsyncThunk<
    Result<undefined, FetchError>,
    undefined,
    ThunkOptions
>(
    "account/logout",
    async (_, { dispatch }): Promise<Result<null, FetchError>> => {
        // Send a logout request.
        const result = await account.logout();

        // Dispatch an user update if the request succeeds.
        if (result.isOk()) {
            dispatch(clearUser());
        }

        // Return the result.
        return result;
    },
);

/**
 * Object containing all thunkgs of the session.
 */
export const thunks = {
    registerPassword,
    loginPassword,
    getUser,
    logout,
};
