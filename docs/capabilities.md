# Mark identity graph

**Status:** Identity registry. Not live proof.
**Scope:** Mark (`readme-mark`) — README images from one URL.
**Decision:** [`adr/ADR-0005-readme-visuals-toolkit.md`](adr/ADR-0005-readme-visuals-toolkit.md)

```text
ID | Identity | Fate | Depends on | Done when
```

One colloquial name has one row and one fate (`live`, `dead`, or
`rename-to:<ID>`). **Done when** is an oracle, not a claim it is true today.

## Graph

| ID | Identity | Fate | Depends on | Done when |
| --- | --- | --- | --- | --- |
| MARK-GRAMMAR | Native render grammar | live | — | `GET /api/v1/mark/{form}` renders a deterministic SVG for every form in `/api/v1/catalog`; unknown input normalizes (unknown form → hero, unknown art → `waving`, unknown theme/layout/animation → default). |
| MARK-FOREVER | Public URLs never break | live | MARK-GRAMMAR | Every `tests/snapshots/legacy-*.url` answers `200 image/svg+xml` on the live host. |
| MARK-BADGE | Shields-compatible static badge | live | MARK-GRAMMAR, MARK-ICONS | `/badge/{label}-{message}-{color}` (and `{message}-{color}`) and `/static/v1?label&message&color` honor shields escaping (`--`, `__`, `_`), `style` (`flat` `flat-square` `plastic` `for-the-badge` `social`), `logo`, `logoColor`, `labelColor`, `color` (shields names, hex, CSS names, `rgb()`/`hsl()`), and the `.svg` suffix, with badge-maker geometry and Verdana widths; `/api/v1/mark/score?label&value&max` renders a graded score pill. |
| MARK-DIALECTS | Drop-in URL dialects | live | MARK-GRAMMAR | A URL written for shields, skill-icons, readme-typing-svg, capsule-render, or github-readme-stats renders the equivalent image when only the host changes. |
| MARK-ICONS | Brand icon set | live | — | Simple Icons slugs (thousands) plus short aliases render as badge logos and as tech-icon tiles. Every skill-icons id resolves (`/icons?i=` with skillicons.dev geometry, `theme`/`t`, `perline`, `i=all`), and the strip form paints any Simple Icons slug or title. |
| MARK-TYPING | Typing-text SVG | live | MARK-GRAMMAR | `/typing?lines=…` renders an animated typing SVG that works inside `<img>`. |
| MARK-LIVE | Live GitHub data cards and badges | live | MARK-GRAMMAR | Stats, top-languages, streak, repo, and star-history cards plus dynamic badges (GitHub, Actions workflow status, npm, pub.dev, Packagist, Bundlephobia, Chrome Web Store) render from public upstream data with no user token, a bounded cached upstream, stale-on-error, and a `200` fallback card. |
| MARK-SVG | Valid SVG + XSS-safe paint | live | MARK-GRAMMAR | SVG is well-formed, user text is escaped, non-canonical paint falls back, and responses carry `Content-Security-Policy: script-src 'none'` plus `X-Content-Type-Options: nosniff`. |
| MARK-CDN | Cacheable responses | live | MARK-GRAMMAR | Static routes send immutable long cache + strong ETag + `304`; live routes send hour-scale `s-maxage` with `stale-while-revalidate`/`stale-if-error`; every image route also answers with a `.svg` suffix for edge caching. |
| MARK-CATALOG | Public vocabulary | live | MARK-GRAMMAR | `/api/v1/catalog` publishes forms, art, layouts, themes, icons, fonts, and limits that the render honors. Theme ids stay neutral. |
| MARK-STUDIO | Composer at `/` | live | MARK-CATALOG | `/` offers live preview, presets, copy URL / markdown / HTML, and recovers state from a pasted URL. |
| MARK-HOST | Canonical host | live | MARK-GRAMMAR | `https://mark.sylphx.com` serves the product. |
| MARK-PROFILE | Text-driven profile card | live | MARK-GRAMMAR | `/api/v1/mark/profile?text&desc` renders name and tagline from the URL; `identity` maps here. |
| MARK-DEPLOY | Conversion mark | live | MARK-GRAMMAR | `/api/v1/mark/deploy?service=…` renders the "deployed on Sylphx" pill. |
| MARK-STATS | Live GitHub stats | rename-to:MARK-LIVE | — | Superseded by MARK-LIVE (ADR-0005). |
| MARK-IDENTITY | Fleet identity form | rename-to:MARK-PROFILE | — | Not a second product. |

## Release boundary

- **Public probe:** anonymous `GET https://mark.sylphx.com/badge/build-passing-brightgreen`
  and one URL per dialect return SVG. `/health` 200 is deploy proof
  (`revision`), not product proof.
- **Owned writers:** `sylphx.toml` (the Apps service spec). No database, no
  migrations.
- **Runtime effects:** render SVG; live routes read public GitHub/npm APIs
  through a bounded in-memory cache. No persistence.
- **Forbidden writes:** kube, HTTPRoute, or platform state; any write to
  GitHub or npm.
