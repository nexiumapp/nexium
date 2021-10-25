import { h, FunctionalComponent } from "preact";

import * as style from "./style.scss";

/**
 * Divider in the sidebar.
 * Used to visually separate it into different categories.
 * @param props Props to render with.
 * @returns JSX of the divider.
 */
export const SidebarDivider: FunctionalComponent = () => (
    <hr class={style.divider} />
);
