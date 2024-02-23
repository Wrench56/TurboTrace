<script lang="ts">
    import Console from "$lib/components/console_tab/Console.svelte";
    import { logs } from "$lib/data.ts";

</script>

<Console items={logs} />