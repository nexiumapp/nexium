import { h, FunctionalComponent, RenderableProps } from "preact";
import { IconDefinition } from "@fortawesome/fontawesome-common-types";

import { Icon } from "/src/components/media";

import * as style from "./style.scss";

/**
 * Sidebar link component.
 * @param props Props to render with.
 * @returns JSX of the link.
 */
export const SidebarLink: FunctionalComponent<RenderableProps<Props>> = (
    props,
) => {
    const styles = `${style.link} ${props.header ? style.header : ""} ${
        props.sub ? style.sub : ""
    } ${
        window.location.pathname.startsWith(props.href) ? style.activelink : ""
    }`;
    const badge = props.count ? (
        <span class={style.badge}>{props.count}</span>
    ) : (
        ""
    );

    /**
     * Click listener.
     * This fires the onClick event defined in the props, if the href property is not set.
     * @param e The event fired.
     */
    const click = (e: Event) => {
        if (props.href) return;

        e.preventDefault();
        props.onClick();
    };

    return (
        <a class={styles} href={props.href} onClick={(e) => click(e)}>
            <Icon icon={props.icon} color={props.color} />
            <span>{props.children}</span>
            {badge}
        </a>
    );
};

/**
 * Props for the component.
 */
interface Props {
    // Icon to add to the link.
    icon: IconDefinition;
    // Link to activate when the link is clicked.
    href?: string;
    // Additional count shown as an badge, optional.
    count?: number;
    // Indicates that this link is a header link, which stands out more.
    header?: boolean;
    // Indicates that this link is a sublink, which stands out less.
    sub?: boolean;
    // override the color of the icon.
    color?: string;

    // Click event when clicking on the button.
    // Only fired when the `.href` option is not set.
    onClick?(): void;
}
