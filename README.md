# DoAnCoSo_Polkadot
DACS_02_Tìm hiểu ngôn ngữ lập trình Rust và nền tảng blockchain Polkadot.

> - [Mã nguồn](https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git) được lưu trữ tại: https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git
> - Tất cả các dòng lệnh cần được thực thi trên **Terminal** (Linux) hoặc **WSL** (Windows).

---

## Phương pháp 1: Sử dụng Docker (Khuyến nghị)

Docker giúp tránh các lỗi liên quan đến thư viện và đảm bảo môi trường nhất quán trên WSL, Ubuntu, và macOS.

### Yêu cầu
- **Docker** >= 20.10
- **Docker Compose** >= 2.0
- **RAM** >= 8GB (khuyến nghị 16GB)

### Cài đặt Docker

**Ubuntu/WSL:**
```bash
# Cài đặt Docker
curl -fsSL https://get.docker.com | sh
sudo usermod -aG docker $USER
newgrp docker

# Kiểm tra
docker --version
docker compose version
```

**macOS:**
- Tải và cài đặt [Docker Desktop](https://www.docker.com/products/docker-desktop/)

### Sử dụng nhanh

```bash
# Clone dự án
git clone https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git
cd DoAnCoSo_Polkadot

# Cấp quyền thực thi script
chmod +x scripts/docker-build.sh

# Xem tất cả commands
./scripts/docker-build.sh help
```

### Workflow đầy đủ với Docker

```bash
# 1. Vào môi trường development
./scripts/docker-build.sh dev

# 2. Build project (trong container hoặc bên ngoài)
./scripts/docker-build.sh build

# 3. Tạo chain spec (thay 4814 bằng Para ID của bạn)
./scripts/docker-build.sh chain-spec 4814

# 4. Chỉnh sửa plain_chain_spec.json theo hướng dẫn bên dưới

# 5. Chuyển đổi sang raw chain spec
./scripts/docker-build.sh chain-spec-raw

# 6. Xuất genesis files
./scripts/docker-build.sh export-genesis

# 7. Tạo node key
./scripts/docker-build.sh generate-node-key

# 8. Chạy collator
./scripts/docker-build.sh run-collator

# 9. Chèn session key
./scripts/docker-build.sh insert-key

# 10. Xem logs
./scripts/docker-build.sh logs collator
```

### Các lệnh Docker hữu ích

| Lệnh | Mô tả |
|------|-------|
| `./scripts/docker-build.sh dev` | Vào môi trường development |
| `./scripts/docker-build.sh build` | Build release binary |
| `./scripts/docker-build.sh test` | Chạy tests |
| `./scripts/docker-build.sh run-collator` | Chạy collator node |
| `./scripts/docker-build.sh stop-collator` | Dừng collator |
| `./scripts/docker-build.sh clean` | Xóa Docker resources |

---

## Phương pháp 2: Cài đặt thủ công (Native)

> **Lưu ý:** Nếu gặp lỗi RocksDB khi biên dịch, hãy sử dụng [Phương pháp 1: Docker](#-phương-pháp-1-sử-dụng-docker-khuyến-nghị).

### 1. Cài đặt các thành phần phụ thuộc (Dependencies)

Trước khi bắt đầu, đảm bảo hệ thống đã được cài đặt các thư viện cần thiết.

### a) Đối với Linux (Ubuntu)

```bash
# Cài đặt các gói build cơ bản
sudo apt install build-essential

# Cài đặt các thư viện phụ thuộc
sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler

# Cài đặt Rustup (Trình quản lý phiên bản Rust)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cập nhật biến môi trường
source $HOME/.cargo/env

# Cập nhật Rust và thêm target WASM
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

### b) Đối với Windows (WSL)

```bash
# Cài đặt WSL (nếu chưa có)
wsl --install

# Cập nhật danh sách gói
sudo apt update

# Cài đặt các thư viện phụ thuộc
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

# Cài đặt Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Cập nhật biến môi trường
source $HOME/.cargo/env

# Cập nhật Rust và thêm target WASM
rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src

# (Tùy chọn) Fix lỗi RocksDB - cài thêm các thư viện sau nếu gặp lỗi
sudo apt install --assume-yes librocksdb-dev liblz4-dev libzstd-dev libsnappy-dev libbz2-dev
```

### 2. Cài đặt toolchain

Cài đặt phiên bản Rust cụ thể (1.86.0) để đảm bảo tính tương thích.

```bash
rustup toolchain install 1.86.0
rustup default 1.86.0
rustup target add wasm32-unknown-unknown --toolchain 1.86.0
rustup component add rust-src --toolchain 1.86.0
```

### 3. Cài đặt công cụ tiện ích

- **Chain spec builder** (Tiện ích Polkadot SDK dùng để tạo thông số kỹ thuật chuỗi):

  ```bash
  cargo install --locked staging-chain-spec-builder@10.0.0
  ```

- **Polkadot Omni Node**: Tệp nhị phân nhãn trắng (white-labeled binary), được phát hành như một phần của bộ công cụ Polkadot SDK, có thể đóng vai trò là người thu thập (collator) cho một parachain trong môi trường sản xuất. Nó sở hữu tất cả các chức năng phụ trợ liên quan mà một nút collator thông thường có như: máy chủ RPC, lưu trữ trạng thái, v.v. Hơn nữa, nó cũng có thể chạy tệp wasm (wasm blob) của parachain ngay trên môi trường cục bộ để phục vụ cho việc kiểm thử và phát triển.

  ```bash
  cargo install --locked polkadot-omni-node@0.5.0
  ```

### 4. Cài đặt dự án

Sử dụng mã nguồn được lưu trữ trong thư mục `./DoAnCoSo_Polkadot` 
**Hoặc** Sao chép kho lưu trữ mẫu:

   ```bash
   git clone https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git

   # Điều hướng đến thư mục dự án:
   cd DoAnCoSo_Polkadot
   ```

Biên dịch runtime:

   ```bash
  # Điều hướng đến thư mục dự án:
   cargo build --release
   ```

> Ghi chú: Thời gian biên dịch phụ thuộc vào cấu hình máy (khoảng 20–90 phút).

---

## 5. Triển khai lên Testnet Paseo

### 5.1. Chuẩn bị tài khoản và token

1. Truy cập địa chỉ: https://polkadot.js.org/apps/?rpc=wss://paseo.dotters.network#/explorer.
   Đây cũng chính là giao diện Web chính thức của `Polkadot.js Apps`
2. Vào mục Accounts (tab Accounts → Accounts).
3. Chọn **From JSON** để nhập tài khoản trong `./DoAnCoSo_Polkadot/accounts` (password: `123456`).
4. Sao chép địa chỉ tài khoản vừa thêm bằng cách nhấp chuột vào tên tài khoản → Copy.
5. Nhận PAS test token tại https://faucet.polkadot.io/ với cấu hình:
   - Network: Polkadot testnet (Paseo)
   - Chain: Paseo Relay
   - Paseo Address: dán địa chỉ vừa sao chép, sau đó chọn **Get some PASs**
6. Sau khi nhận token, quý thầy/cô và các bạn có thể chọn một trong hai hướng thử nghiệm:
   - **Cách 1:** Sử dụng tài nguyên do nhóm chuẩn bị sẵn.
   - **Cách 2:** Đăng ký Para ID và Parathread riêng.

### 5.2. Cách 1 – Sử dụng tài nguyên sẵn có

**Bước 1.** Tạo khóa nút (node key). Khóa này chịu trách nhiệm giao tiếp với các nút khác qua Libp2p:

```bash
polkadot-omni-node key generate-node-key \
--base-path data \
--chain raw_chain_spec.json
```

**Bước 2.** Khởi động collator node:

```bash
polkadot-omni-node \
--collator \
--chain raw_chain_spec.json \
--base-path data \
--port 40333 \
--rpc-port 8845 \
--force-authoring \
--node-key-file ./data/chains/DACS-CTK47B/network/secret_ed25519 \
-- \
--sync warp \
--chain paseo \
--port 50343 \
--rpc-port 9988
```

**Bước 3.** Chèn session key vào collator:

```bash
curl -H "Content-Type: application/json" \
--data '{
  "jsonrpc":"2.0",
  "method":"author_insertKey",
  "params":[
    "aura",
    "soul light hawk decline crane deputy universe unable seven save keen clap",
    "0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27"
  ],
  "id":1
}' \
http://localhost:8845
```

Kết quả mong đợi: `{ "jsonrpc":"2.0", "result":null, "id":1 }`.

> **Session keys** giúp định danh collator khi sản xuất block và nên được xoay vòng thường xuyên để tránh rủi ro lộ khóa.

**Session key mẫu đã chuẩn bị:**

```
Secret phrase:       soul light hawk decline crane deputy universe unable seven save keen clap
Network ID:          substrate
Secret seed:         0x8f42e4e88d602039636018b11916a15b9429f4fd3040717267a3dbbc71981747
Public key (hex):    0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27
Account ID:          0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27
Public key (SS58):   5EJ1tAuSoUxKRKRYTcttRy7o9fBq96a58CTKuroZMF96tNSg
SS58 Address:        5EJ1tAuSoUxKRKRYTcttRy7o9fBq96a58CTKuroZMF96tNSg
```

### 5.3. Cách 2 – Para ID và Parathread riêng

**a) Đặt trước Para ID**

- Vào tab **Network → Parachains** trên Polkadot.js Apps.
- Chọn tab **Parathreads** và nhấp **+ ParaId**.
- Xác nhận giao dịch (lưu ý ParaId), nhấn **+ Submit**.
- Kiểm tra tab **Explorer** để tìm sự kiện `registrar.Reserved` xác nhận thành công.

**b) Chuẩn bị khóa cho collator**

- Sử dụng tài khoản đã nhập ở bước 5.1 làm account key.
- Tạo session key chuyên dụng:

  ```bash
  subkey generate --scheme sr25519
  ```
- Lưu toàn bộ thông số để tiếp tục tham khảo sau.

**c) Định nghĩa chain specification**

- plain chain spec giới thiệu cấu hình dễ đọc; raw chain spec dùng để khởi chạy node.
- Tạo plain chain spec:

  ```bash
  chain-spec-builder \
  --chain-spec-path ./plain_chain_spec.json \
  create \
  --relay-chain paseo \
  --para-id INSERT_PARA_ID \
  --runtime target/release/wbuild/dacs-runtime/dacs_runtime.compact.compressed.wasm \
  named-preset local_testnet
  ```

- Chỉnh sửa `plain_chain_spec.json`:

  1. Đặt `name` và `id` thành giá trị duy nhất.
  2. Cập nhật `para_id` và `parachainInfo.parachainId` bằng Para ID đã đăng ký (dạng số).
  3. Khai báo số dư khởi tạo trong `balances` bằng địa chỉ SS58 đã chuẩn bị.
  4. Thêm địa chỉ collator và session key vào `collatorSelection.invulnerables` và `session.keys.aura` (đều là SS58).
  5. Chỉ định tài khoản sudo trong trường `sudo`.

- Kết quả sẽ tương tự như phía dưới, hoặc tham khảo trong `./plain_chain_spec.json`:
```bash
{
    "bootNodes": [],
    "chainType": "Live",
    "codeSubstitutes": {},
    "genesis": {
        "runtimeGenesis": {
            "code": "0x...",
            "patch": {
                "aura": {
                    "authorities": []
                },
                "auraExt": {},
                "balances": {
                    "balances": [["INSERT_SUDO_ACCOUNT", 1152921504606846976]]
                },
                "collatorSelection": {
                    "candidacyBond": 16000000000,
                    "desiredCandidates": 0,
                    "invulnerables": ["INSERT_ACCOUNT_ID_COLLATOR_1"]
                },
                "parachainInfo": {
                    "parachainId": "INSERT_PARA_ID"
                },
                "parachainSystem": {},
                "polkadotXcm": {
                    "safeXcmVersion": 5
                },
                "session": {
                    "keys": [
                        [
                            "INSERT_ACCOUNT_ID_COLLATOR_1",
                            "INSERT_ACCOUNT_ID_COLLATOR_1",
                            {
                                "aura": "INSERT_SESSION_KEY_COLLATOR_1"
                            }
                        ]
                    ],
                    "nonAuthorityKeys": []
                },
                "sudo": {
                    "key": "INSERT_SUDO_ACCOUNT"
                },
                "system": {},
                "transactionPayment": {
                    "multiplier": "1000000000000000000"
                }
            }
        }
    },
    "id": "INSERT_ID",
    "name": "INSERT_NAME",
    "para_id": "INSERT_PARA_ID",
    "properties": {
        "tokenDecimals": 12,
        "tokenSymbol": "UNIT"
    },
    "protocolId": "INSERT_PROTOCOL_ID",
    "relay_chain": "paseo",
    "telemetryEndpoints": null
}
```

- Chuyển đổi tập tin `plain_chain_spec.json` đã sửa đổi thành tập tin `raw_chain_spec.json` (tệp thông số kỹ thuật chuỗi thô):

  ```bash
  chain-spec-builder \
  --chain-spec-path ./raw_chain_spec.json \
  convert-to-raw plain_chain_spec.json
  ```

**d) Xuất runtime Wasm và genesis state**

```bash
polkadot-omni-node export-genesis-wasm \
--chain raw_chain_spec.json para-wasm

polkadot-omni-node export-genesis-head \
--chain raw_chain_spec.json para-state
```

**e) Đăng ký Parathread**

- Tại tab **Parachains → Parathreads**, nhấp **+ Parathread**.
- Cung cấp Para ID cùng hai tệp `para-wasm` và `para-state` vừa xuất.
- Kiểm tra thông tin, nhấn **+ Submit** để tạo Parathread mới (nút Deregister sẽ xuất hiện sau khi đăng ký).
- Thời gian tích hợp thành Parathread hoàn chỉnh có thể kéo dài ~2 giờ.

**f) Bắt đầu nút Collator**
Bước này sẽ tương tự như bước 5.2.

- Tạo khóa nút (node key):

```bash
polkadot-omni-node key generate-node-key \
--base-path data \
--chain raw_chain_spec.json
```

- Khởi động collator node:

```bash
polkadot-omni-node \
--collator \
--chain raw_chain_spec.json \
--base-path data \
--port 40333 \
--rpc-port 8845 \
--force-authoring \
--node-key-file ./data/chains/DACS-CTK47B/network/secret_ed25519 \
-- \
--sync warp \
--chain paseo \
--port 50343 \
--rpc-port 9988
```

- Chèn session key vào collator (Quan trọng):
Thay thế `INSERT_SECRET_PHRASE` và `INSERT_PUBLIC_KEY_HEX_FORMAT` bằng các giá trị từ session key đã tạo trong phần 5.3.b)

```bash
curl -H "Content-Type: application/json" \
--data '{
  "jsonrpc":"2.0",
  "method":"author_insertKey",
  "params":[
    "aura",
    "INSERT_SECRET_PHRASE",
    "INSERT_PUBLIC_KEY_HEX_FORMAT"
  ],
  "id":1
}' \
http://localhost:8845
```

Kết quả mong đợi: `{ "jsonrpc":"2.0", "result":null, "id":1 }`.

Sau khi collator được đồng bộ hóa với chuỗi chuyển tiếp (RelayChain) Paseo và Parathread hoàn tất việc tích hợp, khi đó sẽ sẵn sàng để bắt đầu tạo khối.

### 5.4. Cấu hình Coretime
Sau khi triển khai thành công parachain lên Paseo TestNet, bước tiếp theo là cấu hình **Coretime**. Đây là cơ chế phân bổ tài nguyên xác thực từ Relay Chain cho các tác vụ cụ thể, ví dụ như parachain. Một parachain chỉ có thể sản xuất và hoàn tất khối (finalize) trên Relay Chain khi đã có được Coretime.

Có hai phương thức để sở hữu Coretime:

* **On-demand coretime (Theo nhu cầu):** Cho phép mua Coretime theo từng khối (block-by-block).
* **Bulk coretime (Số lượng lớn):** Cho phép sở hữu toàn bộ hoặc một phần của một lõi (core). Loại này được mua theo khoảng thời gian, tối đa lên đến 28 ngày và cần được gia hạn khi hết hạn thuê.

#### Hướng dẫn sử dụng On-demand Coretime

**Bước 1.** Truy cập Polkadot.js Apps kết nối với **Paseo Relay Chain**:

- URL: https://polkadot.js.org/apps/?rpc=wss://paseo.dotters.network#/extrinsics

**Bước 2.** Đảm bảo tài khoản có đủ PAS token:

- Kiểm tra số dư tại tab **Accounts**
- Nếu cần thêm token, sử dụng [Faucet](https://faucet.polkadot.io/) (đã hướng dẫn ở bước 5.1)

**Bước 3.** Đặt On-demand Order:

1. Vào tab **Developer → Extrinsics**
2. Chọn tài khoản có PAS token
3. Chọn extrinsic: `onDemand` → `placeOrderKeepAlive`
4. Điền các tham số:
   - `maxAmount`: Số PAS tối đa bạn sẵn sàng trả (ví dụ: `1000000000000` = 1 PAS)
   - `paraId`: Para ID của parachain (`5100` cho bước `5.2 Cách 1 – Sử dụng tài nguyên sẵn có` hoặc Para ID đã đăng ký trong bước `5.3 Cách 2 – Para ID và Parathread riêng`)
5. Nhấn **Submit Transaction** và ký giao dịch

```
┌─────────────────────────────────────────────────────────────┐
│  Extrinsic: onDemand.placeOrderKeepAlive                    │
├─────────────────────────────────────────────────────────────┤
│  maxAmount: 1000000000000 (1 PAS)                           │
│  paraId: 5100                                               │
└─────────────────────────────────────────────────────────────┘
```

**Bước 4.** Kiểm tra kết quả:

- Vào tab **Network → Explorer**
- Tìm sự kiện `onDemand.OnDemandOrderPlaced` để xác nhận order thành công
- Sau khi order được xử lý, parachain sẽ được phép tạo và xác thực một block

> **Lưu ý quan trọng:**
> - Mỗi order chỉ có hiệu lực cho một block
> - Order sẽ được xử lý theo thứ tự hàng đợi (queue)

### 5.5. Demo: Thực hiện giao dịch với Blog Pallet

Phần này hướng dẫn cách demo việc tạo giao dịch trên parachain và sử dụng on-demand coretime để đưa giao dịch lên block.

#### Chuẩn bị: Mở 2 tab Polkadot.js Apps

| Tab | Kết nối đến | Mục đích |
|-----|-------------|----------|
| **Tab 1** | `wss://paseo.dotters.network` (Paseo Relay Chain) | Mua on-demand coretime |
| **Tab 2** | `ws://localhost:8845` (Parachain local) | Thực hiện giao dịch blog-pallet |

