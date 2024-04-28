<script lang="ts">
  /* Non-typescript module*/
  // @ts-ignore
  import LazyList from "lazy-load-list/svelte";

  import Overview from "$lib/components/console_tab/Overview.svelte";

  import LogEntry from "$lib/components/console_tab/LogEntry.svelte";
  import Print from "$lib/components/console_tab/LogPrint.svelte";
  import Header from "$lib/components/console_tab/Header.svelte";

  import type { ConsolePrint } from "$lib/types/console_tab.types";
  import type { Event } from "$lib/types/event.types";

  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";
  import LogStore from "$lib/stores/LogStore";

  onMount(async () => {
    await listen("ttlog", (event: Event<LogEntry>) => {
      LogStore.update((items) => {
        // @ts-ignore
        items.push(event.payload as ConsolePrint);
        return items;
      });
    });
  });

  export let items: Array<ConsolePrint>;
</script>

<div class="center">
  <div class="console">
    <div class="center-horiz">
      <Overview />
    </div>
    <LazyList
      data={items}
      itemsPerRender={120}
      defaultLoadingColor="#222"
      let:item
    >
      {#if item.level !== undefined}
        <LogEntry log={item} />
      {:else if item.header !== undefined}
        <Header />
      {:else if item.line !== undefined}
        <hr />
      {:else}
        <Print data={item} />
      {/if}
    </LazyList>
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
</style>
