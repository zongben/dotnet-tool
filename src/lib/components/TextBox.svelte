<script lang="ts">
  let { value = $bindable(), class: classes = "" } = $props();
</script>

<input
  type="text"
  class="border border-gray-300 rounded px-1 py-0 focus:outline-none focus:ring-2 focus:ring-blue-500 w-full {classes}"
  bind:value
/>
