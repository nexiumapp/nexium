import { h, FunctionalComponent, Fragment } from "preact";

import { useInput } from "/src/hooks";
import { Select } from "/src/components/forms/select";
import { AvailableThemes, useTheme } from "/src/components/theme";
import { useEffect } from "preact/hooks";

/**
 * This is the general settings page.
 * @param props Props to render with.
 * @returns JSX of the component.
 */
export const General: FunctionalComponent = () => {
    const [current, setTheme] = useTheme();
    const theme = useInput<AvailableThemes>(false, current, () => true);

    useEffect(() => setTheme(theme[0]), [theme[0]]);

    return (
        <Fragment>
            <h3>Theme</h3>

            <Select
                name="themeSelector"
                label="Set the current color scheme of the application."
                hook={theme}
            >
                <option value={AvailableThemes.Light}>Light</option>
                <option value={AvailableThemes.Dark}>Dark</option>
            </Select>
        </Fragment>
    );
};
