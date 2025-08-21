# ecipher Technical Specifications

## 1. Project Overview

### 1.1 Project Description
**ecipher** is a secure key management system built with Rust, featuring a cross-platform GUI client and a Linux-based server. The system uses a shared workspace architecture to maintain consistency between client and server implementations.

### 1.2 Core Features
- Secure key storage and management
- Cross-platform client with native GUI
- RESTful API server with MySQL backend
- End-to-end encryption with AES-256-GCM
- TLS-secured client-server communication
- Native OS keyring integration

## 2. System Architecture

### 2.1 High-Level Architecture

```
┌─────────────────┐    HTTPS/TLS    ┌─────────────────┐
│  Client (GUI)   │ ◄──────────────► │  Server (API)   │
│   Windows       │                  │     Linux       │
│                 │                  │                 │
│ ┌─────────────┐ │                  │ ┌─────────────┐ │
│ │    Iced     │ │                  │ │    Axum     │ │
│ │   Framework │ │                  │ │  Framework  │ │
│ └─────────────┘ │                  │ └─────────────┘ │
└─────────────────┘                  └─────────────────┘
         │                                     │
         ▼                                     ▼
┌─────────────────┐                  ┌─────────────────┐
│ OS Keyring      │                  │ MySQL Database  │
│ (Windows)       │                  │ (with TDE)      │
└─────────────────┘                  └─────────────────┘
         │                                     │
         └─────────────► shared ◄──────────────┘
                      (Common Logic)
```

### 2.2 Workspace Structure

```
ecipher/
├── Cargo.toml                # Workspace configuration
├── README.md                 # Project documentation
├── shared/                   # Shared library (lib crate)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── crypto/           # Encryption utilities
│       ├── models/           # Data models
│       └── protocol/         # Network protocol
├── server/                   # Server binary (bin crate)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── handlers/         # API handlers
│   │   ├── services/         # Business logic
│   │   └── repositories/     # Data access layer
│   ├── config/
│   │   └── server.toml       # Server configuration
│   └── tests/                # Integration tests
├── client/                   # Client binary (bin crate)
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs
│   │   ├── ui/               # UI components
│   │   │   ├── pages/
│   │   │   ├── components/
│   │   │   └── widgets/
│   │   └── services/         # Client services
│   ├── assets/               # UI resources
│   │   ├── icons/
│   │   ├── fonts/
│   │   └── images/
│   └── tests/                # Unit tests
├── scripts/                  # Development scripts
│   ├── dev.sh               # Development helper
│   ├── build.sh             # Build script
│   └── deploy.sh            # Deployment script
├── docs/                     # Documentation
│   ├── architecture.md
│   ├── api.md
│   └── deployment.md
├── examples/                 # Usage examples
│   └── encrypt_demo.rs
└── .github/
    └── workflows/
        └── ci.yml           # CI/CD pipeline
```

## 3. Technical Stack

### 3.1 Server Technologies
| Component | Technology | Purpose |
|-----------|------------|---------|
| **Web Framework** | Axum | Lightweight, async HTTP server |
| **Database** | MySQL 8.0+ | Persistent data storage with TDE |
| **Database Driver** | SQLx | Type-safe, async database access |
| **Encryption** | ring | AES-GCM and HKDF implementation |
| **Serialization** | serde | JSON/binary data serialization |
| **Async Runtime** | Tokio | Asynchronous runtime |

### 3.2 Client Technologies
| Component | Technology | Purpose |
|-----------|------------|---------|
| **GUI Framework** | Iced | Cross-platform declarative UI |
| **HTTP Client** | reqwest | TLS-enabled HTTP requests |
| **Keyring** | keyring-rs | OS native credential storage |
| **Graphics** | DirectX (Windows) | Hardware-accelerated rendering |

### 3.3 Shared Components
| Component | Technology | Purpose |
|-----------|------------|---------|
| **Serialization** | serde | Data model consistency |
| **Crypto** | ring, rand | Encryption/decryption utilities |
| **Time** | chrono | Timestamp handling |
| **Logging** | tracing | Structured logging |

## 4. Database Design

### 4.1 Schema Definition

```sql
-- Main encrypted keys table
CREATE TABLE encrypted_keys (
    id INT PRIMARY KEY AUTO_INCREMENT,
    user_id VARCHAR(64) NOT NULL,
    key_name VARCHAR(64) NOT NULL,
    encrypted_data BLOB NOT NULL,
    nonce BINARY(12) NOT NULL,
    salt BINARY(32) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    -- Composite unique constraint
    UNIQUE KEY uk_user_key (user_id, key_name),
    
    -- Performance indexes
    INDEX idx_user_id (user_id),
    INDEX idx_created_at (created_at)
);

-- User sessions table
CREATE TABLE user_sessions (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_id VARCHAR(64) NOT NULL,
    session_token VARCHAR(128) NOT NULL,
    expires_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    
    UNIQUE KEY uk_session_token (session_token),
    INDEX idx_user_id (user_id),
    INDEX idx_expires_at (expires_at)
);
```

