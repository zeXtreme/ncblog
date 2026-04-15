# ─────────────────────────────────────────────
# Stage 1: Build React admin UI
# ─────────────────────────────────────────────
FROM node:20-alpine AS frontend

WORKDIR /build/admin-ui

# Install dependencies first (better layer caching)
COPY admin-ui/package*.json ./
RUN npm ci

# Copy source and build
COPY admin-ui/ ./
RUN npm run build


# ─────────────────────────────────────────────
# Stage 2: Build Rust backend
# ─────────────────────────────────────────────
FROM rust:1.77-slim-bookworm AS backend

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Cache dependencies: fetch all crates before copying source
COPY backend/Cargo.toml backend/Cargo.lock ./
RUN cargo fetch

# Copy source and build (deps already cached in the layer above)
COPY backend/src/ ./src/
RUN cargo build --release --offline


# ─────────────────────────────────────────────
# Stage 3: Runtime image
# ─────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

# Hugo extended requires glibc (hence debian, not alpine)
ARG HUGO_VERSION=0.124.1

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Download Hugo extended binary from GitHub releases
RUN curl -fsSL \
    "https://github.com/gohugoio/hugo/releases/download/v${HUGO_VERSION}/hugo_extended_${HUGO_VERSION}_linux-amd64.tar.gz" \
    | tar -xz -C /usr/local/bin hugo \
    && hugo version

# Copy build artifacts
COPY --from=frontend /build/admin-ui/dist/ /app/admin-ui/dist/
COPY --from=backend  /build/target/release/backend /app/backend

# Copy Hugo site (theme + content + config)
# site/public/ is excluded via .dockerignore — Hugo will generate it at runtime
COPY site/ /app/site/

# Runtime configuration via environment variables
ENV PROJECT_ROOT=/app
ENV HUGO_BIN=hugo
ENV PORT=3000
# Override ADMIN_PASSWORD at deploy time — default is for local dev only
ENV ADMIN_PASSWORD=admin123

WORKDIR /app

EXPOSE 3000

CMD ["/app/backend"]
