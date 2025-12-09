# DoAnCoSo_Polkadot
DACS_02_Tìm hiểu ngôn ngữ lập trình Rust và nền tảng blockchain Polkadot.

> - [Mã nguồn](https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git) được lưu trữ tại: https://github.com/2312702-NBTKNguyen/DoAnCoSo_Polkadot.git
> - Toàn bộ hướng dẫn bên dưới được tham khảo từ [Tài liệu chính thức của Polkadot](https://docs.polkadot.com/). 
> - Tất cả các dòng lệnh cần được thực thi trên **Terminal** (Linux) hoặc **WSL** (Windows).

## 1. Cài đặt các thành phần phụ thuộc (Dependencies)

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
```

## 2. Cài đặt toolchain

Cài đặt phiên bản Rust cụ thể (1.86.0) để đảm bảo tính tương thích.

```bash
rustup toolchain install 1.86.0
rustup default 1.86.0
rustup target add wasm32-unknown-unknown --toolchain 1.86.0
rustup component add rust-src --toolchain 1.86.0
```

## 3. Cài đặt công cụ tiện ích

- **Chain spec builder** (Tiện ích Polkadot SDK dùng để tạo thông số kỹ thuật chuỗi):

  ```bash
  cargo install --locked staging-chain-spec-builder@10.0.0
  ```

- **Polkadot Omni Node**: Tệp nhị phân nhãn trắng (white-labeled binary), được phát hành như một phần của bộ công cụ Polkadot SDK, có thể đóng vai trò là người thu thập (collator) cho một parachain trong môi trường sản xuất. Nó sở hữu tất cả các chức năng phụ trợ liên quan mà một nút collator thông thường có như: máy chủ RPC, lưu trữ trạng thái, v.v. Hơn nữa, nó cũng có thể chạy tệp wasm (wasm blob) của parachain ngay trên môi trường cục bộ để phục vụ cho việc kiểm thử và phát triển.

  ```bash
  cargo install --locked polkadot-omni-node@0.5.0
  ```

## 4. Cài đặt dự án

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

### 5.4. 