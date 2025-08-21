# ecipher

Rust 客户端（Windows）+ 服务端（Linux）共用工程的模板，且客户端为 GUI 框架，服务端使用 axum 框架。

# 1.项目架构

---

## 1.1 系统架构

---

### 1.1.1 架构概述

采用Rust工作区(workspace)管理多模块，包含三个主要成员：

| 平台       | 技术栈      | 说明                                       |
| -------- | -------- | ---------------------------------------- |
| **服务端**  | `Axum `  | 服务端可执行程序，运行在 Linux，提供MySQL交互和 REST API服务 |
| **客户端**  | `Slint`  | 客户端 GUI 应用，使用Iced或Egui框架                 |
| **共享代码** | `shared` | 共享库，包含数据模型、加密算法、网络协议等通用组件                |

### 1.1.2 模块说明

- shared：
  共享库，作为库模块（lib crate），存放全项目通用代码，包括密钥数据模型、加密算法实现、网络通信协议定义等基础组件，为服务端与客户端提供统一的逻辑支撑。

- client：
  客户端，二进制模块（bin crate），构建客户端 GUI 应用（计划采用 Iced 或 Egui 框架），同样依赖 `shared` 库进行数据处理与网络通信，确保前后端逻辑一致性。
  
  - GUI：Iced（跨平台支持好，声明式API），支持Windows（DirectX）、macOS（Metal）、Linux（OpenGL）
  - 网络：reqwest（TLS支持）
  - 存储：keyring-rs（系统原生密钥存储）

- server：
  服务端，二进制模块（bin crate），实现密钥管理服务端功能，提供 MySQL 数据库交互、API 服务接口等核心能力，通过路径依赖引用 `shared` 库以复用通用逻辑。
  
  - Web框架：Axum（轻量高效，支持异步）
  - 数据库连接：SQLx（类型安全，支持MySQL异步连接）
  - 加密库：ring（提供AES-GCM和HKDF实现）

- tests：
  推荐每个端独立维护 tests 目录做集成/单元测试，保证质量。

- assets/config/scripts/docs/examples：
  
  - assets：客户端本地资源（图片/音频/字体等）。
  - config：服务端配置文件，便于环境切换和参数管理。
  - scripts：常用脚本自动化开发、部署、测试。
  - docs：架构/API/设计文档，方便团队协作。
  - examples：独立的使用示例，便于新成员快速上手。

- CI/CD集成：
  
  - .github/workflows 目录专用，易于维护持续集成流程。

### 1.1.3 目录规划

---

基于架构描述，目录规划如下：

```
ecipher/
├── Cargo.toml                # workspace 管理
├── README.md                 # 项目文档
├── shared/                   # 公用模块（协议、类型、加密等）
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── server/                   # 服务端
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── tests/                # 服务端集成测试
│   └── config/               # 服务端配置（使用 server.toml）
├── client/                   # 客户端（使用 iced）
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── tests/                # 客户端集成测试
│   ├── ui/                   # 客户端 UI 界面
│   │   └── assets/           # 客户端资源（如图片、icon）
├── scripts/                  # 开发/运维脚本（如启动、部署、数据迁移）
│   └── dev.sh
├── docs/                     # 项目文档（API, 架构, 设计）
│   └── architecture.md
├── examples/                 # 示例代码（如API调用示例、加密用例）
│   └── encrypt_demo.rs
├── .github/
│   └── workflows/
│       └── ci.yml            # CI/CD配置
```

## 1.2 技术选型

---

### 1.2.1 数据设计

- 表结构：
  
  ```sql
  CREATE TABLE encrypted_keys (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id VARCHAR(64) NOT NULL,
    key_name VARCHAR(64) NOT NULL,
    encrypted_data BLOB NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY (user_id, key_name)
  );
  ```

- 索引优化：复合索引(user_id, key_name)提升查询效率

### 1.2.2 加密算法

- 数据加密：AES-256-GCM（带12字节随机nonce）
- 密钥派生：PBKDF2-HMAC-SHA256（100000次迭代）
- 签名验证：Ed25519（用于验证服务端响应完整性）

### 1.2.3 安全配置

- 通信配置
  
  - 数据传输强制TLSv1.2+加密连接，服务器证书验证（ssl-mode=VERIFY_IDENTITY）；
  
  - 证书配置：
  
  ```rust
  // 客户端连接配置
  let builder = MySqlPoolOptions::new()
      .ssl(Some(SslOptions::default().verify_server_cert(
          Path::new("ca-cert.pem"),
          Path::new("client-cert.pem"),
          Path::new("client-key.pem"),
  )));
  ```

- 密钥储存：
  
  - 服务端：MySQL透明数据加密(TDE)
  
  - 客户端：使用keyring-rs存储主密钥，示例：
    
    ```rust
    let entry = Entry::new("key_manager", "user@example.com");
    entry.set_password(&master_key)?; // 存储加密后的主密钥
    ```

## 1.3 构建编译

### 1.3.1 依赖共享

通过`workspace.dependencies`统一管理公共依赖（如serde、tokio、sqlx等），确保版本一致性；

- Windows：启用winapi特性，处理系统证书存储
- macOS：链接Security.framework
- Linux：依赖libsecret-1-dev

### 1.3.2 编译运行

- 启动服务端：```cargo run -p server```
- 启动客户端：```cargo run -p client```
- 详细指令
  
  ```shell
  mkdir ecipher && cd ecipher
  # 创建共享库模块（存放公共逻辑与数据结构）
  cargo new --lib shared
  # 创建服务端二进制模块（处理密钥存储与业务逻辑）
  cargo new --bin server
  # 创建客户端二进制模块（提供用户交互与密钥操作界面）
  cargo new --bin client
  ```

### 1.3.3 构建配置：

## 1.4 扩展建议

---

可以将 shared 拆分成多个 crate（如 protocol, crypto, models），进一步细化公用逻辑。
支持多语言文档（docs/zh-cn, docs/en）。
client 目录下根据 GUI 框架可再细分（如 pages/components/widgets）。
server 目录下根据业务逻辑可再细分（如 handlers/services/repositories）。

## 贡献指南
