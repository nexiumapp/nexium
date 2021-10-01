import { h, FunctionalComponent } from "preact";
import { route } from "preact-router";

import { useAppSelector } from "/src/store";

/**
 * Main application route.
 * Used to route the main pages when logged in.
 * @returns JSX of the authentication screen.
 */
export const App: FunctionalComponent = () => {
    const isLoggedin = useAppSelector((state) => state.session.loggedIn);

    // Go to the authentication screen if the user is not logged in.
    if (!isLoggedin) {
        route("/auth");
        return;
    }

    // Get the current user from the state.
    const user = useAppSelector((state) => state.session.user);

    return (
        <div>
            <h2>
                Hello there {user.fullName} ({user.username})!
            </h2>
            <span>Your session expires in 0 seconds.</span>
        </div>
    );
};