**Mở Tab 1 - Paseo Relay Chain:**
```
https://polkadot.js.org/apps/?rpc=wss://paseo.dotters.network#/extrinsics
```

**Mở Tab 2 - Parachain Local:**
```
https://polkadot.js.org/apps/?rpc=ws://localhost:8845#/extrinsics
```

#### Bước 1: Tạo giao dịch Blog Pallet (Tab 2 - Parachain)

1. Chuyển sang **Tab 2** (kết nối `ws://localhost:8845`)
2. Vào **Developer → Extrinsics**
3. Chọn tài khoản có token (ví dụ: tài khoản sudo)
4. Chọn extrinsic: `blog` → chọn hàm muốn gọi, ví dụ:
   - `createPost(title, content)` - Tạo bài viết mới
   - `updatePost(postId, title, content)` - Cập nhật bài viết
   - `deletePost(postId)` - Xóa bài viết
5. Điền các tham số cần thiết
6. Nhấn **Submit Transaction** và ký giao dịch

```
┌─────────────────────────────────────────────────────────────┐
│  Tab 2: Parachain (ws://localhost:8845)                     │
├─────────────────────────────────────────────────────────────┤
│  Extrinsic: blog.createPost                                 │
│  title: "Hello Polkadot"                                    │
│  content: "This is my first blog post on parachain!"        │
└─────────────────────────────────────────────────────────────┘
```

