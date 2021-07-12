import { h, FunctionalComponent, createContext } from "preact";
import { useContext, useEffect } from "preact/hooks";

import { useLocalStorage } from "/src/hooks";

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
    const [theme, setTheme] = useLocalStorage<AvailableThemes>(
        "theme",
        AvailableThemes.Light,
    );
    const updateTheme = (newTheme: AvailableThemes) => setTheme(newTheme);

    useEffect(() => {
        document.body.dataset.theme = theme;
    }, [theme]);

    if (!theme) return undefined;

    return (
        <ThemeContext.Provider value={[theme, updateTheme]}>
            {children}
        </ThemeContext.Provider>
    );
};
