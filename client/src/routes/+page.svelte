<script lang="ts">
    import Console from "$lib/components/console_tab/Console.svelte";
    import LogStore from "$lib/stores/LogStore.ts";

    /* TODO: Make better ConsoleData structure */
    type ConsoleData = {
        message?: string,
        text_align?: string,
        header?: boolean,
        timestamp?: number,
        level?: string
        module?: string
    }

    let logs: Array<ConsoleData>;

    LogStore.subscribe((data: Array<ConsoleData>) => {
        logs = data;
    });
</script>

<Console items={logs} />