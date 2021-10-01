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
