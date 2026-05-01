# Phase 1: Chef - Prepare the recipe
FROM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Phase 2: Builder - Cook the dependencies and build the app
FROM chef AS builder

# Install system dependencies needed for compilation and Dioxus
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

# Install latest stable Dioxus CLI from source
# This ensures compatibility with the Dioxus 0.7.x framework versions
# and prevents the 'GLIBC' errors seen with pre-compiled binaries.
RUN cargo install dioxus-cli --locked

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Cook dependencies (cached layer)
COPY --from=planner /app/recipe.json recipe.json
# Path dependencies must be copied for cargo-chef to work
COPY advanced_markdown_parser ./advanced_markdown_parser
RUN cargo chef cook --release --recipe-path recipe.json

# Copy source and build
COPY . .

# Build for Fullstack. 
# In Dioxus 0.7+, 'dx build --release' automatically handles the multi-target build
# (Server binary + WASM client) correctly. Explicitly passing --features server
# can sometimes leak server-only dependencies into the WASM build, causing errors.
RUN dx build --release --verbose

# Phase 3: Runtime - Final slim image
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the server binary
COPY --from=builder /app/target/release/blogger ./blogger

# 2. Copy the web assets
COPY --from=builder /app/target/dx/blogger/release/web/public ./public

# 3. Copy required data folders and files
COPY --from=builder /app/articles ./articles
COPY --from=builder /app/aboutme.md ./aboutme.md

# Set networking environment variables
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Use absolute path for entrypoint
ENTRYPOINT [ "/app/blogger" ]
