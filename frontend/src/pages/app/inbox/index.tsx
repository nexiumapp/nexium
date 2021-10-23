import { h, FunctionalComponent, Fragment } from "preact";

/**
 * Main inbox component.
 * This renders the main inbox page.
 * @param props Props to render with.
 * @returns JSX of the component.
 */
export const Inbox: FunctionalComponent = () => (
    <Fragment>
        <h1>Inbox</h1>
        <p>
            This is where your most important emails would be, if they would
            exists.
        </p>
    </Fragment>
);
