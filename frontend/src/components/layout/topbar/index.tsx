import { h, FunctionalComponent, RenderableProps } from "preact";
import { faBars } from "@fortawesome/free-solid-svg-icons";

import { IconButton, SearchBar } from "/src/components/forms";
import { useAppSelector } from "/src/store";

import * as style from "./style.scss";

/**
 * Topbar component.
 * @param props Props to render with.
 * @returns JSX of the sidebar.
 */
export const Topbar: FunctionalComponent<RenderableProps<Props>> = (props) => {
    const name = useAppSelector((state) => state.session.user.fullName);

    return (
        <header class={style.topbar}>
            <span class={style.greeting}>
                Hi, <b>{name}</b>!
            </span>
            <SearchBar />
            <div class={style.sidebartoggle}>
                <IconButton
                    alt="Toggle the sidebar"
                    icon={faBars}
                    onClick={() => props.toggleSidebar()}
                />
            </div>
        </header>
    );
};

/**
 * Props for the component.
 */
interface Props {
    // This prop should toggle the sidebar which is not rendered by this component.
    toggleSidebar(): void;
}
