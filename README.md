# Sylphx Mark

**Any URL. One image. Your brand.**

Embeddable **SVG** marks for GitHub READMEs and docs — banners, badges, stats cards, icon rows, brand kits, and “deployed on Sylphx” pills.

Built in **Rust** (`axum`). Stateless. CDN-friendly. Designed as a Sylphx dogfood + brand surface.

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
| `GITHUB_TOKEN` | empty | Higher rate limits for `/stats` `/repo` `/org` |
| `RUST_LOG` | `mark=info` | |

---

## Endpoints (canonical surface, ADR-0002)

| Path | Purpose |
|------|---------|
| `GET /api/v1/banner` | Hero / header / footer banners (42 styles) |
| `GET /api/v1/badge` · `GET /badge/{label}-{message}-{color}` | Shields-style badges |
| `GET /api/v1/stats/{user}` | GitHub user stats card |
| `GET /api/v1/org/{org}` | Org aggregate card |
| `GET /api/v1/repo/{owner}/{repo}` | Single repo card |
| `GET /api/v1/icons?i=rust,ts,k8s` | Tech icon row |
| `GET /api/v1/brand/{name}` | Fleet brand kit card |
| `GET /api/v1/deploy` | `deployed on Sylphx` badge |
| `GET /api/v1/catalog` | Types, themes, icons, limits JSON |
| `GET /health` | Liveness (includes `revision`) |
| `GET /` | Generator UI |

Legacy bare aliases (`/banner`, `/stats/...`, `/org/...`, `/repo/...`, `/icons`, `/brand/...`, `/deploy`) were removed in the clean-break end state.

### Banner

```markdown
![header](https://mark.sylphx.com/api/v1/banner?type=wave&color=7C3AED,00F5D4,F15BB5&text=Ship%20your%20next%20release&desc=Multi-color%20art%20for%20your%20README&height=220&animation=ambient)
```

**Types:**  
`plasma` `holo` `neon` `meteor` `liquid` `prism` `void` `firefly` `silk` `iridescent` `aurora` `mesh` `glass` `soft` `horizon` `dusk` `orbit` `beam` `wave` `waving` `terminal` `constellation` `grid` `blur` `ring` `circuit` `hud` `pulse` `noise` `rounded` `rect` `slice` `cylinder` `checkered` `egg` `shark` `venom` `speech` `product` `oss` `org` `transparent`

**Motion (`animation=`):** SMIL (works when the SVG is loaded as `<img>`):  
`none` · `ambient` (default) · `fade` · `rise` · `scale` · `float` · `glow` · `breathe` · `slide` · `cascade` · `shimmer` · `glitch` · `wave` · `orbit` · `neon` · `bounce` · `type`  
Every style has ambient background motion when motion ≠ `none`.

**Themes:**  
`dark` `light` `tokyonight` `dracula` `nord` `neon` `ocean` `sunset` `forest` `github` `radical` `gruvbox` `monokai` · fleet kits: `sylphx` `cubeage` `epiow` `ozyrix` `kyle`

**Color:** `auto` · `timeAuto` · `gradient` · `timeGradient` · hex · `0:EEFF00,100:a82da8`  
A `theme` sets the full palette; an explicit `color` is used when no theme is given.

**Text:** use `-nl-` for newlines. Optional: `fontSize` `fontColor` `fontAlign` `fontAlignY` `desc*` `rotate` `stroke` `strokeWidth` `textBg` `animation` `section=header|footer` `reversal` `credit=0|1` `layout=default|plate|signal|terminal`  
`fontColor` and `stroke` accept canonical hex colors only (`#rgb` / `#rrggbb` / `#rrggbbaa`).

### Badge

```markdown
![build](https://mark.sylphx.com/badge/build-passing-brightgreen)
![license](https://mark.sylphx.com/api/v1/badge?label=license&message=MIT&color=blue&style=for-the-badge)
```

Styles: `flat` · `plastic` · `for-the-badge` · `social` · `pill`  
Colors: shields named colors (`brightgreen` `green` `yellow` `yellowgreen` `orange` `red` `blue` `lightgrey` …), semantic names (`success` `important` `critical` `informational` `inactive`), fleet names (`sylphx` `cubeage` `epiow` `ozyrix`), or hex. A `theme` defines the full palette and overrides `color`/`labelColor`.

### Stats / repo / org

```markdown
![stats](https://mark.sylphx.com/api/v1/stats/shtse8?theme=sylphx)
![org](https://mark.sylphx.com/api/v1/org/SylphxAI?theme=dark)
![repo](https://mark.sylphx.com/api/v1/repo/SylphxAI/mark?theme=github)
```

GitHub upstream snapshots are short-TTL cached (300 s positive, 45 s negative). On upstream failure the endpoint renders a visible error card — it never renders a zero-data card.

### Icons

```markdown
![stack](https://mark.sylphx.com/api/v1/icons?i=rust,ts,docker,kubernetes,postgres&theme=dark)
```

### Brand + deploy (promotion)

```markdown
![brand](https://mark.sylphx.com/api/v1/brand/sylphx)
![deploy](https://mark.sylphx.com/api/v1/deploy?service=mark&style=for-the-badge)
```

---

## Input limits (public contract)

| Surface | Cap | Behavior |
|---------|-----|----------|
| Banner `text` | 500 chars | truncated with `…` |
| Banner `desc` | 240 chars | truncated with `…` |
| Banner lines | 8 | extra lines dropped |
| Badge `label` | 80 chars | truncated with `…` |
| Badge `message` | 120 chars | truncated with `…` |
| Icon row | 60 icons | extra icons dropped |
| Banner width / height | 1600 / 900 | clamped |

SVG responses carry `Content-Security-Policy: script-src 'none'` and `X-Content-Type-Options: nosniff`; every user-controlled string is escaped, and color-bearing attributes accept only validated hex/named tokens.

---

## Why this exists

GitHub already runs on third-party image hosts (capsule-render, readme-stats, skillicons, shields).  
**Mark** is one Sylphx-owned host with more styles, fleet themes, and platform-native deploy badges — every README hit is optional brand surface, and the service itself dogfoods Sylphx.

---

## Architecture

Capability-first Modular DDD (single crate, module boundaries): see
[`docs/adr/ADR-0001-capability-first-architecture.md`](docs/adr/ADR-0001-capability-first-architecture.md)
and the clean-break end state in
[`docs/adr/ADR-0002-clean-break-end-state.md`](docs/adr/ADR-0002-clean-break-end-state.md).

- `src/capabilities/*` — product outcomes (banner, badge, github_card, …)
- `src/shared/*` — pure color/theme/svg kernel
- `src/interfaces/http` — HTTP composition root
- `src/bootstrap.rs` — config + process shell

## License

MIT — see product intent in `PROJECT.md`.
