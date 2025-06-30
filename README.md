# Clean Axum Demo

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Dự án demo API REST được xây dựng bằng Axum framework trong Rust, cung cấp một mẫu khởi đầu sạch và có cấu trúc tốt để phát triển các ứng dụng web hiệu suất cao.

## Mục lục

- [Tính năng nổi bật](#tính-năng-nổi-bật)
- [Hướng dẫn cài đặt](#hướng-dẫn-cài-đặt)
- [Cách sử dụng](#cách-sử-dụng)
- [Cấu trúc thư mục](#cấu-trúc-thư-mục)
- [Công nghệ sử dụng](#công-nghệ-sử-dụng)
- [Đóng góp](#đóng-góp)
- [License](#license)
- [Liên hệ](#liên-hệ)

## Tính năng nổi bật

✅ **Clean Architecture:** Cấu trúc dự án rõ ràng, tách biệt các lớp (domain, application, infrastructure).  
✅ **RESTful API:** Cung cấp các endpoint theo chuẩn REST.  
✅ **Asynchronous:** Tận dụng sức mạnh của `tokio` cho các tác vụ bất đồng bộ.  
✅ **Cấu hình linh hoạt:** Sử dụng file `.env` để quản lý môi trường.  
✅ **Middleware:** Tích hợp sẵn middleware cho logging và CORS.  
✅ **Database Integration:** Sử dụng `sqlx` để tương tác với cơ sở dữ liệu PostgreSQL.

## Hướng dẫn cài đặt

### Yêu cầu

- [Rust](https://www.rust-lang.org/tools/install)
- [PostgreSQL](https://www.postgresql.org/download/)

### Các bước cài đặt

1.  **Clone a repository**

    ```bash
    git clone https://github.com/your-username/clean_axum_demo.git
    cd clean_axum_demo
    ```

2.  **Cài đặt dependencies**

    ```bash
    cargo build
    ```

3.  **Cấu hình môi trường**

    Sao chép file `.env-example` (nếu có) thành `.env` và điền các thông tin cần thiết, đặc biệt là `DATABASE_URL`.

    ```bash
    cp .env-example .env
    ```

4.  **Chạy migrations**

    (Giả sử bạn đang dùng `sqlx-cli`)
    ```bash
    # Cài đặt sqlx-cli nếu chưa có
    cargo install sqlx-cli
    
    # Chạy migration
    sqlx migrate run
    ```

## Cách sử dụng

1.  **Chạy ứng dụng**

    ```bash
    cargo run
    ```

2.  **Truy cập API**

    Server sẽ chạy tại `http://localhost:3000` (hoặc port được cấu hình trong `.env`). Bạn có thể dùng các công cụ như `curl` hoặc Postman để tương tác với các API endpoints.

    Ví dụ:
    ```bash
    # [GET] Lấy danh sách người dùng (ví dụ)
    curl http://localhost:3000/api/users
    ```

## Cấu trúc thư mục

```
.
├── Cargo.toml
├── README.md
├── migrations
│   ├── ...
└── src
    ├── api         # Định nghĩa các router và handler cho API
    ├── config      # Cấu hình ứng dụng (database, server)
    ├── core        # Các logic nghiệp vụ cốt lõi
    ├── domains     # Định nghĩa các domain model
    ├── errors      # Xử lý lỗi tùy chỉnh
    ├── helpers     # Các hàm tiện ích
    ├── infrastructure # Triển khai các interface (database, external services)
    ├── lib.rs      # Module chính của library
    ├── main.rs     # Điểm khởi đầu của ứng dụng
    ├── models      # Các struct dùng cho request/response
    └── utils       # Các tiện ích chung
```

**Mô tả các file và thư mục chính:**

-   `src/main.rs`: Điểm vào của chương trình, gọi hàm `run` từ `lib.rs`.
-   `src/lib.rs`: Khởi tạo và cấu hình server Axum, database connection, và các middleware.
-   `src/api/`: Chứa các module con định nghĩa API routes và handlers cho từng resource.
-   `src/domains/`: Định nghĩa các thực thể và logic nghiệp vụ chính của ứng dụng.
-   `src/infrastructure/`: Chứa phần triển khai cụ thể cho các thành phần bên ngoài như database.
-   `src/models/`: Các cấu trúc dữ liệu (structs) dùng để serialize/deserialize JSON cho request và response.
-   `Cargo.toml`: File manifest của project, chứa thông tin về các dependency.
-   `migrations/`: Chứa các file SQL để quản lý schema của database.

## Công nghệ sử dụng

-   **Ngôn ngữ:** [Rust](https://www.rust-lang.org/)
-   **Framework:** [Axum](https://github.com/tokio-rs/axum)
-   **Runtime:** [Tokio](https://tokio.rs/)
-   **Serialization:** [Serde](https://serde.rs/)
-   **Database:** [PostgreSQL](https://www.postgresql.org/)
-   **ORM/Query Builder:** [SQLx](https://github.com/launchbadge/sqlx)
-   **Environment Variables:** [Dotenv](https://github.com/dotenv-rs/dotenv)

## Đóng góp

Chúng tôi hoan nghênh mọi đóng góp! Vui lòng tuân thủ các quy tắc sau:

1.  **Fork** repository này.
2.  Tạo một **branch** mới (`git checkout -b feature/ten-tinh-nang`).
3.  **Commit** các thay đổi của bạn (`git commit -am 'Add: Them tinh nang moi'`).
4.  **Push** lên branch (`git push origin feature/ten-tinh-nang`).
5.  Tạo một **Pull Request** mới.

Vui lòng đảm bảo code của bạn tuân thủ code style của dự án và đã được test cẩn thận.

## License

Dự án này được cấp phép theo **MIT License**. Xem file `LICENSE` để biết thêm chi tiết.

## Liên hệ

-   **Tên:** [Tên của bạn]
-   **Email:** mintri@example.com
-   **GitHub:** [Link GitHub của bạn]
