import { DBTable } from "./table";

/**
 * Settings database subclass.
 * This handles any interaction with the settings table.
 */
export class Settings extends DBTable {
    /**
     * Sync the current settings to the server.
     */
    public async sync(): Promise<void> {
        // Todo: add settings sync logic.
    }

    /**
     * Get a setting from the database.
     * @param key The key to get by.
     * @returns The stored value.
     */
    public async get(key: string): Promise<string> {
        return await this.database.get("settings", key);
    }

    /**
     * Set a setting in the database.
     * @param key The key to store with.
     * @param value The attached value.
     * @returns The newly created value.
     */
    public async set(key: string, value: string): Promise<string> {
        return await this.database.put("settings", value, key);
    }
}
