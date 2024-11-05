<script lang="ts">
  import Overview from "$lib/components/console_tab/Overview.svelte";

  import LogEntry from "$lib/components/console_tab/LogEntry.svelte";
  import Print from "$lib/components/console_tab/LogPrint.svelte";
  import Header from "$lib/components/console_tab/Header.svelte";

  import { type ConsolePrint } from "$lib/types/console_tab.types";

  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import LogStore from "$lib/stores/LogStore";
  import { levelFilterStore } from "$lib/stores/FilterStore";
  import VolumeStore from "$lib/stores/VolumeStore";

  onMount(async () => {
    await listen<LogEntry>("ttlog", (event) => {
      LogStore.update((items) => {
        const payload = event.payload as LogEntry;
        VolumeStore.update(
          (current) =>
            (current +=
              payload.timestamp.toString().length +
              payload.level.toString().length +
              payload.module.length +
              payload.message.length)
        );
        items.push(event.payload as ConsolePrint);
        if (autoScroll) {
          container?.scrollTo({
            top: container.scrollHeight,
            behavior: "smooth",
          });
        }
        return items;
      });
    });
  });

  let container: HTMLDivElement;
  let autoScroll = true;

  $: filteredItems = items.filter((o, i, a) => $levelFilterStore(o, i, a));
  export let items: Array<ConsolePrint>;
</script>

<div class="center">
  <div class="console">
    <div class="center-horiz">
      <Overview />
    </div>
    <div
      class="container"
      bind:this={container}
      on:scroll={() =>
        (autoScroll =
          container.scrollTop + container.clientHeight + 120 >=
          container.scrollHeight)}
    >
      {#each filteredItems as item}
        {#if item.level !== undefined}
          <LogEntry log={item} />
        {:else if item.header !== undefined}
          <Header />
        {:else if item.line !== undefined}
          <hr />
        {:else}
          <Print data={item} />
        {/if}
      {/each}
    </div>
  </div>
</div>

<style>
  .console {
    background-color: rgba(40, 44, 52, 1);
    border-radius: 12px;
    width: 92%;
    height: 85%;
    margin: 16px;
    padding: 30px 30px 36px 36px;
    box-shadow:
      12px 12px 16px 0 #191a1b,
      -8px -8px 12px 0 #232425;
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 92vh;
  }

  .center-horiz {
    display: block;
    margin: auto;
    width: 100%;
  }

  .container {
    height: 85%;
    overflow-y: auto;
    padding: 2px;
  }

  .container::-webkit-scrollbar {
    width: 2px;
  }

  .container::-webkit-scrollbar-thumb {
    background-color: #888;
    border-radius: 4px;
  }

  .container::-webkit-scrollbar-thumb:hover {
    background-color: #555;
  }
</style>
