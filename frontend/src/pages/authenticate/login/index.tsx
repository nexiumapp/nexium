import { h, FunctionalComponent, Fragment } from "preact";
import { useMemo, useState } from "preact/hooks";
import { faLock, faKey } from "@fortawesome/free-solid-svg-icons";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";
import { Divider } from "/src/components/layout";

/**
 * Renders the login box with the different authentication methods.
 * @returns The login component.
 */
export const Login: FunctionalComponent = () => (
    <main>
        <h2>Let's Get You Signed In!</h2>
        <h4>
            You can also{" "}
            <a href="/auth/register" alt="Register">
                register here
            </a>
            .
        </h4>
        <PasswordLogin />
        <Divider text="or" />
        <WebauthnLogin />
    </main>
);

/**
 *
 * @returns JSX of the password login component.
 */
const PasswordLogin: FunctionalComponent = () => {
    const [username, setUsername] = useState("");
    const [password, setPassword] = useState("");

    const isValid = useMemo(() => {
        if (username.length < 3) return false;
        if (password.length < 3) return false;
        if (!/^([a-z]|[A-Z]|[0-9]|[-_.]){3,}$/.test(username)) return false;

        return true;
    }, [username, password]);

    return (
        <Fragment>
            <TextInput
                id="username"
                title="Username"
                autocomplete="username"
                placeholder="John.Doe42"
                onInput={(username) => setUsername(username)}
            />
            <TextInput
                id="password"
                title="Password"
                type="password"
                autocomplete="current-password"
                placeholder="hunter123"
                onInput={(password) => setPassword(password)}
            />
            <Button alt="Login with Password" disabled={!isValid} full>
                <Icon icon={faLock} pad />
                Login
            </Button>
        </Fragment>
    );
};

/**
 * Display the elements used for WebAuthn authentication.
 * @returns JSX of the webauthn login component.
 */
const WebauthnLogin: FunctionalComponent = () => (
    <Button alt="Login with WebAuthn" full>
        <Icon icon={faKey} pad />
        Login with WebAuthn
    </Button>
);
