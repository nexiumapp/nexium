import { h, FunctionalComponent, Fragment } from "preact";
import { route } from "preact-router";

import * as style from "./style.scss";

/**
 * Sub-navigation item.
 * @param props Props to render with.
 * @returns JSX of the navigation.
 */
export const SubNav: FunctionalComponent<Props> = (props) => {
    return (
        <Fragment>
            <select
                class={style.selectnav}
                value={window.location.pathname}
                onChange={(e: any) => route(e.target.value)}
            >
                {props.links.map(({ title, url }) => (
                    <option value={url}>{title}</option>
                ))}
            </select>
            <nav class={style.subnav}>
                {props.links.map(({ title, url }) => (
                    <a
                        class={
                            window.location.pathname === url ? style.link : ""
                        }
                        href={url}
                    >
                        {title}
                    </a>
                ))}
            </nav>
        </Fragment>
    );
};

interface Props {
    // List of link entries to render the sub navigation items with.
    links: Entry[];
}

interface Entry {
    // Title of the entry, displayed in the frontend.
    title: string;
    // Local item to route to.
    url: string;
}
