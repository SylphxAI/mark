# Mark identity graph

Clients consume owner ADR-038: peer generated SDKs and peer credentials on dest peels. Mega-clients and `{project}.api.sylphx.com` are not dest.

**Status:** Identity registry. Not live proof.
**Scope:** Mark — stateless SVG identity marks from one grammar.
**Cite:** the **ID** column.

This file is the identity graph. It is not a PRD, ADR index, or live grade. Destination stays in [`vision.md`](vision.md). Field law stays in `src/`, `adr/`, and `DELIVERY-AUTHORITY.md`. If this file conflicts with those, this file is wrong.

```text
ID | Identity | Fate | Depends on | Done when
```

## Graph

| ID | Identity | Fate | Depends on | Done when |
| --- | --- | --- | --- | --- |
| MARK-GRAMMAR | One-grammar stateless render | live | — | `GET /api/v1/mark/{form}` with `form∈{hero,pill,strip,profile,deploy}` and modifiers `type/theme/color/text/desc/font/width/height/animation` renders a deterministic SVG without clock, upstream, or account at the live layer; shields-style `/badge/{label}-{message}-{color}` holds when in contract. |
| MARK-SVG | Valid SVG + XSS-safe paint tokens | live | MARK-GRAMMAR | Any rendered SVG is valid, deterministic, and rejects non-canonical-hex paint injection with a safe fallback at the live layer. |
| MARK-HOST | Canonical host + CDN contract | live | MARK-GRAMMAR | `https://mark.sylphx.com` is the sole canonical host; renders are CDN-friendly and deterministic forever at the live layer. |
| MARK-STATS | Live GitHub stats as product authority | dead | — | Live GitHub stats/clock/upstream cards carry no live fate as product authority (see ADR-0002/0003 clean-break); reintroduction requires explicit capability + network contract, not silent re-add. |

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
  `PUBLIC_BASE_URL`). Mark is stateless and owns no database-schema
  migration writer. It owns no kube or Release-intent writer: Platform
  owns build, binding, HTTPRoute, Deployment, and Kubernetes state.
  The authority that admits Mark's own production Release is not named
  in this graph — `Unknown`. The stale Platform auto host
  `mark-web-prod.sylphx.app` is not a writer. `preview_deploys = true`
  is preview autoDeploy only; it is not production Promote.
- **Consumed receipts:** none for the render contract — dest forbids
  clock, upstream, and account (`MARK-GRAMMAR`, `MARK-STATS` `dead`).
  Platform Deployment and Hands realization receipts for the
  `sylphx.toml` service are consumed as a customer, not owned.
- **Runtime effects:** render and serve deterministic SVG (and the
  catalog / docs shell) from URL parameters only. No persistence, no
  account write, no upstream fetch, no schema change.
- **Forbidden writes:** never a second render authority or grammar
  (vision). Never live GitHub stats, clock, or upstream as product
  authority (`MARK-STATS` `dead`). Never kube, HTTPRoute, or Journal
  `spec` writes. Never `{project}.api.sylphx.com` or a mega-client
  (ADR-038). Never a GitHub check name, webhook receipt, or
  deploy-status projection as Release admission or `Live`. Never
  `GET /health` as the product oracle. Never a runtime Platform host
  as the vanity/canonical URL (`MARK-HOST`). Never personal or company
  names in theme definitions.

Unknown in this declaration: the authority admitting this product's
own production Release; whether the current production Release matches
`sylphx.toml` or passes the public probe. Those are live- or
owning-lease facts, not greened here.