7. Sau khi submit, giao dịch sẽ ở trạng thái **Ready** (chờ xử lý)
   - Kiểm tra tại **Network → Explorer** - giao dịch xuất hiện trong pending/ready pool
   - Giao dịch chưa được đưa vào block vì parachain chưa có coretime

#### Bước 2: Mua On-demand Coretime (Tab 1 - Relay Chain)

1. Chuyển sang **Tab 1** (kết nối `wss://paseo.dotters.network`)
2. Vào **Developer → Extrinsics**
3. Chọn tài khoản có PAS token
4. Chọn extrinsic: `onDemand` → `placeOrderKeepAlive`
5. Điền tham số:
   - `maxAmount`: `1000000000000` (1 PAS)
   - `paraId`: `5100` (hoặc Para ID của bạn)
6. Nhấn **Submit Transaction** và ký giao dịch

```
┌─────────────────────────────────────────────────────────────┐
│  Tab 1: Paseo Relay Chain (wss://paseo.dotters.network)     │
├─────────────────────────────────────────────────────────────┤
│  Extrinsic: onDemand.placeOrderKeepAlive                    │
│  maxAmount: 1000000000000 (1 PAS)                           │
│  paraId: 5100                                               │
└─────────────────────────────────────────────────────────────┘
```

#### Bước 3: Xác nhận giao dịch được đưa lên block

