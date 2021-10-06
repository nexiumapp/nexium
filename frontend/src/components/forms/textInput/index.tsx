import { h, FunctionalComponent } from "preact";

import { InputHook } from "/src/hooks";

import * as style from "./style.scss";

export const TextInput: FunctionalComponent<Props> = (props) => {
    const [input, error, , inputCallback, , blurCallback] = props.hook;

    const inputListener = (e: InputEvent, newValue: string): void => {
        e.preventDefault();
        if (props.disabled) return;

        inputCallback(newValue);
    };

    const errorSpan = error !== false ? <span>{error}</span> : "";

    return (
        <div class={style.container}>
            <div class={style.labels}>
                <label for={props.id}>{props.title}</label>
                {errorSpan}
            </div>
            <input
                id={props.id}
                value={input}
                placeholder={props.placeholder}
                type={props.type || "text"}
                autocomplete={props.autocomplete || "off"}
                disabled={props.disabled}
                onInput={(e: any) => inputListener(e, e.target.value)}
                onBlur={blurCallback}
            />
        </div>
    );
};

interface Props {
    // ID of the input.
    id: string;
    // `useInput` hook to provide from the component. This controls the component.
    hook: InputHook;
    // Title of the input to show as an label.
    title: string;
    // Placeholder to show inside the textbox.
    placeholder: string;
    // Autocomplete configuration.
    autocomplete?:
        | "off"
        | "on"
        | "name"
        | "username"
        | "new-password"
        | "current-password";
    // Type of input to show.
    type?: "password" | "search" | "email" | "text";
    // If this input is disabled.
    disabled?: boolean;
}
