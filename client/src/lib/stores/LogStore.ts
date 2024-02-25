import { writable } from "svelte/store";

const LogStore = writable([
    {
        message: "Welcome to",
        text_align: "left"
    },
    {
        header: true
    },
    {
        timestamp: 0,
        level: "DEBUG",
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: "INFO",
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: "WARN",
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: "ERROR",
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: "CRIT",
        module: "src.lib.default",
        message: "Default"
    }
]);

export default LogStore;