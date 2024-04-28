<script lang="ts">
    import LogStore from "$lib/stores/LogStore";

    import { LogLevel, type ConsolePrint } from "$lib/types/console_tab.types.ts";
    
    /* Do we need this? */
    let processed = false;

    /* Logs counted */
    let debug: number;
    let info: number;
    let warn: number;
    let error: number;
    let crit: number;
    let internal: number;

    const formatNum = (number: number) => {
        if (number > 1000000) {
            // @ts-ignore
            return (+(Math.round(number / 1000000 + "e+2")  + "e-2") + "m").padStart(6, " ");
        } else if (number > 1000) {
            // @ts-ignore
            return (+(Math.round(number / 1000 + "e+2")  + "e-2") + "k").padStart(6, " ");
        } else {
            return number.toString().padStart(6, " ");
        }

    }

    const updateData = (entry: ConsolePrint) => {
        if (entry.level !== undefined) {
            switch (entry.level) {
                case LogLevel.DEBUG:
                    debug++;
                    break;
                case LogLevel.INFO:
                    info++;
                    break;
                case LogLevel.WARN:
                    warn++;
                    break;
                case LogLevel.ERROR:
                    error++;
                    break;
                case LogLevel.CRIT:
                    crit++;
                    break;
                case LogLevel.INTERNAL:
                    internal++;
                    break;
                default:
                    /* TODO: Raise internal critical */
                    break;
            }
        }
    }

    /* Maybe calculate with Rust */
    LogStore.subscribe((data: Array<ConsolePrint>) => {
        if (processed) {
            /* Only process the last element */
            updateData(data[data.length-1]);
        } else {
            debug = 0;
            info = 0;
            warn = 0;
            error = 0;
            crit = 0;
            internal = 0;

            data.forEach(entry => { 
                updateData(entry);
                processed = true;
            });
        }
    });

</script>

<div class="container">
    <span class="debug">
        Debug | <pre>{formatNum(debug)}</pre>
    </span>
    <span class="info">
        Info | <pre>{formatNum(info)}</pre>
    </span>
    <span class="warn">
        Warn | <pre>{formatNum(warn)}</pre>
    </span>
    <span class="error">
        Error | <pre>{formatNum(error + crit)}</pre>
    </span>
</div>

<style>

    span {
        display: inline-block;
        max-width: 150px;
        overflow: hidden;
        width:initial;
        height: 24px;
        max-height: 24px;
        border-radius: 6px;
        text-align: left;
        padding: 4px 16px 4px 16px;
        margin: 1px 10px 1px 10px;
        color: whitesmoke;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        font-weight: bolder;
        vertical-align: middle;
    }

    pre {
        display: inline;
        color: whitesmoke;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        font-weight: bolder;
        font-size: 14px;
    }

    .container {
        display: flex;
        background-color: rgba(26, 30, 38, 1);
        z-index: 2;
        border-radius: 12px;
        padding: 2px 12px 2px 12px;
        margin-bottom: 14px;
        height: 60px;
        align-items: center;
        box-shadow: inset 4px 4px 8px 0 rgba(0, 0, 0, 0.9),
            inset -4px -4px 8px 0 rgba(40, 40, 40, 0.9);
    }

    .debug {
        background-color: rgb(58, 58, 254);
    }

    .info {
        background-color: #61AFEF;
    }

    .warn {
        background-color: #EDA113;
    }

    .error {
        background-color: rgb(240, 52, 52);
    }
</style>