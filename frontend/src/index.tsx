/**
 * Include the debug code if it's a development build.
 * This has to be the first import, therefore the non-standard ordering.
 */
if (process.env.NODE_ENV === "development") {
    import("preact/debug");
}

import { h, render } from "preact";
import { Provider } from "react-redux";
import { Router } from "preact-router";
import { PersistGate } from "redux-persist/integration/react";

import AsyncRoute from "preact-async-route";

import { ThemeProvider } from "/src/components/theme";
import { Database } from "/src/database";
import { store, persistor } from "/src/store";

import "./global.scss";

/**
 * Start the application.
 */
const start = (): void => {
    // Open the IndexedDB connection.
    Database.get();

    render(
        <Provider store={store}>
            <PersistGate loading={null} persistor={persistor}>
                <ThemeProvider>
                    <Router>
                        <AsyncRoute
                            path="/app/:rest*"
                            getComponent={() =>
                                import("/src/pages/app").then((f) => f["App"])
                            }
                        />
                        <AsyncRoute
                            default
                            path="/auth/:rest*"
                            getComponent={() =>
                                import("/src/pages/authenticate").then(
                                    (f) => f["Auth"],
                                )
                            }
                        />
                    </Router>
                </ThemeProvider>
            </PersistGate>
        </Provider>,
        document.body,
    );
};

/**
 * Let's start!
 */
start();
