import { h, FunctionalComponent } from "preact";

import * as style from "./style.scss";

/**
 * The searchbar included in the topbar.
 * @param props Props to render with.
 * @returns JSX of the search bar.
 */
export const SearchBar: FunctionalComponent = () => {
    return (
        <input
            type="search"
            placeholder="Search Something.."
            class={style.input}
        />
    );
};
