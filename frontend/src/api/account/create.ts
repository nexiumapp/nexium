import { Result } from "neverthrow";

import { Account } from "/src/models/account";
import { AuthType } from "/src/models/session";

import { ApiError, post } from "../";

/**
 * Create a new account.
 * @param username the username to set.
 * @param auth authentication method to attach to the account.
 */
export const create = async (
    username: string,
    auth: AuthType,
): Promise<Result<CreateResponse, ApiError<CreateError>>> => {
    return await post<CreateResponse, CreateError>("/api/account/new", "", {
        username,
        auth,
    });
};

/**
 * Interface of the success response expecting from the server.
 */
export interface CreateResponse {
    account: Account;
    refreshToken: string;
    accessToken: string;
}

/**
 * All types of errors from the create route.
 */
export type CreateError =
    | "invalidusername"
    | "internalerror"
    | "passwordcomplexity"
    | "databaseerror"
    | "accountexists";
