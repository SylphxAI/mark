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
