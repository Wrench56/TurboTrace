<script lang="ts">
  import { levelFilterStore } from "$lib/stores/FilterStore";
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

  let enabledFilters: Set<LogLevel> = new Set();
  let filterEnableFlags: { [key in LogLevel]: boolean } = {
    [LogLevel.DEBUG]: false,
    [LogLevel.INFO]: false,
    [LogLevel.WARN]: false,
    [LogLevel.ERROR]: false,
    [LogLevel.CRIT]: false,
    [LogLevel.INTERNAL]: false,
  };

  const formatNum = (number: number) => {
    if (number > 1000000) {
      // @ts-ignore
      return (+(Math.round(number / 1000000 + "e+2") + "e-2") + "m").padStart(
        6,
        " "
      );
    } else if (number > 1000) {
      // @ts-ignore
      return (+(Math.round(number / 1000 + "e+2") + "e-2") + "k").padStart(
        6,
        " "
      );
    } else {
      return number.toString().padStart(6, " ");
    }
  };

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
  };

  LogStore.subscribe((data: Array<ConsolePrint>) => {
    if (processed) {
      /* Only process the last element */
      updateData(data[data.length - 1]);
    } else {
      debug = 0;
      info = 0;
      warn = 0;
      error = 0;
      crit = 0;
      internal = 0;

      data.forEach((entry) => {
        updateData(entry);
        processed = true;
      });
    }
  });

  function setLevelFilter(level: LogLevel) {
    let flag = filterEnableFlags[level];
    if (flag !== undefined) {
      if (level === LogLevel.ERROR) {
        flag
          ? enabledFilters.add(LogLevel.CRIT)
          : enabledFilters.delete(LogLevel.CRIT);
      }
      flag ? enabledFilters.add(level) : enabledFilters.delete(level);
    }

    levelFilterStore.set((o) => {
      if (enabledFilters.size === 0) return true;
      return o.level !== undefined && enabledFilters.has(o.level);
    });
  }
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div class="container">
  <span
    class="debug{filterEnableFlags[LogLevel.DEBUG] ? '-filter' : ''}"
    on:click={() => {
      filterEnableFlags[LogLevel.DEBUG] = !filterEnableFlags[LogLevel.DEBUG];
      setLevelFilter(LogLevel.DEBUG);
    }}
  >
    <pre>Debug | {formatNum(debug)}</pre>
  </span>
  <span
    class="info{filterEnableFlags[LogLevel.INFO] ? '-filter' : ''}"
    on:click={() => {
      filterEnableFlags[LogLevel.INFO] = !filterEnableFlags[LogLevel.INFO];
      setLevelFilter(LogLevel.INFO);
    }}
  >
    <pre>Info  | {formatNum(info)}</pre>
  </span>
  <span
    class="warn{filterEnableFlags[LogLevel.WARN] ? '-filter' : ''}"
    on:click={() => {
      filterEnableFlags[LogLevel.WARN] = !filterEnableFlags[LogLevel.WARN];
      setLevelFilter(LogLevel.WARN);
    }}
  >
    <pre>Warn  | {formatNum(warn)}</pre>
  </span>
  <span
    class="error{filterEnableFlags[LogLevel.ERROR] ? '-filter' : ''}"
    on:click={() => {
      filterEnableFlags[LogLevel.ERROR] = !filterEnableFlags[LogLevel.ERROR];
      setLevelFilter(LogLevel.ERROR);
    }}
  >
    <pre>Error | {formatNum(error + crit)}</pre>
  </span>
</div>

<style>
  span {
    display: inline-block;
    max-width: 150px;
    overflow: hidden;
    width: initial;
    height: 24px;
    max-height: 24px;
    border-radius: 6px;
    text-align: left;
    padding: 4px 16px 4px 16px;
    margin: 1px 10px 1px 10px;
    color: whitesmoke;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif, monospace;
    font-weight: bolder;
    vertical-align: middle;
  }

  pre {
    display: inline;
    color: whitesmoke;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif, monospace;
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
    box-shadow:
      inset 4px 4px 8px 0 rgba(0, 0, 0, 0.9),
      inset -4px -4px 8px 0 rgba(40, 40, 40, 0.9);
  }

  .debug {
    background-color: rgb(62, 64, 78);
  }

  .debug:hover {
    background-color: rgb(58, 58, 254);
  }

  .debug-filter {
    background: rgb(58, 58, 254);
    box-shadow:
      inset 2px 2px 5px rgba(0, 0, 0, 0.5),
      inset -2px -2px 5px rgba(255, 255, 255, 0.2);
  }

  .info {
    background-color: rgb(62, 64, 78);
  }

  .info:hover {
    background-color: rgb(97, 175, 239);
  }

  .info-filter {
    background-color: rgb(97, 175, 239);
    box-shadow:
      inset 2px 2px 5px rgba(0, 0, 0, 0.5),
      inset -2px -2px 5px rgba(255, 255, 255, 0.2);
  }

  .warn {
    background-color: rgb(62, 64, 78);
  }

  .warn:hover {
    background-color: rgb(237, 161, 19);
  }

  .warn-filter {
    background-color: rgb(237, 161, 19);
    box-shadow:
      inset 2px 2px 5px rgba(0, 0, 0, 0.5),
      inset -2px -2px 5px rgba(255, 255, 255, 0.2);
  }

  .error {
    background-color: rgb(62, 64, 78);
  }

  .error:hover {
    background-color: rgb(240, 52, 52);
  }

  .error-filter {
    background-color: rgb(240, 52, 52);
    box-shadow:
      inset 2px 2px 5px rgba(0, 0, 0, 0.5),
      inset -2px -2px 5px rgba(255, 255, 255, 0.2);
  }
</style>
