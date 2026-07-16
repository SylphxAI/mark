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

## Public surfaces

- HTTP API: `/api/v1/*`, `/badge/*`, `/health`
- Generator UI: `/` (`static/index.html`)
- Repo: https://github.com/SylphxAI/mark

## Delivery

- Ordinary reversible work: roleless direct-trunk to `main`
- Validate: `cargo test` · `cargo build --release`
- Runtime: container or `cargo run` on port `8787`
