<script lang="ts">
  interface Props {
    tag: string;
    title: string;
    description: string;
  }

  let { tag, title, description }: Props = $props();
</script>

<article class="feature">
  <span class="tag">{tag}</span>
  <h3>{title}</h3>
  <p class="muted">{description}</p>
</article>

<style>
  .feature {
    background: var(--color-bg-panel);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 1.25rem 1.375rem;
    height: 100%;
    border-left: 2px solid var(--color-accent-dim);
  }

  .feature h3 {
    margin: 0.75rem 0 0.5rem;
  }

  .feature p {
    margin: 0;
    font-size: 0.9375rem;
    line-height: 1.6;
  }
</style>
