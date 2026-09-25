# Mark

Mark is the dependable image language for software identity: a maintainer expresses a complete mark in one public URL, embeds that URL in a README, site, or product surface, and receives a polished SVG without an account, asset build, or data dependency.

- Ordinary: https://mark.sylphx.com — product-declared customer host (`docs/vision.md` shipped terminal, `PUBLIC_BASE_URL`, `sylphx.toml`). A `200` is not the product contract.
- Preview: `none` — this repository has no current honest preview URL. GitHub Pages and deployments are absent. `https://mark-web-prod.sylphx.app` is a stale platform auto host (404), not a preview.
- Vision: [`docs/vision.md`](docs/vision.md)
- Capabilities: [`docs/capabilities.md`](docs/capabilities.md)

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

**mark = form × art (`type`) × paint (`theme` / `color`, pill `labelColor`) × content (`text` / `desc` / `font`) × geometry (`width` / `height`, hero `layout`) × motion (`animation`)**

One endpoint: `GET /api/v1/mark/{form}` — plus the shields static badge dialect `GET /badge/{label}-{message}-{color}` and `GET /static/v1`. Every image route also answers with `.svg` appended to its path (edge-cacheable by extension).

| Form | What it is | Key params |
|------|-----------|-----------|
| `hero` | The flagship banner (42 art types, 4 layouts) | `type` `text` `desc` `layout` `height` `width` |
| `pill` | Atomic status mark (shields-style) | `label` `message` `style` `labelColor` |
| `strip` | Tech identity row (any brand icon) | `icons` `perline` |
| `profile` | Name + tagline card (text-driven) | `text` `desc` `type` (art background) `width` `height` |
| `deploy` | “deployed on Sylphx” conversion pill | `service` `style` |
| `score` | Graded score pill with a progress ring | `label` `value` `max` `style` |

Shared params on every form: `theme` · `color` · `animation` · `credit` · `font` (`sans` | `mono`).
Anything outside this grammar is unknown input, never a second vocabulary: the
retired predecessor knobs (`fontSize`, `descSize`, `fontColor`, align/rotate/
`stroke`/`strokeWidth`, `textBg`, `section`, `reversal`) and retired ids/aliases
(`badge`, `icons`, `card`, `deploymark`, layout aliases) are ignored.
A theme pack defines the full palette; an explicit `color` is used when there is no theme pack. An unknown theme name is not a theme pack.
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

**Text:** use `-nl-` for newlines.

### Pill (shields-compatible badges)

Any img.shields.io static badge works by changing only the host — same
syntax, same geometry, same widths:

```markdown
![build](https://mark.sylphx.com/badge/build-passing-brightgreen)
![build-fat](https://mark.sylphx.com/badge/build-passing-brightgreen?style=for-the-badge)
![escaped](https://mark.sylphx.com/badge/agent--ready-92%2F100-brightgreen.svg)
![legacy](https://mark.sylphx.com/static/v1?label=license&message=MIT&color=blueviolet)
![license](https://mark.sylphx.com/api/v1/mark/pill?label=license&message=MIT&color=blue&style=for-the-badge&theme=github)
```

- Path: `label-message-color` or `message-color`; `--` → `-`, `__` → `_`, `_` or `%20` → space.
- Query: `label` and `color` override the path; `labelColor`, `style`, `logo` (base64 `data:image/svg+xml`/`png` URI), `logoWidth`; `cacheSeconds` and `link` are accepted and ignored.
- Styles: `flat` · `flat-square` · `plastic` · `for-the-badge` · `social` · `pill`
- Colors: shields names (`brightgreen` … `lightgrey`, `success` `important` `critical` `informational` `inactive`), 3/6-digit hex with or without `#`, CSS color names, `rgb()`/`hsl()`. A theme pack defines the palette and overrides `color`/`labelColor`. Motion applies at text level.

### Score

```markdown
![agent-ready](https://mark.sylphx.com/api/v1/mark/score.svg?label=agent-ready&value=92)
![quality](https://mark.sylphx.com/api/v1/mark/score.svg?label=quality&value=7.5&max=10&style=for-the-badge)
```

A progress ring plus the value; the color grades from `value / max` (below 50%
red, 70% orange, 80% yellow, 90% green, then bright green) unless `color` or
`theme` is set.

### Strip

```markdown
![stack](https://mark.sylphx.com/api/v1/mark/strip?icons=rust,ts,docker,kubernetes,postgres&theme=dark)
```

