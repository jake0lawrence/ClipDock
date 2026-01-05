<script lang="ts">
  import { clips, refresh, toggle, copyToClipboard } from '../stores';
  import { onMount } from 'svelte';
  import Fuse from 'fuse.js';
  let query = '';
  let fuse: Fuse<any>;

  $: filtered = query
      ? fuse?.search(query).map(r => r.item) ?? []
      : $clips;

  $: if ($clips) {
      fuse = new Fuse($clips, { keys:['text'], threshold:0.4 });
  }

  onMount(refresh);
</script>

<div class="dark:bg-zinc-800 bg-white p-4 rounded-xl w-[30rem] shadow-xl dark:text-white text-zinc-900">
  <input
    bind:value={query}
    placeholder="Search…"
    class="w-full mb-3 px-3 py-2 dark:bg-zinc-900 bg-zinc-100 dark:text-white text-zinc-900 rounded"
  />
  <ul class="space-y-1 max-h-80 overflow-auto">
    {#each filtered as clip}
      <li class="flex items-center justify-between dark:hover:bg-zinc-700 hover:bg-zinc-200 p-2 rounded">
        <span
          class="truncate cursor-pointer flex-1"
          on:click={() => copyToClipboard(clip.text)}
        >
          {clip.text}
        </span>
        <button on:click={() => toggle(clip.id)}>
          {clip.pinned ? '★' : '☆'}
        </button>
      </li>
    {/each}
  </ul>
</div>

