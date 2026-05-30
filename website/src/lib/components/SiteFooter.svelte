<script lang="ts">
  import { base } from "$app/paths";

  const year = new Date().getFullYear();
</script>

<footer class="footer">
  <div class="footer-inner">
    <div class="footer-top">
      <div class="footer-brand">
        <span class="brand-name">AppMeta</span>
        <span class="tag">dev.maskedsyntax.appmeta</span>
      </div>

      <div class="footer-links">
        <a href="{base}/features/">features</a>
        <a href="{base}/privacy/">privacy</a>
        <a href="{base}/download/">download</a>
        <a href="https://github.com/maskedsyntax/appmeta" target="_blank" rel="noopener noreferrer">
          github
        </a>
      </div>
    </div>

    <p class="copyright muted">
      &copy; {year} Masked Syntax — prepares App Store Connect metadata from confirmed project
      facts. macOS · Rust · Svelte.
    </p>
  </div>
</footer>

<style>
  .footer {
    border-top: 1px solid var(--color-border-subtle);
    padding: 2.5rem 1.5rem;
    margin-top: auto;
    background: var(--color-bg);
  }

  .footer-inner {
    max-width: var(--site-max-width);
    margin: 0 auto;
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .footer-top {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .footer-brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .brand-name {
    font-family: var(--font-mono);
    font-size: 0.875rem;
    font-weight: 500;
  }

  .footer-links {
    display: flex;
    flex-wrap: wrap;
    gap: 1.25rem;
  }

  .footer-links a {
    font-family: var(--font-mono);
    font-size: 0.8125rem;
    color: var(--color-text-muted);
    text-decoration: none;
  }

  .footer-links a:hover {
    color: var(--color-text);
  }

  .copyright {
    font-family: var(--font-mono);
    font-size: 0.75rem;
    margin: 0;
    line-height: 1.6;
  }
</style>
