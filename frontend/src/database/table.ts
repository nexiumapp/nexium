import { IDBPDatabase } from "idb";

import { Datamodel } from "./";

/**
 * Table inside of the database.
 * This should be extended by another class, and not used directly.
 */
export abstract class DBTable {
    /**
     * IndexedDB database instance.
     */
    protected database: IDBPDatabase<Datamodel>;

    /**
     * Set the database to use.
     * @param database The instance of the database to use.
     */
    public setDB(database: IDBPDatabase<Datamodel>): void {
        this.database = database;
    }

    /**
     * Sync the current table to the server.
     */
    public abstract sync(): Promise<void>;
}
