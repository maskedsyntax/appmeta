<script lang="ts">
  import { base } from "$app/paths";

  interface Props {
    title: string;
    subtitle: string;
    primaryHref?: string;
    primaryLabel?: string;
    secondaryHref?: string;
    secondaryLabel?: string;
  }

  let {
    title,
    subtitle,
    primaryHref = `${base}/download/`,
    primaryLabel = "Get AppMeta",
    secondaryHref = `${base}/features/`,
    secondaryLabel = "See features",
  }: Props = $props();
</script>

<section class="hero">
  <div class="hero-glow" aria-hidden="true"></div>
  <div class="hero-inner">
    <p class="eyebrow">Privacy-first · BYOK · macOS</p>
    <h1>{@html title}</h1>
    <p class="subtitle">{subtitle}</p>
    <div class="actions">
      <a href={primaryHref} class="btn btn-primary">{primaryLabel}</a>
      {#if secondaryHref}
        <a href={secondaryHref} class="btn btn-secondary">{secondaryLabel}</a>
      {/if}
    </div>
  </div>
</section>

<style>
  .hero {
    position: relative;
    padding: 5rem 1.5rem 4rem;
    overflow: hidden;
  }

  .hero-glow {
    position: absolute;
    inset: -40% -20% auto;
    height: 500px;
    background: var(--gradient-brand-subtle);
    filter: blur(60px);
    pointer-events: none;
  }

  .hero-inner {
    position: relative;
    max-width: 720px;
    margin: 0 auto;
    text-align: center;
  }

  .eyebrow {
    font-size: 0.8125rem;
    font-weight: 600;
    letter-spacing: 0.08em;
    text-transform: uppercase;
    color: var(--color-primary);
    margin: 0 0 1rem;
  }

  .subtitle {
    font-size: 1.125rem;
    color: var(--color-text-muted);
    margin: 1.25rem 0 2rem;
    line-height: 1.7;
  }

  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 0.75rem;
    justify-content: center;
  }
</style>
