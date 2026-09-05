# Mark identity graph

**Status:** Identity registry. Not live proof.
**Scope:** Mark — stateless SVG marks from one grammar.
**Cite:** the **ID** column.

This file is the identity graph. Destination stays in [`vision.md`](vision.md). This file and that file own identity, fate, dependency, and oracle. Historical ADRs and [`north-star/DELIVERY-AUTHORITY.md`](north-star/DELIVERY-AUTHORITY.md) do not add identities. If they conflict with this graph on identity or fate, they are leftover.

```text
ID | Identity | Fate | Depends on | Done when
```

One colloquial name has one row and one fate (`live`, `dead`, or `rename-to:<ID>`). **Depends on** is a truth prerequisite, not a work-queue order. **Done when** is an oracle. This file does not claim the oracle is already true.

The render contract consumes no peer APIs. Mega-clients and `{project}.api.sylphx.com` are not dest. Mark is an ordinary Apps Service; Hands is generic kube origin only.

## Graph

| ID | Identity | Fate | Depends on | Done when |
| --- | --- | --- | --- | --- |
| MARK-GRAMMAR | One-grammar stateless render | live | — | Live `GET https://mark.sylphx.com/api/v1/mark/{form}` for `form∈{hero,pill,strip,profile,deploy}` with dest grammar `type`/`theme`/`color`/pill `labelColor`/`text`/`desc`/`font`/`width`/`height`/hero `layout`/`animation` returns a deterministic SVG; no clock, upstream, or account. Pill dest paint includes `labelColor` when no theme pack. An unknown theme name is not a theme pack. Unknown form renders hero, except `identity` → profile. Predecessor capsule-render typography and placement knobs are leftover, not dest. |
| MARK-BADGE | Shields-style pill shorthand | live | MARK-GRAMMAR | Live `GET /badge/{label}-{message}-{color}` is the same pill as `/api/v1/mark/pill` with those tokens. Query `style`, `theme`, `animation`, `labelColor`, `font`, and `credit` compose the same way as `/pill`. |
| MARK-SVG | Valid SVG + XSS-safe paint | live | MARK-GRAMMAR | Live SVG is well-formed, user text is escaped, non-canonical paint falls back, and responses carry `Content-Security-Policy: script-src 'none'` plus `X-Content-Type-Options: nosniff`. |
| MARK-HOST | Canonical customer host | live | MARK-GRAMMAR | Ordinary URL is `https://mark.sylphx.com` and a grammar GET there returns the product SVG. The Apps auto host `mark-web-prod.sylphx.app` is not dest. Naming the locator is not live-success. |
| MARK-CDN | Immutable URL cache | live | MARK-GRAMMAR, MARK-HOST | Live grammar/badge GET returns origin `Cache-Control: public, max-age=31536000, s-maxage=31536000, immutable` plus a strong ETag, `If-None-Match` → 304, and `CDN-Cache-Control` / `Cloudflare-CDN-Cache-Control`. Origin headers are this product's write. Edge `HIT` is Apps (SaaS Custom Hostname + cache rule keyed on the full query), not this identity. Hands is generic kube origin only. |
| MARK-CATALOG | Public vocabulary | live | MARK-GRAMMAR | Live `GET /api/v1/catalog` publishes forms, art, layouts, themes, icons, fonts, and limits that the live render honors, plus `notes.grammar` that matches dest grammar (`mark = form × art (type) × paint (theme/color, pill labelColor) × content (text/desc/font) × geometry (width/height, hero layout) × motion (animation)`). Theme and icon ids contain no personal or company names. |
| MARK-STUDIO | URL composer | live | MARK-GRAMMAR, MARK-CATALOG | Live `GET /` is the no-account composer: catalog-backed SVG preview, copy of the public mark URL, copy of the README markdown embed `![alt](url)`, and download of that SVG. Loading the studio with a public mark URL recovers dest composer state, including pill `labelColor` when no theme pack. The studio page uses system font stacks only — no webfont origin. Noscript still offers grammar links. |
| MARK-PROFILE | Text-driven profile card | live | MARK-GRAMMAR | Live `/api/v1/mark/profile?text&desc` renders name and tagline from the URL. Retired `identity` URLs map here, not silent hero. |
| MARK-DEPLOY | Conversion mark | live | MARK-GRAMMAR | Live `/api/v1/mark/deploy?service=…` renders the “deployed on Sylphx” pill. |
| MARK-STATS | Live GitHub stats / clock / upstream as product authority | dead | — | Must not return as a silent dependency. Reintroduction needs an explicit capability and a network contract. |
| MARK-IDENTITY | Fleet identity form | rename-to:MARK-PROFILE | — | Not a second product. Successor mapping is dest. |

