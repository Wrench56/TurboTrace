import { writable } from "svelte/store";
import { TextAlign } from "$lib/types/console_tab.types";


const LogStore = writable([
    {
        message: "Welcome to",
        text_align: TextAlign.LEFT
    },
    {
        header: true
    },
]);

export default LogStore;