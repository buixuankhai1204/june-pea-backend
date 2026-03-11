# ==========================================
# Stage 1: Backend Builder
# ==========================================
FROM rust:slim AS backend-builder

# Install build dependencies (pkg-config, libssl-dev needed for sqlx/reqwest etc.)
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the entire workspace
COPY . .

# Build only the api-gateway package
RUN cargo build --release -p api-gateway


# ==========================================
# Stage 2: Frontend Wasm Builder
# ==========================================
FROM rust:slim AS frontend-builder

# Install dependencies for trunk
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Install trunk and add wasm target
RUN curl -sL https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
RUN rustup target add wasm32-unknown-unknown

# Install tailwindcss standalone and create npx shim to bypass npm issues
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod +x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss \
    && echo '#!/bin/sh' > /usr/local/bin/npx \
    && echo 'while [ "$1" = "--yes" ] || [ "$1" = "tailwindcss" ]; do shift; done' >> /usr/local/bin/npx \
    && echo 'exec tailwindcss "$@"' >> /usr/local/bin/npx \
    && chmod +x /usr/local/bin/npx

WORKDIR /app

# Copy the entire workspace because frontend might depend on shared crates
COPY . .

# Build the frontend crate using trunk
WORKDIR /app/crates/frontend
RUN trunk build --release


# ==========================================
# Target A: Backend Runtime target
# ==========================================
FROM debian:bookworm-slim AS backend

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary
COPY --from=backend-builder /app/target/release/api-gateway ./app

# Expose the port that api-gateway binds to
EXPOSE 3000

# Set default environment variables (can be overridden by docker-compose)
ENV PORT=3000
ENV RUST_LOG=info

CMD ["./app"]


# ==========================================
# Target B: Frontend Nginx Runtime target
# ==========================================
FROM nginx:alpine AS frontend

# Copy the generated static files
COPY --from=frontend-builder /app/crates/frontend/dist /usr/share/nginx/html

# Replace default config with our custom one to proxy /api
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]
