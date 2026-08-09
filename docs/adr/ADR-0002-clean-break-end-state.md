# ADR-0002: Clean-break end state for Mark

- Status: accepted
- Date: 2026-08-09
- Authority: product repository decision implementing binding `engineering-standard`; supersedes ADR-0001 clause 6 (public contracts preserved) for the surfaces listed below.

## Context

A full audit of Mark (source, CI, local build, and live hosts) found one live
critical vulnerability and several correctness/ops gaps:

- **Live SVG attribute injection** on `/api/v1/banner`: `fontColor` and `stroke`
  were emitted into SVG attributes without validation or escaping
  (`fill="{font_color}"`), proven exploitable on `mark.sylphx.com`
  (`fill="#" onload="alert(7)"`). Served as `image/svg+xml` with
  `Access-Control-Allow-Origin: *` on a first-party brand domain.
- **Unbounded input amplification**: banner `text`/`desc`, badge `message`, and
  icon lists had no caps; `animation=type` emits per-character SMIL nodes, so a
  large `text` produced hundreds of thousands of nodes per request on a public
  host with no rate limiting.
- **Dual surfaces and dead host**: every capability was mounted at both
  `/api/v1/*` and a bare legacy path; docs/config split between a live
  `mark.sylphx.com` and a dead `img.sylphx.com` (404 on all paths).
- **Deploy identity broken**: `/health` reports `revision:"unknown"` on both
  live hosts; the platform records `GIT_SHA` on deploys but never passes it
  into the image, and no gate exists to fail such builds.
- **Observability/lifecycle gaps**: no request logging (`tower-http` `trace`
  feature unused), no graceful shutdown despite `graceful_shutdown = 15` in
  `sylphx.toml`.
- **Dead wire fields**: `avatar_url`, `html_url`, `open_issues_count` fetched
  from the GitHub API and never rendered.
- **Fail-open card**: `user_stats` swallowed the repo-snapshot fetch error and
  rendered a zero-data card as if true.
- Unused deps/features (`once_cell`, `axum` `macros`, `tower-http`
  `set-header`, `tokio` `full`); stale docs (`img.sylphx.com`, "30 styles",
  TS-backend fence with a retired Enact work ID, stale merge-queue comments).

## Decision

Clean-break, no backward compatibility, no legacy aliases:

1. **Strict SVG attribute grammar.** New shared kernel
   `shared::svg::normalize_hex_token` is the only path for color-bearing
   attributes: canonical `#rgb` / `#rrggbb` / `#rrggbbaa` (3-digit expanded),
   anything else → fallback paint. Banner `fontColor`/`stroke` and badge colors
   use it. User text remains escaped via `shared::svg::esc`. SVG responses add
   `Content-Security-Policy: default-src 'none'; style-src 'unsafe-inline';
   script-src 'none'; object-src 'none'; base-uri 'none'` and
   `X-Content-Type-Options: nosniff` as defense-in-depth.

2. **Bounded input contract.** Banner `text` ≤ 500 / `desc` ≤ 240 / ≤ 8 lines,
   badge `label` ≤ 80 / `message` ≤ 120, icon rows ≤ 60. Truncation is marked
   with `…`; limits are enforced in the pure application layer and exposed in
   `/api/v1/catalog` and README.

3. **Single canonical HTTP surface.** `/api/v1/*` for all capabilities, plus
   the shields-style `/badge/{label}-{message}-{color}` path form as the
   distinctive embed format. Bare legacy aliases (`/banner`, `/badge` query
   form, `/stats/{user}`, `/org/{org}`, `/repo/{owner}/{repo}`, `/icons`,
   `/brand/{name}`, `/deploy`) are deleted. `sylphx.toml` `path_prefixes`
   updated.

4. **Single canonical host.** `mark.sylphx.com`. `img.sylphx.com` references
   removed from README, `.env.example`, and `PROJECT.md`.

5. **Deploy identity gate.** Builder copies `.git` so `build.rs` embeds the
   exact checkout revision even when the platform passes no build args
   (runtime env `GIT_SHA`/`SOURCE_COMMIT` still wins when injected). The final
   image runs `mark --version | grep -Eq "rev [0-9a-f]{7,}"` and fails the
   build when no revision is embedded. `mark --version` prints the revision.

6. **Observability + lifecycle.** `TraceLayer` (INFO spans/responses) mounted
   in the composition root; `axum::serve(...).with_graceful_shutdown(...)` on
   SIGTERM/SIGINT drains in-flight requests, matching `graceful_shutdown = 15`.

7. **Correctness.** GitHub cards fail closed: any upstream snapshot failure
   renders the error card. Dead wire fields (`avatar_url`, `html_url`,
   `open_issues_count`) removed from adapter DTOs and domain models.

8. **Dependency surface.** Drop direct `once_cell` (std `LazyLock`), `axum`
   `macros`, `tower-http` `set-header`, `tokio` `full` (→
   `rt-multi-thread, macros, net, signal`).

9. **Docs/CI alignment.** README lists the 42 banner types and 17 animations
   from the catalogs, the limits table, and the canonical host. AGENTS.md
   replaces the stale TS-backend fence (no TypeScript exists in this
   repository) with the clean-break contract. Stale merge-queue comments in
   `ci.yml` and `docs/reference/fast-trunk-ci.md` fixed.

## Consequences

- Breaking changes: bare path aliases 404; `fontColor`/`stroke` accept hex
  only; long text truncates; `user_stats` errors instead of zero-data cards.
  External READMEs using legacy aliases or `img.sylphx.com` must migrate to
  `/api/v1/*` and `mark.sylphx.com`.
- Regression guards: escaping/injection tests, cap tests, legacy-path-404
  contract tests, CSP header tests, catalog↔shape parity test, fail-closed
  source test, dead-field guards, and `Dockerfile` revision gate.

## Evidence

- `cargo test --locked` (44 baseline + new clean-break tests)
- `cargo clippy --all-targets -- -D warnings`
- Live verification: revision readback on `mark.sylphx.com` matches the landed
  SHA; injection payloads render inert; legacy aliases 404; limits enforced.
