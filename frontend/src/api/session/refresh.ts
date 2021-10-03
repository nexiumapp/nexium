import { Result } from "neverthrow";

import { ApiError, post } from "../";

/**
 * Refresh an existing session.
 * @param refresh The refresh token to use.
 */
export const refresh = async (
    refresh: string,
): Promise<Result<RefreshResponse, ApiError<RefreshError>>> => {
    return await post<RefreshResponse, RefreshError>(
        "/api/session/refresh",
        refresh,
    );
};

/**
 * Interface of the success response expecting from the server.
 */
export interface RefreshResponse {
    accessToken: string;
}

/**
 * All types of errors from the refresh route.
 */
export type RefreshError = "accessdenied" | "internalerror" | "databaseerror";
