# Sylphx Mark

**Any URL. One image. Your brand.**

Embeddable **SVG** marks for GitHub READMEs and docs — banners, badges, stats cards, icon rows, brand kits, and “deployed on Sylphx” pills.

Built in **Rust** (`axum`). Stateless. CDN-friendly. Designed as a Sylphx dogfood + brand surface.

Product host: **`https://mark.sylphx.com`**  
Platform auto host: **`https://mark-web-prod.sylphx.app`** (runtime-assigned; not a vanity product URL).

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
docker build -t mark .
docker run --rm -p 8787:8787 mark
```

Env (see `.env.example`):

| Variable | Default | Notes |
|----------|---------|--------|
| `PORT` | `8787` | |
| `HOST` | `0.0.0.0` | |
| `PUBLIC_BASE_URL` | derived | Used in docs / generator copy |
| `DEFAULT_CREDIT` | `0` | Opt-in soft `mark` watermark (`credit=1`) |
| `GITHUB_TOKEN` | empty | Higher rate limits for `/stats` `/repo` `/org` |
| `RUST_LOG` | `mark=info` | |

---

## Endpoints

| Path | Purpose |
|------|---------|
| `GET /api/v1/banner` | Hero / header / footer banners (30 styles) |
| `GET /api/v1/badge` · `GET /badge/...` | Shields-style badges |
| `GET /api/v1/stats/{user}` | GitHub user stats card |
| `GET /api/v1/org/{org}` | Org aggregate card |
| `GET /api/v1/repo/{owner}/{repo}` | Single repo card |
| `GET /api/v1/icons?i=rust,ts,k8s` | Tech icon row |
| `GET /api/v1/brand/{name}` | Fleet brand kit card |
| `GET /api/v1/deploy` | `deployed on Sylphx` badge |
| `GET /api/v1/catalog` | Types, themes, icons JSON |
| `GET /health` | Liveness |
| `GET /` | Generator UI |

### Banner

```markdown
![header](https://mark.sylphx.com/api/v1/banner?type=wave&color=7C3AED,00F5D4,F15BB5&text=Ship%20your%20next%20release&desc=Multi-color%20art%20for%20your%20README&height=220&animation=ambient)
```

**Types (v1):**  
`wave` `waving` `soft` `rounded` `rect` `slice` `cylinder` `blur` `pulse` `checkered` `egg` `shark` `venom` `speech` `transparent` `aurora` `mesh` `noise` `glass` `grid` `constellation` `terminal` `hud` `circuit` `orbit` `ring` `beam` `product` `oss` `org`

**Motion (`animation=`):** SMIL (works when the SVG is loaded as `<img>`):  
`none` · `ambient` (default) · `fade` · `rise` · `scale` · `float` · `glow` · `breathe` · `slide` · `cascade` · `shimmer` · `glitch` · `wave` · `orbit`  
Every style has ambient background motion when motion ≠ `none`.

**Themes:**  
`dark` `light` `tokyonight` `dracula` `nord` `neon` `ocean` `sunset` `forest` `github` `radical` · fleet kits: `sylphx` `cubeage` `epiow` `ozyrix` `kyle`

**Color:** `auto` · `timeAuto` · `gradient` · `timeGradient` · hex · `0:EEFF00,100:a82da8`

**Text:** use `-nl-` for newlines. Optional: `fontSize` `fontColor` `fontAlign` `fontAlignY` `desc*` `rotate` `stroke` `strokeWidth` `textBg` `animation` `section=header|footer` `reversal` `credit=0|1`

### Badge

```markdown
![build](https://img.sylphx.com/badge/build-passing-brightgreen)
![license](https://img.sylphx.com/api/v1/badge?label=license&message=MIT&color=blue&style=for-the-badge)
```

Styles: `flat` · `plastic` · `for-the-badge` · `social` · `pill`

### Stats / repo / org

```markdown
![stats](https://img.sylphx.com/api/v1/stats/shtse8?theme=sylphx)
![org](https://img.sylphx.com/api/v1/org/SylphxAI?theme=dark)
![repo](https://img.sylphx.com/api/v1/repo/SylphxAI/mark?theme=github)
```

### Icons

```markdown
![stack](https://img.sylphx.com/api/v1/icons?i=rust,ts,docker,kubernetes,postgres&theme=dark)
```

### Brand + deploy (promotion)

```markdown
![brand](https://img.sylphx.com/api/v1/brand/sylphx)
![deploy](https://img.sylphx.com/api/v1/deploy?service=mark&style=for-the-badge)
```

---

## Why this exists

GitHub already runs on third-party image hosts (capsule-render, readme-stats, skillicons, shields).  
**Mark** is one Sylphx-owned host with more styles, fleet themes, and platform-native deploy badges — every README hit is optional brand surface, and the service itself dogfoods Sylphx.

---

## Architecture

Capability-first Modular DDD (single crate, module boundaries): see
[`docs/adr/ADR-0001-capability-first-architecture.md`](docs/adr/ADR-0001-capability-first-architecture.md).

- `src/capabilities/*` — product outcomes (banner, badge, github_card, …)
- `src/shared/*` — pure color/theme/svg kernel
- `src/interfaces/http` — HTTP composition root
- `src/bootstrap.rs` — config + process shell

## License

MIT — see product intent in `PROJECT.md`.
