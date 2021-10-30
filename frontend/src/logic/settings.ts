import { Database } from "/src/database";

/**
 * Get a settings value out of the database.
 * @param key The settings key to get.
 * @returns The value belonging to the key.
 */
export const get = async (key: string): Promise<string> => {
    const db = await Database.get();
    return await db.settings.get(key);
};

/**
 * Set a setting value in the database.
 * @param key The key to store with.
 * @param value The corresponding value.
 * @returns The newly created value.
 */
export const set = async (key: string, value: string): Promise<string> => {
    const db = await Database.get();
    return await db.settings.set(key, value);
};
