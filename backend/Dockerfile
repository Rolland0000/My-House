# syntax=docker/dockerfile:1

# ── Builder ───────────────────────────────────────────────────────────────────
FROM rust:1-slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends build-essential pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Build dependencies in their own layer, cached as long as Cargo.toml/Cargo.lock
# are unchanged, so source-only edits don't recompile the whole dependency tree.
COPY Cargo.toml Cargo.lock ./
RUN mkdir src \
    && echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm -rf src

# `sqlx::migrate!()` embeds this directory's SQL files into the binary at
# compile time — no live DATABASE_URL is required to build (no `query!`/
# `query_as!` macros are used yet; if those are introduced, compiling will
# additionally require a reachable DB or a committed `.sqlx/` offline cache).
COPY migrations ./migrations
COPY src ./src
RUN touch src/main.rs && cargo build --release

# ── Runtime ───────────────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system backend \
    && useradd --system --gid backend --no-create-home backend

WORKDIR /app
COPY --from=builder /app/target/release/backend-my-house ./backend-my-house
RUN mkdir -p /app/storage && chown -R backend:backend /app

USER backend

EXPOSE 3000

HEALTHCHECK --interval=10s --timeout=3s --start-period=10s --retries=5 \
    CMD curl -f "http://localhost:${APP_PORT:-3000}/health" || exit 1

CMD ["./backend-my-house"]
