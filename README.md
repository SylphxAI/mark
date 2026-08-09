# Sylphx Mark

**Any URL. One image. Your brand.**

The identity layer of the README: embeddable **SVG** marks — hero banners, status pills, tech strips, fleet identity cards, and “deployed on Sylphx” conversion marks — all from **one grammar** and **one URL**, rendered deterministically forever.

Built in **Rust** (`axum`). Stateless. No clock, no upstream, no account. CDN-friendly.

Product host: **`https://mark.sylphx.com`** (sole canonical host)
Platform auto host: `https://mark-web-prod.sylphx.app` (runtime-assigned; not a vanity product URL).

---

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

**mark = form × art (`type`) × paint (`theme` / `color`) × geometry (`width` / `height`) × text × motion (`animation`)**

One endpoint: `GET /api/v1/mark/{form}` — plus the shields-style pill shorthand `GET /badge/{label}-{message}-{color}`.

| Form | What it is | Key params |
|------|-----------|-----------|
| `hero` | The flagship banner (42 art types, 4 layouts) | `type` `text` `desc` `layout` `height` `width` |
| `pill` | Atomic status mark (shields-style) | `label` `message` `style` |
| `strip` | Tech identity row (32 icons) | `icons` `perline` |
| `identity` | Fleet brand card | `brand` `tagline` `type` (art background) `width` |
| `deploy` | “deployed on Sylphx” conversion pill | `service` `style` |

Shared params on every form: `theme` · `color` · `animation` · `credit`.
A `theme` defines the full palette; an explicit `color` is used when no theme is given.

### Hero

```markdown
![header](https://mark.sylphx.com/api/v1/mark/hero?type=wave&theme=sylphx&text=Ship%20your%20next%20release&desc=Multi-color%20art%20for%20your%20README&height=220&animation=ambient)
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
Colors: shields named colors, semantic names (`success` `important` `critical` `informational` `inactive`), fleet names, or hex. A `theme` defines the palette and overrides `color`/`labelColor`. Motion applies at text level — a glowing pill is a valid mark.

### Strip

```markdown
![stack](https://mark.sylphx.com/api/v1/mark/strip?icons=rust,ts,docker,kubernetes,postgres&theme=dark)
```

### Identity

```markdown
![brand](https://mark.sylphx.com/api/v1/mark/identity?brand=sylphx)
![brand-art](https://mark.sylphx.com/api/v1/mark/identity?brand=cubeage&type=aurora&theme=neon&width=480)
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
| Hero `text` | 500 chars | truncated with `…` |
| Hero `desc` | 240 chars / 8 lines | truncated with `…` |
| Pill `label` / `message` | 80 / 120 chars | truncated with `…` |
| Strip icons | 60 | extra icons dropped |
| Identity `brand` / `tagline` | 40 / 120 chars | truncated with `…` |
| Deploy `service` | 40 chars | truncated with `…` |
| Hero width / height | 1600 / 900 | clamped |

---

## Why this exists

GitHub already runs on third-party image hosts (capsule-render, readme-stats, skillicons, shields). **Mark** is one Sylphx-owned host with more art, fleet themes, and platform-native deploy marks — every README hit is optional brand surface, and the service itself dogfoods Sylphx. Live data is deliberately not offered: a mark that can never break, go stale, or rate-limit is the moat.

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
