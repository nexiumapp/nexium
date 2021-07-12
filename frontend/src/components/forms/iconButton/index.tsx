import { h, FunctionalComponent } from "preact";
import { useState } from "preact/hooks";
import { IconDefinition } from "@fortawesome/fontawesome-common-types";

import { Icon } from "/src/components/media";

import * as style from "./style.scss";

/**
 * Render an icon button.
 * @param props Props to render with.
 * @returns A JSX element of the icon button.
 */
export const IconButton: FunctionalComponent<Props> = (props) => {
    const [loading, setLoading] = useState(false);

    const clickListener = async (): Promise<void> => {
        if (!props.onClick) return;
        if (props.disabled || loading) return;

        setLoading(true);
        await props.onClick();
        setLoading(false);
    };

    return (
        <button
            alt={props.alt}
            class={style.button}
            disabled={props.disabled || loading}
            onClick={clickListener}
        >
            <Icon icon={props.icon} />
        </button>
    );
};

interface Props {
    // Alternative title of the button.
    alt: string;
    // The icon to show.
    icon: IconDefinition;
    // If this button is disabled.
    disabled?: boolean;

    // Click handler.
    // Disables the button when a pending promise is returned.
    onClick?: () => void | Promise<void>;
}
