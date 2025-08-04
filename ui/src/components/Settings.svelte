<script lang="ts">
  import { settings, save, toggleStart, load } from '../lib/settings';
  import { onMount } from 'svelte';
  let local: any;
  onMount(async () => { await load(); settings.subscribe(v => local = { ...v }); });
</script>

<div class="p-4 space-y-4">
  <label class="flex items-center gap-2">
    <input type="checkbox" bind:checked={local.dark} />
    Dark theme
  </label>

  <label class="flex items-center gap-2">
    History size
    <input type="number" min="5" max="50" bind:value={local.history} class="w-16 text-black"/>
  </label>

  <label class="flex items-center gap-2">
    <input type="checkbox" bind:checked={local.autostart} on:change={(e)=>toggleStart((e.target as HTMLInputElement).checked)}/>
    Launch on boot
  </label>

  <button class="bg-blue-600 px-3 py-1 rounded" on:click={()=>save(local)}>
    Save
  </button>
</div>
