import { writable } from "svelte/store";

const VolumeStore = writable<number>(0);

export default VolumeStore;
