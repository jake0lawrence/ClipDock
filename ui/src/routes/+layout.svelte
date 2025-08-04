<script lang="ts">
        import favicon from '$lib/assets/favicon.svg';
        import Settings from '../components/Settings.svelte';

        let { children } = $props();
        let open = false;
</script>

<svelte:head>
	<link rel="icon" href={favicon} />
</svelte:head>

{@render children?.()}

<button class="fixed top-4 right-4" on:click={()=>open=true}>⚙️</button>
{#if open}
  <div class="fixed inset-0 bg-black/60 grid place-items-center" on:click={()=>open=false}>
    <div on:click|stopPropagation>
      <Settings />
    </div>
  </div>
{/if}
