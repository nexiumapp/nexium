import { Result } from "neverthrow";

import { Account } from "/src/models/account";
import { AuthType } from "/src/models/session";

import { ApiError, post } from "../";

/**
 * Log in an existing account.
 * @param username the username of the login to login to.
 * @param auth authentication method to attach to the account.
 */
export const login = async (
    username: string,
    auth: AuthType,
): Promise<Result<LoginResponse, ApiError<LoginError>>> => {
    return await post<LoginResponse, LoginError>("/api/session/login", "", {
        username,
        auth,
    });
};

/**
 * Interface of the success response expecting from the server.
 */
export interface LoginResponse {
    account: Account;
    refreshToken: string;
    accessToken: string;
}

/**
 * All types of errors from the route.
 */
export type LoginError =
    | "unknownuser"
    | "passworderror"
    | "internalerror"
    | "databaseerror";