### 4.2 Security Configuration

```toml
# config/server.toml
[database]
url = "mysql://user:password@localhost/ecipher"
ssl_mode = "VERIFY_IDENTITY"
ca_cert = "/path/to/ca-cert.pem"
client_cert = "/path/to/client-cert.pem"
client_key = "/path/to/client-key.pem"
max_connections = 20
min_connections = 5

[security]
tde_enabled = true
backup_encryption = true
```

## 5. Security Implementation

### 5.1 Encryption Specifications

#### 5.1.1 Data Encryption
- **Algorithm**: AES-256-GCM
- **Key Size**: 256 bits
- **Nonce**: 12 bytes (random per operation)
- **Authentication Tag**: 16 bytes

#### 5.1.2 Key Derivation
- **Algorithm**: PBKDF2-HMAC-SHA256
- **Iterations**: 100,000
- **Salt Size**: 32 bytes (random per user)
- **Output**: 256-bit encryption key

#### 5.1.3 Digital Signatures
- **Algorithm**: Ed25519
- **Purpose**: Server response integrity verification
- **Key Size**: 256 bits

### 5.2 Security Implementation Examples

```rust
// shared/src/crypto/mod.rs
use ring::{aead, pbkdf2, rand};

pub struct EncryptionService {
    rng: rand::SystemRandom,
}

impl EncryptionService {
    pub fn encrypt_data(&self, plaintext: &[u8], password: &str, salt: &[u8]) -> Result<Vec<u8>, CryptoError> {
        // Derive key using PBKDF2
        let mut key = [0u8; 32];
        pbkdf2::derive(
            pbkdf2::PBKDF2_HMAC_SHA256,
            NonZeroU32::new(100_000).unwrap(),
            salt,
            password.as_bytes(),
            &mut key,
        );

        // Generate random nonce
        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce)?;

        // Encrypt with AES-256-GCM
        let sealing_key = aead::SealingKey::new(
            &aead::AES_256_GCM,
            &key,
        )?;

        let mut ciphertext = plaintext.to_vec();
        ciphertext.resize(plaintext.len() + aead::AES_256_GCM.tag_len(), 0);
        
        sealing_key.seal_in_place_append_tag(
            aead::Nonce::assume_unique_for_key(nonce),
            aead::Aad::empty(),
            &mut ciphertext,
        )?;

        Ok(ciphertext)
    }
}
```

### 5.3 Client Key Storage

```rust
// client/src/services/keyring.rs
use keyring::Entry;

pub struct KeyringService;

impl KeyringService {
    pub fn store_master_key(user_id: &str, encrypted_key: &str) -> Result<(), KeyringError> {
        let entry = Entry::new("ecipher", user_id)?;
        entry.set_password(encrypted_key)?;
        Ok(())
    }

    pub fn retrieve_master_key(user_id: &str) -> Result<String, KeyringError> {
        let entry = Entry::new("ecipher", user_id)?;
        entry.get_password()
    }
}
```

## 6. API Specification

### 6.1 Authentication Endpoints

#### POST /api/v1/auth/login
```json
// Request
{
    "user_id": "string",
    "password": "string"
}

// Response
{
    "session_token": "string",
    "expires_at": "2024-12-31T23:59:59Z"
}
```

#### POST /api/v1/auth/logout
```json
// Request
{
    "session_token": "string"
}

// Response
{
    "message": "Logged out successfully"
}
```

### 6.2 Key Management Endpoints

#### GET /api/v1/keys
```json
// Response
{
    "keys": [
        {
            "key_name": "string",
            "created_at": "2024-01-01T00:00:00Z",
            "updated_at": "2024-01-01T00:00:00Z"
        }
    ]
}
```

#### POST /api/v1/keys
```json
// Request
{
    "key_name": "string",
    "encrypted_data": "base64_encoded_string",
    "nonce": "base64_encoded_string",
    "salt": "base64_encoded_string"
}

// Response
{
    "message": "Key stored successfully",
    "key_id": "integer"
}
```

#### GET /api/v1/keys/{key_name}
```json
// Response
{
    "key_name": "string",
    "encrypted_data": "base64_encoded_string",
    "nonce": "base64_encoded_string",
    "salt": "base64_encoded_string",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": "2024-01-01T00:00:00Z"
}
```

