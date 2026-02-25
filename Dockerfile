# Stage 1: Builder
FROM rust:1.80-slim AS builder

WORKDIR /app

# Chỉ copy file config trước để tận dụng Docker layer caching cho dependencies
COPY Cargo.toml Cargo.lock ./

# Tạo một file main giả để fetch dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -f src/main.rs

# Copy code thực tế và build lại
COPY . .
RUN cargo build --release

# Stage 2: Runtime (Dùng bản slim để giảm dung lượng từ >1GB xuống ~30MB)
FROM debian:bookworm-slim

WORKDIR /app

# Cài đặt thư viện cần thiết (nếu app dùng SSL/TLS)
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy file thực thi từ stage builder
COPY --from=builder /app/target/release/app .

# Chạy ứng dụng
CMD ["./app"]