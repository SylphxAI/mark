# Official multi-stage image for Sylphx Platform (dockerfile strategy).
FROM rust:1.97-bookworm AS builder
ARG GIT_SHA=unknown
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
COPY tests ./tests
# Embed revision at compile time when provided by the build system.
ENV GIT_SHA=${GIT_SHA}
RUN cargo build --release --locked

FROM debian:bookworm-slim
ARG GIT_SHA=unknown
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && update-ca-certificates \
  && rm -rf /var/lib/apt/lists/* \
  && test -s /etc/ssl/certs/ca-certificates.crt \
  && useradd --system --uid 65532 --create-home --home-dir /app nonroot
WORKDIR /app
ENV PORT=8787 \
    HOST=0.0.0.0 \
    RUST_LOG=mark=info \
    DEFAULT_CREDIT=0 \
    GIT_SHA=${GIT_SHA} \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs
COPY --from=builder /app/target/release/mark /usr/local/bin/mark
COPY static ./static
# Prove CA bundle + binary are real. `mark --help` must exit (not start the server).
RUN test -s /etc/ssl/certs/ca-certificates.crt \
  && test -x /usr/local/bin/mark \
  && mark --help | grep -q "Sylphx Mark"
EXPOSE 8787
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -fsS http://127.0.0.1:8787/health >/dev/null || exit 1
USER nonroot
CMD ["mark"]
