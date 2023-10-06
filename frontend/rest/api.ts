import axios from "axios";

export const api = axios.create({
  baseURL: "http://127.0.0.1:8080",
  headers: {
    "Content-Type": "application/json",
  },
});

export const getCurrent = async () => await api.get(`/current`);
export const getNext = async () => await api.get(`/next`);
export const getPrevious = async () => await api.get(`/previous`);
