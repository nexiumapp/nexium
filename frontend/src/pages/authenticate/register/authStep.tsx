import { h, FunctionalComponent, Fragment } from "preact";
import { useMemo, useState } from "preact/hooks";
import {
    faAsterisk,
    faKey,
    faStepBackward,
} from "@fortawesome/free-solid-svg-icons";
import { Result } from "neverthrow";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";
import { Divider } from "/src/components/layout";
import { useAppDispatch } from "/src/store";
import { registerPassword } from "/src/store/session";
import { ApiError } from "/src/api";
import { CreateError } from "/src/api/account";
import { Account } from "/src/models";

import * as style from "./style.scss";

/**
 * Authentication provide step of the registration procedure.
 * @param props Props to render the step with.
 * @returns The JSX for the step.
 */
export const AuthStep: FunctionalComponent<Props> = (props) => {
    const [error, setError] = useState("");
    const [password, setPassword] = useState("");
    const [repeatedPassword, setRepeatedPassword] = useState("");
    const dispatch = useAppDispatch();

    const isPasswordValid = useMemo(() => {
        if (password.length < 3) return false;
        if (password !== repeatedPassword) return false;

        return true;
    }, [password, repeatedPassword]);

    const register = async () => {
        // Dispatch the register with password thunk.
        const dispatched = await dispatch(
            registerPassword({
                fullName: "John Doe",
                username: "johndoe",
                password,
            }),
        );

        // Decode the result, this contains the result of the registration.
        const res: Result<
            Account,
            ApiError<CreateError>
        > = dispatched.payload as any;

        // Display the error if the registration failed.
        if (res.isErr()) {
            setError(res.error.error);
        }
    };

    return (
        <Fragment>
            {error && <span>{error}</span>}
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
                onClick={() => register()}
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
