import { IDBPDatabase } from "idb";

import { Datamodel } from "./";

/**
 * Settings database subclass.
 * This handles any interaction with the settings table.
 */
export class Settings {
    /**
     * The database connection to use.
     */
    private database: IDBPDatabase<Datamodel>;

    /**
     * Create a new instance of the database.
     * This should only be called by Database class.
     * @param db The IndexedDB database connection.
     */
    public constructor(db: IDBPDatabase<Datamodel>) {
        this.database = db;
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
