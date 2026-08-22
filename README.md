# Mark

Mark is the dependable image language for software identity: a maintainer expresses a complete mark in one public URL, embeds that URL in a README, site, or product surface, and receives a polished SVG without an account, asset build, or data dependency.

- Ordinary: https://mark.sylphx.com — product-declared customer host (`docs/vision.md` shipped terminal, `PUBLIC_BASE_URL`, `sylphx.toml`). A `200` is not the product contract.
- Preview: `none` — this repository has no current honest preview URL. GitHub Pages and deployments are absent. `https://mark-web-prod.sylphx.app` is a stale platform auto host (404), not a preview.
- Vision: [`docs/vision.md`](docs/vision.md)
- Capabilities: [`docs/capabilities.md`](docs/capabilities.md)
- Decisions: [`docs/adr/`](docs/adr/)

Embeddable **SVG** marks — hero banners, status pills, tech strips, profile cards, and “deployed on Sylphx” conversion marks — from **one grammar** and **one URL**, rendered deterministically. Built in **Rust** (`axum`). Stateless. No clock, no upstream, no account. CDN-friendly.

## Quick start

```bash
cargo run
# → http://127.0.0.1:8787
```

```bash
cargo test
cargo build --release
```

```bash
docker build --build-arg GIT_SHA="$(git rev-parse HEAD)" -t mark .
docker run --rm -p 8787:8787 mark
```

Env (see `.env.example`):

| Variable | Default | Notes |
|----------|---------|--------|
| `PORT` | `8787` | |
| `HOST` | `0.0.0.0` | |
| `PUBLIC_BASE_URL` | derived | Canonical host `https://mark.sylphx.com`; used in docs / generator copy |
| `DEFAULT_CREDIT` | `0` | Opt-in soft `mark` watermark (`credit=1`) |
| `RUST_LOG` | `mark=info` | |

---

## The grammar

**mark = form × art (`type`) × paint (`theme` / `color`) × content (`text` / `desc` / `font`) × geometry (`width` / `height`) × motion (`animation`)**

One endpoint: `GET /api/v1/mark/{form}` — plus the shields-style pill shorthand `GET /badge/{label}-{message}-{color}`.

| Form | What it is | Key params |
|------|-----------|-----------|
| `hero` | The flagship banner (42 art types, 4 layouts) | `type` `text` `desc` `layout` `height` `width` |
| `pill` | Atomic status mark (shields-style) | `label` `message` `style` |
| `strip` | Tech identity row (32 icons) | `icons` `perline` |
| `profile` | Name + tagline card (text-driven) | `text` `desc` `type` (art background) `width` `height` |
| `deploy` | “deployed on Sylphx” conversion pill | `service` `style` |

Shared params on every form: `theme` · `color` · `animation` · `credit` · `font` (`sans` | `mono`).
A `theme` defines the full palette; an explicit `color` is used when no theme is given.
Themes are **neutral design themes** — no personal or company names anywhere in the product.

### Hero

The default hero is the **restrained capsule-class look**: a deep ink canvas
(theme base, never a full-color wash) with the color living only in the layered
gradient waves and text — negative space first. `type=transparent` gives a
fully transparent canvas for typing-line compositions.

```markdown
![header](https://mark.sylphx.com/api/v1/mark/hero?type=wave&color=0:1A1A2E,50:4A90E2,100:D87000&text=Ship%20your%20next%20release&desc=Multi-color%20art%20for%20your%20README&height=220&animation=ambient)
```

**Art types:** `plasma` `holo` `neon` `meteor` `liquid` `prism` `void` `firefly` `silk` `iridescent` `aurora` `mesh` `glass` `soft` `horizon` `dusk` `orbit` `beam` `wave` `waving` `terminal` `constellation` `grid` `blur` `ring` `circuit` `hud` `pulse` `noise` `rounded` `rect` `slice` `cylinder` `checkered` `egg` `shark` `venom` `speech` `product` `oss` `org` `transparent`

**Layouts:** `default` · `plate` (left monogram product cover) · `signal` (centered hero) · `terminal` (left mono systems look)

**Motion (`animation=`):** SMIL (works when the SVG is loaded as `<img>`): `none` · `ambient` (default) · `fade` · `rise` · `scale` · `float` · `glow` · `breathe` · `slide` · `cascade` · `shimmer` · `glitch` · `wave` · `orbit` · `neon` · `bounce` · `type`

