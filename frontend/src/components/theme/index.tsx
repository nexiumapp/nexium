import { h, FunctionalComponent, createContext } from "preact";
import { useContext, useEffect, useState } from "preact/hooks";

import { settings } from "/src/logic";
import { useAsyncEffect } from "/src/hooks";

/**
 * A list of all available themes.
 */
export enum AvailableThemes {
    Light = "light",
    Dark = "dark",
}

/**
 * Theme context used to provide the theme.
 */
const ThemeContext = createContext<
    [AvailableThemes, (newTheme: AvailableThemes) => void]
>([AvailableThemes.Light, () => {}]);

/**
 * Hook to get and / or set the current theme.
 * @returns A tuple with the theme and a function to set the new theme.
 */
export const useTheme = () => useContext(ThemeContext);

/**
 * Provider for the theme.
 * Adds CSS variables to be used in other components.
 * @param props Props containing the children to be rendered when the theme is loaded.
 * @returns The component to be added to the root of the tree.
 */
export const ThemeProvider: FunctionalComponent = ({ children }) => {
    const [theme, setTheme] = useState<AvailableThemes>(AvailableThemes.Light);
    const updateTheme = async (newTheme: AvailableThemes) => {
        setTheme(newTheme);
        await settings.set("theme", newTheme);
    };

    // Fetch the set theme from the database.
    useAsyncEffect(async () => {
        const theme = (await settings.get("theme")) as any;

        // Validate the theme in the store.
        if (!Object.values(AvailableThemes).includes(theme)) return;

        setTheme(theme as AvailableThemes);
    }, []);

    // Update the body data when the theme state changes.
    useEffect(() => {
        document.body.dataset.theme = theme;
    }, [theme]);

    return (
        <ThemeContext.Provider value={[theme, updateTheme]}>
            {children}
        </ThemeContext.Provider>
    );
};
