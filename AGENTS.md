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

## Clean-break contract (ADR-0002)

This repository is Rust sole authority — there is no TypeScript backend tree.
The following are hard contract floors; do not regress them:

1. **Single canonical HTTP surface:** `/api/v1/*` plus the shields-style
   `/badge/{label}-{message}-{color}` path form. Bare legacy aliases
   (`/banner`, `/stats`, `/org`, `/repo`, `/icons`, `/brand`, `/deploy`) are
   deleted and must not be reintroduced.
2. **Single canonical product host:** `mark.sylphx.com`. `img.sylphx.com` is
   retired; do not re-add it to docs, config, or examples.
3. **Bounded inputs:** banner text ≤ 500 / desc ≤ 240, badge label ≤ 80 /
   message ≤ 120, icon rows ≤ 60. Truncation is marked with `…`.
4. **SVG attribute grammar:** attribute values come only from validated hex
   color tokens, named colors, or static strings; all user text is escaped.
   Never inject unvalidated strings into SVG.
5. **Deploy identity:** the Docker image must embed a git revision
   (`COPY .git` + `build.rs`, or platform build args). `mark --version` fails
   the image build when the revision is unknown. `/health.revision` is deploy
   proof, never capability proof.
6. **GitHub cards fail closed:** an upstream snapshot failure renders the
   error card, never a zero-data card.
