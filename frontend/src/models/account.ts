/**
 * Model of an user account.
 */
export interface Account {
    id: string;
    fullName: string;
    username: string;
}

/**
 * Every type of authentication with the server.
 */
export type AuthType = PasswordAuth;

/**
 * Password authentication type.
 */
export interface PasswordAuth {
    type: "password";
    password: string;
}
