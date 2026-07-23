# ADR-0001: Capability-first architecture for Mark

- Status: accepted
- Date: 2026-07-23
- Authority: product repository decision implementing binding `engineering-standard`

## Context

Mark is a durable product (embeddable URL → SVG image API). The repository had
grown as a flat technical module layout (`routes`, `badge`, `banner`, `stats`,
`github`, …) with:

- HTTP, pure render, and GitHub I/O mixed across modules
- `deploy_badge` living under stats without ownership clarity
- no explicit ports between application policy and the GitHub network adapter
- no repository ADR mapping capabilities to code ownership

Binding Skills (`engineering-standard`) require Capability-first Modular DDD
with Clean/Hexagonal boundaries and Functional Core / Imperative Shell from the
first durable product commit. Project size is not an exemption.

## Decision

1. **Single crate, module-first.** Rust modules are the semantic unit. No
   multi-crate split: there is no independent release, deployment, security, or
   isolation boundary that justifies additional crates today.

2. **Capabilities (bounded outcomes):**
   | Capability | Consumer outcome | Code root |
   | --- | --- | --- |
   | `banner` | Artistic header/footer banner SVG | `src/capabilities/banner` |
   | `badge` | Shields-style badge SVG | `src/capabilities/badge` |
   | `github_card` | User/org/repo stats card SVG | `src/capabilities/github_card` |
   | `icon_row` | Tech icon strip SVG | `src/capabilities/icon_row` |
   | `brand_kit` | Fleet brand kit card SVG | `src/capabilities/brand_kit` |
   | `deploy_mark` | “deployed on Sylphx” promo badge SVG | `src/capabilities/deploy_mark` |

3. **Shared kernel (not capabilities):** `src/shared/{color,theme,svg}` hold
   pure primitives reused across capabilities. They have no independent product
   outcome.

4. **Layering inside each capability:**
   - `domain/` — pure models, catalogs, normalization, aggregation
   - `application/` — pure render use cases; ports for effects
   - `adapters/` — only where external effects exist (GitHub HTTP)
   - `interfaces/` — HTTP query translation

5. **Composition root:** `src/bootstrap.rs` loads config and serves;
   `src/interfaces/http` wires routes. Domain/application never import `axum`,
   `reqwest`, or process env for policy decisions (GitHub token read stays in
   the adapter).

6. **Public contracts preserved:** HTTP paths, query parameters, SVG semantics,
   and soft-credit watermark rules remain unchanged unless a later ADR says
   otherwise. Crate-root re-exports (`mark::banner`, `mark::badge`, …) keep
   internal test ergonomics without creating a second semantic authority.

7. **Supersedes:** informal flat-module layout. No prior numbered ADR existed.

## Amendment (2026-07-23 residual)

Clock sampling for `timeAuto` / `timeGradient` is an imperative-shell effect.
The shared kernel accepts an injected `clock_seed`; HTTP interfaces sample
`chrono::Utc::now()` via `current_time_seed()`. Pure render remains deterministic
when tests omit or fix the seed.

GitHub JSON wire types (`GhUserDto` / `GhRepoDto`) live only in the HTTP adapter.
Domain models are serde-free product views mapped at the boundary.

Unused direct dependencies (`anyhow`, `thiserror`, bare `tower`,
`serde_urlencoded`) were removed; `tower` remains a dev-dependency for HTTP
contract tests.

## Consequences

- New product outcomes land as capabilities (or explicit sub-capabilities), not
  as loose files under `src/`.
- GitHub network access is replaceable via `GitHubSource` for tests.
- Architecture boundary tests enforce domain purity and forbidden imports.
- Empty ceremony folders are avoided: capabilities without external effects omit
  `adapters/`; thin capabilities may keep domain folded into application when
  no independent domain model exists (`brand_kit`, `deploy_mark`).

## Evidence

- `cargo test --locked`
- `tests/architecture_boundaries.rs`
- `tests/render_smoke.rs`, `tests/banner_motion.rs` (behavior parity)
