<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let text = "";
  let sb = "";

  const convert_text = async () => {
    const result: string = await invoke("convert_text", { text });
    sb = result;
  };

  const revert_stringbuilder = async () => {
    const result: string = await invoke("revert_stringbuilder", { sb });
    text = result;
  };
</script>

<div class="flex flex-col gap-5 h-full">
  <div class="flex-1 flex flex-col">
    <h1 class="text-xl">Text</h1>
    <textarea
      class="border-2 border-gray-300 rounded w-full flex-1"
      bind:value={text}
    ></textarea>
  </div>

  <div class="flex-1 flex flex-col">
    <h1 class="text-xl">StringBuilder</h1>
    <textarea
      class="border-2 border-gray-300 rounded w-full flex-1"
      bind:value={sb}
    ></textarea>
  </div>

  <div class="my-5">
    <div class="flex justify-center gap-20">
      <button
        class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-400"
        on:click={convert_text}>Convert</button
      >
      <button
        class="bg-amber-500 text-white px-4 py-2 rounded hover:bg-amber-400"
        on:click={revert_stringbuilder}>Revert</button
      >
    </div>
  </div>
</div>
