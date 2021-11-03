import { h, FunctionalComponent } from "preact";
import { useState } from "preact/hooks";
import { faLock, faKey } from "@fortawesome/free-solid-svg-icons";
import { Result } from "neverthrow";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";
import { Divider } from "/src/components/layout";
import { ApiError } from "/src/api";
import { LoginError } from "/src/api/account";
import { useAppDispatch } from "/src/store";
import { loginPassword } from "/src/store/account";
import { Account } from "/src/models";
import { useInput } from "/src/hooks";

import * as style from "./style.scss";

/**
 * Renders the login box with the different authentication methods.
 * @returns The login component.
 */
export const Login: FunctionalComponent = () => {
    const dispatch = useAppDispatch();
    const [error, setError] = useState("");
    const fields = {
        username: useInput(true, "", (value) => {
            if (value.length < 3) {
                return "Username is too short.";
            }

            if (value.length > 50) {
                return "Username is too long.";
            }

            if (!/^[A-Za-z0-9]+(\.[A-Za-z0-9]+)*$/.test(value)) {
                return "Only letters and numbers are allowed.";
            }

            return true;
        }),
        password: useInput(true, "", (value) => {
            if (value.length < 3) {
                return "Password is too short.";
            }

            if (value.length > 200) {
                return "Password is too long.";
            }

            return true;
        }),
    };

    // Login with an password.
    const passwordLogin = async () => {
        // Dispatch the login with password thunk.
        const dispatched = await dispatch(
            loginPassword({
                username: fields.username[0],
                password: fields.password[0],
            }),
        );

        // Decode the result, this contains the result of logging in.
        const res: Result<
            Account,
            ApiError<LoginError>
        > = dispatched.payload as any;

        // Display the error if logging in failed.
        if (res.isErr()) {
            const error = res.error.code;

            if (error === "loginfailed") {
                fields.password[4]("Username and/or password is incorrect!");
            } else if (error === "loggedin") {
                setError("You are already logged in.");
            } else {
                setError("An server error occured.");
            }
        }
    };

    // Global error component.
    const errorComponent =
        error !== "" ? <p class={style.error}>{error}</p> : "";

    return (
        <main>
            <h2>Let's Get You Signed In!</h2>
            <h4>
                New here? {""}
                <a href="/auth/register" alt="Register">
                    Register Instead
                </a>
                .
            </h4>
            {errorComponent}
            <TextInput
                id="username"
                title="Username"
                autocomplete="username"
                placeholder="John.Doe42"
                hook={fields.username}
            />
            <span class={style.usernamesplit} />
            <TextInput
                id="password"
                title="Password"
                type="password"
                autocomplete="current-password"
                placeholder="hunter123"
                hook={fields.password}
            />
            <Button
                full
                alt="Login with Password"
                disabled={!(fields.password[2] && fields.username[2])}
                onClick={() => passwordLogin()}
            >
                <Icon icon={faLock} pad />
                Login
            </Button>
            <Divider text="or" />
            <Button full alt="Login with WebAuthn">
                <Icon icon={faKey} pad />
                Login with WebAuthn
            </Button>
        </main>
    );
};
