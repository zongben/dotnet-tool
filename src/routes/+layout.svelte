<script lang="ts">
  import "../app.css";
  import { page } from "$app/state";

  let { children } = $props();

  let active = (path: string) => {
    return page.url.pathname === path
      ? "bg-(--sidebar-btn-active-bg-color)"
      : "";
  };
</script>

<div class="flex h-full">
  <nav class="bg-(--sidebar-bg-color) px-5 pt-3">
    <ul>
      <li class="mb-5">
        <a href="/" class="text-white text-[24px] block text-center font-bold"
          >DotnetTool</a
        >
      </li>
      <li>
        <a
          href="/tools/stringbuilder"
          class="text-white block text-center hover:bg-(--sidebar-btn-hover-bg-color) active:bg-(--sidebar-btn-active-bg-color) rounded-[10px] py-1 px-5 {active(
            '/tools/stringbuilder',
          )}">StringBuilder</a
        >
      </li>
    </ul>
  </nav>
  <main class="flex-1">
    {@render children()}
  </main>
</div>
