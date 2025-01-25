import { writable } from "svelte/store";

export const title = writable("");
export const description = writable("");
export const backendPort = writable(8080);