# ADR-0004: Neutral UI/UX redesign — no personal or company names in the product

- Status: accepted
- Date: 2026-08-09
- Authority: product repository decision implementing binding `engineering-standard`; amends ADR-0003 (form vocabulary, theme catalog, content axis).

## Context

A UI/UX review against the competitor class (capsule-render, readme-typing-svg,
shields, github-readme-stats, skillicons) found Mark out of step where it
matters most:

- The public theme catalog baked in **personal and company names** (`kyle`,
  `sylphx`, `cubeage`, `epiow`, `ozyrix`), and the identity form mapped brands
  to hardcoded taglines including the owner's name ("Kyle Tse — Builder ·
  multi-company portfolio"). Competitors' themes are strictly neutral
  (tokyonight, dracula, dark…); identity lives in the *content* the user
  supplies, never in the product.
- The profile/identity card concept was brand-table-driven instead of
  text-driven.
- Typography was dated on the pill (Verdana) and offered no mono option, so
  Mark could not reproduce the typing-line look (readme-typing-svg) the
  community loves.
- The studio was capability-shaped and cluttered rather than grammar-shaped
  with one-click presets for the actual README patterns people use (header,
  footer, typing lines, profile card, status pill, tech strip).

## Decision

1. **Strictly neutral catalog.** Theme names, icon glyphs, and pill named
   colors contain no personal or company names. Removed: `kyle` / `sylphx` /
   `cubeage` / `epiow` / `ozyrix` themes, the `sylphx` tech glyph, and fleet
   named colors. The only Sylphx-branded surfaces are the deploy mark
   ("deployed on Sylphx") and the credit watermark — both explicit, opt-in,
   and product-level, never theme-level.
2. **`identity` → `profile`, text-driven.** The brand table is deleted; the
   profile card renders `text` (name) and `desc` (tagline) from the URL.
   Content is never baked into the product. The `identity` form name no longer
   parses (unknown forms normalize to hero — totality preserved).
3. **Content axis in the grammar.** `text` and `desc` move to the top level of
   `MarkSpec` (hero title/desc, profile name/tagline), plus a `font` axis
   (`sans` | `mono`, system stacks — no embedded fonts). This enables the
   typing-line composition: transparent hero + mono + `animation=type`.
4. **Pill typography** moves from Verdana to the modern system stack
   (`-apple-system, BlinkMacSystemFont, Segoe UI, Helvetica, Arial`), matching
   capsule-render's rendering on GitHub.
5. **Studio redesign.** Grammar-shaped UI with one-click presets that map
   directly to the README patterns the user's own profile uses: README header
   (waving hero + signature gradient), README footer (reversed waving),
   Typing lines (mono type animation), Profile card, Status pill, Tech strip.
   Controls are grouped Content / Palette / Style / Mark; preview is a
   dominant stage with checkerboard; URL bar with copy/open.
6. **Defaults.** Studio default = waving hero with the signature gradient
   (navy → blue → orange), neutral sample text. No personal names anywhere.

## Consequences

- Breaking: `identity` form and `brand`/`tagline` params are gone (normalize
  to hero / ignored); fleet themes and named colors removed; pill font
  changes rendering.
- The product is now a neutral, competitor-class design surface: themes never
  embarrass, identity is always the user's own content, and the studio
  produces the exact README patterns people already love.
- Regression guards: architecture test asserts no personal/company names in
  themes or icons; smoke tests cover profile (text/art/width/mono), mono
  composition, and the neutral catalog.

## Evidence

- `cargo test --locked` — 57 tests green
- `cargo clippy --all-targets -- -D warnings`
- Local e2e: neutral catalog (13 themes, no brands), profile form renders,
  mono typing composition renders, pill falls back safely for removed named
  colors, studio serves the presets.
