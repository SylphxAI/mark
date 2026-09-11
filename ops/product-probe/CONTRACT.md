# Mark public delivery probe

Contract `mark.public-svg.v1` owns product postconditions only. Apps admits the
release and binds one selected Mark member; Hands executes this image; provider
observations bind its runtime. This runner reads only that context's HTTPS origin,
without credentials, redirects, host fallback, product writes, or health probes.
It does not claim studio browser usability, edge HIT, performance, soak, recovery,
or proof that the selected revision serves the canonical vanity hostname.

| Required assertion ID | Owning capability and oracle |
| --- | --- |
| `mark.svg-grammar.v1` | MARK-GRAMMAR: all five forms return well-formed SVG with supplied content; geometry applies. |
| `mark.svg-escaping.v1` | MARK-SVG: hostile text stays text; invalid paint cannot add SVG attributes; CSP and nosniff bind the response. |
| `mark.deterministic-paint.v1` | MARK-GRAMMAR: repeated URL has identical bytes and ETag; valid paint changes bytes; unknown paint falls back. |
| `mark.badge-equivalence.v1` | MARK-BADGE: shorthand and canonical pill produce byte-identical SVG with composed query options. |
| `mark.conditional-cache.v1` | MARK-CDN: strong ETag and immutable origin/edge headers; conditional fetch is empty 304 with retained ETag; changed content changes ETag. |
| `mark.catalog.v1` | MARK-CATALOG: published forms/limits match owned grammar, neutral theme/icon ids, sampled advertised art and icon produce their product output. |

The no-argument entrypoint consumes `APPS_PRODUCT_PROBE_CONTEXT` and
`APPS_PRODUCT_PROBE_CONTEXT_DIGEST` (SHA-256 of RFC 8785 canonical whole JSON).
Context carries organization/project/environment IDs, release/decision IDs,
selectionGeneration as decimal uint64 string, rolloutAttempt as an integer,
stage `DELIVERY_RELEASE_EXECUTION_STAGE_POST_DEPLOYMENT_PROBE`, and intent with
contractId, runnerImageDigest, policyRevision, requiredAssertionIds, and one
target containing memberId, runtimeObservationDigest, resourceGeneration,
revisionUid, origin, sourceCommitSha, sourceRepositoryId. Unknown assertions are
rejected before network access; v1 requires the entire six-assertion contract.

Results contain contextDigest, six assertion IDs/statuses, startedAt/finishedAt
UTC RFC3339 timestamps and fit `/dev/termination-log` within 4096 bytes. Failure
exits nonzero; each attempted assertion defaults to FAILED, later unattempted
assertions remain SKIPPED. Invalid context emits no successful result. An
absolute 50-second workflow deadline, five-second request deadline, two-second
socket timeout and 1 MiB response cap bound execution. No response or context
body is logged. The numeric nonroot runner image is built by normal Apps builds
from `ops/product-probe/Dockerfile`; no product-owned image publisher is added.

`tests/product_probe.rs` starts the actual owning Axum router on a local ephemeral
port and runs this same six-assertion workflow against it. Python protocol tests
exercise context refusal and result failures. This is source evidence, not live
release evidence. The local test calls the workflow directly; production context
parsing always requires HTTPS and has no localhost override.
