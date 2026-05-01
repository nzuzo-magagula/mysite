FROM rust:1.81-bookworm AS builder

# Install system dependencies
# - pkg-config and libssl-dev are often needed for Rust crates
# - binaryen provides wasm-opt
# - curl and nodejs/npm might be needed for asset processing (Tailwind)
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    binaryen \
    curl \
    && curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs \
    && rm -rf /var/lib/apt/lists/*

# Install Dioxus CLI
RUN cargo install dioxus-cli --version 0.7.0-rc.3 --locked

# Add WASM target
RUN rustup target add wasm32-unknown-unknown

WORKDIR /usr/src/app

# Copy everything first (simplified build for troubleshooting)
COPY . .

# Install npm dependencies if they exist
RUN if [ -f package.json ]; then npm install; fi

# Build the project with verbose output to catch errors
RUN dx build --release --features server --verbose

# Final stage
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Ensure the /app directory exists and we have permissions
RUN mkdir -p /app
WORKDIR /app

# Copy assets and binary from builder
# We use absolute paths from the builder stage
COPY --from=builder /usr/src/app/target/release/blogger /app/blogger
COPY --from=builder /usr/src/app/target/dx/blogger/release/web/public /app/public
COPY --from=builder /usr/src/app/articles /app/articles
COPY --from=builder /usr/src/app/aboutme.md /app/aboutme.md

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Use absolute path for entrypoint
ENTRYPOINT ["/app/blogger"]
