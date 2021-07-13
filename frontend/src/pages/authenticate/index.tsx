import { h, FunctionalComponent } from "preact";
import { faMoon, faSun } from "@fortawesome/free-solid-svg-icons";

import Router from "preact-router";
import AsyncRoute from "preact-async-route";

import { IconButton } from "/src/components/forms";
import { AvailableThemes, useTheme } from "/src/components/theme";

import * as style from "./style.scss";

/**
 * Authentication component.
 * This also routes between logging in and registering.
 * @returns JSX of the authentication screen.
 */
export const Auth: FunctionalComponent = () => {
    const [theme, setTheme] = useTheme();

    const switchTheme = (): void => {
        const newTheme =
            theme === AvailableThemes.Light
                ? AvailableThemes.Dark
                : AvailableThemes.Light;
        setTheme(newTheme);
    };

    const iconTheme = theme === AvailableThemes.Light ? faMoon : faSun;

    return (
        <div class={style.container}>
            <div class={style.box}>
                <span class={style.themeswitch}>
                    <IconButton
                        alt="Switch Theme"
                        icon={iconTheme}
                        onClick={() => switchTheme()}
                    />
                </span>
                <Router>
                    <AsyncRoute
                        path="/auth/login"
                        getComponent={() =>
                            import("./login").then((f) => f["Login"])
                        }
                    />
                    <AsyncRoute
                        default
                        path="/auth/register"
                        getComponent={() =>
                            import("./register").then((f) => f["Register"])
                        }
                    />
                </Router>
            </div>
        </div>
    );
};
