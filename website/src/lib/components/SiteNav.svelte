<script lang="ts">
  import { base } from "$app/paths";
  import { page } from "$app/stores";

  const links = [
    { href: `${base}/features/`, label: "Features" },
    { href: `${base}/privacy/`, label: "Privacy" },
    { href: `${base}/download/`, label: "Download" },
  ];

  let menuOpen = $state(false);

  function isActive(href: string, pathname: string): boolean {
    if (href === `${base}/` || href === base) return pathname === `${base}/` || pathname === base;
    return pathname.startsWith(href);
  }

  function closeMenu() {
    menuOpen = false;
  }
</script>

<header class="nav">
  <div class="nav-inner">
    <a href="{base}/" class="brand" onclick={closeMenu}>
      <img src="{base}/logo.png" alt="" class="brand-logo" aria-hidden="true" />
      <span class="brand-name">AppMeta</span>
      <span class="brand-version">v0.1</span>
    </a>

    <button
      class="menu-toggle"
      type="button"
      aria-label={menuOpen ? "Close menu" : "Open menu"}
      aria-expanded={menuOpen}
      onclick={() => (menuOpen = !menuOpen)}
    >
      <span class="bar"></span>
      <span class="bar"></span>
      <span class="bar"></span>
    </button>

    <nav class="links" class:open={menuOpen}>
      {#each links as link}
        <a
          href={link.href}
          class:active={isActive(link.href, $page.url.pathname)}
          onclick={closeMenu}
        >
          {link.label}
        </a>
      {/each}
      <a href="{base}/download/" class="btn btn-primary nav-cta" onclick={closeMenu}>
        download
      </a>
    </nav>
  </div>
</header>

<style>
  .nav {
    position: sticky;
    top: 0;
    z-index: 100;
    height: var(--nav-height);
    background: rgba(12, 12, 14, 0.92);
    backdrop-filter: blur(8px);
    border-bottom: 1px solid var(--color-border-subtle);
  }

  .nav-inner {
    max-width: var(--site-max-width);
    margin: 0 auto;
    padding: 0 1.5rem;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    color: var(--color-text);
    text-decoration: none;
  }

  .brand:hover {
    color: var(--color-text);
    text-decoration: none;
  }

  .brand-logo {
    width: 24px;
    height: 24px;
    opacity: 0.9;
  }

  .brand-name {
    font-family: var(--font-mono);
    font-weight: 500;
    font-size: 0.875rem;
    letter-spacing: 0.02em;
  }

  .brand-version {
    font-family: var(--font-mono);
    font-size: 0.6875rem;
    color: var(--color-text-dim);
    padding: 0.15rem 0.35rem;
    border: 1px solid var(--color-border-subtle);
    border-radius: var(--radius-sm);
  }

  .links {
    display: flex;
    align-items: center;
    gap: 1.75rem;
  }

  .links a:not(.btn) {
    font-family: var(--font-mono);
    font-size: 0.8125rem;
    color: var(--color-text-muted);
    text-decoration: none;
  }

  .links a:not(.btn):hover,
  .links a:not(.btn).active {
    color: var(--color-text);
  }

  .nav-cta {
    padding: 0.4rem 0.875rem;
    font-size: 0.75rem;
  }

  .menu-toggle {
    display: none;
    flex-direction: column;
    gap: 5px;
    background: none;
    border: none;
    cursor: pointer;
    padding: 0.25rem;
  }

  .bar {
    display: block;
    width: 20px;
    height: 1.5px;
    background: var(--color-text-muted);
  }

  @media (max-width: 640px) {
    .menu-toggle {
      display: flex;
    }

    .links {
      display: none;
      position: absolute;
      top: var(--nav-height);
      left: 0;
      right: 0;
      flex-direction: column;
      align-items: stretch;
      gap: 0;
      background: var(--color-bg-main);
      border-bottom: 1px solid var(--color-border);
      padding: 0.5rem 0;
    }

    .links.open {
      display: flex;
    }

    .links a {
      padding: 0.875rem 1.5rem;
    }

    .nav-cta {
      margin: 0.5rem 1.5rem 0.75rem;
      text-align: center;
    }
  }
</style>
