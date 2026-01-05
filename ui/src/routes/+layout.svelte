<script lang="ts">
        import favicon from '$lib/assets/favicon.svg';
        import Settings from '../components/Settings.svelte';
        import { settings, load } from '../lib/settings';
        import { onMount } from 'svelte';

        let { children } = $props();
        let open = false;

        onMount(load);
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="{$settings.dark ? 'dark' : 'light'}">
  {@render children?.()}

  <button class="fixed top-4 right-4 text-2xl" on:click={()=>open=true}>⚙️</button>
  {#if open}
    <div class="fixed inset-0 bg-black/60 grid place-items-center" on:click={()=>open=false}>
      <div on:click|stopPropagation>
        <Settings />
      </div>
    </div>
  {/if}
</div>
