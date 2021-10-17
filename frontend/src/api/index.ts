import { err, ok, Result } from "neverthrow";

import { AppDispatch, store } from "/src/store";
import { logoutUser, setToken } from "/src/store/session";

/**
 * Global semaphore to set when a renew request is being send.
 */
let isRenewing = false;
/**
 * Queue of requests to send after the renewing has finished.
 */
const retryQueue: RetryEntry<unknown, unknown>[] = [];

/**
 * Send an get request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @returns The body of the response, or an ApiError.
 */
export async function get<T, E>(
    dispatch: AppDispatch,
    url: string,
): Promise<Result<T, ApiError<E>>> {
    let token = store.getState().session.token;
    return await send(dispatch, url, "get", token);
}

/**
 * Send an post request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @param body The body to send with the request.
 * @returns The body of the response, or an ApiError.
 */
export async function post<T, E>(
    dispatch: AppDispatch,
    url: string,
    body?: any,
): Promise<Result<T, ApiError<E>>> {
    let token = store.getState().session.token;
    return await send(dispatch, url, "post", token, body);
}

/**
 * Send an delete request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @returns The body of the response, or an ApiError.
 */
export async function remove<T, E>(
    dispatch: AppDispatch,
    url: string,
): Promise<Result<T, ApiError<E>>> {
    let token = store.getState().session.token;
    return await send(dispatch, url, "delete", token);
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
    url: string,
    method: string,
    auth?: string,
    body?: any,
): Promise<Result<T, ApiError<E>>> {
    /**
     * Do not send the request if the session token is being renewed.
     * Instead add it to the renew queue immidiately.
     */
    if (isRenewing) {
        return new Promise((resolve) => {
            retryQueue.push({
                resolve,
                url,
                method,
                body,
            } as RetryEntry<T, E>);
        });
    }

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
        // Renew the session and send the request again with an delay.
        if (req.status === 401) {
            await renewAuth(authHeader, dispatch, url, method, body);
            return;
        }

        // An error occured, return it.
        return err({
            type: "ApiError",
            code: json.code as E,
            error: json.error,
        });
    }

    return ok(json);
}

/**
 * Renew the authentication.
 * @param authHeader The session token to send with the request. Because this is an renew request, it does accept an expired token.
 * @param dispatch Redux dispatch function.
 * @param url The url to send the origional request to.
 * @param method The method to send the origional request with.
 * @param body The body data of the origional request.
 * @returns Promise which only resolves when the origional request succeeds.
 */
const renewAuth = async (
    authHeader: string,
    dispatch: AppDispatch,
    url: string,
    method: string,
    body?: any,
) => {
    if (!isRenewing) {
        // Set the renewing semaphore.
        isRenewing = true;

        // Send the renew request.
        const req = await fetch("/api/session/refresh", {
            method: "post",
            cache: "no-cache",
            redirect: "error",
            headers: {
                Authorization: authHeader,
            },
        });

        // Decode the JSON.
        const json = await req.json();

        // Handle a failure by logging out
        if (!req.ok) {
            dispatch(logoutUser());

            return err({
                type: "ApiError",
                code: json.code,
                error: json.error,
            });
        }

        // Update the authentication token.
        dispatch(setToken(json.token));

        // Clear the semaphore.
        isRenewing = false;

        // Empty the retry queue, send all requests.
        while (retryQueue.length > 0) {
            let { resolve, url, method, body } = retryQueue.shift();

            send(dispatch, url, method, json.token, body).then((res) =>
                resolve(res),
            );
        }

        // Send the current request at the end.
        return await send(dispatch, url, method, json.token, body);
    } else {
        // Retry in process, add it to the queue.
        return new Promise((resolve) => {
            retryQueue.push({
                resolve,
                url,
                method,
                body,
            } as RetryEntry<any, any>);
        });
    }
};

/**
 * Entry for the retry queue.
 */
interface RetryEntry<T, E> {
    resolve(res: Result<T, ApiError<E>>): void;
    url: string;
    method: string;
    body?: any;
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
