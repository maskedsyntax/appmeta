<script lang="ts">
  import "../app.css";
  import SiteNav from "$lib/components/SiteNav.svelte";
  import SiteFooter from "$lib/components/SiteFooter.svelte";

  let { children } = $props();
</script>

<div class="site">
  <SiteNav />
  <main>{@render children()}</main>
  <SiteFooter />
</div>

<style>
  .site {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: var(--color-bg-main);
    background-image: radial-gradient(circle at 1px 1px, var(--color-border-subtle) 1px, transparent 0);
    background-size: 24px 24px;
  }

  main {
    flex: 1;
  }
</style>
