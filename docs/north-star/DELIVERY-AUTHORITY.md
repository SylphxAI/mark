# Mark — Delivery Authority

Status: **Normative** · 2026-08-11

## Product ambition

**Any URL. One image. Your brand.**  
Stateless SVG identity marks from **one grammar** — hero, pill, strip, **profile**, deploy marks (`identity` form retired → use profile; silent hero fallback is a defect) — deterministic, CDN-friendly. Host: `mark.sylphx.com`.

## Clean-break class (ADR-0002/0003)

**Focusing redesign (allowed):** retire live GitHub stats/clock/upstream if product is pure grammar identity CDN.  
**Not allowed:** delete the public mark render surfaces without successor; dual render authorities.

## Required jobs (floor)

| Job | Surface |
|---|---|
| Render mark by form | `GET /api/v1/mark/{form}` (and documented aliases if any) |
| Shields-style badge | `/badge/...` if in product contract |
| Deterministic pure render | No clock/upstream required for core forms |
| XSS-safe paint tokens | Canonical hex only |

## Optional / retired honestly

Live GitHub stats cards: **named product focus decision** (not residual purge theater). If reintroduced, must be explicit capability with network policy — not silent.

## Rules

```text
IF remove core mark/badge render without successor: REJECT.
IF reintroduce unvalidated SVG attribute injection: REJECT.
IF claim third-party live data product without contract: REJECT.
```

## Golden journeys

| ID | Journey |
|---|---|
| M1 | GET mark form → valid SVG |
| M2 | Invalid paint → safe fallback |
| M3 | Deploy identity mark renders |


## Form SSOT
Canonical forms: `hero|pill|strip|profile|deploy` (ADR-0004). ADR-0003 `identity` wording is superseded for form ids.
