# Mark

Sylphx **Mark** is the identity layer of the README: an embeddable image API —
**URL → SVG** — where every mark is a pure function of its URL.

## Lifecycle

- Lifecycle: `active` (internal dogfood → public free promo surface)
- Layer: `product` / acquisition
- Owner org: `SylphxAI`
- Stack: Rust (`axum`), pure SVG (no headless browser)

## Goals

- One host (`mark.sylphx.com`) for all embeddable marks
- One grammar: form (`hero` `pill` `strip` `profile` `deploy`) × art × paint ×
  content × geometry × motion
- Neutral design themes only — no personal names, no company names in the
  public catalog (ADR-0004); content is always supplied by the URL
- High cacheability, stateless, deterministic render kernel
- Dogfood Sylphx Platform when deploying the public endpoint
- Soft brand exposure via optional credit watermark + deploy marks

## Non-goals

- **Not a data host:** no live GitHub stats/org/repo cards, no upstream
  network adapter, no tokens, no caches of remote state. Same URL renders the
  same mark forever. Use specialist hosts for live data.
- **Not time-dependent:** no clock-sampled fills (`timeAuto`/`timeGradient`
  are retired). Determinism is the reliability moat.
- Not a full shields.io replacement for every CI vendor
- Not AI image generation on the hot path
- Not a general CDN for arbitrary user uploads
- Not star-history / contribution time-series analytics

## Positioning

**Art + brand embed product.** Beauty is non-negotiable: hierarchy, calm field,
name craft, contrast, crop-honest surfaces. The mark is the first sentence of
your README; every render is optional Sylphx brand surface. Composition is the
product: any form × any palette × any layout × any motion × any size, from one
URL. Studio is **palette-first** (2–3 color fields, Surprise me, session
inspiration). Every mark owns a **chromatic system**
(base/mid/accent/accent2/warm/glow): theme drives motif color, not only the
field wash. Ambient motion is color-bearing (gradient drift + motif phase).

## Public surfaces

- HTTP API: `/api/v1/mark/{form}` · `/badge/{label}-{message}-{color}` · `/api/v1/catalog` · `/health`
- Generator UI: `/` (`static/index.html`)
- Repo: https://github.com/SylphxAI/mark

## Architecture

- Binding shape: one capability (`mark`), one grammar (ADR-0003)
- Decision: `docs/adr/ADR-0001-capability-first-architecture.md` →
  `docs/adr/ADR-0003-one-grammar-end-state.md`
- Code roots: `src/capabilities/mark/*` (domain + application + interfaces),
  `src/interfaces/*`, `src/bootstrap.rs`
- Default semantic unit: Rust module (single crate)

## Delivery

- Ordinary reversible work: roleless direct-trunk to `main`
- Validate: `cargo test` · `cargo build --release`
- Runtime: container or `cargo run` on port `8787`
