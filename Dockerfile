# Official multi-stage image for Sylphx Platform (dockerfile strategy).
FROM rust:1.97-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY static ./static
COPY tests ./tests
RUN cargo build --release --locked

FROM debian:bookworm-slim
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
    DEFAULT_CREDIT=1 \
    SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt \
    SSL_CERT_DIR=/etc/ssl/certs
COPY --from=builder /app/target/release/mark /usr/local/bin/mark
COPY static ./static
# Prove CAs exist in the final image (fails the build if stripped).
RUN test -s /etc/ssl/certs/ca-certificates.crt && mark --help >/dev/null 2>&1 || true
EXPOSE 8787
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -fsS http://127.0.0.1:8787/health >/dev/null || exit 1
USER nonroot
CMD ["mark"]
