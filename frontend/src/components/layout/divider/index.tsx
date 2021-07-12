import { h, FunctionalComponent, RenderableProps } from "preact";

import * as style from "./style.scss";

/**
 * Visual divider to show.
 * @param props Props to render with.
 * @returns JSX of the divider.
 */
export const Divider: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const classlist = `${style.divider} ${props.text && style.withtext}`;

    return <hr class={classlist}>{props.text}</hr>;
};

interface Props {
    // Optional text to show.
    text?: string;
}
