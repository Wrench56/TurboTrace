<script lang="ts">
    type Print = {
        message?: string,
        italic?: boolean,
        text_align?: string
    };

    export let data: Print;

    data.message ??= "default";
    data.italic ??= true;
    data.text_align ??= "center";
    
    let lines = data.message.split("\n");
</script>

{#each lines as line}
    <p style="text-align: {data.text_align};" class:default={data.italic}>{line}</p>
{/each}

<style>

    p {
        color: #ABB2BF;
        font-weight: bold;
        font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
        margin: 1px;
    }

    .default {
        font-style: italic;
    }

</style>