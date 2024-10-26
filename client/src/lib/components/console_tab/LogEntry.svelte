<script lang="ts">
    import { LogLevel, type ConsolePrint } from "$lib/types/console_tab.types";

    export let log: ConsolePrint;

    log.level ??= LogLevel.INTERNAL;
    log.timestamp ??= 0;
    let date = new Date(log.timestamp);

    /* Maybe use another variable instead of log.ftimestamp? */
    log.ftimestamp = `${date.getHours().toString().padStart(2, '0')}:${date.getMinutes().toString().padStart(2, '0')}:${date.getSeconds().toString().padStart(2, '0')}.${date.getMilliseconds().toString().padStart(4, '0')}`;
</script>

<div>
    <p class="timestamp">[{log.ftimestamp}]</p>
    <span class="{log.level}">{log.level}</span>
    <p class="module"> [{log.module}]</p>
    <p class="message">{log.message}</p>
</div>

<style>
    p {
        display: inline;
        color: #ABB2BF;
        font-weight: bold;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif, monospace;
        margin: 1px;
        width: 100%
    }

    span {
        display: inline-block;
        width:70px;
        max-width:70px;
        border-radius: 30px;
        text-align: center;
        font-weight: bold;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif, monospace;
        padding: 2px 8px 2px 8px;
        margin: 1px 8px 1px 8px;
        font-size: smaller;
        color: whitesmoke;
        text-align: center;
    }

    .timestamp {
        display: inline-block;
        width: 120px;
        max-width: 120px;
        color: #ABB2BF;
    }

    .DEBUG {
        background-color: rgb(58, 58, 254);
    }

    .INFO {
        background-color: #61AFEF;
    }

    .WARN {
        background-color: #EDA113;
    }

    .ERROR {
        background-color: #E06C75;
    }

    .CRIT {
        background-color: rgb(240, 52, 52);
    }

    /* Program/Internal error */
    .PRGME {
        background-color: rgb(92, 0, 0);
    }

    .module {
        display: inline-block;
        width: 300px;
        max-width: 300px;
        color: #ABB2BF;
    }

</style>