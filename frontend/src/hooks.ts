import { useState } from "preact/hooks";

/**
 * Preact hook for the local storage api.
 * Can be used to persist data.
 * This does not update automatically when the store is changed externally.
 * @param key The key to use for the store.
 * @param initialValue The initial value when no value has been set.
 * @returns A tuple with the stored value, and a function to update the store.
 */
export const useLocalStorage = <T = any>(
    key: string,
    initialValue: T,
): [T, (value: T) => void] => {
    const [storedValue, setStoredValue] = useState<T>(() => {
        const item = window.localStorage.getItem(key);
        return item ? JSON.parse(item) : initialValue;
    });

    const setValue = (value: T) => {
        setStoredValue(value);
        window.localStorage.setItem(key, JSON.stringify(value));
    };

    return [storedValue, setValue];
};

/**
 * React hook to create an controlled input.
 * @param required If the controlled input is required.
 * @param validate Validation function.
 * @returns An array to interact with the input.
 */
export const useInput = (
    required: boolean,
    validate: (input: string) => string | boolean,
): InputHook => {
    const [input, setInput] = useState<string>("");
    const [valid, setValid] = useState<boolean>(!(required || !validate("")));
    const [blurred, setBlurred] = useState<boolean>(false);
    const [error, setError] = useState<string>("");

    // Callback to call when the input is changed.
    const inputCallback = (input: string) => {
        const result = validate(input);
        setInput(input);

        if (typeof result !== "boolean") {
            setValid(false);
            setError(blurred ? result : "");
        } else {
            setValid(result);
            setError("");
        }
    };

    // Callback to call when the input is blurred.
    const blurCallback = () => {
        const result = validate(input);
        setBlurred(true);

        if (typeof result !== "boolean") {
            setValid(false);
            setError(result);
        } else {
            setValid(result);
            setError("");
        }
    };

    return [input, error, valid, inputCallback, setError, blurCallback];
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
export type InputHook = [
    string,
    string | false,
    boolean,
    (input: string) => void,
    (error: string) => void,
    () => void,
];
