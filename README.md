# ecipher
Rust 客户端（Windows）+ 服务端（Linux）共用工程的模板，并选用 egui 作为客户端 GUI 框架，服务端使用 axum 框架。


## 项目架构
---
### 文件结构
---
```
rust-desktop-server-template/
├── Cargo.toml        # workspace 管理
├── client/           # 客户端，egui GUI
│   └── Cargo.toml
│   └── src/
├── server/           # 服务端，axum REST API
│   └── Cargo.toml
│   └── src/
├── shared/           # 公共类型/协议
│   └── Cargo.toml
│   └── src/
└── .github/
    └── workflows/
        └── ci.yml   # GitHub Actions 工作流
```
### 目录说明
---
shared 目录存放数据结构，客户端和服务端都可以引用。
server 用 axum 框架，监听 /hello POST 请求，返回打招呼信息。
client 用 egui 框架，输入名字并通过 HTTP 请求与服务端交互。
.github/workflows/ci.yml 提供 CI 编译与测试，保证在 GitHub Actions 上能编译和运行。


### 使用方法
- 启动服务端：cargo run -p server
- 启动客户端：cargo run -p client
- 客户端可新建密钥/获取密钥，用密钥做加密解密
可根据实际需求扩展密钥管理、加密算法、权限校验等。
如需更详细功能、扩展多页面、完善交互等，随时告知！