1. **Tại Tab 1 (Relay Chain):**
   - Chờ sự kiện `onDemand.OnDemandOrderPlaced` xuất hiện trong Explorer
   - Điều này xác nhận parachain đã được cấp slot để tạo block

2. **Tại Tab 2 (Parachain):**
   - Chuyển sang **Network → Explorer**
   - Quan sát block mới được tạo (block number tăng lên)
   - Giao dịch `blog.createPost` xuất hiện trong block với trạng thái `Success`
   - Kiểm tra sự kiện `blog.PostCreated` (hoặc event tương ứng)

```
┌─────────────────────────────────────────────────────────────┐
│  Kết quả trên Tab 2 - Parachain Explorer                    │
├─────────────────────────────────────────────────────────────┤
│  Block #123                                                 │
│  ├─ Extrinsics:                                             │
│  │  └─ blog.createPost       #Success                       │
│  └─ Events:                                                 │
│     └─ blog.PostCreated                                     │
│        - postId: 0                                          │
│        - author: 5GrwvaEF...                                │
└─────────────────────────────────────────────────────────────┘
```

#### Bước 4: Kiểm tra dữ liệu đã lưu (Tùy chọn)

1. Tại **Tab 2**, vào **Developer → Chain State**
2. Chọn: `blog` → `posts` (hoặc storage tương ứng)
3. Nhập `postId` (ví dụ: `0`) và nhấn **+**
4. Kết quả hiển thị nội dung bài viết đã tạo

