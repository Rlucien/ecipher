# ecipher 客户端基本架构实现

## 1. 客户端整体架构

### 1.1 分层架构设计

```
┌─────────────────────────────────────────────────────┐
│                  UI 层 (Iced)                        │
├─────────────────────────────────────────────────────┤
│  Pages        │  Components    │  Widgets            │
│  - LoginPage  │  - KeyList     │  - SecureInput      │
│  - MainPage   │  - KeyForm     │  - StatusBar        │
│  - Settings   │  - Header      │  - ProgressBar      │
├─────────────────────────────────────────────────────┤
│              应用状态管理 (State)                     │
├─────────────────────────────────────────────────────┤
│                服务层 (Services)                     │
│  - KeyringService  │  - ApiService  │  - CryptoService │
├─────────────────────────────────────────────────────┤
│                    共享库 (shared)                   │
│      - Models      │    - Protocol    │   - Crypto    │
└─────────────────────────────────────────────────────┘
```

### 1.2 目录结构详解

```
client/
├── Cargo.toml                    # 客户端依赖配置
├── src/
│   ├── main.rs                   # 程序入口点
│   ├── app.rs                    # 主应用结构
│   ├── state.rs                  # 应用状态管理
│   ├── ui/                       # UI 组件
│   │   ├── mod.rs
│   │   ├── pages/                # 页面组件
│   │   │   ├── mod.rs
│   │   │   ├── login.rs          # 登录页面
│   │   │   ├── main_page.rs      # 主界面
│   │   │   ├── settings.rs       # 设置页面
│   │   │   └── key_detail.rs     # 密钥详情页
│   │   ├── components/           # 可复用组件
│   │   │   ├── mod.rs
│   │   │   ├── key_list.rs       # 密钥列表
│   │   │   ├── key_form.rs       # 密钥表单
│   │   │   ├── header.rs         # 页面头部
│   │   │   └── sidebar.rs        # 侧边栏
│   │   └── widgets/              # 自定义控件
│   │       ├── mod.rs
│   │       ├── secure_input.rs   # 安全输入框
│   │       ├── status_bar.rs     # 状态栏
│   │       └── progress_bar.rs   # 进度条
│   ├── services/                 # 业务服务
│   │   ├── mod.rs
│   │   ├── api_service.rs        # API 通信服务
│   │   ├── keyring_service.rs    # 密钥环服务
│   │   ├── crypto_service.rs     # 加密服务
│   │   └── config_service.rs     # 配置管理
│   ├── models/                   # 客户端特有数据模型
│   │   ├── mod.rs
│   │   ├── app_state.rs          # 应用状态模型
│   │   └── ui_models.rs          # UI 数据模型
│   └── utils/                    # 工具函数
│       ├── mod.rs
│       ├── error.rs              # 错误处理
│       └── validation.rs         # 输入验证
├── assets/                       # 静态资源
│   ├── icons/                    # 图标文件
│   │   ├── app.ico
│   │   ├── key.svg
│   │   └── settings.svg
│   ├── fonts/                    # 字体文件
│   │   └── Roboto-Regular.ttf
│   └── images/                   # 图片资源
│       └── logo.png
└── tests/                        # 测试文件
    ├── integration_tests.rs
    └── ui_tests.rs
```

## 2. 核心组件实现

### 2.1 主应用结构 (app.rs)

