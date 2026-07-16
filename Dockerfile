FROM rust:1.97-bookworm AS builder
WORKDIR /app
COPY Cargo.toml ./
COPY src ./src
COPY static ./static
COPY tests ./tests
# Create dummy lock-friendly build
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates curl \
  && rm -rf /var/lib/apt/lists/*
WORKDIR /app
ENV PORT=8787 HOST=0.0.0.0 RUST_LOG=mark=info
COPY --from=builder /app/target/release/mark /usr/local/bin/mark
COPY static ./static
EXPOSE 8787
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -fsS http://127.0.0.1:8787/health >/dev/null || exit 1
USER nobody
CMD ["mark"]
