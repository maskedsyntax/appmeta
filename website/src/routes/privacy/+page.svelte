<script lang="ts">
  import CTABanner from "$lib/components/CTABanner.svelte";

  const updated = "May 31, 2026";
</script>

<svelte:head>
  <title>Privacy Policy — AppMeta</title>
  <meta
    name="description"
    content="How AppMeta handles your project files, API keys, and data. Local-first scanning with BYOK AI."
  />
</svelte:head>

<section class="page-hero section">
  <div class="section-inner prose">
    <span class="label">legal</span>
    <h1>Privacy Policy</h1>
    <p class="mono muted">last-updated: {updated}</p>
    <p>
      This policy describes how <strong>AppMeta</strong> (the desktop application) handles
      information when you use it. It applies to the AppMeta app itself — not to privacy policies
      you generate for your own apps using AppMeta.
    </p>
  </div>
</section>

<section class="section">
  <div class="section-inner prose">
    <h2>Summary</h2>
    <ul>
      <li>AppMeta scans project files <strong>locally on your device</strong>.</li>
      <li>Your full codebase is <strong>not uploaded</strong> to AppMeta servers — we do not operate cloud storage for your projects.</li>
      <li>When you generate content, <strong>confirmed facts</strong> from your project truth file are sent to your chosen AI provider using <strong>your API key</strong>.</li>
      <li>We do not currently collect analytics or telemetry in the app.</li>
    </ul>

    <h2>Information processed locally</h2>
    <p>When you connect a project folder, AppMeta reads files on your machine to extract facts, such as:</p>
    <ul>
      <li>Package manifests (e.g. pubspec.yaml, package.json)</li>
      <li>Platform config (e.g. Info.plist, AndroidManifest.xml)</li>
      <li>Source file names and paths for feature detection</li>
      <li>Documentation files you choose to import (README, CHANGELOG, spec)</li>
    </ul>
    <p>
      This processing happens on your computer. AppMeta does not transmit your project files to
      Masked Syntax or any third party as part of the scanning step.
    </p>

    <h2>Data stored on your device</h2>
    <p>AppMeta stores the following locally in your app data directory:</p>
    <ul>
      <li><strong>Settings</strong> — AI provider, model preferences, and API keys</li>
      <li><strong>Project profiles</strong> — truth files and generated metadata per submission project</li>
      <li><strong>Export history</strong> — when you save submission packs to a folder you choose</li>
    </ul>
    <p>
      In the current MVP, API keys are stored in local JSON settings. macOS Keychain integration
      is planned for a future release. You are responsible for securing access to your machine and
      app data directory.
    </p>

    <h2>AI provider requests</h2>
    <p>
      When you explicitly trigger generation, AppMeta sends a prompt to your configured AI provider
      (e.g. OpenAI or OpenRouter) using your API key. The prompt includes:
    </p>
    <ul>
      <li>Confirmed and verified facts from your project truth file</li>
      <li>Instructions for the specific field being generated</li>
      <li>Character limits and formatting requirements</li>
    </ul>
    <p>
      Your relationship with the AI provider is governed by that provider's terms and privacy
      policy. Review their policies before entering an API key.
    </p>

    <h2>What we do not collect</h2>
    <ul>
      <li>No account system or login in the current version</li>
      <li>No analytics, crash reporting, or usage telemetry (as of this policy date)</li>
      <li>No automatic upload of project source code</li>
    </ul>
    <p>
      If we add optional telemetry or crash reporting in the future, this policy will be updated
      and the app will disclose it clearly before collection.
    </p>

    <h2>Website</h2>
    <p>
      This marketing website is a static site hosted on GitHub Pages. It does not use cookies or
      tracking scripts in the current version. GitHub may collect standard web server logs as
      described in
      <a href="https://docs.github.com/en/site-policy/privacy-policies/github-general-privacy-statement" target="_blank" rel="noopener noreferrer">GitHub's privacy statement</a>.
    </p>

    <h2>Contact</h2>
    <p>
      Questions about this policy? Open an issue on
      <a href="https://github.com/maskedsyntax/appmeta/issues" target="_blank" rel="noopener noreferrer">GitHub</a>.
    </p>

    <h2>Changes</h2>
    <p>
      We may update this policy as AppMeta evolves. The "Last updated" date at the top will reflect
      the most recent revision.
    </p>
  </div>
</section>

<CTABanner
  title="Privacy-first by design"
  subtitle="Local scan. BYOK. Confirmed facts only."
  buttonLabel="download"
/>

<style>
  .page-hero {
    border-top: none;
    padding-bottom: 1rem;
  }

  .page-hero .label {
    margin-bottom: 0.75rem;
  }

  .prose {
    max-width: 720px;
  }

  .prose h2 {
    margin-top: 2.5rem;
    margin-bottom: 0.75rem;
    font-family: var(--font-sans);
    font-size: 1rem;
    font-weight: 600;
    letter-spacing: 0.02em;
    text-transform: uppercase;
    color: var(--color-text-dim);
  }

  .prose p,
  .prose ul {
    color: var(--color-text-muted);
    line-height: 1.75;
  }

  .prose strong {
    color: var(--color-text);
  }

  .prose a {
    text-decoration: underline;
    text-underline-offset: 2px;
  }
</style>
