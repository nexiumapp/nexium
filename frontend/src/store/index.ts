import { combineReducers, configureStore } from "@reduxjs/toolkit";
import { TypedUseSelectorHook, useDispatch, useSelector } from "react-redux";
import { persistReducer, persistStore } from "redux-persist";

import storage from "redux-persist/lib/storage";

import sessionReducer from "./account";

// Configuration for persisting the store.
const persistConfig = {
    key: "nexium",
    storage: storage,
    whitelist: ["session"],
};

// All reducers combined.
const reducers = combineReducers({
    session: sessionReducer,
});

// The global Redux store.
export const store = configureStore({
    // The types are re-assigned as the `persistReducer` function is not properly typed.
    // Blocked by https://github.com/rt2zz/redux-persist/pull/1085.
    reducer: persistReducer(persistConfig, reducers) as typeof reducers,
    middleware: (getDefaultMiddleware) =>
        // disable the serializability check in order for dispatched thunks to be able to return `Result` from the neverthrow dependency.
        // This is fine accoding to the FAQ of Redux, as this is done by middleware and not an reducer.
        getDefaultMiddleware({
            serializableCheck: false,
        }),
});

// The persistor for the Redux store, used with `PersistGate`.
export const persistor = persistStore(store);

// The root state type of Redux.
export type RootState = ReturnType<typeof store.getState>;
// Type of the dispatch function.
export type AppDispatch = typeof store.dispatch;
// Type of the options for thunk functions.
export type ThunkOptions = { dispatch: AppDispatch; state: RootState };

// Typed useDispatch hook.
export const useAppDispatch = () => useDispatch<AppDispatch>();
// Typed useSelector hook.
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;