`icons` takes any [Simple Icons](https://simpleicons.org) slug or title
(`fastify`, `Node.js`, `bun`), plus every skill-icons id.

### Icon tiles (skill-icons compatible)

```markdown
![skills](https://mark.sylphx.com/icons?i=js,ts,rust,go,react,docker,k8s,postgres)
![skills](https://mark.sylphx.com/icons?i=py,pytorch,fastapi,redis&theme=light&perline=4)
```

A drop-in for `skillicons.dev`: swap the host and keep the URL. `i` (or
`icons`) takes skill-icons ids or any Simple Icons slug; `theme` (or `t`) is
`dark` (default) or `light`; `perline` is 1–50 (default 15); `i=all` renders
the whole skill-icons set. Unknown ids are skipped.

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

### Capsule banners (capsule-render compatible)

```markdown
![header](https://mark.sylphx.com/api?type=waving&color=gradient&height=300&section=header&text=Hello&fontSize=90&fontAlignY=38)
![footer](https://mark.sylphx.com/api?type=waving&color=gradient&height=100&section=footer)
```

A drop-in for `capsule-render.vercel.app`: swap the host and keep the URL.
All capsule-render types (`wave`, `waving`, `egg`, `shark`, `slice`, `rect`,
`soft`, `rounded`, `cylinder`, `venom`, `speech`, `transparent`, `blur`,
`pulse`, `checkered`), colors (hex, `0:EEFF00,100:a82da8`, `gradient`,
`auto`, `random`, `timeAuto`, `timeGradient`, `customColorList`, `theme`), and
typography knobs (`fontSize`, `fontColor`, `fontAlign`, `fontAlignY`,
`fontFamily`, `desc`, `descSize`, `descAlign`, `descAlignY`, `rotate`,
`stroke`, `strokeWidth`, `textBg`, `section`, `reversal`) keep their upstream
defaults. Animations (`fadeIn`, `scaleIn`, `blink`, `blinking`, `twinkling`)
are SMIL, so they run inside GitHub's image proxy. Colors that upstream picks
at random or by the clock are picked from a hash of the URL instead: one URL,
one image. These knobs exist only in this dialect; the native grammar
(`/api/v1/mark/hero`) does not read them.

### Typing SVG (readme-typing-svg compatible)

```markdown
![typing](https://mark.sylphx.com/?lines=First+line;Second+line&font=Fira+Code&center=true&width=435&height=50&color=36BCF7&vCenter=true&pause=1000&size=20)
![typing](https://mark.sylphx.com/typing?lines=Hello;World&multiline=true&height=80)
```

A drop-in for `readme-typing-svg.demolab.com`: swap the host and keep the
URL (`/?lines=` or `/typing?lines=`). Every upstream parameter works with its
upstream default: `lines`, `separator`, `font`, `weight`, `size`, `color`,
`background`, `center`, `vCenter`, `multiline`, `width`, `height`,
`duration`, `pause`, `repeat`, `random`, `letterSpacing`. Mark never fetches
webfonts: `font` is named first, then a system fallback stack (monospace for
code fonts). `random=true` is deterministic: the order is derived from a hash
of the URL, so one URL always renders one order. The native form is
`/api/v1/mark/typing` (`text` works in place of `lines`).

### Deploy

```markdown
![deploy](https://mark.sylphx.com/api/v1/mark/deploy?service=mark&style=for-the-badge)
```

---

## The contract

- **Determinism:** same URL, same mark, forever. No clock-sampled fills, no upstream, no state, no secrets. (Retired: `timeAuto`/`timeGradient`, GitHub stats/org/repo cards, all legacy capability routes.)
- **Totality:** rendering never fails. Unknown form → hero, unknown art → `waving` (the shipped default), invalid colors → fallback paint, unknown theme/layout/animation → the documented default.
- **CSP + escaping:** SVG responses carry `Content-Security-Policy: script-src 'none'` + `X-Content-Type-Options: nosniff`; every user string is escaped; color-bearing attributes accept only validated hex/named tokens.
- **Cache:** every mark URL pins its bytes (pure function of the URL, including SMIL-animated variants) and caches long as immutable (`max-age=31536000, s-maxage=31536000, immutable` + `ETag` + `CDN-Cache-Control`/`Cloudflare-CDN-Cache-Control`); conditional `If-None-Match` returns `304`. Origin headers are this product's write. Live edge `HIT` on dest extensionless `/api/v1/mark*` + `/badge/*` is Apps (Cloudflare for SaaS Custom Hostname + grey CNAME to `cname.sylphx.com`, plus Cache Everything / eligible-for-cache keyed on the full query string). Hands is generic kube origin only.

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

One capability, one grammar.

- `src/capabilities/mark/*` — the whole product: domain (spec, catalog, kernel, art, motion) · application (pure renderers) · interfaces (HTTP)
- `src/interfaces/http` — HTTP composition root
- `src/bootstrap.rs` — config + process shell

## License

MIT — see product intent in `PROJECT.md`.

Brand icons come from [Simple Icons](https://github.com/simple-icons/simple-icons)
(path data CC0-1.0; version in `data/simple-icons.tsv`, refreshed by
`scripts/update-simple-icons.py`). Brand names and logos are trademarks of
their owners; showing one does not imply endorsement. See the Simple Icons
[disclaimer](https://github.com/simple-icons/simple-icons/blob/develop/DISCLAIMER.md).

## Destination

Product destination: [`docs/vision.md`](docs/vision.md). Identity graph: [`docs/capabilities.md`](docs/capabilities.md). Historical north-star notes remain under [`docs/north-star/`](docs/north-star/) and are not dest.