#### Tóm tắt quy trình Demo

```
┌──────────────────┐      ┌──────────────────┐
│  Tab 2:          │      │   Tab 1:         │
│  Parachain       │      │   Relay Chain    │
│  localhost:8845  │      │   Paseo          │
└────────┬─────────┘      └────────┬─────────┘
         │                         │
    ┌────▼──────────────────┐      │
    │ 1. Tạo giao dịch blog │      │
    └────┬──────────────────┘      │
         │                         │
         │ Giao dịch ở             │
         │ trạng thái              │
         │ "Ready"                 │
         │                    ┌────▼──────────────────────┐
         │                    │ 2. Mua on-demand coretime │
         │                    └────┬──────────────────────┘
         │                         │
         │◄────────────────────────┤
         │   Coretime được cấp 
         │
    ┌────▼────────────────────────────┐
    │ 3. Giao dịch được đưa lên block │
    └─────────────────────────────────┘
```

> **Mẹo demo:**
> - Mở 2 cửa sổ browser cạnh nhau để dễ theo dõi
> - Tạo giao dịch blog trước, sau đó mới mua coretime để thấy rõ sự liên kết
> - Có thể tạo nhiều giao dịch pending, sau đó mua coretime một lần để xử lý tất cả

---

## Troubleshooting

