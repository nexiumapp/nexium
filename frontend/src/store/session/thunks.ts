import { createAsyncThunk } from "@reduxjs/toolkit";
import { Err, Ok, ok, Result } from "neverthrow";

import { ApiError } from "/src/api";
import { create, CreateError, CreateResponse } from "/src/api/account";
import { Account, PasswordAuth } from "/src/models";

import { setUser } from "./";

/**
 * Register using password authentication.
 */
const registerPassword = createAsyncThunk<
    Result<Account, ApiError<CreateError>>,
    RegisterPasswordAction
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

        // Dispatch an update for the store.
        dispatch(
            setUser({
                tokens: {
                    access: data.value.accessToken,
                    refresh: data.value.refreshToken,
                },
                user: data.value.account,
            }),
        );

        return ok(data.value.account);
    },
);

/**
 * Registration data required for the corresponding thunk.
 */
interface RegisterPasswordAction {
    fullName: string;
    username: string;
    password: string;
}

/**
 * Object containing all thunkgs of the session.
 */
export const thunks = {
    registerPassword,
};