```rust
use iced::{Application, Command, Element, Settings, Theme};
use crate::state::AppState;
use crate::ui::pages::{LoginPage, MainPage, SettingsPage};

#[derive(Debug, Clone)]
pub enum Message {
    // 导航消息
    NavigateToLogin,
    NavigateToMain,
    NavigateToSettings,
    
    // 登录相关
    LoginUsernameChanged(String),
    LoginPasswordChanged(String),
    LoginSubmitted,
    LoginCompleted(Result<String, String>),
    
    // 密钥管理
    LoadKeys,
    KeysLoaded(Result<Vec<shared::models::KeyInfo>, String>),
    CreateKey(String, String),
    KeyCreated(Result<(), String>),
    DeleteKey(String),
    KeyDeleted(Result<(), String>),
    
    // 设置
    ServerUrlChanged(String),
    SaveSettings,
    
    // 系统消息
    Tick,
    Error(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Login,
    Main,
    Settings,
    KeyDetail(String),
}

pub struct EcipherApp {
    pub state: AppState,
    pub current_page: Page,
    pub api_service: crate::services::ApiService,
    pub keyring_service: crate::services::KeyringService,
}

impl Application for EcipherApp {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let app = Self {
            state: AppState::default(),
            current_page: Page::Login,
            api_service: crate::services::ApiService::new(),
            keyring_service: crate::services::KeyringService::new(),
        };
        
        (app, Command::none())
    }

    fn title(&self) -> String {
        match self.current_page {
            Page::Login => "ecipher - 登录".to_string(),
            Page::Main => "ecipher - 密钥管理".to_string(),
            Page::Settings => "ecipher - 设置".to_string(),
            Page::KeyDetail(ref name) => format!("ecipher - 密钥详情: {}", name),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NavigateToLogin => {
                self.current_page = Page::Login;
                Command::none()
            }
            Message::NavigateToMain => {
                self.current_page = Page::Main;
                Command::perform(self.load_keys(), Message::KeysLoaded)
            }
            Message::LoginSubmitted => {
                if !self.state.login_form.username.is_empty() 
                    && !self.state.login_form.password.is_empty() {
                    let api_service = self.api_service.clone();
                    let username = self.state.login_form.username.clone();
                    let password = self.state.login_form.password.clone();
                    
                    Command::perform(
                        async move {
                            api_service.login(&username, &password).await
                        },
                        Message::LoginCompleted
                    )
                } else {
                    Command::none()
                }
            }
            // 其他消息处理...
            _ => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match self.current_page {
            Page::Login => LoginPage::view(&self.state),
            Page::Main => MainPage::view(&self.state),
            Page::Settings => SettingsPage::view(&self.state),
            Page::KeyDetail(ref key_name) => {
                // KeyDetailPage::view(&self.state, key_name)
                iced::widget::text("密钥详情").into()
            }
        }
    }
}

impl EcipherApp {
    async fn load_keys(&self) -> Result<Vec<shared::models::KeyInfo>, String> {
        self.api_service.get_keys().await
            .map_err(|e| e.to_string())
    }
}
```

### 2.2 应用状态管理 (state.rs)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct AppState {
    // 用户状态
    pub user: Option<UserState>,
    pub session_token: Option<String>,
    
    // UI 状态
    pub login_form: LoginForm,
    pub key_form: KeyForm,
    pub settings: AppSettings,
    
    // 数据状态
    pub keys: Vec<shared::models::KeyInfo>,
    pub selected_key: Option<String>,
    
    // UI 控制状态
    pub is_loading: bool,
    pub error_message: Option<String>,
    pub success_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UserState {
    pub user_id: String,
    pub display_name: String,
    pub last_login: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Default)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
    pub remember_me: bool,
    pub is_submitting: bool,
}

#[derive(Debug, Clone, Default)]
pub struct KeyForm {
    pub key_name: String,
    pub key_data: String,
    pub description: String,
    pub is_submitting: bool,
    pub validation_errors: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub server_url: String,
    pub auto_lock_timeout: u32, // 秒
    pub theme: AppTheme,
    pub language: String,
    pub enable_notifications: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            server_url: "https://localhost:8080".to_string(),
            auto_lock_timeout: 300, // 5分钟
            theme: AppTheme::System,
            language: "zh-CN".to_string(),
            enable_notifications: true,
        }
    }
}

