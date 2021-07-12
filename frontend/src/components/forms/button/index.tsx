import { h, FunctionalComponent, RenderableProps } from "preact";
import { useState } from "preact/hooks";

import * as style from "./style.scss";

/**
 * Render a stylized button.
 * @param props Props to render with
 * @returns The JSX component for the button.
 */
export const Button: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const [loading, setLoading] = useState(false);

    const full = props.full ? style.full : "";
    const secondary = props.secondary ? style.secondary : "";
    const styles = `${style.button} ${full} ${secondary}`;

    const clickListener = async (): Promise<void> => {
        if (!props.onClick) return;
        if (props.disabled || loading) return;

        setLoading(true);
        await props.onClick();
        setLoading(false);
    };

    return (
        <button
            class={styles}
            alt={props.alt}
            disabled={props.disabled || loading}
            onClick={clickListener}
        >
            {props.children}
        </button>
    );
};

interface Props {
    // Alternative text for the button.
    alt: string;
    // If the button is disabled.
    disabled?: boolean;
    // If this button should fill the width of the container.
    full?: boolean;
    // If to style the button as a secondary button.
    secondary?: boolean;

    // Click handler
    // This disabled disables the button when an pending promise is returned.
    onClick?: () => void | Promise<void>;
}
