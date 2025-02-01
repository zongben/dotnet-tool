<script lang="ts">
  import trash from "../../../assets/icons/trash.png";
  import { invoke } from "@tauri-apps/api/core";

  const spanClass = "md:text-end";
  const inputClass = "border-b border-b-gray-500 md:col-span-3 mb-3 md:mb-0";
  const selectClass = "md:col-span-3 mb-3 md:mb-0";

  const connect = async () => {
    await invoke("connect")
      .then((value) => {
        console.log(value);
      })
      .catch((error) => {
        console.error(error);
      });
  };
</script>

<div class="flex justify-center items-center h-full flex-col">
  <div class="grid md:grid-cols-4 md:gap-x-2 md:gap-y-4 w-2/3 md:pe-14">
    <span class={spanClass}>recent:</span>
    <div class="{selectClass} flex items-center">
      <select class="me-2 flex-1"> </select>
      <img
        src={trash}
        alt="trashcan icon"
        class="cursor-pointer size-[18px] text-red-500"
      />
    </div>
    <span class={spanClass}>database:</span>
    <select class={selectClass}>
      <option>mssql</option>
      <option>sqlite</option>
    </select>
    <span class={spanClass}>host:</span>
    <input type="text" class={inputClass} />
    <span class={spanClass}>init db:</span>
    <input type="text" class={inputClass} />
    <span class={spanClass}>username:</span>
    <input type="text" class={inputClass} />
    <span class={spanClass}>password:</span>
    <input class={inputClass} />
  </div>
  <div class="mt-8">
    <button
      onclick={connect}
      class="bg-blue-500 text-white px-3 py-1 rounded hover:bg-blue-400 cursor-pointer"
      >Connect</button
    >
  </div>
</div>
