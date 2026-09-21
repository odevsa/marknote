# --- Stage 1: Build Frontend ---
FROM node:22-alpine AS frontend-builder
WORKDIR /app/frontend

COPY frontend/package*.json ./
RUN npm ci

COPY frontend/ ./
RUN npm run build

# --- Stage 2: Build Backend ---
FROM rust:1-slim-bookworm AS backend-builder
WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Copy backend source
COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
COPY backend/migrations ./backend/migrations
COPY backend/src ./backend/src

# Copy built frontend assets into the expected path for rust-embed
COPY --from=frontend-builder /app/frontend/build ./frontend/build

WORKDIR /app/backend
RUN cargo build --release

# --- Stage 3: Minimal Runtime Image ---
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    tzdata \
    && rm -rf /var/lib/apt/lists/*

# Create persistent storage directories
RUN mkdir -p /app/data/base /app/data/notes

COPY --from=backend-builder /app/backend/target/release/marknote /usr/local/bin/marknote
COPY docker/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh /usr/local/bin/marknote

ENV HOST=0.0.0.0 \
    PORT=3000 \
    DATA_DIR=/app/data/base \
    NOTES_DIR=/app/data/notes

EXPOSE 3000

VOLUME ["/app/data/base", "/app/data/notes"]

ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]
CMD ["marknote"]

