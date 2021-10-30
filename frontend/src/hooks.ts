import { useEffect, useState } from "preact/hooks";

/**
 * Equivalent to the `useEffect` hook from native Preact, but allowes the use of async functions with help of an IIFE.
 * @param func The function to call with the effect.
 * @param dependencies Any dependency variables.
 */
export const useAsyncEffect = (
    func: () => Promise<void>,
    dependencies?: any[],
): void => {
    useEffect(() => {
        (async () => await func())();
    }, dependencies);
};

/**
 * React hook to create an controlled input.
 * @param required If the controlled input is required.
 * @param validate Validation function.
 * @returns An array to interact with the input.
 */
export const useInput = <T = string>(
    required: boolean,
    initial: T,
    validate: (input: T) => string | boolean,
): InputHook<T> => {
    const [input, setInput] = useState<T>(initial);
    const [valid, setValid] = useState<boolean>(
        !(required || !validate(initial)),
    );
    const [error, setError] = useState<string>("");

    // Callback to call when the input is changed.
    const inputCallback = (input: T) => {
        const result = validate(input);
        setInput(input);

        if (typeof result !== "boolean") {
            setValid(false);
            setError(result);
        } else {
            setValid(result);
            setError("");
        }
    };

    return [input, error, valid, inputCallback, setError];
};

/**
 * Response for the `useInput` hook.
 *
 * 0: The current input value.
 * 1: Current error message derived from the the validate parameter, empty if there is no current error.
 * 2: Boolean indicating if the input is valid.
 * 3: Function to call when received new input.
 * 4: Function to call when setting an custom error.
 * 5: Function to call when the input is blurred.
 */
export type InputHook<T> = [
    T,
    string | false,
    boolean,
    (input: T) => void,
    (error: string) => void,
];
