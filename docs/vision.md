# Mark Vision

**Status:** Canonical product destination
**Identity graph:** [`capabilities.md`](capabilities.md)
**Binding ADRs:** [`adr/ADR-0002-clean-break-end-state.md`](adr/ADR-0002-clean-break-end-state.md), [`adr/ADR-0003-one-grammar-end-state.md`](adr/ADR-0003-one-grammar-end-state.md), [`adr/ADR-0004-neutral-ux-redesign.md`](adr/ADR-0004-neutral-ux-redesign.md)
**Delivery authority:** [`north-star/DELIVERY-AUTHORITY.md`](north-star/DELIVERY-AUTHORITY.md)

This document owns the long-term product destination. It does not claim the destination is landed or live.

## Destination

**Any URL. One image. Your brand.** The identity layer of the README: embeddable SVG marks — hero banners, status pills, tech strips, fleet identity cards, and `deployed on Sylphx` conversion marks — all from one grammar and one URL, rendered deterministically forever. Built in Rust (`axum`), stateless, no clock, no upstream, no account, CDN-friendly. Canonical host `https://mark.sylphx.com`; runtime Platform host is not a vanity URL.

Grammar: `mark = form × art (type) × paint (theme/color) × content (text/desc/font) × geometry (width/height) × motion (animation)` via `GET /api/v1/mark/{form}` plus shields-style `GET /badge/{label}-{message}-{color}`.

## Users and their jobs

- **README authors** who need one deterministic URL to render a brand-correct SVG that never breaks.
- **Fleet operators** who need fleet identity cards and deployment marks from the same grammar without live-account coupling.

## Not doing

- Live GitHub stats/clock/upstream as a core render product (named focus decision; may not return as silent dependency).
- A second render authority or grammar.
- Personal or company names in theme definitions — themes are neutral.

## Product oracle

The destination is true only when a customer can `GET /api/v1/mark/{form}` (or `/badge/...`) with form+art+paint+content+geometry+motion and receive a deterministic, XSS-safe SVG at the live layer on `https://mark.sylphx.com`, with canonical hex-only paint tokens and pure render without clock/upstream, at the live layer.

`cargo test` green is not the live fetch oracle.

## Clients (company dest)

Consume owner ADR-038. This product calls peer public APIs with those
products' credentials and their generated Rust or TypeScript SDKs. It
does not implement Backend-as-a-Service, compile a mega-client, or use
`{project}.api.sylphx.com` as dest.

