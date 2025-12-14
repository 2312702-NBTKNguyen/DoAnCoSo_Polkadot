# Docker Build Guide - DoAnCoSo

Hướng dẫn sử dụng Docker để biên dịch dự án, tránh lỗi RocksDB trên mọi nền tảng.

## Yêu cầu

- **Docker** >= 20.10
- **Docker Compose** >= 2.0
- **RAM** >= 8GB (khuyến nghị 16GB)
- **Disk** >= 20GB trống

## Cách sử dụng

### Cách 1: Sử dụng Script (Khuyến nghị)

```bash
# Cấp quyền thực thi (chỉ cần 1 lần)
chmod +x scripts/docker-build.sh

# Vào môi trường development
./scripts/docker-build.sh dev

# Build release binary
./scripts/docker-build.sh build

# Chạy tests
./scripts/docker-build.sh test

# Xem tất cả commands
./scripts/docker-build.sh help
```

### Cách 2: Sử dụng Docker Compose trực tiếp

```bash
# Vào shell development
docker compose run --rm dev bash

# Trong container, build như bình thường:
cargo build --release
cargo test --all
```

### Cách 3: Build Docker Image

```bash
# Build image production
docker build -t dacs-node:latest .

# Chạy node
docker run -p 9944:9944 -p 30333:30333 dacs-node:latest --dev
```

## Cấu trúc Docker Files

```
DoAnCoSo/
├── Dockerfile          # Multi-stage build cho production
├── Dockerfile.dev      # Development environment
├── docker-compose.yml  # Orchestration
├── .dockerignore       # Tối ưu build context
└── scripts/
    └── docker-build.sh # Helper script
```

## 💻 Hỗ trợ các nền tảng

| Nền tảng | Trạng thái |
|----------|------------|
| WSL2 (Ubuntu) | ✅ |
| Ubuntu 22.04+ | ✅ |
| macOS (Intel) | ✅ |
| macOS (Apple Silicon) | ✅ |

### Lưu ý cho macOS Apple Silicon (M1/M2/M3)

```bash
# Build với platform linux/amd64
docker build --platform linux/amd64 -t dacs-node:latest .
```

## 📊 Tối ưu Build

### Cache Cargo Dependencies

Docker Compose đã cấu hình volumes để cache:
- `cargo-registry`: Cache crates.io packages
- `cargo-git`: Cache git dependencies
- `cargo-target`: Cache build artifacts

### Tăng tốc với LLD Linker

Dockerfiles sử dụng `lld` linker thay vì `ld` mặc định:
```bash
RUSTFLAGS="-C link-arg=-fuse-ld=lld"
```

## 🧹 Cleanup

```bash
# Xóa containers và volumes
./scripts/docker-build.sh clean

# Hoặc thủ công:
docker compose down -v --rmi local
docker volume prune
```

## Troubleshooting

### Lỗi "out of memory"

Tăng memory cho Docker:
- **Docker Desktop**: Settings > Resources > Memory >= 8GB
- **WSL2**: Tạo file `~/.wslconfig`:
  ```ini
  [wsl2]
  memory=12GB
  swap=4GB
  ```

### Build quá chậm

1. Đảm bảo đang dùng volume mounts (không bind mounts cho target/)
2. Tăng CPU cores cho Docker
3. Sử dụng `cargo check` trước khi `cargo build`

### Permission denied

```bash
# Fix ownership
sudo chown -R $USER:$USER .
```