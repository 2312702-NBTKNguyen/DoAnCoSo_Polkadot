DoAnCoSo/                             # Dự án gốc: parachain dựa trên Substrate
├── Cargo.toml                        # Tập tin cấu hình workspace, khai báo các crate con
├── Cargo.lock                        # Khóa phiên bản thư viện phụ thuộc sau khi biên dịch
├── LICENSE                           # Thông tin giấy phép của dự án
├── README.md                         # Hướng dẫn tổng quan, cách biên dịch & chạy
├── raw_chain_spec.json               # Cấu hình mạng dạng thô dùng để khởi chạy mạng
├── node/                             # Mã nguồn chương trình node
│   ├── build.rs                      # Tập lệnh biên dịch cho crate node
│   ├── Cargo.toml                    # Tập tin cấu hình crate node, khai báo phụ thuộc vào runtime, thư viện Substrate
│   └── src/
│       ├── chain_spec.rs             # Định nghĩa chain spec, cấu hình mạng, genesis cho node
│       ├── cli.rs                    # Định nghĩa giao diện dòng lệnh (CLI), tham số dòng lệnh
│       ├── command.rs                # Xử lý lệnh từ CLI, khởi tạo node, nhập/xuất chain spec
│       ├── main.rs                   # Điểm vào chính của chương trình nút
│       ├── rpc.rs                    # Cấu hình RPC, khai báo các API RPC tùy chỉnh
│       └── service.rs                # Khởi tạo dịch vụ của node, kết nối mạng, đồng thuận, quản lý tác vụ
├── pallets/                          # Các pallet tùy chỉnh (mô-đun runtime)
│   └── blog-pallet/                  
│       ├── Cargo.toml                # Tập tin cấu hình cho pallet blog
│       └── src/
│           ├── benchmarking.rs       # Đo hiệu năng để sinh trọng số cho pallet
│           ├── lib.rs                # Logic chính của pallet: lưu trữ (storage), sự kiện (events), giao dịch (extrinsics)
│           ├── mock.rs               # Môi trường mô phỏng cho kiểm thử đơn vị của pallet
│           ├── tests.rs              # Kiểm thử đơn vị cho pallet blog
│           └── weights.rs            # Trọng số được sinh ra tự động từ benchmark, dùng trong runtime
└── runtime/                          # Runtime của parachain và cấu hình liên quan
    ├── build.rs                      # Tập lệnh biên dịch runtime
    ├── Cargo.toml                    # Tập tin cấu hình runtime, khai báo pallets, tính năng
    └── src/
        ├── apis.rs                   # Định nghĩa Runtime API
        ├── benchmarks.rs             # Điểm tập trung cấu hình benchmark cho các pallet
        ├── genesis_config_presets.rs # Các cấu hình mẫu cho genesis của runtime
        ├── lib.rs                    # Tập tin runtime chính: cấu trúc runtime, các kiểu, macro
        ├── configs/
        │   ├── mod.rs                # Tập hợp các cấu hình phụ
        │   └── xcm_config.rs         # Cấu hình XCM, định tuyến, tài sản, nguồn gốc cho giao tiếp liên chuỗi
        └── weights/
            ├── block_weights.rs      # Giới hạn trọng số cho mỗi khối
            ├── extrinsic_weights.rs  # Trọng số mặc định cho các giao dịch
            ├── mod.rs                # Tổ chức và xuất lại  các mô-đun weights
            ├── paritydb_weights.rs   # Trọng số liên quan đến hệ quản trị dữ liệu ParityDB
            └── rocksdb_weights.rs    # Trọng số liên quan đến hệ quản trị dữ liệu RocksDB

