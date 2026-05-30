# AppMeta

App Store Connect submission assistant — generate metadata from verified project facts.

AppMeta scans a local Flutter project, helps you confirm detected facts, and uses your own AI API key (BYOK) to generate App Store Connect-ready fields. Source code stays on your machine.

## Requirements

- macOS (MVP target)
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) stable
- [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/)

## Development

```bash
npm install
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

## Tests

```bash
# Rust unit tests
cd src-tauri && cargo test

# Frontend typecheck
npm run check
```

## Workflow

1. **Dashboard** — Connect a Flutter project folder
2. **Scan** — Review detected app identity, dependencies, permissions
3. **Facts** — Confirm facts and answer questionnaire
4. **Settings** — Add your OpenAI (or OpenRouter) API key
5. **Product Page / Review / Privacy / IAP** — Generate fields
6. **Export** — Save `submission-pack.md` and `submission-pack.json`

## Project structure

```
src-tauri/src/   Rust backend (scanner, AI, validation, export)
src/             SvelteKit frontend (desktop app)
website/         Marketing site (SvelteKit, GitHub Pages)
spec.md          Full product specification
TODO.md          Implementation checklist
```

## Marketing website

The public site lives in `website/` as a separate SvelteKit app (static prerender for GitHub Pages).

```bash
# Install website dependencies (first time)
npm ci --prefix website

# Local dev (http://localhost:5173)
npm run website:dev

# Typecheck
npm run website:check

# Production build (custom domain — no base path)
npm run website:build
```

Live site: **https://appmeta.maskedsyntax.com**

### Deployment

Pushes to `master` that touch `website/**` run [`.github/workflows/deploy-website.yml`](.github/workflows/deploy-website.yml), which builds the static site and publishes to GitHub Pages.

1. **Settings → Pages → Build and deployment → Source:** GitHub Actions
2. **Custom domain:** `appmeta.maskedsyntax.com` (also in [`website/static/CNAME`](website/static/CNAME))
3. **Manual deploy:** Actions → Deploy Website → Run workflow

DNS: `CNAME` record `appmeta` → `<user-or-org>.github.io` (or the target shown in Pages settings).

## License

MIT — see [LICENSE](LICENSE)
