import { Result } from "neverthrow";

import { Account } from "/src/models/account";
import { AuthType } from "/src/models/session";
import { AppDispatch } from "/src/store";

import { ApiError, post } from "../";

/**
 * Create a new account.
 * @param dispatch The Redux dispatch function.
 * @param username the username to set.
 * @param auth authentication method to attach to the account.
 */
export const create = async (
    dispatch: AppDispatch,
    username: string,
    auth: AuthType,
): Promise<Result<CreateResponse, ApiError<CreateError>>> => {
    return await post<CreateResponse, CreateError>(
        dispatch,
        "/api/account/new",
        {
            username,
            auth,
        },
    );
};

/**
 * Interface of the success response expecting from the server.
 */
export interface CreateResponse {
    account: Account;
    token: string;
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
