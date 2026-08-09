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

Env: see `.env.example` (`PORT`, `HOST`, `PUBLIC_BASE_URL`, `DEFAULT_CREDIT`, `RUST_LOG`).

## Hazards

- Stateless SVG only on hot path — do not add headless browser / AI generation without cache design.
- Soft watermark via `credit`; never force heavy branding that kills adoption.
- Do not reintroduce upstream data, clocks, secrets, or state of any kind —
  determinism is the product.

## The one grammar (ADR-0003)

This repository is the north-star end state: **one capability, one grammar**.

1. **One surface:** `GET /api/v1/mark/{form}` is the whole product — forms
   `hero` `pill` `strip` `identity` `deploy` — plus the shields-style pill
   shorthand `/badge/{label}-{message}-{color}`. Every legacy capability route
   (banner, badge, icons, brand, deploy, stats, org, repo) is deleted; do not
   reintroduce them.
2. **One grammar:** mark = form × art (`type`) × paint (`theme`/`color`) ×
   geometry (`width`/`height`) × text × motion (`animation`). Composition is
   the product: any form can carry any palette, art (hero/identity), and
   text-level motion (pill/identity/strip).
3. **Determinism:** a mark is a pure function of its URL. No clock
   (`timeAuto`/`timeGradient`/clock seeds are retired), no upstream, no state,
   no secrets. Same URL renders the same SVG forever.
4. **No live data:** GitHub cards (stats/org/repo) and the network adapter are
   retired. Specialist hosts own data; Mark renders only what the URL says.
5. **Bounded inputs:** hero text ≤ 500 / desc ≤ 240 / 8 lines, pill label ≤ 80
   / message ≤ 120, strip ≤ 60 icons, identity brand ≤ 40 / tagline ≤ 120,
   deploy service ≤ 40. Truncation is marked with `…`.
6. **SVG attribute grammar:** attribute values come only from validated hex
   tokens, named colors, or static strings; all user text is escaped. Never
   inject unvalidated strings into SVG.
7. **Deploy identity:** the Docker image must embed a git revision via the
   platform build-arg contract (`SYLPHX_GIT_COMMIT_SHA` / `SYLPHX_GIT_SHA`,
   then `SOURCE_COMMIT` / `GIT_SHA`). `mark --version` fails the image build
   when the revision is unknown. `/health.revision` is deploy proof, never
   capability proof.
8. **Rendering never fails:** unknown inputs normalize (unknown form → hero,
   unknown art → aurora, invalid colors → fallback paint). There is no
   error-SVG path by construction.
