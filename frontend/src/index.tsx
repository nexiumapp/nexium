import { h, render } from "preact";

import { ThemeProvider } from "./components/theme";
import { Auth } from "./pages/authenticate";

import "./global.scss";

/**
 * Start the application.
 */
const start = (): void => {
    render(
        <ThemeProvider>
            <Auth />
        </ThemeProvider>,
        document.body,
    );
};

/**
 * Include the debug code if it's a development build.
 */
if (process.env.NODE_ENV === "development") {
    import("preact/debug");

    if (module.hot) {
        module.hot.accept();
    }
}

/**
 * Let's start!
 */
start();
