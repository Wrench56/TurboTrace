import { writable } from "svelte/store";
import { TextAlign, type ConsolePrint } from "$lib/types/console_tab.types";

const LogStore = writable<ConsolePrint[]>([
  {
    message: "Welcome to",
    text_align: TextAlign.LEFT,
  },
  {
    header: true,
  },
]);

export default LogStore;
