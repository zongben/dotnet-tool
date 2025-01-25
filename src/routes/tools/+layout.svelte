<script lang="ts">
  let { children } = $props();
</script>

<div class="pt-5 pb-5 mb-5 px-5 bg-blue-200">
  <a href="/">Return</a>
</div>
<div class="px-5">
  {@render children()}
</div>