#### DELETE /api/v1/keys/{key_name}
```json
// Response
{
    "message": "Key deleted successfully"
}
```

## 7. Build and Deployment

### 7.1 Development Setup

```bash
# Clone and setup workspace
mkdir ecipher && cd ecipher

# Create workspace members
cargo new --lib shared
cargo new --bin server
cargo new --bin client

# Run development server
cargo run -p server

# Run client application
cargo run -p client
```

### 7.2 Workspace Configuration

```toml
# Cargo.toml (workspace root)
[workspace]
members = ["shared", "server", "client"]
resolver = "2"

[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "mysql", "chrono"] }
reqwest = { version = "0.11", features = ["json", "rustls-tls"] }
ring = "0.16"
rand = "0.8"
chrono = { version = "0.4", features = ["serde"] }
tracing = "0.1"
iced = "0.10"
keyring = "2.0"

[workspace.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]
```

### 7.3 Platform-Specific Build

```toml
# client/Cargo.toml
[target.'cfg(windows)'.dependencies]
winapi = { version = "0.3", features = ["winuser", "wincrypt"] }

[target.'cfg(target_os = "macos")'.dependencies]
security-framework = "2.9"

[target.'cfg(target_os = "linux")'.dependencies]
secret-service = "3.0"
```

## 8. CI/CD Pipeline

### 8.1 GitHub Actions Configuration

```yaml
# .github/workflows/ci.yml
name: CI/CD Pipeline

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    name: Test Suite
    runs-on: ubuntu-latest
    services:
      mysql:
        image: mysql:8.0
        env:
          MYSQL_ROOT_PASSWORD: test
          MYSQL_DATABASE: ecipher_test
        options: >-
          --health-cmd="mysqladmin ping"
          --health-interval=10s
          --health-timeout=5s
          --health-retries=3

    steps:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Run tests
      run: |
        cargo test --workspace --verbose
        cargo test --package server --features integration-tests

  build:
    name: Build Artifacts
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest]
    
    steps:
    - uses: actions/checkout@v4
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
    
    - name: Build server (Linux)
      if: matrix.os == 'ubuntu-latest'
      run: cargo build --release --package server
    
    - name: Build client (Windows)
      if: matrix.os == 'windows-latest'
      run: cargo build --release --package client
```

## 9. Performance Requirements

### 9.1 Server Performance
- **Response Time**: < 100ms for key operations
- **Throughput**: 1000+ concurrent connections
- **Database**: Connection pooling (5-20 connections)
- **Memory Usage**: < 512MB base memory footprint

### 9.2 Client Performance
- **Startup Time**: < 2 seconds cold start
- **UI Responsiveness**: 60 FPS rendering
- **Memory Usage**: < 100MB typical usage
- **Encryption Speed**: < 10ms for typical key operations

## 10. Security Considerations

### 10.1 Transport Security
- **Protocol**: TLS 1.2+ mandatory
- **Certificate Validation**: Full chain verification
- **Cipher Suites**: Modern AEAD ciphers only
- **HSTS**: Enabled with long max-age

### 10.2 Data Protection
- **At Rest**: MySQL TDE enabled
- **In Transit**: TLS encryption
- **In Memory**: Secure memory wiping
- **Backups**: Encrypted with separate keys

### 10.3 Authentication
- **Session Management**: JWT with short expiry
- **Rate Limiting**: Prevent brute force attacks
- **Account Lockout**: After failed attempts
- **Password Policy**: Strong password requirements

## 11. Monitoring and Logging

### 11.1 Structured Logging
```rust
// Logging configuration
use tracing::{info, warn, error};
use tracing_subscriber;

// Server logging
info!(user_id = %user_id, key_name = %key_name, "Key stored successfully");
warn!(user_id = %user_id, attempt_count = failed_attempts, "Login failed");
error!(error = %e, "Database connection failed");
```

### 11.2 Metrics Collection
- **Response Time**: API endpoint latency
- **Error Rates**: HTTP 4xx/5xx responses  
- **Database**: Connection pool metrics
- **Security**: Authentication failure rates

## 12. Future Enhancements

### 12.1 Planned Features
- **Multi-user Support**: User management system
- **Key Sharing**: Secure key sharing between users
- **Mobile Client**: iOS/Android applications
- **Hardware Security**: HSM integration
- **Audit Logging**: Comprehensive activity logs

### 12.2 Scalability Considerations
- **Database Sharding**: Horizontal scaling strategy
- **Caching Layer**: Redis for session management
- **Load Balancing**: Multiple server instances
- **CDN Integration**: Asset delivery optimization

---

*This specification serves as the comprehensive technical guide for the ecipher project. All implementation details should align with the security and architecture principles outlined in this document.*