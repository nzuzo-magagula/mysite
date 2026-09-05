# syntax=docker/dockerfile:1

##############################################################################
# Stage 1: assets — build the stylesheet and the two embedded demo bundles.
#
# These outputs are also committed under assets/ and public/ so that a plain
# `dx build` needs no Node toolchain — `dx` provisions its own Tailwind but
# does not regenerate assets/tailwind.css, so the committed copy is what ships.
#
# That is exactly the failure mode worth designing out: the algorithm
# visualiser once shipped a months-stale bundle because its source had moved
# on and nobody re-ran the build. Rebuilding both demos and the stylesheet
# here means the image is derived from source every time, and the committed
# artifacts are only a convenience for local work.
##############################################################################
FROM node:22-bookworm-slim AS assets
WORKDIR /build

# Dependencies first, so edits to source don't invalidate the install layers.
COPY package.json package-lock.json ./
RUN npm ci --no-audit --no-fund

COPY react_demo/package.json react_demo/package-lock.json ./react_demo/
RUN cd react_demo && npm ci --no-audit --no-fund

COPY svelte_demo/package.json svelte_demo/package-lock.json ./svelte_demo/
RUN cd svelte_demo && npm ci --no-audit --no-fund

# .dockerignore keeps every node_modules out of the context, so the installs
# above survive this copy.
COPY . .

# 1. react_demo  -> public/algovis
# 2. svelte_demo -> public/neuralnet
# 3. tailwind.css -> assets/tailwind.css (minified; ~40 KB smaller than the
#    committed development build)
# The greps then fail the build if the custom themes are missing, rather than
# letting an unstyled site ship. Minification drops the attribute quotes, so
# the pattern has to tolerate both forms.
RUN set -eux; \
    (cd react_demo  && npm run build); \
    (cd svelte_demo && npm run build); \
    npx @tailwindcss/cli -i tailwind.css -o assets/tailwind.css --minify; \
    grep -qE 'data-theme="?paper"?' assets/tailwind.css; \
    grep -qE 'data-theme="?ink"?'   assets/tailwind.css

##############################################################################
# Stage 2: chef — dependency recipe for cached Rust builds
##############################################################################
FROM rust:bookworm AS chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

##############################################################################
# Stage 3: builder — compile the Dioxus app to WASM + server binary
##############################################################################
FROM chef AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    binaryen \
    && rm -rf /var/lib/apt/lists/*

# Pinned to the 0.7 line: the project depends on dioxus 0.7.0-rc-3, and an
# unpinned install would pick up the next major CLI and break the build.
RUN cargo install dioxus-cli --locked --version '^0.7.10'

RUN rustup target add wasm32-unknown-unknown

# Cook dependencies (cached layer)
COPY --from=planner /app/recipe.json recipe.json
COPY advanced_markdown_parser ./advanced_markdown_parser
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .

# Replace the committed front-end artifacts with the ones just built from
# source. The demo directories are removed first because COPY merges: vite
# emits content-hashed filenames, so a stale bundle would otherwise linger
# alongside the new one.
RUN rm -rf ./public/algovis ./public/neuralnet
COPY --from=assets /build/public ./public
COPY --from=assets /build/assets/tailwind.css ./assets/tailwind.css

RUN dx build --release

# Fail here rather than at container start if the CLI's output layout changes
# or the stylesheet lost its themes somewhere in the pipeline.
RUN set -eux; \
    out=/app/target/dx/blogger/release/web; \
    test -x "$out/server"; \
    test -f "$out/public/index.html"; \
    test -f "$out/public/neuralnet/index.html"; \
    test -f "$out/public/algovis/index.html"; \
    test -f "$out/public/js/motion.js"; \
    ls "$out"/public/fonts/*.woff2 > /dev/null; \
    grep -qrE 'data-theme="?paper"?' "$out"/public/assets/*.css

##############################################################################
# Stage 4: runtime — slim image with just the binary and its data
##############################################################################
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# The server binary. dx also hardlinks it to
# target/x86_64-unknown-linux-gnu/server-release/blogger, but the path below
# is the one dx documents as its output.
COPY --from=builder /app/target/dx/blogger/release/web/server ./blogger

# Web assets: hashed JS/WASM/CSS plus everything from public/ (fonts,
# motion.js, favicon, and both demo bundles).
COPY --from=builder /app/target/dx/blogger/release/web/public ./public

# Content read at runtime.
COPY --from=builder /app/articles ./articles
COPY --from=builder /app/aboutme.md ./aboutme.md

ENV PORT=8080
ENV IP=0.0.0.0
ENV DIOXUS_ASSET_DIR=/app/public
EXPOSE 8080

RUN chmod +x ./blogger

# Run unprivileged.
RUN useradd --system --uid 10001 --no-create-home blogger \
    && chown -R blogger:blogger /app
USER blogger

HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD curl -fsS "http://127.0.0.1:${PORT}/" > /dev/null || exit 1

ENTRYPOINT [ "/app/blogger" ]
