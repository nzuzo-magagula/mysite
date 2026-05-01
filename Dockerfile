FROM rust:1 AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
# Install Dioxus CLI and dependencies
RUN apt-get update && apt-get install -y binaryen && rm -rf /var/lib/apt/lists/*
RUN cargo install dioxus-cli --version 0.7.0-rc.3 --locked
# Add wasm target
RUN rustup target add wasm32-unknown-unknown

COPY --from=planner /app/recipe.json recipe.json
# Copy path dependencies needed by cargo-chef
COPY advanced_markdown_parser ./advanced_markdown_parser
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .

# Build for Fullstack. 'dx build' handles both WASM and the server binary.
# We explicitly enable the 'server' feature to ensure the backend logic is compiled.
RUN dx build --release --features server

# Use a slim runtime image for smaller deployments
FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# 1. Copy the REAL server binary (not a symlink)
COPY --from=builder /app/target/release/blogger /app/blogger

# 2. Copy the static assets (WASM, JS, CSS) to the 'public' folder the server expects
COPY --from=builder /app/target/dx/blogger/release/web/public /app/public

# 3. Copy required data folders and files
COPY --from=builder /app/articles /app/articles
COPY --from=builder /app/aboutme.md /app/aboutme.md

# Set networking environment variables
ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

# Use the absolute path to the binary
ENTRYPOINT [ "/app/blogger" ]
