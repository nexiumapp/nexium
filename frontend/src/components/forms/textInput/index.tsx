import { h, FunctionalComponent } from "preact";
import { useState } from "preact/hooks";

import * as style from "./style.scss";

export const TextInput: FunctionalComponent<Props> = (props) => {
    const [input, setInput] = useState("");

    const inputListener = (e: InputEvent, newValue: string): void => {
        e.preventDefault();

        if (props.disabled) return;
        if (!props.onInput) return;

        setInput(newValue);
        props.onInput(newValue);
    };

    return (
        <div class={style.container}>
            <label for={props.id}>{props.title}</label>
            <input
                id={props.id}
                value={input}
                placeholder={props.placeholder}
                type={props.type || "text"}
                disabled={props.disabled}
                onInput={(e: any) => inputListener(e, e.target.value)}
            />
        </div>
    );
};

interface Props {
    // ID of the input.
    id: string;
    // Title of the input to show as an label.
    title: string;
    // Placeholder to show inside the textbox.
    placeholder: string;
    // Autocomplete configuration.
    autocomplete?:
        | "off"
        | "on"
        | "username"
        | "new-password"
        | "current-password";
    // Type of input to show.
    type?: "password" | "search" | "email" | "text";
    // If this input is disabled.
    disabled?: boolean;

    // Change event of the input.
    onInput?: (newValue: string) => void;
}
