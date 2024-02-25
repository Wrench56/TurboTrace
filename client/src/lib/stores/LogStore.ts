import { writable } from "svelte/store";
import { LogLevel, TextAlign } from "$lib/types/console_tab.types"; 


const LogStore = writable([
    {
        message: "Welcome to",
        text_align: TextAlign.LEFT
    },
    {
        header: true
    },
    {
        timestamp: 0,
        level: LogLevel.DEBUG,
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: LogLevel.INFO,
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: LogLevel.WARN,
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: LogLevel.ERROR,
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: LogLevel.CRIT,
        module: "src.lib.default",
        message: "Default"
    },
    {
        timestamp: 0,
        level: LogLevel.INTERNAL,
        module: "src.lib.default",
        message: "Default"
    }
]);

export default LogStore;