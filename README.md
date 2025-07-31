# ecipher
Rust 客户端（Windows）+ 服务端（Linux）共用工程的模板，并选用 egui 作为客户端 GUI 框架，服务端使用 axum 框架。


## 项目架构
---
### 文件结构
---
```
rust-app/
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
│   └── config/               # 服务端配置（如server.toml）
├── client/                   # 客户端（如 egui GUI）
│   ├── Cargo.toml
│   ├── src/
│   │   └── main.rs
│   ├── tests/                # 客户端集成测试
│   └── assets/               # 客户端资源（如图片、icon）
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
### 目录说明
---
1. shared 公共模块：
所有类型、协议、加密逻辑等都放在 shared，仅需维护一份代码，便于复用和测试。

2. client/server 逻辑清晰分离：
客户端和服务端各自独立，方便单独开发、测试和部署。每端都可以有自己的测试、配置、资源。

3. tests 目录：
推荐每个端独立维护 tests 目录做集成/单元测试，保证质量。

4. assets/config/scripts/docs/examples：

  - assets：客户端本地资源（图片/音频/字体等）。
  - config：服务端配置文件，便于环境切换和参数管理。
  - scripts：常用脚本自动化开发、部署、测试。
  - docs：架构/API/设计文档，方便团队协作。
  - examples：独立的使用示例，便于新成员快速上手。

5. CI/CD集成：
.github/workflows 目录专用，易于维护持续集成流程。


### 使用方法
- 启动服务端：cargo run -p server
- 启动客户端：cargo run -p client
- 客户端可新建密钥/获取密钥，用密钥做加密解密
可根据实际需求扩展密钥管理、加密算法、权限校验等。
如需更详细功能、扩展多页面、完善交互等，随时告知！


### 扩展建议
---
可以将 shared 拆分成多个 crate（如 protocol, crypto, models），进一步细化公用逻辑。
支持多语言文档（docs/zh-cn, docs/en）。
client 目录下根据 GUI 框架可再细分（如 pages/components/widgets）。
server 目录下根据业务逻辑可再细分（如 handlers/services/repositories）。
## 贡献指南
