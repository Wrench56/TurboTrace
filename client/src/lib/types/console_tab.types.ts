/* TODO: Make better ConsoleData structure */

export enum LogLevel {
    DEBUG = "DEBUG",
    INFO = "INFO",
    WARN = "WARN",
    ERROR = "ERROR",
    CRIT = "CRIT",
    INTERNAL = "PRGME"
}

export enum TextAlign {
    LEFT = "left",
    RIGHT = "right",
    CENTER = "center"
}

export type ConsolePrint = {
    message?: string,
    text_align?: TextAlign,
    header?: boolean,
    timestamp?: number,
    level?: LogLevel,
    module?: string,
    ftimestamp?: string
}

export type LogEntry = {
    timestamp: number,
    level: LogLevel,
    module: string,
    message: string
}