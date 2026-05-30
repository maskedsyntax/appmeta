<script lang="ts">
  import { base } from "$app/paths";

  interface Props {
    title?: string;
    subtitle?: string;
    buttonLabel?: string;
    href?: string;
  }

  let {
    title = "Ready to prepare your submission?",
    subtitle = "Scan locally, confirm facts, generate fields with your own API key.",
    buttonLabel = "download",
    href = `${base}/download/`,
  }: Props = $props();
</script>

<section class="cta section">
  <div class="section-inner">
    <div class="cta-inner">
      <span class="label">next step</span>
      <h2>{title}</h2>
      <p class="muted">{subtitle}</p>
      <a {href} class="btn btn-primary">{buttonLabel}</a>
    </div>
  </div>
</section>

<style>
  .cta {
    padding-top: 2rem;
    padding-bottom: 5rem;
    border-top: none;
  }

  .cta-inner {
    max-width: 560px;
    padding: 2rem 0 0;
    border-top: 1px solid var(--color-border);
  }

  .cta-inner .label {
    margin-bottom: 0.75rem;
  }

  .cta-inner h2 {
    margin-bottom: 0.75rem;
  }

  .cta-inner p {
    margin-bottom: 1.5rem;
    font-size: 1rem;
  }
</style>
