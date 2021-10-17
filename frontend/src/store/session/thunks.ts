import { createAsyncThunk } from "@reduxjs/toolkit";
import { Err, Ok, ok, Result } from "neverthrow";

import { ApiError } from "/src/api";
import { login, LoginError, LoginResponse } from "/src/api/session/login";
import { create, CreateError, CreateResponse } from "/src/api/account/create";
import { whoami, WhoamiError, WhoamiResponse } from "/src/api/account/whoami";
import { Account, PasswordAuth } from "/src/models";

import { ThunkOptions } from "../";
import { setUser, setToken } from "./";

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
        const result = await create(dispatch, action.username, authType);

        // Return the error if the creation failed.
        if (result.isErr()) {
            return result as Err<any, ApiError<CreateError>>;
        }

        // Assign the correct type.
        const data = result as Ok<CreateResponse, ApiError<CreateError>>;

        // Dispatch an authentication update to the store.
        dispatch(setToken(data.value.token));

        // Dispatch an user update to the store.
        dispatch(
            setUser({
                id: data.value.account.id,
                fullName: data.value.account.fullName,
                username: data.value.account.username,
            }),
        );

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
        const result = await login(dispatch, action.username, authType);

        // Return the error if logging in failed.
        if (result.isErr()) {
            return result as Err<any, ApiError<LoginError>>;
        }

        // Assign the correct type.
        const data = result as Ok<LoginResponse, ApiError<LoginError>>;

        // Dispatch an authentication update to the store.
        dispatch(setToken(data.value.token));

        // Dispatch an user update to the store.
        dispatch(
            setUser({
                id: data.value.account.id,
                fullName: data.value.account.fullName,
                username: data.value.account.username,
            }),
        );

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
 * Get the currently logged in user.
 */
const getUser = createAsyncThunk<
    Result<Account, ApiError<WhoamiError>>,
    undefined,
    ThunkOptions
>(
    "session/getUser",
    async (
        _,
        { dispatch },
    ): Promise<Result<Account, ApiError<WhoamiError>>> => {
        // Get the user from the API.
        const result = await whoami(dispatch);

        // Return the error if an error occured.
        if (result.isErr()) {
            return result as Err<any, ApiError<WhoamiError>>;
        }

        // Assign the correct type.
        const data = result as Ok<WhoamiResponse, ApiError<WhoamiError>>;

        // Dispatch an user update.
        dispatch(setUser(data.value.account));

        return ok(data.value.account);
    },
);

/**
 * Object containing all thunkgs of the session.
 */
export const thunks = {
    registerPassword,
    loginPassword,
    getUser,
};
