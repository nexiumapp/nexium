import { Err, err, ok, Result, ResultAsync } from "neverthrow";

/**
 * Send an get request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @returns The body of the response, or an ApiError.
 */
export async function get<T, E>(
    url: string,
): Promise<Result<T, ApiError<E> | FetchError>> {
    return await send(url, "get");
}

/**
 * Send an post request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @param body The body to send with the request.
 * @returns The body of the response, or an ApiError.
 */
export async function post<T, E>(
    url: string,
    body?: any,
): Promise<Result<T, ApiError<E> | FetchError>> {
    return await send(url, "post", body);
}

/**
 * Send an delete request.
 * @param dispatch Dispatch function for the Redux store.
 * @param url The url to send the request to.
 * @returns The body of the response, or an ApiError.
 */
export async function remove<T, E>(
    url: string,
): Promise<Result<T, ApiError<E> | FetchError>> {
    return await send(url, "delete");
}

/**
 * Send an network request to the backend.
 * This is an internal helper funtion, use the functions named after the methods instead.
 * @param url The url to send the request to.
 * @param method The HTTP method to attach.
 * @param body The body to attach. Will be stringified before the request is send.
 * @returns The body of the response, or an Error.
 */
async function send<T, E>(
    url: string,
    method: string,
    body?: any,
): Promise<Result<T, ApiError<E> | FetchError>> {
    // Send the request, catching any errors.
    const req = await ResultAsync.fromPromise(
        fetch(url, {
            method: method,
            cache: "no-cache",
            redirect: "error",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(body),
        }),
        fetchError,
    );

    // Return the error if the request failed.
    if (req.isErr()) {
        return req as Err<undefined, FetchError>;
    }

    // Server errors are by definition failing behaviour, treating it as an fetch error.
    if (req.value.status >= 500) {
        return err(fetchError());
    }

    // Decode the JSON, making sure to allow an empty body.
    const json = await req.value
        .json()
        .then((json) => json)
        .catch(() => null);

    // Return the error if the request was not sucessful.
    if (!req.value.ok) {
        return err({
            type: "ApiError",
            code: json.code as E,
            error: json.error,
        });
    }

    // Return the success response.
    return ok(json);
}

/**
 * Creates a new `FetchError` instance.
 * This is used as a shorthand.
 * @returns A new instance of `FetchError`.
 */
export const fetchError = (): FetchError => {
    return {
        type: "FetchError",
    };
};

/**
 * Definition of `FetchError`.
 * This can be returned by API calls if it fails.
 */
export interface FetchError {
    type: "FetchError";
}

/**
 * Possible error types from the api.
 * Generic T is an with the possible error codes.
 */
export interface ApiError<E> {
    type: "ApiError";
    code: E | "notauthenticated";
    error: string;
}
