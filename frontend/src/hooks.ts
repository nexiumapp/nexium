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
