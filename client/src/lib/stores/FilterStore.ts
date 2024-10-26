import type { ConsoleEntry } from "$lib/types/console_tab.types";
import { writable } from "svelte/store";

export type ConsoleFilterFunction = (
  o: ConsoleEntry,
  index?: number,
  array?: ConsoleEntry[]
) => boolean;

const levelFilterStore = writable<ConsoleFilterFunction>(() => true);

export { levelFilterStore };
