# DoAnCoSo
DACS_02_Tìm hiểu ngôn ngữ lập trình Rust và nền tảng blockchain Polkadot
Toàn bộ hướng dẫn bên dưới đều tham khảo từ trang chủ chính thức của Polkadot: https://docs.polkadot.com/
Tất cả các dòng lệnh cần phải triển khai trên Terminal của hệ thống

1. Cài đặt phụ thuộc của Polkadot SDK
a) Linux (Ubuntu)
sudo apt install build-essential

sudo apt install --assume-yes git clang curl libssl-dev protobuf-compiler

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env

rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src

b) Windows (WSL)
wsl --install

sudo apt update

sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source ~/.cargo/env

rustup default stable
rustup update
rustup target add wasm32-unknown-unknown
rustup component add rust-src

2. Cài đặt toolchain
rustup toolchain install 1.86.0
rustup default 1.86.0
rustup target add wasm32-unknown-unknown --toolchain 1.86.0
rustup component add rust-src --toolchain 1.86.0

3. Cài đặt công cụ tiện ích
a) Chain spec builder: Tiện ích SDK Polkadot để tạo thông số kỹ thuật chuỗi.
cargo install --locked staging-chain-spec-builder@10.0.0
b) Polkadot Omni Node: A white-labeled binary, released as a part of Polkadot SDK that can act as the collator of a parachain in production, with all the related auxiliary functionalities that a normal collator node has: RPC server, archiving state, etc. Moreover, it can also run the wasm blob of the parachain locally for testing and development.
cargo install --locked polkadot-omni-node@0.5.0

4. Cài đặt dự án
a) Sao chép kho lưu trữ mẫu:
git clone https://github.com/2312702-NBTKNguyen/DoAnCoSo.git
b) Điều hướng đến thư mục dự án:
cd DoAnCoSo
c) Biên dịch Runtime:
cargo build --release
*Notes: Thời gian biên dịch phụ thuộc vào cấu hình của máy, thông thường sẽ mất khoảng 20 - 90p tùy vào điều kiện thực tế.

5. Triển khai lên Testnet Paseo
a) Bắt đầu với 1 tài khoản và tokens:
B1: Chuẩn bị tài khoản
1. Truy cập https://polkadot.js.org/apps/?rpc=wss://paseo.dotters.network#/explorer
2. Điều hướng đến phần Accounts :
- Nhấp vào tab Accounts ở menu trên cùng.
- Chọn tùy chọn Accounts từ menu thả xuống.
4. Nhấp vào nút From JSON để thêm tài khoản đã được chuẩn bị sẵn tại thư mục ./DoAnCoSo/accounts; Mật khẩu: 123456
5. Sau khi thêm tài khoản thành công, bấm vào tài khoản để sao chép địa chỉ của tài khoản
6. Để có thể thực hiện bất kỳ hành động nào như tương tác và giao dịch trên Testnet, cần có token PAS dùng cho mục đích thử nghiệm. 
Truy cập https://faucet.polkadot.io/, chọn thông tin các trường, bao gồm:
- Network: Polkadot testnet (Paseo)
- Chain: Paseo Relay
- Paseo Address: dán địa chỉ tài khoản đã sao chép, sau đó nhấn nút Get some PASs
Sau vài giây, 5000 tokens PAS sẽ được thêm vào tài khoản.
7. Từ đây, có 2 hướng để quý thầy/cô và các bạn có thể thử nghiệm đồ án: sử dụng tài nguyên nhóm đã chuẩn bị sẵn, hoặc thử nghiệm trên 1 Parachain Identifier(Para ID) cùng với Parathread của riêng mình.
Cách 1: Sử dụng tài nguyên nhóm đã chuẩn bị sẵn:
- Bắt đầu với 1 node Collator: Trước khi bắt đầu 1 node Collator, cần tạo một khóa nút (node key). Khóa này chịu trách nhiệm giao tiếp với các nút khác qua Libp2p

polkadot-omni-node key generate-node-key \
--base-path data \
--chain raw_chain_spec.json

- Sau đó, bắt đầu node Collator thông qua chuỗi lệnh sau:

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

- Sau khi node Collator đã khởi chạy thành công, bước tiếp theo là chèn khóa phiên (Session key*) vào node Collator. Mở một cửa sổ Terminal mới và nhập chuỗi lệnh sau:

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

Kết quả của đoạn lệnh trên sẽ tương tự như thế này: {"jsonrpc":"2.0","result":null,"id":1}

*Session keys (Khóa phiên): Used in block production to identify your node and its blocks on the network. These keys are stored in the parachain keystore and function as disposable "hot wallet" keys. If these keys are leaked, someone could impersonate your node, which could result in the slashing of your funds. To minimize these risks, rotating your session keys frequently is essential. Treat them with the same level of caution as you would a hot wallet to ensure the security of your node.

*Đây là Session key đã được chuẩn bị sẵn:
Secret phrase:       soul light hawk decline crane deputy universe unable seven save keen clap
  Network ID:        substrate
  Secret seed:       0x8f42e4e88d602039636018b11916a15b9429f4fd3040717267a3dbbc71981747
  Public key (hex):  0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27
  Account ID:        0x629f56793f8b376c490da7c63a704f186fb60d44e0a4145eb19ea0b93506dd27
  Public key (SS58): 5EJ1tAuSoUxKRKRYTcttRy7o9fBq96a58CTKuroZMF96tNSg
  SS58 Address:      5EJ1tAuSoUxKRKRYTcttRy7o9fBq96a58CTKuroZMF96tNSg

