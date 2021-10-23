import { h, FunctionalComponent, RenderableProps, Fragment } from "preact";

import { InputHook } from "/src/hooks";

import * as style from "./style.scss";

/**
 * Select component .
 * @param props Props to render with.
 * @returns JSX of the component.
 */
export const Select: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const [input, , valid, inputCallback] = props.hook;
    const styles = `${style.select} ${!valid ? style.error : ""}`;

    return (
        <Fragment>
            <label for={props.name}>{props.label}</label>
            <select
                id={props.name}
                class={styles}
                value={input}
                onChange={(e: any) => inputCallback(e.target.value)}
            >
                {props.children}
            </select>
        </Fragment>
    );
};

interface Props {
    // An unique name for the property.
    name: string;
    // The label of the select string.
    label: string;
    // `useInput` hook to provide from the component. This controls the component.
    hook: InputHook<string>;
}
