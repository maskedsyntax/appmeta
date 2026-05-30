<div class="terminal">
  <div class="terminal-bar" aria-hidden="true">
    <span class="terminal-dot"></span>
    <span class="terminal-dot"></span>
    <span class="terminal-dot"></span>
    <span class="terminal-title">appmeta — zsh</span>
  </div>
  <div class="terminal-body">
    <div><span class="prompt">$</span> appmeta scan ./my-flutter-app</div>
    <div class="out">→ pubspec.yaml · Info.plist · 12 dependencies</div>
    <div><span class="prompt">$</span> appmeta facts review</div>
    <div class="out">→ truth file: 38 verified · 4 need confirmation</div>
    <div><span class="prompt">$</span> appmeta generate --provider openai</div>
    <div class="out ok">→ wrote submission-pack.md · submission-pack.json</div>
  </div>
</div>

<style>
  .terminal-title {
    margin-left: 0.5rem;
    font-family: var(--font-mono);
    font-size: 0.6875rem;
    color: var(--color-text-dim);
  }
</style>
