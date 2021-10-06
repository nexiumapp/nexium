import { createAsyncThunk } from "@reduxjs/toolkit";
import { Err, Ok, ok, Result } from "neverthrow";

import { ApiError } from "/src/api";
import { create, CreateError, CreateResponse } from "/src/api/account";
import { login, LoginError, LoginResponse } from "/src/api/session";
import { refresh } from "/src/api/session";
import { Account, PasswordAuth } from "/src/models";

import { ThunkOptions } from "../";
import { setUser, setToken, logoutUser, setIntervalID } from "./";

/**
 * Register using password authentication.
 */
const registerPassword = createAsyncThunk<
    Result<Account, ApiError<CreateError>>,
    RegisterPasswordAction,
    ThunkOptions
>(
    "session/registerPassword",
    async (
        action,
        { dispatch },
    ): Promise<Result<Account, ApiError<CreateError>>> => {
        const authType: PasswordAuth = {
            type: "password",
            password: action.password,
        };

        // Create the account by the API.
        const result = await create(action.username, authType);

        // Return the error if the creation failed.
        if (result.isErr()) {
            return result as Err<any, ApiError<CreateError>>;
        }

        // Assign the correct type.
        const data = result as Ok<CreateResponse, ApiError<CreateError>>;

        // Dispatch an authentication update to the store.
        dispatch(
            setToken({
                access: data.value.accessToken,
                refresh: data.value.refreshToken,
            }),
        );

        // Dispatch an user update to the store.
        dispatch(
            setUser({
                id: data.value.account.id,
                fullName: data.value.account.fullName,
                username: data.value.account.username,
            }),
        );

        // Enable session refreshing.
        dispatch(enableSessionRefresh());

        return ok(data.value.account);
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
    Result<Account, ApiError<LoginError>>,
    LoginPasswordAction,
    ThunkOptions
>(
    "session/loginPassword",
    async (
        action,
        { dispatch },
    ): Promise<Result<Account, ApiError<LoginError>>> => {
        const authType: PasswordAuth = {
            type: "password",
            password: action.password,
        };

        // Login the account by the API.
        const result = await login(action.username, authType);

        // Return the error if logging in failed.
        if (result.isErr()) {
            return result as Err<any, ApiError<LoginError>>;
        }

        // Assign the correct type.
        const data = result as Ok<LoginResponse, ApiError<LoginError>>;

        // Dispatch an authentication update to the store.
        dispatch(
            setToken({
                access: data.value.accessToken,
                refresh: data.value.refreshToken,
            }),
        );

        // Dispatch an user update to the store.
        dispatch(
            setUser({
                id: data.value.account.id,
                fullName: data.value.account.fullName,
                username: data.value.account.username,
            }),
        );

        // Enable session refreshing.
        dispatch(enableSessionRefresh());

        return ok(data.value.account);
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
 * Enable session refreshing.
 */
const enableSessionRefresh = createAsyncThunk<void, undefined, ThunkOptions>(
    "session/enableSessionRefresh",
    (_, { dispatch, getState }) => {
        const doRefresh = async () => {
            const state = getState();

            // Sanity check if the tokens are set.
            if (!state.session.tokens) {
                dispatch(disableSessionRefresh());
                dispatch(logoutUser());
                return;
            }

            // Send the refresh request.
            const res = await refresh(state.session.tokens.refresh);

            // Check if the request succeeded.
            if (res.isErr()) {
                // Ignore the error if the refresh token is not invalid but did return an error.
                if (!(res.error.code === "accessdenied")) {
                    return;
                }

                // If the access is denied, log the user out.
                dispatch(disableSessionRefresh());
                dispatch(logoutUser());
                return;
            }

            // Set the new access token.
            dispatch(
                setToken({
                    access: res.value.accessToken,
                    refresh: getState().session.tokens.refresh,
                }),
            );
        };

        // Create the interval.
        const id = window.setInterval(doRefresh, 12 * 60 * 1000);
        dispatch(setIntervalID({ id }));

        // Run the refresh immidiately.
        doRefresh();
    },
);

/**
 * Disable the refreshing of the access token.
 */
const disableSessionRefresh = createAsyncThunk<void, undefined, ThunkOptions>(
    "session/disableSessionRefresh",
    (_, { dispatch, getState }) => {
        const state = getState();

        // Ignore the call when the interval ID is not set.
        if (!state.session.refreshIntervalID) {
            return;
        }

        // Clear the interval.
        window.clearInterval(state.session.refreshIntervalID);
        dispatch(setIntervalID({ id: undefined }));
    },
);

/**
 * Object containing all thunkgs of the session.
 */
export const thunks = {
    registerPassword,
    loginPassword,
    enableSessionRefresh,
    disableSessionRefresh,
};
