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
src/             SvelteKit frontend
spec.md          Full product specification
TODO.md          Implementation checklist
```

## License

MIT — see [LICENSE](LICENSE)
