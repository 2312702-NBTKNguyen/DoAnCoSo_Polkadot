#!/bin/bash
# ============================================================================
# Script build với Docker - Hỗ trợ WSL, Ubuntu, macOS
# ============================================================================

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
info() { echo -e "${BLUE}[INFO]${NC} $1"; }
success() { echo -e "${GREEN}[SUCCESS]${NC} $1"; }
warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Detect OS
detect_os() {
    if [[ "$OSTYPE" == "linux-gnu"* ]]; then
        if grep -qi microsoft /proc/version 2>/dev/null; then
            echo "wsl"
        else
            echo "linux"
        fi
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        echo "macos"
    else
        echo "unknown"
    fi
}

OS=$(detect_os)
info "Detected OS: $OS"

# Check Docker
if ! command -v docker &> /dev/null; then
    error "Docker chưa được cài đặt. Vui lòng cài đặt Docker trước."
fi

# Check Docker Compose
if ! docker compose version &> /dev/null; then
    if ! docker-compose version &> /dev/null; then
        error "Docker Compose chưa được cài đặt."
    fi
    COMPOSE_CMD="docker-compose"
else
    COMPOSE_CMD="docker compose"
fi

# Parse arguments
ACTION=${1:-"dev"}

case $ACTION in
    # =========================================================================
    # Vào môi trường development
    # =========================================================================
    "dev"|"shell")
        info "Khởi động môi trường development..."
        $COMPOSE_CMD run --rm dev bash
        ;;

    # =========================================================================
    # Build trong container
    # =========================================================================
    "build")
        info "Building trong Docker container..."
        $COMPOSE_CMD run --rm dev cargo build --release
        success "Build hoàn tất! Binary tại: target/release/dacs-node"
        ;;

    # =========================================================================
    # Build với cache clean
    # =========================================================================
    "build-clean")
        info "Cleaning và rebuilding..."
        $COMPOSE_CMD run --rm dev cargo clean
        $COMPOSE_CMD run --rm dev cargo build --release
        success "Clean build hoàn tất!"
        ;;

    # =========================================================================
    # Chạy tests
    # =========================================================================
    "test")
        info "Chạy tests..."
        $COMPOSE_CMD run --rm dev cargo test --all
        success "Tests hoàn tất!"
        ;;

    # =========================================================================
    # Check code
    # =========================================================================
    "check")
        info "Checking code..."
        $COMPOSE_CMD run --rm dev cargo check --all
        success "Check hoàn tất!"
        ;;

    # =========================================================================
    # Format code
    # =========================================================================
    "fmt")
        info "Formatting code..."
        $COMPOSE_CMD run --rm dev cargo fmt --all
        success "Format hoàn tất!"
        ;;

    # =========================================================================
    # Build Docker image production
    # =========================================================================
    "image")
        info "Building Docker image..."
        docker build -t dacs-node:latest .
        success "Docker image built: dacs-node:latest"
        ;;

    # =========================================================================
    # Tạo chain spec (Bước 5.3.c trong README)
    # =========================================================================
    "chain-spec")
        PARA_ID=${2:-"4814"}
        info "Tạo chain spec với Para ID: $PARA_ID..."
        $COMPOSE_CMD run --rm dev chain-spec-builder \
            --chain-spec-path ./plain_chain_spec.json \
            create \
            --relay-chain paseo \
            --para-id "$PARA_ID" \
            --runtime target/release/wbuild/dacs-runtime/dacs_runtime.compact.compressed.wasm \
            named-preset local_testnet
        success "Đã tạo plain_chain_spec.json. Hãy chỉnh sửa file này theo hướng dẫn README."
        ;;

    # =========================================================================
    # Chuyển đổi chain spec sang raw format
    # =========================================================================
    "chain-spec-raw")
        info "Chuyển đổi plain_chain_spec.json sang raw_chain_spec.json..."
        $COMPOSE_CMD run --rm dev chain-spec-builder \
            --chain-spec-path ./raw_chain_spec.json \
            convert-to-raw plain_chain_spec.json
        success "Đã tạo raw_chain_spec.json!"
        ;;

    # =========================================================================
    # Xuất genesis wasm và state (Bước 5.3.d trong README)
    # =========================================================================
    "export-genesis")
        info "Xuất genesis wasm và state..."
        $COMPOSE_CMD run --rm dev polkadot-omni-node export-genesis-wasm \
            --chain raw_chain_spec.json para-wasm
        $COMPOSE_CMD run --rm dev polkadot-omni-node export-genesis-head \
            --chain raw_chain_spec.json para-state
        success "Đã xuất para-wasm và para-state!"
        ;;

    # =========================================================================
    # Tạo node key (Bước 5.2 Bước 1 trong README)
    # =========================================================================
    "generate-node-key")
        info "Tạo node key..."
        $COMPOSE_CMD run --rm dev polkadot-omni-node key generate-node-key \
            --base-path data \
            --chain raw_chain_spec.json
        success "Đã tạo node key trong thư mục data/"
        ;;

    # =========================================================================
    # Tạo session key mới (Bước 5.3.b trong README)
    # =========================================================================
    "generate-session-key")
        info "Tạo session key mới..."
        $COMPOSE_CMD run --rm dev subkey generate --scheme sr25519
        ;;

    # =========================================================================
    # Chạy collator node trên Paseo testnet (Bước 5.2 Bước 2 trong README)
    # =========================================================================
    "run-collator")
        info "Khởi động collator node trên Paseo testnet..."
        $COMPOSE_CMD up -d collator
        success "Collator đang chạy! RPC tại http://localhost:8845"
        info "Xem logs: docker compose logs -f collator"
        ;;

    # =========================================================================
    # Dừng collator
    # =========================================================================
    "stop-collator")
        info "Dừng collator node..."
        $COMPOSE_CMD stop collator
        success "Collator đã dừng!"
        ;;

    # =========================================================================
    # Chèn session key vào collator (Bước 5.2 Bước 3 trong README)
    # =========================================================================
    "insert-key")
        SECRET_PHRASE=${2:-"soul light hawk decline crane deputy universe unable seven save keen clap"}
        PUBLIC_KEY=${3:-"0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27"}
        info "Chèn session key vào collator..."
        curl -H "Content-Type: application/json" \
            --data "{
              \"jsonrpc\":\"2.0\",
              \"method\":\"author_insertKey\",
              \"params\":[
                \"aura\",
                \"$SECRET_PHRASE\",
                \"$PUBLIC_KEY\"
              ],
              \"id\":1
            }" \
            http://localhost:8845
        echo ""
        success "Đã gửi request chèn session key!"
        ;;

    # =========================================================================
    # Chạy node dev mode (local testing)
    # =========================================================================
    "run-node")
        info "Khởi động DACS node (dev mode)..."
        $COMPOSE_CMD up node
        ;;

    # =========================================================================
    # Cleanup
    # =========================================================================
    "clean")
        warning "Xóa tất cả Docker resources..."
        $COMPOSE_CMD down -v --rmi local
        docker volume rm dacs-cargo-registry dacs-cargo-git dacs-cargo-target dacs-node-data dacs-collator-data 2>/dev/null || true
        success "Cleanup hoàn tất!"
        ;;

    # =========================================================================
    # Logs
    # =========================================================================
    "logs")
        SERVICE=${2:-"collator"}
        $COMPOSE_CMD logs -f "$SERVICE"
        ;;

    # =========================================================================
    # Help
    # =========================================================================
    *)
        echo ""
        echo "╔══════════════════════════════════════════════════════════════════╗"
        echo "║           DoAnCoSo - Docker Build Script                         ║"
        echo "╚══════════════════════════════════════════════════════════════════╝"
        echo ""
        echo "Usage: $0 [command] [options]"
        echo ""
        echo "🔧 Development Commands:"
        echo "  dev, shell           - Vào môi trường development"
        echo "  build                - Build release binary"
        echo "  build-clean          - Clean và build lại từ đầu"
        echo "  test                 - Chạy tất cả tests"
        echo "  check                - Check code không build"
        echo "  fmt                  - Format code với rustfmt"
        echo ""
        echo "🔗 Chain Spec Commands (Bước 5.3):"
        echo "  chain-spec [PARA_ID] - Tạo plain_chain_spec.json"
        echo "  chain-spec-raw       - Chuyển đổi sang raw_chain_spec.json"
        echo "  export-genesis       - Xuất para-wasm và para-state"
        echo ""
        echo "🔑 Key Management (Bước 5.2, 5.3.b):"
        echo "  generate-node-key    - Tạo node key cho Libp2p"
        echo "  generate-session-key - Tạo session key mới"
        echo "  insert-key [PHRASE] [PUBKEY] - Chèn session key vào collator"
        echo ""
        echo "🚀 Run Commands (Bước 5.2):"
        echo "  run-collator         - Chạy collator trên Paseo testnet"
        echo "  stop-collator        - Dừng collator"
        echo "  run-node             - Chạy node dev mode (local)"
        echo "  logs [service]       - Xem logs (collator/node)"
        echo ""
        echo "🐳 Docker Commands:"
        echo "  image                - Build Docker image production"
        echo "  clean                - Xóa tất cả Docker resources"
        echo ""
        echo "📋 Ví dụ workflow đầy đủ:"
        echo "  $0 build                    # 1. Build project"
        echo "  $0 chain-spec 4814          # 2. Tạo chain spec"
        echo "  $0 chain-spec-raw           # 3. Chuyển sang raw"
        echo "  $0 generate-node-key        # 4. Tạo node key"
        echo "  $0 run-collator             # 5. Chạy collator"
        echo "  $0 insert-key               # 6. Chèn session key"
        echo ""
        ;;
esac
