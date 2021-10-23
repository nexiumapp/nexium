import { h, FunctionalComponent, Fragment } from "preact";
import { route } from "preact-router";
import { useEffect, useState } from "preact/hooks";
import {
    faAddressBook,
    faArchive,
    faBookmark,
    faCogs,
    faEnvelope,
    faPaperclip,
    faPaperPlane,
    faPencilAlt,
    faPlusCircle,
    faSignOutAlt,
    faTrashAlt,
} from "@fortawesome/free-solid-svg-icons";

import AsyncRoute from "preact-async-route";
import Router from "preact-router";

import {
    Sidebar,
    SidebarDivider,
    SidebarLink,
    Topbar,
} from "/src/components/layout";
import { useAppDispatch, useAppSelector } from "/src/store";
import { getUser, logoutUser } from "/src/store/session";

import * as style from "./style.scss";

/**
 * Main application route.
 * Used to route the main pages when logged in.
 * @returns JSX of the authentication screen.
 */
export const App: FunctionalComponent = () => {
    const dispatch = useAppDispatch();
    const isLoggedin = useAppSelector((state) => state.session.loggedIn);
    const [sidebarOpen, setSidebarOpen] = useState(false);

    // Go to the authentication screen if the user is not logged in.
    if (!isLoggedin) {
        route("/auth/login", true);
        return;
    }

    // Update the current user.
    useEffect(() => {
        dispatch(getUser());
    }, []);

    return (
        <div class={style.container}>
            <Topbar toggleSidebar={() => setSidebarOpen(!sidebarOpen)} />
            <Sidebar open={sidebarOpen}>
                <SidebarLink
                    header
                    href="/app/inbox"
                    icon={faEnvelope}
                    count={42}
                >
                    Inbox
                </SidebarLink>
                <SidebarLink header href="/app/drafts" icon={faPencilAlt}>
                    Drafts
                </SidebarLink>
                <SidebarLink header href="/app/contacts" icon={faAddressBook}>
                    Contacts
                </SidebarLink>
                <SidebarLink href="/app/attachments" icon={faPaperclip}>
                    Attachments
                </SidebarLink>
                <SidebarLink href="/app/send" icon={faPaperPlane}>
                    Sent
                </SidebarLink>
                <SidebarLink href="/app/archive" icon={faArchive}>
                    Archived
                </SidebarLink>
                <SidebarLink href="/app/deleted" icon={faTrashAlt}>
                    Deleted
                </SidebarLink>
                <SidebarDivider />
                <SidebarLink
                    href="/app/filter/work"
                    icon={faBookmark}
                    color="#4fc3f7"
                >
                    Work
                </SidebarLink>
                <SidebarLink sub href="/app/filter/add" icon={faPlusCircle}>
                    Add new
                </SidebarLink>
                <SidebarDivider />
                <SidebarLink href="/app/settings" icon={faCogs}>
                    Settings
                </SidebarLink>
                <SidebarLink
                    onClick={() => dispatch(logoutUser())}
                    icon={faSignOutAlt}
                >
                    Sign out
                </SidebarLink>
            </Sidebar>
            <main>
                <Router onChange={() => setSidebarOpen(false)}>
                    <AsyncRoute
                        path="/app/inbox"
                        getComponent={() =>
                            import("./inbox").then((f) => f["Inbox"])
                        }
                    />
                    <AsyncRoute
                        path="/app/settings/:rest*"
                        getComponent={() =>
                            import("./settings").then((f) => f["Settings"])
                        }
                    />
                    <Fragment default>
                        <h1>404 :(</h1>
                        <p>
                            The page you are trying to access could not be
                            found, please try another way.
                        </p>
                    </Fragment>
                </Router>
            </main>
        </div>
    );
};
