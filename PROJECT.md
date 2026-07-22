# Mark

Sylphx **Mark** is an embeddable image API: **URL → SVG** for README banners,
badges, GitHub stats cards, icon rows, brand kits, and deploy badges.

## Lifecycle

- Lifecycle: `active` (internal dogfood → public free promo surface)
- Layer: `product` / acquisition
- Owner org: `SylphxAI`
- Stack: Rust (`axum`), pure SVG (no headless browser)

## Goals

- One host (`img.sylphx.com` target) for all embeddable marks
- High cacheability, stateless render kernel
- Fleet brand themes (Sylphx, Cubeage, Epiow, Ozyrix, Kyle)
- Dogfood Sylphx Platform when deploying the public endpoint
- Soft brand exposure via optional credit watermark + deploy badges

## Non-goals

- Not a full shields.io replacement for every CI vendor
- Not AI image generation on the hot path
- Not a general CDN for arbitrary user uploads (v1)
- Not star-history / contribution time-series analytics (use specialist hosts;
  art kernel stays stateless SVG + optional short-TTL GH snapshots)

## Positioning

**Art + brand embed product.** Beauty is non-negotiable: hierarchy, calm field,
name craft, contrast, crop-honest surfaces. Stats/badges/icons are thin
completeness so users need not leave for a second host — not the product thesis.
Layout families (`layout=plate|signal|terminal`) outrank novelty background types.
Every banner owns a **chromatic system** (base/mid/accent/accent2/warm/glow):
theme drives motif color, not only the field wash. Ambient motion is color-bearing
(gradient drift + motif phase), not monochrome opacity flicker.

## Public surfaces

- HTTP API: `/api/v1/*`, `/badge/*`, `/health`
- Generator UI: `/` (`static/index.html`)
- Repo: https://github.com/SylphxAI/mark

## Delivery

- Ordinary reversible work: roleless direct-trunk to `main`
- Validate: `cargo test` · `cargo build --release`
- Runtime: container or `cargo run` on port `8787`