### Lỗi RocksDB trên Ubuntu/WSL

```bash
# Cài đặt đầy đủ dependencies
sudo apt install --assume-yes \
    librocksdb-dev \
    libclang-dev \
    clang \
    llvm \
    liblz4-dev \
    libzstd-dev \
    libsnappy-dev \
    libbz2-dev

# Hoặc sử dụng Docker (khuyến nghị)
./scripts/docker-build.sh build
```

### Lỗi "out of memory" khi build

```bash
# Tăng swap space
sudo fallocate -l 8G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# Hoặc với Docker, chỉnh memory limit trong docker-compose.yml
```

### Tốc độ build chậm

```bash
# Sử dụng cargo check trước
cargo check --release

# Hoặc dùng Docker với cache
./scripts/docker-build.sh build
```

---

## Thành viên nhóm

| MSSV | Họ và tên | Email |
|------|-----------|-------|
| 2312622 | Nguyễn Đức Hoàng | 2312622@dlu.edu.vn |
| 2312635 | Trần Đình Việt Huy | 2312635@dlu.edu.vn |
| 2312675 | Hồ Quốc Long | 2312675@dlu.edu.vn |
| 2312702 | Nguyễn Bá Thiều Khôi Nguyên | 2312702@dlu.edu.vn |

---

## Tài liệu tham khảo:
- [Polkadot](https://docs.polkadot.com/)
