import { h, FunctionalComponent } from "preact";
import { faMoon, faSun } from "@fortawesome/free-solid-svg-icons";
import { route } from "preact-router";

import Router from "preact-router";

import { IconButton } from "/src/components/forms";
import { AvailableThemes, useTheme } from "/src/components/theme";
import { useAppSelector } from "/src/store";

import { Login } from "./login";
import { Register } from "./register";

import * as style from "./style.scss";

/**
 * Authentication component.
 * This also routes between logging in and registering.
 * @returns JSX of the authentication screen.
 */
export const Auth: FunctionalComponent = () => {
    const [theme, setTheme] = useTheme();
    const isLoggedin = useAppSelector((state) => state.session.loggedIn);

    // Redirect to the app if the user is logged in.
    if (isLoggedin) {
        route("/app/inbox", true);
        return;
    }

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
                    <Login path="/auth/login" />
                    <Register default path="/auth/register" />
                </Router>
            </div>
        </div>
    );
};
