# Mark — local agent notes

Static engineering and delivery standards load from the active Skills runtime
([SylphxAI/skills](https://github.com/SylphxAI/skills) is binding instruction
SSOT). Doctrine and Mission Control are retired historical lineage and must not
be loaded as current instruction authority.

This file is local commands/hazards only.

## Local commands

```bash
cargo test
cargo test --test architecture_boundaries
cargo run
cargo build --release
```

Env: see `.env.example` (`PORT`, `GITHUB_TOKEN`, `DEFAULT_CREDIT`, `PUBLIC_BASE_URL`).

## Hazards

- Stateless SVG only on hot path — do not add headless browser / AI generation without cache design.
- Soft watermark via `credit`; never force heavy branding that kills adoption.
- GitHub upstream for stats needs cache + optional `GITHUB_TOKEN`.

## Backend false-authority fence

Work: wi_01KYFN6993PMG8WD00Q51AE231

If this repository has completed a **Rust backend** cutover:

1. Production backend behavior authority is the Rust crate/binary/service path declared in deploy manifests / package native bin / Docker ENTRYPOINT / `sylphx.toml`.
2. Residual TypeScript service trees are **not** product authority unless explicitly proven still on the live path.
3. Do not "fix production" by editing residual TypeScript and assuming runtime will pick it up.
4. Prefer deleting residual TS backend trees after Rust sole proof; keep history in Git.
5. Intentional TypeScript frontends, npm packaging wrappers, and native-binding surfaces may remain.