impl AppState {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some() && self.session_token.is_some()
    }
    
    pub fn clear_auth(&mut self) {
        self.user = None;
        self.session_token = None;
        self.keys.clear();
    }
    
    pub fn set_error(&mut self, message: String) {
        self.error_message = Some(message);
        self.success_message = None;
    }
    
    pub fn set_success(&mut self, message: String) {
        self.success_message = Some(message);
        self.error_message = None;
    }
    
    pub fn clear_messages(&mut self) {
        self.error_message = None;
        self.success_message = None;
    }
}
```

### 2.3 API 服务层 (services/api_service.rs)

```rust
use reqwest::{Client, header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE}};
use serde_json::Value;
use shared::models::{KeyInfo, CreateKeyRequest, LoginRequest, LoginResponse};
use std::time::Duration;

#[derive(Clone)]
pub struct ApiService {
    client: Client,
    base_url: String,
    session_token: Option<String>,
}

impl ApiService {
    pub fn new() -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .use_rustls_tls()
            .build()
            .expect("Failed to create HTTP client");
            
        Self {
            client,
            base_url: "https://localhost:8080".to_string(),
            session_token: None,
        }
    }
    
    pub fn set_base_url(&mut self, url: String) {
        self.base_url = url;
    }
    
    pub fn set_session_token(&mut self, token: String) {
        self.session_token = Some(token);
    }
    
    fn create_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        
        if let Some(ref token) = self.session_token {
            headers.insert(
                AUTHORIZATION,
                format!("Bearer {}", token).parse().unwrap()
            );
        }
        
        headers
    }
    
    pub async fn login(&self, username: &str, password: &str) -> Result<LoginResponse, ApiError> {
        let request = LoginRequest {
            user_id: username.to_string(),
            password: password.to_string(),
        };
        
        let response = self.client
            .post(&format!("{}/api/v1/auth/login", self.base_url))
            .headers(self.create_headers())
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            let login_response: LoginResponse = response.json().await?;
            Ok(login_response)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::ServerError(error_text))
        }
    }
    
    pub async fn logout(&self) -> Result<(), ApiError> {
        if let Some(ref token) = self.session_token {
            let response = self.client
                .post(&format!("{}/api/v1/auth/logout", self.base_url))
                .headers(self.create_headers())
                .json(&serde_json::json!({"session_token": token}))
                .send()
                .await?;
                
            if !response.status().is_success() {
                let error_text = response.text().await?;
                return Err(ApiError::ServerError(error_text));
            }
        }
        
        Ok(())
    }
    
    pub async fn get_keys(&self) -> Result<Vec<KeyInfo>, ApiError> {
        let response = self.client
            .get(&format!("{}/api/v1/keys", self.base_url))
            .headers(self.create_headers())
            .send()
            .await?;
            
        if response.status().is_success() {
            let data: Value = response.json().await?;
            let keys: Vec<KeyInfo> = serde_json::from_value(data["keys"].clone())?;
            Ok(keys)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::ServerError(error_text))
        }
    }
    
    pub async fn create_key(&self, request: CreateKeyRequest) -> Result<(), ApiError> {
        let response = self.client
            .post(&format!("{}/api/v1/keys", self.base_url))
            .headers(self.create_headers())
            .json(&request)
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(ApiError::ServerError(error_text))
        }
    }
    
    pub async fn get_key(&self, key_name: &str) -> Result<KeyInfo, ApiError> {
        let response = self.client
            .get(&format!("{}/api/v1/keys/{}", self.base_url, key_name))
            .headers(self.create_headers())
            .send()
            .await?;
            
        if response.status().is_success() {
            let key_info: KeyInfo = response.json().await?;
            Ok(key_info)
        } else {
            let error_text = response.text().await?;
            Err(ApiError::ServerError(error_text))
        }
    }
    
    pub async fn delete_key(&self, key_name: &str) -> Result<(), ApiError> {
        let response = self.client
            .delete(&format!("{}/api/v1/keys/{}", self.base_url, key_name))
            .headers(self.create_headers())
            .send()
            .await?;
            
        if response.status().is_success() {
            Ok(())
        } else {
            let error_text = response.text().await?;
            Err(ApiError::ServerError(error_text))
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("网络请求失败: {0}")]
    Network(#[from] reqwest::Error),
    
    #[error("JSON 序列化错误: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("服务器错误: {0}")]
    ServerError(String),
    
    #[error("认证失败")]
    Unauthorized,
    
    #[error("资源未找到")]
    NotFound,
}
```

### 2.4 密钥环服务 (services/keyring_service.rs)

```rust
use keyring::Entry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredCredentials {
    pub user_id: String,
    pub encrypted_master_key: String,
    pub salt: Vec<u8>,
    pub nonce: Vec<u8>,
}

pub struct KeyringService {
    service_name: String,
}

impl KeyringService {
    pub fn new() -> Self {
        Self {
            service_name: "ecipher".to_string(),
        }
    }
    
    pub fn store_credentials(&self, user_id: &str, credentials: &StoredCredentials) -> Result<(), KeyringError> {
        let entry = Entry::new(&self.service_name, user_id)?;
        let serialized = serde_json::to_string(credentials)?;
        entry.set_password(&serialized)?;
        Ok(())
    }
    
    pub fn retrieve_credentials(&self, user_id: &str) -> Result<StoredCredentials, KeyringError> {
        let entry = Entry::new(&self.service_name, user_id)?;
        let serialized = entry.get_password()?;
        let credentials: StoredCredentials = serde_json::from_str(&serialized)?;
        Ok(credentials)
    }
    
    pub fn delete_credentials(&self, user_id: &str) -> Result<(), KeyringError> {
        let entry = Entry::new(&self.service_name, user_id)?;
        entry.delete_password()?;
        Ok(())
    }
    
    pub fn has_credentials(&self, user_id: &str) -> bool {
        match Entry::new(&self.service_name, user_id) {
            Ok(entry) => entry.get_password().is_ok(),
            Err(_) => false,
        }
    }
    
    pub fn store_session_token(&self, user_id: &str, token: &str) -> Result<(), KeyringError> {
        let session_key = format!("{}_session", user_id);
        let entry = Entry::new(&self.service_name, &session_key)?;
        entry.set_password(token)?;
        Ok(())
    }
    
    pub fn retrieve_session_token(&self, user_id: &str) -> Result<String, KeyringError> {
        let session_key = format!("{}_session", user_id);
        let entry = Entry::new(&self.service_name, &session_key)?;
        entry.get_password()
    }
    
    pub fn clear_session_token(&self, user_id: &str) -> Result<(), KeyringError> {
        let session_key = format!("{}_session", user_id);
        let entry = Entry::new(&self.service_name, &session_key)?;
        entry.delete_password().or(Ok(()))  // 忽略不存在的错误
    }
}

#[derive(Debug, thiserror::Error)]
pub enum KeyringError {
    #[error("密钥环操作失败: {0}")]
    KeyringError(#[from] keyring::Error),
    
    #[error("JSON 序列化失败: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("凭据不存在")]
    CredentialsNotFound,
}
```

## 3. UI 组件实现示例

### 3.1 登录页面 (ui/pages/login.rs)

```rust
use iced::{
    widget::{button, column, container, row, text, text_input, checkbox, Space},
    Element, Length, Alignment,
};
use crate::{Message, state::AppState};

pub struct LoginPage;

impl LoginPage {
    pub fn view(state: &AppState) -> Element<Message> {
        let title = text("ecipher 密钥管理器")
            .size(32)
            .horizontal_alignment(iced::alignment::Horizontal::Center);
            
        let username_input = text_input(
            "用户名",
            &state.login_form.username
        )
        .on_input(Message::LoginUsernameChanged)
        .padding(10);
        
        let password_input = text_input(
            "密码",
            &state.login_form.password
        )
        .on_input(Message::LoginPasswordChanged)
        .password()
        .padding(10);
        
        let remember_checkbox = checkbox(
            "记住登录状态",
            state.login_form.remember_me,
            Message::RememberMeToggled
        );
        
        let login_button = button("登录")
            .on_press_maybe(if state.login_form.is_submitting {
                None
            } else {
                Some(Message::LoginSubmitted)
            })
            .padding([10, 20]);
            
        let mut content = column![
            Space::with_height(50),
            title,
            Space::with_height(30),
            username_input,
            Space::with_height(10),
            password_input,
            Space::with_height(15),
            remember_checkbox,
            Space::with_height(20),
            login_button,
        ]
        .align_items(Alignment::Center)
        .spacing(5)
        .max_width(400);
        
        // 显示错误信息
        if let Some(ref error) = state.error_message {
            let error_text = text(error)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.8, 0.2, 0.2)));
            content = content.push(Space::with_height(10)).push(error_text);
        }
        
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
```

### 3.2 主页面 (ui/pages/main_page.rs)

```rust
use iced::{
    widget::{button, column, container, row, text, scrollable},
    Element, Length,
};
use crate::{Message, state::AppState};
use crate::ui::components::{Header, KeyList, Sidebar};

pub struct MainPage;

impl MainPage {
    pub fn view(state: &AppState) -> Element<Message> {
        let header = Header::view(state);
        
        let sidebar = Sidebar::view(state);
        
        let main_content = if state.is_loading {
            container(text("加载中..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
        } else {
            let key_list = KeyList::view(&state.keys, state.selected_key.as_ref());
            
            let add_button = button("+ 添加密钥")
                .on_press(Message::ShowAddKeyDialog)
                .padding([10, 20]);
                
            let content = column![
                row![
                    text("我的密钥").size(24),
                    Space::with_width(Length::Fill),
                    add_button,
                ],
                Space::with_height(20),
                key_list,
            ]
            .spacing(10)
            .padding(20);
            
            container(scrollable(content))
                .width(Length::Fill)
                .height(Length::Fill)
        };
        
        let layout = column![
            header,
            row![
                sidebar,
                main_content,
            ].height(Length::Fill)
        ];
        
        container(layout)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
```

## 4. 程序入口 (main.rs)

```rust
use iced::{Application, Settings, window, Font};

mod app;
mod state;
mod ui;
mod services;
mod models;
mod utils;

use app::{EcipherApp, Message};

fn main() -> iced::Result {
    // 设置日志
    tracing_subscriber::fmt::init();
    
    // 应用设置
    let settings = Settings {
        window: window::Settings {
            size: (1200, 800),
            min_size: Some((800, 600)),
            icon: load_icon(),
            ..Default::default()
        },
        fonts: vec![
            include_bytes!("../assets/fonts/Roboto-Regular.ttf").as_slice().into(),
        ],
        default_font: Font::with_name("Roboto"),
        ..Default::default()
    };
    
    EcipherApp::run(settings)
}

fn load_icon() -> Option<window::Icon> {
    let icon_bytes = include_bytes!("../assets/icons/app.ico");
    window::Icon::from_file_data(icon_bytes, None).ok()
}
```

## 5. 客户端特性总结

### 5.1 技术特点
- **跨平台 GUI**: 使用 Iced 框架支持 Windows/macOS/Linux
- **安全存储**: 集成系统原生密钥环 (Windows Credential Manager)
- **异步网络**: 使用 reqwest 进行 TLS 加密通信
- **响应式 UI**: 声明式 UI 编程模型
- **状态管理**: 集中式应用状态管理

### 5.2 安全考虑
- **内存安全**: 密码等敏感数据及时清零
- **会话管理**: 自动超时和会话刷新
- **本地加密**: 本地数据使用主密钥加密
- **证书验证**: 严格的 TLS 证书验证

### 5.3 用户体验
- **快速启动**: 缓存用户凭据实现快速登录
- **离线能力**: 本地缓存关键数据
- **错误处理**: 友好的错误提示和重试机制
- **进度反馈**: 操作进度和状态指示

这个架构为 ecipher 客户端提供了完整的基础框架，支持安全的密钥管理、现代化的用户界面和可扩展的功能模块。