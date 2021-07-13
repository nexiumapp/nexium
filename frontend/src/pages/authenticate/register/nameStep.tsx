import { h, FunctionalComponent, Fragment } from "preact";
import { useMemo } from "preact/hooks";
import { faStepForward } from "@fortawesome/free-solid-svg-icons";

import { Button, TextInput } from "/src/components/forms";
import { Icon } from "/src/components/media";

import * as style from "./style.scss";

/**
 * Component for the names step of the registration procedure.
 * @param props Props to render the step with.
 * @returns The JSX of the component.
 */
export const NameStep: FunctionalComponent<Props> = (props) => {
    const isValid = useMemo(() => {
        if (props.name.length < 3) return false;
        if (props.username.length < 3) return false;
        if (!/^([a-z]|[A-Z]|[0-9]|[-_.]){3,}$/.test(props.username))
            return false;

        return true;
    }, [props.name, props.username]);

    return (
        <Fragment>
            <TextInput
                id="fullname"
                title="Full Name"
                autocomplete="name"
                placeholder="John Doe"
                value={props.name}
                onInput={(username) => props.setName(username)}
            />
            <TextInput
                id="username"
                title="Username"
                autocomplete="username"
                placeholder="John.Doe42"
                value={props.username}
                onInput={(username) => props.setUsername(username)}
            />
            <div class={style.bottombutton}>
                <Button
                    alt="Go to the next step"
                    full
                    disabled={!isValid}
                    onClick={() => props.changeStep(props.step + 1)}
                >
                    <Icon icon={faStepForward} pad />
                    Next Step
                </Button>
            </div>
        </Fragment>
    );
};

interface Props {
    // The current name set.
    name: string;
    // The current username set.
    username: string;
    // The current step.
    step: number;

    // Change the current step.
    changeStep: (newStep: number) => void;
    // Set the name
    setName: (newName: string) => void;
    // Set the username
    setUsername: (newUsername: string) => void;
}
