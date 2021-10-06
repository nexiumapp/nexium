import { h, FunctionalComponent, Fragment } from "preact";
import { faLock, faKey } from "@fortawesome/free-solid-svg-icons";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";
import { Divider } from "/src/components/layout";
import { useInput } from "/src/hooks";

/**
 * Renders the login box with the different authentication methods.
 * @returns The login component.
 */
export const Login: FunctionalComponent = () => {
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
    };

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
            <Fragment>
                <TextInput
                    id="username"
                    title="Username"
                    autocomplete="username"
                    placeholder="John.Doe42"
                    hook={fields.username}
                />
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
                >
                    <Icon icon={faLock} pad />
                    Login
                </Button>
            </Fragment>
            <Divider text="or" />
            <Button full alt="Login with WebAuthn">
                <Icon icon={faKey} pad />
                Login with WebAuthn
            </Button>
        </main>
    );
};
