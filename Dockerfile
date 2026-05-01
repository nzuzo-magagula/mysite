# Stage 1: Builder
FROM rust:1.81-bookworm AS builder

# Install system dependencies
# - pkg-config, libssl-dev for Rust crates
# - binaryen for wasm-opt
# - curl and nodejs/npm for Tailwind and CLI install
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binaryen \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# 1. Use the official Dioxus install script as requested
# This script downloads a pre-compiled binary, which is much faster.
RUN curl -sSL https://dioxus.dev/install.sh | bash
# Add the cargo bin directory to PATH so 'dx' is available
ENV PATH="/root/.cargo/bin:${PATH}"

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

# Use /usr/src/app which is the standard Rust working directory
WORKDIR /usr/src/app

# Copy files
COPY . .

# Install npm dependencies for Tailwind plugins
RUN if [ -f package.json ]; then npm install; fi

# Build the project with verbose output
# 'dx build' will now use the version installed by the script
RUN dx build --release --features server --verbose

# Stage 2: Runtime
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Using the same WORKDIR as builder for consistency
WORKDIR /usr/src/app

# Copy assets and binary from builder
COPY --from=builder /usr/src/app/target/release/blogger ./blogger
COPY --from=builder /usr/src/app/target/dx/blogger/release/web/public ./public
COPY --from=builder /usr/src/app/articles ./articles
COPY --from=builder /usr/src/app/aboutme.md ./aboutme.md

# Set networking environment variables
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Run the binary
ENTRYPOINT ["./blogger"]
