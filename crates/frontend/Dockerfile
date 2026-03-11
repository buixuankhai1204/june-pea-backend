# ==========================================
# Stage 1: Frontend Builder (Trunk + Wasm)
# ==========================================
FROM rust:slim AS builder

# Install dependencies for trunk
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl && rm -rf /var/lib/apt/lists/*

# Install trunk and add wasm target
RUN curl -sL https://github.com/trunk-rs/trunk/releases/download/v0.21.4/trunk-x86_64-unknown-linux-gnu.tar.gz | tar -xzf- -C /usr/local/bin
RUN rustup target add wasm32-unknown-unknown

# Install tailwindcss standalone CLI
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    && chmod +x tailwindcss-linux-x64 \
    && mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss \
    && echo '#!/bin/sh' > /usr/local/bin/npx \
    && echo 'while [ "$1" = "--yes" ] || [ "$1" = "tailwindcss" ]; do shift; done' >> /usr/local/bin/npx \
    && echo 'exec tailwindcss "$@"' >> /usr/local/bin/npx \
    && chmod +x /usr/local/bin/npx

WORKDIR /app
# We need the root context for shared crates if any, but since this is in crates/frontend,
# let's assume we copy everything from root to get workspace dependencies right.
COPY . .

WORKDIR /app/crates/frontend
RUN trunk build --release

# ==========================================
# Stage 2: Final Runtime (Nginx)
# ==========================================
FROM nginx:alpine
ENV NGINX_ENVSUBST_FILTER='$PORT'
RUN sed -i 's/worker_processes  auto;/worker_processes  1;/g' /etc/nginx/nginx.conf

COPY --from=builder /app/crates/frontend/dist /usr/share/nginx/html
COPY crates/frontend/nginx.conf /etc/nginx/templates/default.conf.template


CMD ["nginx", "-g", "daemon off;"]
