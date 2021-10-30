import { DBSchema, openDB } from "idb";

import { Settings } from "./settings";

/**
 * This class handles persistent storage with the IndexedDB database.
 */
export class Database {
    /**
     * Singleton instance of the database.
     */
    private static instance: Database;
    /**
     * Settings database object.
     */
    public readonly settings: Settings;

    /**
     * Get the instance of the database class.
     */
    public static async get(): Promise<Database> {
        if (Database.instance) {
            // Return the database if it is already initialized.
            return Database.instance;
        }

        // Create it already if it wasn't.
        Database.instance = new Database();
        return await Database.instance.open();
    }

    /**
     * Prevent new constructions of this class, requiring you to use the `.get()` function.
     */
    private constructor() {
        // Create all table classes.
        this.settings = new Settings();
    }

    /**
     * Open a connection with IndexedDB, and create the subclass instances.
     * @returns A promise which resolves with this database.
     */
    private async open(): Promise<Database> {
        const database = await openDB<Datamodel>("nexium", 1, {
            upgrade: (db, oldVersion) => {
                if (oldVersion < 1) {
                    // Upgrade to version 1.
                    db.createObjectStore("settings");
                }
            },
        });

        // Insert the databse instance into the tables.
        this.settings.setDB(database);

        // Sync all tables.
        await Promise.all([this.settings.sync()]);

        return Database.instance;
    }
}

/**
 * Current datamodel of the database.
 */
export interface Datamodel extends DBSchema {
    settings: {
        key: string;
        value: string;
    };
}
