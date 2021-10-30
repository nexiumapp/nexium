import { h, FunctionalComponent, Fragment } from "preact";

import Router from "preact-router";

import { SubNav } from "/src/components/layout";

import { General } from "./general";

/**
 * Settings page.
 * In this page there should be an collection of all kind of settings to adjust.
 * @param props Props to render with.
 * @returns JSX of the component.
 */
export const Settings: FunctionalComponent = () => (
    <Fragment>
        <h1>Settings</h1>

        <SubNav
            links={[
                { title: "General", url: "/app/settings" },
                { title: "Domains", url: "/app/settings/domains" },
            ]}
        />

        <Router>
            <Fragment default path="/app/settings">
                <General />
            </Fragment>
            <Fragment path="/app/settings/domains">Domains</Fragment>
        </Router>
    </Fragment>
);
