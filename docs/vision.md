# Mark Vision

**Status:** Canonical product destination
**Identity graph:** [`capabilities.md`](capabilities.md)
**Current decision:** [`adr/ADR-0005-readme-visuals-toolkit.md`](adr/ADR-0005-readme-visuals-toolkit.md) (amends ADR-0003 and ADR-0004; earlier ADRs are history)

## Destination

**Beautiful README images from one URL.** Mark (repository `readme-mark`) is the
free README-visuals toolkit: animated hero banners, shields-compatible
badges, typing-text SVGs, tech-icon strips, profile cards, and live GitHub
stats cards — one host, no token, no signup, fast and always up.

Switching is a host change. Mark speaks the URL dialects people already have
in their READMEs (shields, skill-icons, readme-typing-svg, capsule-render,
github-readme-stats) and translates each into one render kernel.

Canonical host: `https://mark.sylphx.com`. Every URL that was ever public
there keeps working forever.

## For whom

- **README authors** who want a great-looking profile or project README
  without learning six tools, and without broken images when a free host
  runs out of quota.
- **Maintainers** who need project badges (version, stars, scores) and
  banners that match their brand.

## Principles

- **Zero-config beauty.** Every default must look good. A bare URL renders a
  polished image; knobs are for taste, not for fixing ugliness.
- **Never a broken image.** Unknown input normalizes. Upstream failures
  render a calm fallback card with `200`.
- **Deterministic by default.** Static routes are pure functions of their URL,
  cached immutable. Only live routes (`MARK-LIVE`) read upstream, and they do
  so under the ADR-0005 network contract.
- **Fast.** Sub-millisecond render, edge-cacheable `.svg` URLs, published
  numbers.
- **Safe.** User text is escaped; paint is validated tokens; SVG responses
  carry `script-src 'none'`.

## Not doing

- Accounts, saved marks, uploads, or PNG on the hot path.
- AI generation on the hot path.
- Requiring a user token for anything.
- Personal or company names in theme or palette ids (brand icons chosen by the
  user are content and are fine).

## Product oracle

A stranger opens `https://mark.sylphx.com/`, composes an image in the studio,
pastes the markdown into a README, and it renders on GitHub — and a URL copied
from shields, skill-icons, readme-typing-svg, capsule-render, or
github-readme-stats with only the host changed renders the equivalent image.

## Hosting

Mark is an ordinary Sylphx Apps customer: this repository owns
`sylphx.toml`. Origin cache headers are this product's write; edge caching of
`.svg` URLs follows Cloudflare's extension rule.
