import { getCurrent, getNext, getPrevious } from "@/rest/api";
import { createSlice } from "@reduxjs/toolkit";

const counterSlice = createSlice({
  name: "counter",
  initialState: { value: null },
  reducers: {
    setCounter: (state, action) => {
      state.value = action.payload;
    },
  },
});

export const { setCounter } = counterSlice.actions;

export const fetchPreviousCounter = () => async (dispatch: any) => {
  try {
    const response = await getPrevious();
    dispatch(setCounter(response.data));
  } catch (error) {
    console.error("Error fetching previous counter:", error);
  }
};

export const fetchNextCounter = () => async (dispatch: any) => {
  try {
    const response = await getNext();
    dispatch(setCounter(response.data));
  } catch (error) {
    console.error("Error fetching next counter:", error);
  }
};

export const fetchInitialState = () => async (dispatch: any) => {
  try {
    const response = await getCurrent();
    dispatch(setCounter(response.data));
  } catch (error) {
    console.error("Error fetching initial counter:", error);
  }
};

export default counterSlice.reducer;