Cách 2: Thử nghiệm trên 1 Parachain Identifier(Para ID) cùng với Parathread của riêng mình.
a) Đặt trước 1 mã định danh Parachain (Parachain Identifier - Para ID):
- Điều hướng đến phần Parachains:
    Nhấp vào tab Network ở menu trên cùng.
    Chọn tùy chọn Parachains từ menu thả xuống.
- Đăng ký ParaId:
    Chọn tab Parathreads.
    Nhấp vào nút "+ ParaId".
- Xem lại giao dịch (đặc biệt là ParaId) và nhấp vào nút "+ Submit".
- Sau khi gửi giao dịch, có thể điều hướng đến tab Explorer và kiểm tra danh sách các sự kiện gần đây để biết giao dịch thành công thông qua sự kiện "registrar.Reserved".

b) Tạo khóa phiên (Session key) cho node Collator: Để triển khai parachain một cách an toàn, đây là điều cần thiết là tạo khóa tùy chỉnh dành riêng cho các Collator (nhà sản xuất khối). Ở đây, sẽ cần 2 bộ khóa cho mỗi collator:
- Account keys: Được sử dụng để tương tác với mạng và quản lý tiền. Ở đây, để thuận tiện, quý thầy/cô và các bạn nên sử dụng địa chỉ tài khoản đã được thêm trong Polkadot.js Apps ở mục 4.
- Session key: Tương tự như đã giải thích ở mục 7, có thể tạo thông qua lệnh: 

subkey generate --scheme sr25519



c) Khởi tạo 1 mã định danh chuỗi (Chain Specification):
Các blockchain dựa trên Polkadot SDK được định nghĩa bằng một tệp gọi là chain specification (thông số kỹ thuật chuỗi), hay gọi tắt là chain spec. Có hai loại tệp chain spec:
- Plain chain spec : Tệp JSON dễ đọc, có thể được chỉnh sửa để phù hợp với yêu cầu của parachain. Tệp này đóng vai trò là mẫu cho cấu hình ban đầu và bao gồm các khóa và cấu trúc dễ đọc.
- Raw Chain Spec : Tệp được mã hóa nhị phân dùng để khởi động nút parachain của bạn. Tệp này được tạo từ thông số kỹ thuật chuỗi đơn giản và chứa thông tin được mã hóa cần thiết để nút parachain đồng bộ hóa với mạng blockchain. Nó đảm bảo khả năng tương thích giữa các phiên bản runtime khác nhau bằng cách cung cấp dữ liệu ở định dạng mà runtime của nút có thể diễn giải trực tiếp, bất kể các bản nâng cấp kể từ khi chuỗi được hình thành.

Để định nghĩa Chain Specification:
- Khởi tạo Plain chain specification cho node thông qua chuỗi lệnh:

chain-spec-builder \
--chain-spec-path ./plain_chain_spec.json \
create \
--relay-chain paseo \
--para-id INSERT_PARA_ID \
--runtime target/release/wbuild/dacs-runtime/dacs_runtime.compact.compressed.wasm \
named-preset local_testnet

* Thay đổi trường INSERT_PARA_ID bằng mã định danh Parachain (Para ID) đã đăng ký trước đó ở mục a)

- Chỉnh sửa tập tin plain_chain_spec.json:
1. Cập nhật các trường "name, id" thành các giá trị duy nhất cho parachain (Tùy chỉnh theo mong muốn).
2. Thay đổi các trường "para_id, parachainInfo.parachainId" thành ID Parachain đã đăng ký trước đó. Đảm bảo sử dụng số không có dấu ngoặc kép.
3. Sửa đổi trường "balances" bằng địa chỉ tài khoản đã chuẩn bị sẵn cùng số dư ban đầu (mặc định) cho tài khoản theo định dạng SS58. 
4. Tiếp tục sử dụng địa chỉ tài khoản đã chuẩn bị sẵn để chèn vào trường "collatorSelection.invulnerables" cùng khóa phiên (Session key) theo định dạng SS58 để chèn vào trường "session.keys.aura".
5. Sửa đổi trường "sudo" để chỉ định tài khoản sẽ có quyền truy cập sudo vào parachain. Ở đây, tiếp tục sử dụng địa chỉ tài khoản đã chuẩn bị sẵn.
- Lưu tập tin và đóng tệp.
- Chuyển đổi tập tin plain_chain_spec.json đã sửa đổi thành tập tin raw_chain_spec.json (tệp thông số kỹ thuật chuỗi thô):

chain-spec-builder \
--chain-spec-path ./raw_chain_spec.json \
convert-to-raw plain_chain_spec.json

d) Xuất các tập tin cần thiết:
Để chuẩn bị cho Parachain collator có thể đăng ký trên Paseo Testnet, cần 2 chuỗi quan trọng là Runtime Wasm và trạng thái genesis (genesis state). Xuất 2 tập tin thông qua 2 nhóm lệnh sau:

polkadot-omni-node export-genesis-wasm \
--chain raw_chain_spec.json para-wasm

polkadot-omni-node export-genesis-head \
--chain raw_chain_spec.json para-state

e) Đăng ký Parathread:
- Vào tab Parachains > Parathreads và chọn "+ Parathread".
- Chọn 2 tập tin đã xuất vào các trường để đặt trạng thái Wasm và trạng thái genesis tương ứng, cùng với ID Parachain.
Xác nhận thông tin chi tiết và nút "+ Submit", tại đó sẽ có một Parathread mới có ID Parachain tương ứng và nút Deregister đang hoạt động.
*Notes: Thời gian để Parachain được tích hợp và trở thành Parathread cần mất khoảng 2 giờ đồng hồ.

f) 