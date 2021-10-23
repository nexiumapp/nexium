import { h, FunctionalComponent, RenderableProps } from "preact";

import * as style from "./style.scss";

export { SidebarDivider } from "./divider";
export { SidebarLink } from "./link";

/**
 * Sidebar component.
 * @param props Props to render with.
 * @returns JSX of the sidebar.
 */
export const Sidebar: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const styles = `${style.sidebar} ${props.open ? style.open : ""}`;

    return <nav class={styles}>{props.children}</nav>;
};

/**
 * Props for the component.
 */
interface Props {
    // Set to true to open the sidebar.
    open: boolean;
}
