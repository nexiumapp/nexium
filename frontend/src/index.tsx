import { h, render } from "preact";

/**
 * Start the application.
 */
const start = (): void => {
    render(<div>Hello Nexium!</div>, document.body);
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
