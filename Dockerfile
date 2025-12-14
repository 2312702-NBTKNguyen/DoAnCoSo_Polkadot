# ============================================================================
# Dockerfile cho DoAnCoSo - Substrate Parachain Node
# Hỗ trợ biên dịch trên: WSL, Ubuntu, macOS (qua Docker)
# ============================================================================

# Stage 1: Builder
FROM rust:1.81-bookworm AS builder

LABEL maintainer="DoAnCoSo Team <2312702@dlu.edu.vn>"
LABEL description="Build environment for DACS Substrate Parachain"

# Tránh interactive prompts
ENV DEBIAN_FRONTEND=noninteractive

# ============================================================================
# Cài đặt dependencies cần thiết cho Substrate
# ============================================================================
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Build essentials
    build-essential \
    cmake \
    pkg-config \
    git \
    # RocksDB dependencies (QUAN TRỌNG)
    librocksdb-dev \
    libclang-dev \
    clang \
    llvm \
    # Protobuf (cần cho Substrate)
    protobuf-compiler \
    libprotobuf-dev \
    # SSL và networking
    libssl-dev \
    ca-certificates \
    # Các thư viện hệ thống khác
    libudev-dev \
    zlib1g-dev \
    libbz2-dev \
    liblz4-dev \
    libzstd-dev \
    libsnappy-dev \
    # Cleanup
    && rm -rf /var/lib/apt/lists/*

# ============================================================================
# Cấu hình Rust
# ============================================================================
# Cài đặt toolchain theo README (1.86.0)
RUN rustup toolchain install 1.86.0 \
    && rustup default 1.86.0 \
    && rustup target add wasm32-unknown-unknown --toolchain 1.86.0 \
    && rustup component add rust-src --toolchain 1.86.0

# ============================================================================
# Cài đặt công cụ tiện ích Polkadot SDK
# ============================================================================
RUN cargo install --locked staging-chain-spec-builder@10.0.0 \
    && cargo install --locked polkadot-omni-node@0.5.0

# ============================================================================
# Environment variables cho RocksDB
# ============================================================================
ENV ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV SNAPPY_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV LZ4_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV ZSTD_LIB_DIR=/usr/lib/x86_64-linux-gnu
ENV LIBCLANG_PATH=/usr/lib/llvm-14/lib

# Tối ưu build
ENV CARGO_INCREMENTAL=0
ENV CARGO_NET_RETRY=10
ENV RUSTFLAGS="-C link-arg=-fuse-ld=lld"

# Cài đặt lld linker để tăng tốc linking
RUN apt-get update && apt-get install -y lld && rm -rf /var/lib/apt/lists/*

# ============================================================================
# Build Application
# ============================================================================
WORKDIR /app

# Copy toàn bộ source code
COPY . .

# Build với release profile
RUN cargo build --release --locked || cargo build --release

# ============================================================================
# Stage 2: Runtime (Lightweight)
# ============================================================================
FROM debian:bookworm-slim AS runtime

# Cài đặt runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    librocksdb7.8 \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 -U -s /bin/sh -d /node node

WORKDIR /node

# Copy binary từ builder stage
COPY --from=builder /app/target/release/dacs-node /usr/local/bin/

# Copy công cụ tiện ích
COPY --from=builder /usr/local/cargo/bin/chain-spec-builder /usr/local/bin/
COPY --from=builder /usr/local/cargo/bin/polkadot-omni-node /usr/local/bin/

# Copy chain spec nếu có
COPY --from=builder /app/raw_chain_spec.json /node/ 

# Đổi quyền sở hữu
RUN chown -R node:node /node

USER node

# Expose các ports
# P2P parachain
EXPOSE 40333
# RPC parachain
EXPOSE 8845
# P2P relay chain
EXPOSE 50343
# RPC relay chain  
EXPOSE 9988
# Prometheus
EXPOSE 9615

# Volume cho chain data
VOLUME ["/node/data"]

ENTRYPOINT ["polkadot-omni-node"]
CMD ["--help"]
