<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { state } from "$lib/states/stringbuilder.svelte";

  const convert_text = async () => {
    const result: string = await invoke("convert", { text: state.text });
    state.sb = result;
  };

  const revert_stringbuilder = async () => {
    const result: string = await invoke("revert", { sb: state.sb });
    state.text = result;
  };
</script>

<div class="flex flex-col gap-5 h-full px-5 pt-2">
  <div class="flex-1 flex flex-col">
    <h1 class="text-xl">PlainText</h1>
    <textarea
      class="border-2 border-gray-300 rounded w-full flex-1"
      bind:value={state.text}
    ></textarea>
  </div>

  <div class="flex-1 flex flex-col">
    <h1 class="text-xl">StringBuilder</h1>
    <textarea
      class="border-2 border-gray-300 rounded w-full flex-1"
      bind:value={state.sb}
    ></textarea>
  </div>

  <div class="my-5">
    <div class="flex justify-center gap-20">
      <button
        class="bg-blue-500 text-white px-4 py-2 rounded hover:bg-blue-400"
        onclick={convert_text}>Convert</button
      >
      <button
        class="bg-amber-500 text-white px-4 py-2 rounded hover:bg-amber-400"
        onclick={revert_stringbuilder}>Revert</button
      >
    </div>
  </div>
</div>
