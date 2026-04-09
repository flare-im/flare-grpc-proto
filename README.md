# flare-grpc-proto

Flare IM 的 **gRPC 服务层 Protocol Buffers** 与 **Rust 生成代码**（`tonic` + `prost`）。业务服务 crate 通过依赖本库获得各服务的 client / server 类型及消息定义。

## 职责边界

| 组件 | 说明 |
|------|------|
| **`flare-proto`** | 域模型与公共类型（如 `flare.common.v1`、消息体 `Message` 等） |
| **本 crate** | 各 gRPC 服务的 `.proto` 与编译产物；通过 `extern_path` 引用 `flare_proto` 中的公共包，避免重复生成 |

## 目录结构

```
flare-grpc-proto/
├── proto/           # 服务定义（*.proto）
├── build.rs         # tonic-prost-build 编译入口
├── src/lib.rs       # include_proto! 与便捷 re-export
├── Cargo.toml
└── README.md
```

## 本仓库中的服务 Proto

`build.rs` 当前编译的文件（顺序与依赖解析相关）：

- `access_gateway.proto`
- `conversation_service.proto`
- `hooks.proto`
- `media_service.proto`
- `message_service.proto`
- `online.proto`
- `push_service.proto`
- `router.proto`
- `storage_service.proto`
- `sync_service.proto`

基础类型的 `import` 由 **`flare-proto/proto`** 提供；编译时 include 路径为「基础目录优先，再本目录」，以减少同名 import 歧义。

## 依赖

- **Rust**：见根目录 `Cargo.toml` 中 `rust-version`（当前为 **1.94+**，Edition **2024**）。
- **crate**：`flare-proto`、`prost`、`tonic`、`serde` 等，详见 `Cargo.toml`。

> `flare-proto` 的 path 以 **`Cargo.toml`** 为准；在不同工作区布局下可能使用相对路径锚定。

## 构建

```bash
cargo build -p flare-grpc-proto
```

修改任意 `proto/*.proto` 或 `flare-proto/proto` 下被引用的文件后，重新构建即可触发 `build.rs` 重新生成代码。

## 使用方式

在其它 crate 的 `Cargo.toml` 中：

```toml
flare-grpc-proto = { path = "../flare-grpc-proto" }
```

按需引用模块，例如：

```rust
use flare_grpc_proto::message;      // flare.message.v1 服务类型
use flare_grpc_proto::conversation; // 会话服务（非 wasm 时部分符号可用）
use flare_grpc_proto::Message;      // 从 flare_proto 再导出的公共消息类型
```

具体路径以 `src/lib.rs` 中的 `pub mod` 与 `pub use` 为准。

## 生成选项说明

- **Client / Server**：均在 `build.rs` 中开启（`build_client(true)`, `build_server(true)`）。
- **Well-known types**：`compile_well_known_types(false)`，与 `flare-proto` 策略对齐。
- **Serde**：部分 `flare.media.v1` 请求/响应在 `build.rs` 中附加 `Serialize` / `Deserialize`，便于 HTTP 网关等场景。

## 许可

MIT（见 `Cargo.toml`）。
