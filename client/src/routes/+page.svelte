<script lang="ts">
  import Console from "$lib/components/console_tab/Console.svelte";
  import StatusBar from "$lib/components/StatusBar.svelte";
  import LogStore from "$lib/stores/LogStore.ts";

  import type { ConsolePrint } from "$lib/types/console_tab.types";

  let logs: Array<ConsolePrint>;

  LogStore.subscribe((data: Array<ConsolePrint>) => {
    logs = data;
  });
</script>

<Console items={logs} />
<StatusBar />
