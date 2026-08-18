# Vision — Mark

This file is the product destination. It is not a delivery claim, work status,
or release plan.

## What finished is

Mark is the dependable image language for software identity: a maintainer
expresses a complete mark in one public URL, embeds that URL in a README, site,
or product surface, and receives a polished SVG without an account, asset
build, or data dependency.

One grammar composes form × art × paint × content × geometry × motion across
five canonical forms: `hero`, `pill`, `strip`, `profile`, and `deploy`. The
same URL has one meaning and renders the same image. The catalog and studio
project that grammar for discovery; neither is a second rendering authority.

## For whom

- Maintainers and documentation authors who need durable, expressive images
  that work as ordinary `<img>` or Markdown embeds.
- Product teams that need one neutral visual vocabulary across banners, status
  marks, technology strips, profile cards, and explicit Sylphx deploy marks.
- Operators who need an artifact-bound, directly testable HTTP product with no
  upstream-data failure path on the render hot path.

## Product promise

Every syntactically accepted Mark URL resolves through a bounded grammar to a
valid, finite, injection-safe SVG. Unknown or out-of-range values normalize to
documented safe semantics; they do not create a second dialect or an error SVG.
User content remains user-supplied, neutral themes remain unbranded, and the
only Sylphx-bearing compositions are the explicit deploy mark and optional
credit watermark.

Determinism is the reliability moat: rendering does not consult a clock,
network, account, secret, mutable store, or remote asset. A Mark therefore
stays cacheable and meaningful wherever an image URL can be embedded.

## Product boundaries

Mark owns the URL grammar, vocabulary, input normalization, pure SVG kernel,
form composition, public HTTP contract, catalog projection, and generator
studio. The runtime platform owns generic build, deployment, routing, and
runtime availability. `/health.revision` identifies the artifact being served;
it is not proof that the Mark contract passes.

Mark does not own live GitHub or repository cards, analytics, arbitrary file
hosting, AI generation on the hot path, clocks, customer state, or a general
replacement for every badge vendor. A feature that needs upstream data or
mutable identity is a different product, not another Mark rendering path.

## Exact shipped oracle

The shipped customer terminal is the canonical host serving an admitted
artifact revision and passing all of these observations against that same
instance:

1. `/health.revision` equals the admitted artifact SHA.
2. `GET /api/v1/mark/{form}` returns HTTP 200 `image/svg+xml` for each of
   `hero`, `pill`, `strip`, `profile`, and `deploy`; `/badge/{label}-{message}-{color}`
   returns the same pill product through its documented shorthand.
3. Two requests for the same complete URL return byte-identical SVG bodies.
4. Representative unknown, oversized, hostile, non-finite, and out-of-range
   inputs produce bounded SVG with no raw scriptable attribute, `NaN`, `inf`,
   or non-finite gradient offset.
5. SVG responses carry the contract security and cache headers, user text is
   escaped, and animated/static cache policy follows the normalized motion.
6. `/api/v1/catalog` projects the accepted grammar, the studio composes that
   grammar, and retired capability routes remain unavailable.

Repository tests are the source-level executable oracle for these semantics:
`tests/http_contracts.rs`, `tests/clean_break.rs`, `tests/mark_smoke.rs`, and
`tests/architecture_boundaries.rs`. A local pass proves the checked source; it
does not substitute for the artifact-bound shipped observations above.

## Durable decisions

- [`ADR-0002`](adr/ADR-0002-clean-break-end-state.md) owns the hardened public
  contract and clean break.
- [`ADR-0003`](adr/ADR-0003-one-grammar-end-state.md) owns the one-grammar,
  stateless product shape.
- [`ADR-0004`](adr/ADR-0004-neutral-ux-redesign.md) owns the neutral vocabulary
  and canonical `profile` form.
- [`capabilities.md`](capabilities.md) owns stable capability IDs, hard edges,
  and falsifiable done-when predicates.
