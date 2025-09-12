# Reef Core API

Reef Core library in Rust - 一个用 Rust 实现的核心钱包 API 库。

## 项目结构

这是一个 Rust workspace 项目，包含以下组件：

- `apps/` - 应用程序
  - `wallet_api` - 钱包 API 应用
- `crates/` - 库 crates
  - `settings` - 配置管理库

## 开发指南

### 添加新的 Crate

#### 添加库 crate

```bash
# 在 crates 目录下创建新的库
cargo new crates/<crate_name> --lib

# 例如：创建一个名为 utils 的库
cargo new crates/utils --lib
```

#### 添加应用 crate

```bash
# 在 apps 目录下创建新的应用
cd apps
cargo new <app_name>

# 例如：创建一个名为 web_api 的应用
cd apps
cargo new web_api
```

创建后需要在根目录的 `Cargo.toml` 中的 `members` 数组里添加新的 crate 路径：

```toml
[workspace]
members = [
    "apps/wallet_api",
    "apps/web_api",           # 新添加的应用
    "crates/settings",
    "crates/utils"            # 新添加的库
]
```

### 运行项目

#### 运行特定的应用

```bash
# 从项目根目录运行 wallet_api
cargo run --package wallet_api

# 或者简写
cargo run -p wallet_api

# 运行其他应用（如果有的话）
cargo run --package <app_name>
```

#### 在应用目录下直接运行

```bash
# 进入应用目录
cd apps/wallet_api

# 直接运行
cargo run
```

### 构建和测试

```bash
# 构建整个 workspace
cargo build

# 构建特定包
cargo build --package wallet_api

# 运行测试
cargo test

# 检查代码
cargo check
```

### 依赖管理

#### Workspace 级别的依赖

在根目录的 `Cargo.toml` 中的 `[workspace.dependencies]` 部分添加共享依赖：

```toml
[workspace.dependencies]
tokio = { version = "1.47.1", features = ["macros", "rt-multi-thread"] }
serde = { version = "1.0.219", features = ["derive"] }
config = { version = "0.15.14", features = ["yaml"] }
```

#### 在子项目中使用 workspace 依赖

在子项目的 `Cargo.toml` 中引用：

```toml
[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
config = { workspace = true }
```

#### 添加项目特定的依赖

```bash
# 在项目根目录下添加到 workspace dependencies
cargo add <dependency_name>

# 在子项目目录下添加到特定项目
cd apps/wallet_api
cargo add <dependency_name>
```

## 配置

项目使用 `Settings.yaml` 文件进行配置管理，通过 `crates/settings` 库来处理配置读取和管理。

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件。

## 链接

- 主页：https://reef.top/
- 仓库：https://github.com/zzispp/reef-core-api
- 文档：https://github.com/zzispp
