# ==========================================
# Stage 1: Dependency Planner (cargo-chef)
# ==========================================
FROM rust:slim AS planner
RUN cargo install cargo-chef
WORKDIR /app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ==========================================
# Stage 2: Backend & Shared Dependency Builder
# ==========================================
FROM rust:slim AS backend-builder
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=planner /app/recipe.json recipe.json
# This step builds all dependencies and caches them
RUN cargo chef cook --release --recipe-path recipe.json

# Now build the actual application
COPY . .
RUN cargo build --release -p api-gateway


# ==========================================
# Stage 3: Final Runtime
# ==========================================
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from backend builder
COPY --from=backend-builder /app/target/release/api-gateway ./app

# Expose the API port
EXPOSE 3000

ENV PORT=3000
ENV RUST_LOG=info

CMD ["./app"]
