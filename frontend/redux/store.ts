import thunk from "redux-thunk";
import { configureStore } from "@reduxjs/toolkit";
import apiReducer from "./fib/reducer";

const store = configureStore({
  reducer: {
    fib: apiReducer,
  },
  middleware: (getDefaultMiddleware) => getDefaultMiddleware().concat(thunk),
});

export default store;