**Text:** use `-nl-` for newlines. Optional: `fontSize` `fontColor` `fontAlign` `fontAlignY` `desc*` `rotate` `stroke` `strokeWidth` `textBg` `section=header|footer` `reversal`. `fontColor` and `stroke` accept canonical hex colors only.

### Pill

```markdown
![build](https://mark.sylphx.com/badge/build-passing-brightgreen)
![license](https://mark.sylphx.com/api/v1/mark/pill?label=license&message=MIT&color=blue&style=for-the-badge&theme=github)
```

Styles: `flat` · `plastic` · `for-the-badge` · `social` · `pill`
Colors: shields named colors, semantic names (`success` `important` `critical` `informational` `inactive`), or hex. A `theme` defines the palette and overrides `color`/`labelColor`. Motion applies at text level — a glowing pill is a valid mark.

### Strip

```markdown
![stack](https://mark.sylphx.com/api/v1/mark/strip?icons=rust,ts,docker,kubernetes,postgres&theme=dark)
```

### Profile

```markdown
![profile](https://mark.sylphx.com/api/v1/mark/profile?text=Kyle%20Tse&desc=Infrastructure%20for%20AI%20agents&theme=tokyonight)
![profile-art](https://mark.sylphx.com/api/v1/mark/profile?text=Kyle%20Tse&desc=AI-native%20platforms&type=wave&width=480)
```

The profile card is text-driven: the URL supplies the name (`text`) and tagline (`desc`) — nothing is baked into the product. Retired `identity` URLs render this card (they no longer silently fall back to hero).

### Typing lines (mono)

```markdown
![typing](https://mark.sylphx.com/api/v1/mark/hero?type=transparent&font=mono&animation=type&layout=signal&color=4A90E2&text=MCP%20%26%20AI-agent%20tooling%20-nl-20%20years%20shipping%20at%20scale)
```

### Deploy

```markdown
![deploy](https://mark.sylphx.com/api/v1/mark/deploy?service=mark&style=for-the-badge)
```

---

## The contract

- **Determinism:** same URL, same mark, forever. No clock-sampled fills, no upstream, no state, no secrets. (Retired: `timeAuto`/`timeGradient`, GitHub stats/org/repo cards, all legacy capability routes.)
- **Totality:** rendering never fails. Unknown form → hero, unknown art → `aurora`, invalid colors → fallback paint.
- **CSP + escaping:** SVG responses carry `Content-Security-Policy: script-src 'none'` + `X-Content-Type-Options: nosniff`; every user string is escaped; color-bearing attributes accept only validated hex/named tokens.
- **Cache:** static marks are immutable-by-URL and cache long; animated marks cache short.

## Input limits (public contract)

| Surface | Cap | Behavior |
|---------|-----|----------|
| `text` (hero title / profile name) | 500 chars | truncated with `…` |
| `desc` (hero / profile tagline) | 240 chars / 8 lines | truncated with `…` |
| Pill `label` / `message` | 80 / 120 chars | truncated with `…` |
| Strip icons | 60 | extra icons dropped |
| Deploy `service` | 40 chars | truncated with `…` |
| Hero width / height | 1600 / 900 | clamped |

---

## Why this exists

GitHub already runs on third-party image hosts (capsule-render, readme-stats, skillicons, shields). **Mark** is one Sylphx-owned host with more art, neutral themes, and platform-native deploy marks — every README hit is optional brand surface, and the service itself dogfoods Sylphx. Live data is deliberately not offered: a mark that can never break, go stale, or rate-limit is the moat.

---

## Architecture

One capability, one grammar: see
[`docs/adr/ADR-0003-one-grammar-end-state.md`](docs/adr/ADR-0003-one-grammar-end-state.md)
(and the lineage in ADR-0001 / ADR-0002).

- `src/capabilities/mark/*` — the whole product: domain (spec, catalog, kernel, art, motion) · application (pure renderers) · interfaces (HTTP)
- `src/interfaces/http` — HTTP composition root
- `src/bootstrap.rs` — config + process shell

## License

MIT — see product intent in `PROJECT.md`.

## Delivery authority

[docs/north-star/DELIVERY-AUTHORITY.md](docs/north-star/DELIVERY-AUTHORITY.md)
