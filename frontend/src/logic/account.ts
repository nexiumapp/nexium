import { err, ok, Result } from "neverthrow";

import {
    FetchError,
    post,
    get as getReq,
    fetchError,
    ApiError,
} from "/src/api";
import { Account, AuthType } from "/src/models";

/**
 * Get the currently logged in user.
 * @returns A result which is the account if successful, otherwise a FetchError.
 */
export const get = async (): Promise<Result<Account, FetchError>> => {
    const res = await getReq<GetResponse, undefined>("/api/account/whoami");

    if (res.isErr()) {
        return err(fetchError());
    }

    return ok(res.value.account);
};

/**
 * Expected response from the server.
 * This is only used internally.
 */
interface GetResponse {
    account: Account;
}

/**
 * Logs out the current user.
 * This is only possible when online.
 * @returns A result which is empty when successful, otherwise it returns a FetchError.
 */
export const logout = async (): Promise<Result<null, FetchError>> => {
    const res = await post<undefined, undefined>("/api/account/logout");

    if (res.isErr()) {
        return err(fetchError());
    }

    return ok(null);
};

/**
 * Login the session with an account.
 * @param username The username to login with.
 * @param auth The authentication method used.
 * @returns A result which is either the logged in account, or the error.
 */
export const login = async (
    username: string,
    auth: AuthType,
): Promise<Result<Account, ApiError<LoginError> | FetchError>> => {
    const res = await post<LoginResponse, LoginError>("/api/account/login", {
        username,
        auth,
    });

    if (res.isErr()) {
        return err(res.error);
    }

    return ok(res.value.account);
};

/**
 * Expected error from logging in, when successful.
 * Only used internally.
 */
interface LoginResponse {
    account: Account;
}

/**
 * Possible errors while logging in.
 */
export type LoginError =
    | "loggedin"
    | "loginfailed"
    | "internalerror"
    | "databaseerror";

/**
 * Create a new account, and log it in into the current session.
 * @param username The username to create the account with.
 * @param auth The authentication method to associate with the account.
 * @returns A result which is either the newly created account, or an error.
 */
export const create = async (
    username: string,
    auth: AuthType,
): Promise<Result<Account, ApiError<CreateError> | FetchError>> => {
    const res = await post<CreateResponse, CreateError>("/api/account/new", {
        username,
        auth,
    });

    if (res.isErr()) {
        return err(res.error);
    }

    return ok(res.value.account);
};

/**
 * Expected response from the server after logging in.
 * Only used internally.
 */
interface CreateResponse {
    account: Account;
}

/**
 * Possible errors when creating the account.
 */
export type CreateError =
    | "loggedin"
    | "invalidusername"
    | "internalerror"
    | "passwordcomplexity"
    | "accountexists"
    | "databaseerror";
