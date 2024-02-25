<script lang="ts">
    /* Non-typescript module*/
    // @ts-ignore
    import LazyList from 'lazy-load-list/svelte' 

    import Overview from '$lib/components/console_tab/Overview.svelte';

    import LogEntry from '$lib/components/console_tab/LogEntry.svelte';
    import Print from '$lib/components/console_tab/LogPrint.svelte';
    import Header from '$lib/components/console_tab/Header.svelte';

    import type { ConsolePrint } from "$lib/types/console_tab.types";

    export let items: Array<ConsolePrint>;
</script>

<div class="center">
    <div class="console">
        <LazyList
            data={items}
            itemsPerRender={120}
            defaultLoadingColor="#222"
            let:item={item}
        >
            {#if item.level !== undefined}
                <LogEntry log={item} />
            {:else if item.header !== undefined}
                <Header />
            {:else if item.line !== undefined}
                <hr>
            {:else}
                <Print data={item} />
            {/if}
        </LazyList>
    </div>
</div>

<style>
    .console {
        background-color: rgba(40, 44, 52, 0.9);
        border-radius: 12px;
        width: 92%;
        height: 85%;
        margin: 16px;
        padding: 30px 30px 36px 36px;
    }

    /* Fix this */
    .center {
        display: flex;
        align-items: center;
        justify-content: center;
        height: 97vh;
    }
</style>
