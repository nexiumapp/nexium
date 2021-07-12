import { h, FunctionalComponent, RenderableProps } from "preact";
import { IconDefinition } from "@fortawesome/free-solid-svg-icons";

import * as style from "./style.scss";

/**
 * Font Awesome icon component.
 * @param props Props to render with.
 * @returns JSX of the icon.
 */
export const Icon: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const styles = `${style.icon} ${props.pad ? style.pad : ""}`;
    const definition = props.icon.icon;
    const path =
        typeof definition[4] === "string"
            ? definition[4]
            : definition[4].reduce((c: string, v: string) => `${c} ${v}`, "");

    return (
        <svg
            viewBox={`0 0 ${definition[0]} ${definition[1]}`}
            height={props.height || "1em"}
            style={`color: ${props.color}`}
            class={styles}
        >
            <path d={path} />
        </svg>
    );
};

interface Props {
    // The icon from FontAwesome to show.
    icon: IconDefinition;
    // Height of the icon.
    height?: string;
    // Color of the icon.
    // This is infered from the text color if not set.
    color?: string;
    // Whether or not to add padding to the icon.
    pad?: boolean;
}
