<script lang="ts">
  let { text = $bindable() } = $props();
</script>

<textarea
  class="border-2 border-gray-300 rounded w-full flex-1"
  bind:value={text}
></textarea>
