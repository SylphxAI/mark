# Mark (readme-mark)

**Beautiful README images from one URL.** Banners, badges, typing text, tech
icons, and GitHub stats cards — free, no token, no signup.

## Lifecycle

- Lifecycle: `active` — open-source flagship (star program)
- Owner org: `SylphxAI`
- Stack: Rust (`axum`), pure SVG (no headless browser)
- Decision of record: [`docs/adr/ADR-0005-readme-visuals-toolkit.md`](docs/adr/ADR-0005-readme-visuals-toolkit.md)

## Goals

- The best free README-visuals toolkit; drop-in for shields, skill-icons,
  readme-typing-svg, capsule-render, and github-readme-stats URLs
- Every URL ever public on `https://mark.sylphx.com` keeps working
- Beautiful zero-config defaults; delightful studio at `/`
- Fast and cacheable: deterministic static routes, cached live routes

## Non-goals

- Accounts, uploads, saved marks, PNG or AI generation on the hot path
- Requiring a user token
- Personal or company names in theme ids

## Public surfaces

- Studio: `/`
- Native grammar: `/api/v1/mark/{form}` · catalog `/api/v1/catalog` · `/health`
- Dialects: `/badge/…`, `/icons?i=…`, `/typing?lines=…`, `/api?type=…`,
  `/api?username=…` (see README)
- Repo: https://github.com/SylphxAI/readme-mark

## Delivery

- PRs through the merge queue; the Sylphx platform deploys `main`
- Validate: `cargo test` · `cargo clippy --all-targets -- -D warnings`
