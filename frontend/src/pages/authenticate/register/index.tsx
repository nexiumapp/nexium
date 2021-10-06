import { h, FunctionalComponent } from "preact";
import { useState } from "preact/hooks";
import { faAsterisk, faKey } from "@fortawesome/free-solid-svg-icons";
import { Result } from "neverthrow";

import { Button, TextInput } from "/src/components/forms";
import { Divider } from "/src/components/layout";
import { Icon } from "/src/components/media";
import { ApiError } from "/src/api";
import { CreateError } from "/src/api/account";
import { Account } from "/src/models";
import { useAppDispatch } from "/src/store";
import { registerPassword } from "/src/store/session";
import { useInput } from "/src/hooks";

import * as style from "./style.scss";

/**
 * Register screen.
 * Includes multiple steps, which are not routed.
 * @returns The JSX of the registration screen.
 */
export const Register: FunctionalComponent = () => {
    const dispatch = useAppDispatch();
    const [error, setError] = useState("");
    const fields = {
        username: useInput(true, (value) => {
            if (value.length < 3) {
                return "Username is too short.";
            }

            if (value.length > 50) {
                return "Username is too long.";
            }

            if (!/^([A-Z]|[a-z]|\d)+$/.test(value)) {
                return "Only letters and numbers are allowed.";
            }

            return true;
        }),
        password: useInput(true, (value) => {
            if (value.length < 3) {
                return "Password is too short.";
            }

            if (value.length > 200) {
                return "Password is too long.";
            }

            return true;
        }),
        passwordConfirm: useInput(true, (value) => {
            if (fields.password[0] !== value) {
                return "Passwords do not match.";
            }

            return true;
        }),
    };

    // Register with an password.
    const passwordRegister = async () => {
        // Dispatch the register with password thunk.
        const dispatched = await dispatch(
            registerPassword({
                username: fields.username[0],
                password: fields.password[0],
            }),
        );

        // Decode the result, this contains the result of the registration.
        const res: Result<
            Account,
            ApiError<CreateError>
        > = dispatched.payload as any;

        // Display the error if the registration failed.
        if (res.isErr()) {
            const error = res.error.code;

            if (error === "accountexists") {
                fields.username[4]("This account already exists.");
            } else if (error === "invalidusername") {
                fields.username[4]("The username is invalid.");
            } else if (error === "passwordcomplexity") {
                fields.password[4]("This password is not complex enough.");
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
            <h2>Hello! Register to get Started.</h2>
            <h4>
                <a href="/auth/login" alt="Login">
                    Login
                </a>{" "}
                if you already have an account.
            </h4>
            {errorComponent}
            <TextInput
                id="username"
                title="Username"
                autocomplete="username"
                placeholder="John.Doe42"
                hook={fields.username}
            />
            <span class={style.usernameSplit} />
            <form onSubmit={(e) => e.preventDefault()}>
                <TextInput
                    id="password"
                    title="Password"
                    type="password"
                    autocomplete="new-password"
                    placeholder="hunter123"
                    hook={fields.password}
                />
                <TextInput
                    id="repeatpassword"
                    title="Repeat Password"
                    type="password"
                    autocomplete="new-password"
                    placeholder="hunter123"
                    hook={fields.passwordConfirm}
                />
                <div class={style.bottombutton}>
                    <Button
                        full
                        type="submit"
                        alt="Register with an Password"
                        disabled={
                            !(
                                fields.username[2] &&
                                fields.password[2] &&
                                fields.passwordConfirm[2]
                            )
                        }
                        onClick={() => passwordRegister()}
                    >
                        <Icon icon={faAsterisk} pad />
                        Register with Password
                    </Button>
                    <Divider text="or" />
                    <Button
                        full
                        type="submit"
                        alt="Register with WebAuthn"
                        disabled={!fields.username[2]}
                    >
                        <Icon icon={faKey} pad />
                        Register with WebAuthn
                    </Button>
                </div>
            </form>
        </main>
    );
};
