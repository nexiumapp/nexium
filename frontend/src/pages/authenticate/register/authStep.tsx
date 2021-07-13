import { h, FunctionalComponent, Fragment } from "preact";
import { useMemo, useState } from "preact/hooks";
import {
    faAsterisk,
    faKey,
    faStepBackward,
} from "@fortawesome/free-solid-svg-icons";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";
import { Divider } from "/src/components/layout";

import * as style from "./style.scss";

/**
 * Authentication provide step of the registration procedure.
 * @param props Props to render the step with.
 * @returns The JSX for the step.
 */
export const AuthStep: FunctionalComponent<Props> = (props) => {
    const [password, setPassword] = useState("");
    const [repeatedPassword, setRepeatedPassword] = useState("");

    const isPasswordValid = useMemo(() => {
        if (password.length < 3) return false;
        if (password !== repeatedPassword) return false;

        return true;
    }, [password, repeatedPassword]);

    return (
        <Fragment>
            <TextInput
                id="password"
                title="Password"
                type="password"
                autocomplete="new-password"
                placeholder="hunter123"
                onInput={(password) => setPassword(password)}
            />
            <TextInput
                id="repeatpassword"
                title="Repeat Password"
                type="password"
                autocomplete="new-password"
                placeholder="hunter123"
                onInput={(password) => setRepeatedPassword(password)}
            />
            <Button
                alt="Register with an Password"
                full
                disabled={!isPasswordValid}
            >
                <Icon icon={faAsterisk} pad />
                Register with Password
            </Button>
            <Divider text="or" />
            <Button alt="Register with WebAuthn" full>
                <Icon icon={faKey} pad />
                Register with WebAuthn
            </Button>
            <div class={style.bottombutton}>
                <Button
                    alt="Go back"
                    full
                    secondary
                    onClick={() => props.changeStep(props.step - 1)}
                >
                    <Icon icon={faStepBackward} pad />
                    Go Back
                </Button>
            </div>
        </Fragment>
    );
};

interface Props {
    // The current step.
    step: number;

    // Change the current step.
    changeStep: (newStep: number) => void;
}