## Release boundary (GOV-017)

Company ADR-030 consequence (Owner runbook GOVERNANCE-AUDIT-2026-08-28,
row GOV-017): every Active product declares its public probe, owned
manifest/migration writers, consumed receipts, runtime effects, and
forbidden writes. Declared from this graph and this repository's docs;
not live proof. Facts not establishable here are `Unknown`, never green.

- **Public probe:** `GET https://mark.sylphx.com/api/v1/mark/{form}`
  (or `/badge/{label}-{message}-{color}`) with
  form+art+paint+content+geometry+motion returns a deterministic,
  XSS-safe SVG with canonical hex-only paint tokens and no clock,
  upstream, or account (`MARK-GRAMMAR`, `MARK-SVG`, `MARK-HOST`). That
  anonymous fetch is the cheapest customer-visible falsifier. `cargo
  test` green and `GET /health` 200 are not this probe (vision,
  README). Naming the locator is not a live-success claim.
- **Owned manifest/migration writers:** this repository owns
  `sylphx.toml` (dockerfile `web`, `path_prefixes`, health `/health`,
  `PUBLIC_BASE_URL`) as a customer desired spec. Mark is stateless and
  owns no database-schema migration writer. It owns no kube or
  Release-intent writer: Apps admits production Release; Hands
  realizes generic kube origin. The authority that admits Mark's own
  production Release is not named in this graph — `Unknown`. The stale
  Apps auto host `mark-web-prod.sylphx.app` is not a writer.
  `preview_deploys = true` is preview autoDeploy only; it is not
  production Promote. Do not mint a Mark-owned `MARK-RELEASE` writer.
- **Consumed receipts:** none for the render contract — dest forbids
  clock, upstream, and account (`MARK-GRAMMAR`, `MARK-STATS` `dead`).
  Apps Deployment and Hands realization receipts for the
  `sylphx.toml` service are consumed as a customer, not owned.
- **Runtime effects:** render and serve deterministic SVG (and the
  catalog / studio shell) from URL parameters only. No persistence, no
  account write, no upstream fetch, no schema change. Origin immutable
  cache headers are this product's write (`MARK-CDN`). Edge `HIT` is
  Apps, not this product.
- **Forbidden writes:** never a second render authority or grammar
  (vision). Predecessor capsule-render typography and placement knobs
  are leftover, not dest. Never live GitHub stats, clock, or upstream
  as product authority (`MARK-STATS` `dead`), including a webfont
  origin on the studio page. Never kube, HTTPRoute, or Journal
  `spec` writes. Never `{project}.api.sylphx.com` or a mega-client
  (ADR-038). Never a GitHub check name, webhook receipt, or
  deploy-status projection as Release admission or `Live`. Never
  `GET /health` as the product oracle. Never a runtime Apps auto host
  as the vanity/canonical URL (`MARK-HOST`). Never personal or company
  names in theme definitions. Never claim generated SDK consumption
  on the render path.

Unknown in this declaration: which Apps authority admits this
product's production Release; whether the current production Release
matches HEAD or passes the public probe. Those are live- or
owning-lease facts, not greened here.
