<script lang="ts">
  import { clips, refresh, toggle } from '../stores';
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

<div class="bg-zinc-800 p-4 rounded-xl w-[30rem] shadow-xl text-white">
  <input
    bind:value={query}
    placeholder="Search…"
    class="w-full mb-3 px-3 py-2 bg-zinc-900 rounded"
  />
  <ul class="space-y-1 max-h-80 overflow-auto">
    {#each filtered as clip}
      <li class="flex items-center justify-between hover:bg-zinc-700 p-2 rounded">
        <span class="truncate">{clip.text}</span>
        <button on:click={() => toggle(clip.id)}>
          {clip.pinned ? '★' : '☆'}
        </button>
      </li>
    {/each}
  </ul>
</div>

