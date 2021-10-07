import { err, ok, Result } from "neverthrow";

import { AppDispatch } from "/src/store";
import { refreshAccessToken } from "/src/store/session";

/**
 * Send an get request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @param auth The authentication header to attach.
 * @returns The body of the response, or an ApiError.
 */
export async function get<T, E>(
    dispatch: AppDispatch,
    url: string,
    auth?: string,
): Promise<Result<T, ApiError<E>>> {
    return await send(dispatch, true, url, "get", auth);
}

/**
 * Send an post request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @param auth The authentication header to attach.
 * @param body The body to send with the request.
 * @returns The body of the response, or an ApiError.
 */
export async function post<T, E>(
    dispatch: AppDispatch,
    url: string,
    auth?: string,
    body?: any,
): Promise<Result<T, ApiError<E>>> {
    return await send(dispatch, true, url, "post", auth, body);
}

/**
 * Send an delete request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @param auth The authentication header to attach.
 * @returns The body of the response, or an ApiError.
 */
export async function remove<T, E>(
    dispatch: AppDispatch,
    url: string,
    auth?: string,
): Promise<Result<T, ApiError<E>>> {
    return await send(dispatch, true, url, "delete", auth);
}

/**
 * Send an network request to the backend.
 * This is an internal helper funtion, use the functions named after the methods instead.
 * @param dispatch Dispatch function for the Redux store.
 * @param firstTry indicating if this is the first try to send the request.
 * @param url The url to send the request to.
 * @param method The HTTP method to attach.
 * @param auth Bearer authentication to use. Won't be send if it is empty.
 * @param body The body to attach. Will be stringified before the request is send.
 * @returns The body of the response, or an ApiError.
 */
async function send<T, E>(
    dispatch: AppDispatch,
    firstTry: boolean,
    url: string,
    method: string,
    auth?: string,
    body?: any,
): Promise<Result<T, ApiError<E>>> {
    // Build the authorization header.
    let authHeader = auth && auth !== "" ? `Bearer ${auth}` : undefined;

    // Send the request.
    const req = await fetch(url, {
        method: method,
        cache: "no-cache",
        redirect: "error",
        headers: {
            Authorization: authHeader,
            "Content-Type": "application/json",
        },
        body: JSON.stringify(body),
    });

    // Decode the JSON.
    const json = await req.json();

    // Return the error if the request was not sucessful.
    if (!req.ok) {
        // Refresh authentication token and send the request again with an delay.
        if (req.status === 401 && firstTry) {
            dispatch(refreshAccessToken());

            return new Promise((resolve) =>
                setTimeout(
                    () =>
                        resolve(send(dispatch, false, url, method, auth, body)),
                    1000,
                ),
            );
        }

        return err({
            type: "ApiError",
            code: json.code as E,
            error: json.error,
        });
    }

    return ok(json);
}

/**
 * Possible error types from the api.
 * Generic T is an with the possible error codes.
 */
export interface ApiError<E> {
    type: "ApiError";
    code: E;
    error: string;
}
