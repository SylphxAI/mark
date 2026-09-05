# Mark Vision

**Status:** Canonical product destination
**Identity graph:** [`capabilities.md`](capabilities.md)
**Historical decisions:** [`adr/ADR-0002-clean-break-end-state.md`](adr/ADR-0002-clean-break-end-state.md), [`adr/ADR-0003-one-grammar-end-state.md`](adr/ADR-0003-one-grammar-end-state.md), [`adr/ADR-0004-neutral-ux-redesign.md`](adr/ADR-0004-neutral-ux-redesign.md). Those files are history. They do not own identity or fate.

This document owns the finished-product destination with [`capabilities.md`](capabilities.md). It does not claim that destination is landed or live. [`north-star/DELIVERY-AUTHORITY.md`](north-star/DELIVERY-AUTHORITY.md) is historical and is not dest.

## Destination

**Any URL. One image. Your brand.** Mark is the README image language: a stranger writes one public URL and receives a polished, deterministic SVG. Five forms — hero banners, status pills, tech strips, text-driven profile cards, and `deployed on Sylphx` conversion marks — plus the shields-style pill shorthand, from one grammar. Built in Rust (`axum`). Stateless. No clock, no upstream, no account. Soft `credit` watermark is opt-in.

Grammar: `mark = form × art (type) × paint (theme/color, pill labelColor) × content (text/desc/font) × geometry (width/height, hero layout) × motion (animation)` via `GET /api/v1/mark/{form}` plus `GET /badge/{label}-{message}-{color}`. Pill `labelColor` is dest paint when no theme pack; an unknown theme name is not a theme pack. Unknown form renders hero, except retired `identity` URLs, which are the profile card.

Canonical customer host is `https://mark.sylphx.com`. Mark is an ordinary Apps Service. The runtime auto host `mark-web-prod.sylphx.app` is not dest.

The studio at `GET /` is the no-account composer of that grammar: preview the SVG, copy the public URL, copy the README markdown embed, download the SVG, and recover dest composer state from a public mark URL — including pill `labelColor` when no theme pack. The studio page uses system font stacks only. Noscript still offers grammar links.

## For whom

- **README authors** who need one deterministic URL that embeds as `![alt](url)` and never breaks.
- **People who need an embeddable identity mark** — a name and tagline supplied by the URL, not a baked fleet card.

## Not doing

- Live GitHub stats, clock, or upstream as product authority — including the studio composer. Reintroduction needs an explicit capability and a network contract, not a silent dependency.
- A second render authority or grammar. Predecessor capsule-render typography and placement knobs are leftover compatibility, not dest.
- PNG, upload, AI generation on the hot path, accounts, or saved marks.
- Personal or company names in theme or icon ids. The catalog is neutral. The only Sylphx-branded surfaces are the deploy mark and the opt-in credit watermark.
- A Mark-owned production Release writer. Apps owns production Release. Hands is generic kube origin only.
- Generated SDK consumption on the render path. Render is URL in, SVG out.

## Product oracle

The destination is true only when a stranger `GET https://mark.sylphx.com/api/v1/mark/{form}` (or `/badge/...`) with form+art+paint+content+geometry+motion receives a deterministic, XSS-safe SVG at the live layer, with canonical hex-only paint tokens and no clock, upstream, or account.

`cargo test` green is not that oracle. `GET /health` 200 is not that oracle. Naming the locator is not live-success.

## Hosting

Mark is an ordinary Apps customer: this repository owns `sylphx.toml` desired spec. Apps admits production Release. Hands realizes generic kube origin. Origin cache headers are this product's write. Live edge `HIT` is Apps (SaaS Custom Hostname plus a cache rule keyed on the full query).

## Clients (company dest)

Consume owner ADR-038's fence: this product does not implement Backend-as-a-Service, compile a mega-client, or use `{project}.api.sylphx.com` as dest. The render contract consumes no peer APIs and no generated SDKs — one public URL in, one SVG out. Hosting uses Apps as a customer, not as a second render writer.
