# Phase 1: Chef - Prepare the recipe
FROM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Phase 2: Builder - Cook the dependencies and build the app
FROM chef AS builder

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

# Install latest stable Dioxus CLI
RUN cargo install dioxus-cli --locked

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Cook dependencies (cached layer)
COPY --from=planner /app/recipe.json recipe.json
COPY advanced_markdown_parser ./advanced_markdown_parser
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build
COPY . .
# 'dx build' handles multi-target correctly in 0.7+
RUN dx build --release --verbose

# Phase 3: Runtime - Final slim image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the server binary from the correct Dioxus-specific path
# Note: 'dx build' puts the server binary in target/[target_triple]/server-release/
# and the dummy binary from cargo-chef remains in target/release/.
# We want the REAL one.
COPY --from=builder /app/target/x86_64-unknown-linux-gnu/server-release/blogger ./blogger

# 2. Copy the web assets
COPY --from=builder /app/target/dx/blogger/release/web/public ./public

# 3. Copy required data folders and files
COPY --from=builder /app/articles ./articles
COPY --from=builder /app/aboutme.md ./aboutme.md

# Set networking environment variables
ENV PORT=8080
ENV IP=0.0.0.0
# Tell Dioxus where to find the static assets
ENV DIOXUS_ASSET_DIR=/app/public
EXPOSE 8080

# Ensure binary is executable
RUN chmod +x ./blogger

# Use absolute path for entrypoint
ENTRYPOINT [ "/app/blogger" ]
