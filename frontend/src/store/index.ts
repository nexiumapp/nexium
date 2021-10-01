import { configureStore } from "@reduxjs/toolkit";
import { TypedUseSelectorHook, useDispatch, useSelector } from "react-redux";

import sessionReducer from "./session";

// The global Redux store.
export const store = configureStore({
    reducer: {
        session: sessionReducer,
    },
    middleware: (getDefaultMiddleware) =>
        // disable the serializability check in order for dispatched thunks to be able to return `Result` from the neverthrow dependency.
        // This is fine accoding to the FAQ of Redux, as this is done by middleware and not an reducer.
        getDefaultMiddleware({
            serializableCheck: false,
        }),
});

// The root state type of Redux.
export type RootState = ReturnType<typeof store.getState>;
// Type of the dispatch function.
export type AppDispatch = typeof store.dispatch;

// Typed useDispatch hook.
export const useAppDispatch = () => useDispatch<AppDispatch>();
// Typed useSelector hook.
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;
