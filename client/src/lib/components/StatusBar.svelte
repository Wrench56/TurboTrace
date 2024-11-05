<script lang="ts">
  import { listen } from "@tauri-apps/api/event";
  import { onMount } from "svelte";

  import LogStore from "$lib/stores/LogStore";

  import {
    type BytesRecv,
    type NetStatus,
    type SysStatus,
  } from "$lib/types/status.types";
  import { TextAlign } from "$lib/types/console_tab.types";
  import VolumeStore from "$lib/stores/VolumeStore";

  let current_net_status: NetStatus = {
    connected: false,
  };

  let current_bytes_recv: BytesRecv = {
    raw_bytes: 0,
  };

  let current_sys_status: SysStatus = {
    mem_usage: 0,
    cpu_usage: 0.0,
  };

  let volume = 0;
  $: volume = $VolumeStore;

  onMount(async () => {
    await listen<SysStatus>("sys_stat", (event) => {
      current_sys_status = event.payload;
    });

    await listen<BytesRecv>("raw_bytes", (event) => {
      current_bytes_recv = event.payload;
    });

    await listen<NetStatus>("net_stat", (event) => {
      current_net_status = event.payload;
      if (current_net_status.connected) {
        LogStore.update((items) => {
          let lastItem = items[items.length - 1];
          if (lastItem.header == undefined && lastItem.line == undefined) {
            items.push({
              line: true,
            });
          }
          items.push(
            {
              message: "New connection accepted",
              text_align: TextAlign.CENTER,
            },
            {
              line: true,
            }
          );
          return items;
        });
      }
    });
  });
</script>

<div class="center">
  <div class="container">
    <p class="data">
      Status:
      {#if current_net_status.connected == true}
        <span style="color: #4CBB17; font-weight: bolder">Connected</span>
      {:else}
        <span style="color: #FF0024; font-weight: bolder">Disconnected</span>
      {/if}
    </p>
    <p class="data">
      Mem: {Math.round(current_sys_status.mem_usage * 100) / 100}% &nbsp; |
      &nbsp; CPU: {Math.round(current_sys_status.cpu_usage * 100) / 100}%
    </p>
    <p class="data">
      Recv: {current_bytes_recv.raw_bytes} bytes | Data: {volume} bytes [{Math.round(
        (1 - (current_bytes_recv.raw_bytes / (volume == 0 ? 1 : volume))) * 100
      )}%]
    </p>
  </div>
</div>

<style>
  .data {
    color: rgb(173, 172, 172);
    margin: 0px 20px 2px 20px;
    font-size: small;
    font-weight: bold;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif, monospace;
  }

  .container {
    display: flex;
    position: absolute;
    bottom: 0px;
    width: 100%;
    height: 26px;
    background-color: rgb(27, 31, 37);
    padding: 0px 32px 0px 32px;
    align-items: center;
  }

  .center {
    display: flex;
    align-items: center;
    justify-content: center;
  }
</style>
