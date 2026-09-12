FROM rust:1.89-bookworm

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential pkg-config clang libclang-dev \
    libnice-dev libglib2.0-dev libssl-dev \
    ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /src
COPY . .
RUN cargo build --workspace --release
