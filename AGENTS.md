# Mark — local agent notes

Static engineering and delivery standards load from the active Skills runtime
([SylphxAI/skills](https://github.com/SylphxAI/skills) is binding instruction
SSOT). Doctrine and Mission Control are retired historical lineage and must not
be loaded as current instruction authority.

This file is local commands/hazards only.

## Local commands

```bash
cargo test
cargo run
cargo build --release
```

Env: see `.env.example` (`PORT`, `GITHUB_TOKEN`, `DEFAULT_CREDIT`, `PUBLIC_BASE_URL`).

## Hazards

- Stateless SVG only on hot path — do not add headless browser / AI generation without cache design.
- Soft watermark via `credit`; never force heavy branding that kills adoption.
- GitHub upstream for stats needs cache + optional `GITHUB_TOKEN`.
