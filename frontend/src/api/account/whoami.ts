import { Result } from "neverthrow";

import { Account } from "/src/models/account";
import { AppDispatch } from "/src/store";

import { ApiError, get } from "../";

/**
 * Get the account of the currently logged in user.
 * @param dispatch The Redux dispatch function.
 */
export const whoami = async (
    dispatch: AppDispatch,
): Promise<Result<WhoamiResponse, ApiError<WhoamiError>>> => {
    return await get<WhoamiResponse, WhoamiError>(
        dispatch,
        "/api/account/whoami",
    );
};

/**
 * Interface of the success response expecting from the server.
 */
export interface WhoamiResponse {
    account: Account;
}

/**
 * All types of errors from the create route.
 */
export type WhoamiError = "accessdenied" | "databaseerror";
