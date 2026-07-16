# API Auth 模块 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 搭建 api crate 企业级项目骨架（axum + JWT + argon2），实现账号密码注册/登录/刷新Token/获取用户信息四个接口。

**Architecture:** DDD 分层 — 先改 domain 实体（phone→account + password_hash），再改 db 实现，最后建 api 层（config/error/state/auth）。认证策略：JWT 无状态 access_token（2h）+ UUID refresh_token 存库（30d）。密码用 argon2id 哈希。

**Tech Stack:** axum 0.8.9, jsonwebtoken 10.4.0, argon2 0.5.3, once_cell 1.21.4, regex 1.13.1, toasty 0.8.0 (sqlite)

---

## 文件结构总览

| 操作 | 文件 | 职责 |
|------|------|------|
| 修改 | `crates/domain/src/user.rs` | User 实体 phone→account + password_hash, UserRepository trait |
| 修改 | `crates/domain/src/app.rs` | AppError 新增 Auth 变体 |
| 修改 | `crates/db/src/user.rs` | DB 模型 + From 映射 + Repository 实现同步更新 |
| 修改 | `crates/db/src/lib.rs` | 暴露 session_repository() 工厂方法 |
| 修改 | `Cargo.toml` | 工作区新增 jsonwebtoken/argon2/once_cell/regex 依赖 |
| 修改 | `crates/api/Cargo.toml` | api 新增 db/axum/jsonwebtoken/argon2/once_cell/regex |
| 新建 | `crates/api/src/config.rs` | 环境变量读取 |
| 新建 | `crates/api/src/error.rs` | AppError → axum Response 转换 |
| 新建 | `crates/api/src/app_state.rs` | Arc<Database> 共享状态 |
| 新建 | `crates/api/src/auth/mod.rs` | auth 子路由组装 |
| 新建 | `crates/api/src/auth/dto.rs` | 请求/响应 DTO |
| 新建 | `crates/api/src/auth/jwt.rs` | JWT 签发与验证 |
| 新建 | `crates/api/src/auth/middleware.rs` | Bearer token 提取 (FromRequestParts) |
| 新建 | `crates/api/src/auth/handler.rs` | 4 个接口处理函数 |
| 修改 | `crates/api/src/main.rs` | 启动：初始化 config/DB，挂载路由，bind 端口 |

---

### Task 1: Domain — 更新 User 实体

**Files:**
- Modify: `crates/domain/src/user.rs`

- [ ] **Step 1: 将 phone 改为 account，新增 password_hash**

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

/// 用户实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    /// 账号（唯一，最小6位，字母/数字/下划线）
    pub account: String,
    /// argon2id 密码哈希
    pub password_hash: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    /// 微信 OpenID（唯一，预留）
    pub wechat_open_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// 用户会话（用于 token 管理）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub device_id: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_account(&self, account: &str) -> AppResult<Option<User>>;
    async fn find_by_wechat_open_id(&self, open_id: &str) -> AppResult<Option<User>>;
    async fn update_nickname(&self, id: Uuid, nickname: &str) -> AppResult<()>;
    async fn update_avatar(&self, id: Uuid, avatar: &str) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: UserSession) -> AppResult<UserSession>;
    async fn find_by_refresh_token(&self, token: &str) -> AppResult<Option<UserSession>>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn delete_by_user(&self, user_id: Uuid) -> AppResult<()>;
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p domain 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/domain/src/user.rs
git commit -m "feat(domain): change phone to account, add password_hash to User"
```

---

### Task 2: Domain — 更新 AppError

**Files:**
- Modify: `crates/domain/src/app.rs`

- [ ] **Step 1: 新增 Auth 变体**

```rust
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("unauthorized: {0}")]
    Unauthorized(String),
    #[error("conflict: {0}")]
    Conflict(String),
    #[error("validation: {0}")]
    Validation(String),
    #[error("auth: {0}")]
    Auth(String),
    #[error("database: {0}")]
    Database(String),
    #[error("agent: {0}")]
    Agent(String),
    #[error("internal: {0}")]
    Internal(String),
}


pub type AppResult<T> = Result<T, AppError>;
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p domain 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/domain/src/app.rs
git commit -m "feat(domain): add Auth variant to AppError"
```

---

### Task 3: DB — 更新 User 模型和 From 映射

**Files:**
- Modify: `crates/db/src/user.rs`

- [ ] **Step 1: 更新 DB 模型和 From impl**

注：完整保留 SessionRepository 实现代码不变，仅改 User 模型、From<User> 映射和 UserRepository 方法签名。`parse_datetime` 辅助函数保留不变。

需要修改的部分：

**User 模型**（phone → account + password_hash）：

```rust
/// toasty ORM 模型 — 对应 `users` 表。
///
/// datetime 以 ISO 8601 `String` 存储，由 `From` mapper 转换为 `DateTime<Utc>`。
#[derive(Debug, toasty::Model)]
#[table = "users"]
pub struct User {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[unique]
    pub account: String,

    pub password_hash: String,

    pub nickname: Option<String>,
    pub avatar: Option<String>,
    #[unique]
    pub wechat_open_id: Option<String>,

    pub created_at: String,
}
```

**From<User> for domain_user::User**：

```rust
impl From<User> for domain_user::User {
    fn from(u: User) -> Self {
        domain_user::User {
            id: u.id,
            account: u.account,
            password_hash: u.password_hash,
            nickname: u.nickname,
            avatar: u.avatar,
            wechat_open_id: u.wechat_open_id,
            created_at: parse_datetime(&u.created_at),
        }
    }
}
```

**UserRepository::create**：

```rust
    async fn create(&self, user: domain_user::User) -> AppResult<domain_user::User> {
        let mut db = self.db.clone();
        let now = user.created_at.to_rfc3339();
        let created = toasty::create!(User {
            id: user.id,
            account: user.account,
            password_hash: user.password_hash,
            nickname: user.nickname,
            avatar: user.avatar,
            wechat_open_id: user.wechat_open_id,
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }
```

**find_by_phone → find_by_account**：

```rust
    async fn find_by_account(&self, account: &str) -> AppResult<Option<domain_user::User>> {
        let mut db = self.db.clone();
        let user = User::filter_by_account(account)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p db 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/db/src/user.rs
git commit -m "feat(db): update user model phone→account, add password_hash"
```

---

### Task 4: DB — 暴露 session_repository 工厂方法

**Files:**
- Modify: `crates/db/src/lib.rs`

- [ ] **Step 1: 新增 session_repository()**

在 `Database` impl 块中新增方法（紧接在 `user_repository()` 之后）：

```rust
    pub fn session_repository(&self) -> SessionRepository<'_> {
        SessionRepository::new(&self.db)
    }
```

同时在文件头部补充 import：

```rust
use crate::user::SessionRepository;
```

当前文件头部已有 `use crate::user::UserRepository;`，在它下面加一行即可。

- [ ] **Step 2: 验证编译**

```bash
cargo check -p db 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/db/src/lib.rs
git commit -m "feat(db): expose session_repository factory method"
```

---

### Task 5: Workspace — 新增依赖

**Files:**
- Modify: `Cargo.toml`（根）
- Modify: `crates/api/Cargo.toml`

- [ ] **Step 1: 根 Cargo.toml 新增 workspace 依赖**

在 `[workspace.dependencies]` 末尾追加：

```toml
jsonwebtoken = "10.4.0"
argon2 = "0.5.3"
once_cell = "1.21.4"
regex = "1.13.1"
```

- [ ] **Step 2: api crate Cargo.toml 新增依赖**

```toml
[package]
name = "api"
version = "0.1.0"
edition.workspace = true

[dependencies]
domain = { workspace = true }
db = { workspace = true }
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
axum = { workspace = true }
jsonwebtoken = { workspace = true }
argon2 = { workspace = true }
once_cell = { workspace = true }
regex = { workspace = true }
```

- [ ] **Step 3: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml crates/api/Cargo.toml
git commit -m "chore: add jsonwebtoken, argon2, once_cell, regex dependencies"
```

---

### Task 6: API — config.rs

**Files:**
- Create: `crates/api/src/config.rs`

- [ ] **Step 1: 创建配置文件**

```rust
use once_cell::sync::Lazy;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config {
        database_url: std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:data.db".into()),
        jwt_secret: std::env::var("JWT_SECRET")
            .expect("JWT_SECRET must be set"),
        bind_addr: std::env::var("BIND_ADDR")
            .unwrap_or_else(|_| "0.0.0.0:3000".into()),
    }
});
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/config.rs
git commit -m "feat(api): add config module for env vars"
```

---

### Task 7: API — error.rs

**Files:**
- Create: `crates/api/src/error.rs`

- [ ] **Step 1: 创建错误转换模块**

```rust
use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::app::AppError;
use serde::Serialize;

#[derive(Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

impl ApiResponse<()> {
    pub fn success_empty() -> Json<Self> {
        Json(Self {
            code: 0,
            message: "success".into(),
            data: (),
        })
    }
}

pub struct ApiError(pub AppError);

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match &self.0 {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    code: 1003,
                    message: msg.clone(),
                },
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: 1001,
                    message: msg.clone(),
                },
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    code: 1005,
                    message: msg.clone(),
                },
            ),
            AppError::Validation(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    code: 1004,
                    message: msg.clone(),
                },
            ),
            AppError::Auth(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    code: 1007,
                    message: msg.clone(),
                },
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    code: 9999,
                    message: "internal server error".into(),
                },
            ),
        };

        tracing::error!(error = %self.0, "request error");
        (status, Json(body)).into_response()
    }
}

impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        ApiError(err)
    }
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/error.rs
git commit -m "feat(api): add error conversion from AppError to axum Response"
```

---

### Task 8: API — app_state.rs

**Files:**
- Create: `crates/api/src/app_state.rs`

- [ ] **Step 1: 创建共享状态**

```rust
use std::sync::Arc;

pub struct AppState {
    pub db: db::Database,
}

pub type SharedState = Arc<AppState>;
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/app_state.rs
git commit -m "feat(api): add AppState shared application state"
```

---

### Task 9: API — auth/jwt.rs

**Files:**
- Create: `crates/api/src/auth/jwt.rs`

- [ ] **Step 1: 创建 JWT 签发与验证模块**

```rust
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::config::CONFIG;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn create_jwt(user_id: Uuid) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(2)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/auth/jwt.rs
git commit -m "feat(api): add JWT create/decode utilities"
```

---

### Task 10: API — auth/dto.rs

**Files:**
- Create: `crates/api/src/auth/dto.rs`

- [ ] **Step 1: 创建请求/响应 DTO**

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Debug, Serialize)]
pub struct MeResponse {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub created_at: String,
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/auth/dto.rs
git commit -m "feat(api): add auth DTOs for request/response"
```

---

### Task 11: API — auth/middleware.rs

**Files:**
- Create: `crates/api/src/auth/middleware.rs`

- [ ] **Step 1: 创建 Bearer token 提取中间件**

```rust
use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use uuid::Uuid;

use super::jwt;
use crate::error::ErrorResponse;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<ErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    axum::Json(ErrorResponse {
                        code: 1001,
                        message: "missing authorization header".into(),
                    }),
                )
            })?;

        let token = header.strip_prefix("Bearer ").ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ErrorResponse {
                    code: 1001,
                    message: "invalid authorization format".into(),
                }),
            )
        })?;

        let claims = jwt::decode_jwt(token).map_err(|e| {
            let (code, msg) = match e.kind() {
                jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                    (1002, "token expired".to_owned())
                }
                _ => (1001, "invalid token".to_owned()),
            };
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ErrorResponse {
                    code,
                    message: msg,
                }),
            )
        })?;

        let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(ErrorResponse {
                    code: 1001,
                    message: "invalid token payload".into(),
                }),
            )
        })?;

        Ok(AuthenticatedUser { user_id })
    }
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/auth/middleware.rs
git commit -m "feat(api): add Bearer token auth middleware"
```

---

### Task 12: API — auth/handler.rs

**Files:**
- Create: `crates/api/src/auth/handler.rs`

- [ ] **Step 1: 创建 4 个 handler 函数**

```rust
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use domain::app::AppError;
use regex::Regex;
use uuid::Uuid;

use super::dto::*;
use super::jwt;
use super::middleware::AuthenticatedUser;
use crate::app_state::SharedState;
use crate::error::{ApiError, ApiResponse};

fn validate_account(account: &str) -> Result<(), AppError> {
    if account.len() < 6 {
        return Err(AppError::Validation(
            "account must be at least 6 characters".into(),
        ));
    }
    let re = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !re.is_match(account) {
        return Err(AppError::Validation(
            "account must only contain letters, digits, and underscores".into(),
        ));
    }
    Ok(())
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::Validation(
            "password must be at least 8 characters".into(),
        ));
    }
    let has_letter = password.chars().any(|c| c.is_alphabetic());
    let has_digit = password.chars().any(|c| c.is_ascii_digit());
    if !has_letter || !has_digit {
        return Err(AppError::Validation(
            "password must contain both letters and digits".into(),
        ));
    }
    Ok(())
}

fn validate_nickname(nickname: &str) -> Result<(), AppError> {
    if nickname.is_empty() || nickname.len() > 20 {
        return Err(AppError::Validation(
            "nickname must be between 1 and 20 characters".into(),
        ));
    }
    Ok(())
}

fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);
    let hash = Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(format!("password hashing failed: {e}")))?
        .to_string();
    Ok(hash)
}

fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| AppError::Internal(format!("invalid password hash: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}

pub async fn register(
    State(state): State<SharedState>,
    Json(body): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<ApiResponse<RegisterResponse>>), ApiError> {
    validate_account(&body.account)?;
    validate_password(&body.password)?;
    validate_nickname(&body.nickname)?;

    let user_repo = state.db.user_repository();
    if user_repo.find_by_account(&body.account).await?.is_some() {
        return Err(AppError::Conflict("account already exists".into()).into());
    }

    let password_hash = hash_password(&body.password)?;

    let now = Utc::now();
    let user = domain::user::User {
        id: Uuid::new_v4(),
        account: body.account.clone(),
        password_hash,
        nickname: Some(body.nickname.clone()),
        avatar: None,
        wechat_open_id: None,
        created_at: now,
    };
    let saved = user_repo.create(user).await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            code: 0,
            message: "success".into(),
            data: RegisterResponse {
                user_id: saved.id,
                account: saved.account,
                nickname: saved.nickname.unwrap_or_default(),
                created_at: saved.created_at.to_rfc3339(),
            },
        }),
    ))
}

pub async fn login(
    State(state): State<SharedState>,
    Json(body): Json<LoginRequest>,
) -> Result<Json<ApiResponse<LoginResponse>>, ApiError> {
    let user_repo = state.db.user_repository();
    let user = user_repo
        .find_by_account(&body.account)
        .await?
        .ok_or_else(|| AppError::NotFound("account not found".into()))?;

    if !verify_password(&body.password, &user.password_hash)? {
        return Err(AppError::Auth("invalid password".into()).into());
    }

    let access_token = jwt::create_jwt(user.id)
        .map_err(|e| AppError::Internal(format!("jwt creation failed: {e}")))?;
    let refresh_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::days(30);

    let session_repo = state.db.session_repository();
    session_repo
        .create(domain::user::UserSession {
            id: Uuid::new_v4(),
            user_id: user.id,
            access_token: access_token.clone(),
            refresh_token: refresh_token.clone(),
            expires_at,
            device_id: None,
        })
        .await?;

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: LoginResponse {
            access_token,
            refresh_token,
            expires_in: 7200,
            user: UserInfo {
                user_id: user.id,
                account: user.account,
                nickname: user.nickname,
                avatar: user.avatar,
            },
        },
    }))
}

pub async fn refresh_token(
    State(state): State<SharedState>,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<ApiResponse<RefreshResponse>>, ApiError> {
    let session_repo = state.db.session_repository();
    let session = session_repo
        .find_by_refresh_token(&body.refresh_token)
        .await?
        .ok_or_else(|| AppError::Unauthorized("invalid refresh token".into()))?;

    if session.expires_at < Utc::now() {
        session_repo.delete(session.id).await?;
        return Err(AppError::Unauthorized("refresh token expired".into()).into());
    }

    session_repo.delete(session.id).await?;

    let access_token = jwt::create_jwt(session.user_id)
        .map_err(|e| AppError::Internal(format!("jwt creation failed: {e}")))?;
    let refresh_token = Uuid::new_v4().to_string();
    let expires_at = Utc::now() + chrono::Duration::days(30);

    session_repo
        .create(domain::user::UserSession {
            id: Uuid::new_v4(),
            user_id: session.user_id,
            access_token: access_token.clone(),
            refresh_token: refresh_token.clone(),
            expires_at,
            device_id: None,
        })
        .await?;

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: RefreshResponse {
            access_token,
            expires_in: 7200,
        },
    }))
}

pub async fn get_me(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<Json<ApiResponse<MeResponse>>, ApiError> {
    let user_repo = state.db.user_repository();
    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("user not found".into()))?;

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: MeResponse {
            user_id: user.id,
            account: user.account,
            nickname: user.nickname,
            avatar: user.avatar,
            created_at: user.created_at.to_rfc3339(),
        },
    }))
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/auth/handler.rs
git commit -m "feat(api): add register/login/refresh/me handlers"
```

---

### Task 13: API — auth/mod.rs

**Files:**
- Create: `crates/api/src/auth/mod.rs`

- [ ] **Step 1: 组装 auth 子路由**

```rust
use axum::{routing::{get, post}, Router};

use crate::app_state::SharedState;

pub mod dto;
pub mod handler;
pub mod jwt;
pub mod middleware;

pub fn router() -> Router<SharedState> {
    Router::new()
        .route("/auth/register", post(handler::register))
        .route("/auth/login", post(handler::login))
        .route("/auth/token/refresh", post(handler::refresh_token))
        .route("/auth/me", get(handler::get_me))
}
```

- [ ] **Step 2: 验证编译**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/auth/mod.rs
git commit -m "feat(api): add auth router assembly"
```

---

### Task 14: API — main.rs（启动入口）

**Files:**
- Modify: `crates/api/src/main.rs`

- [ ] **Step 1: 创建 main.rs 启动入口**

```rust
use std::sync::Arc;

use axum::Router;
use tracing_subscriber::EnvFilter;

mod app_state;
mod auth;
mod config;
mod error;

use app_state::AppState;
use config::CONFIG;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("connecting to database: {}", CONFIG.database_url);
    let database = db::Database::connect(&CONFIG.database_url)
        .await
        .expect("failed to connect to database");

    database
        .push_schema()
        .await
        .expect("failed to push database schema");

    let state = Arc::new(AppState { db: database });

    let app = Router::new()
        .merge(auth::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&CONFIG.bind_addr)
        .await
        .expect("failed to bind address");

    tracing::info!("listening on {}", CONFIG.bind_addr);
    axum::serve(listener, app)
        .await
        .expect("server error");
}
```

- [ ] **Step 2: 编译验证**

```bash
cargo check -p api 2>&1
```
Expected: 编译成功

- [ ] **Step 3: Commit**

```bash
git add crates/api/src/main.rs
git commit -m "feat(api): add main entry point with axum server"
```

---

### Task 15: 全量编译 + 端到端验证

**Files:**
- 无新建文件

- [ ] **Step 1: 全量编译**

```bash
cargo build 2>&1
```
Expected: 全部 crate 编译成功。

- [ ] **Step 2: 检查是否有 warning**

```bash
cargo build 2>&1 | grep -i warning || echo "no warnings"
```
Expected: 无 warning 或仅有预期内的 warning。

- [ ] **Step 3: 启动服务验证**

```bash
JWT_SECRET=test-secret cargo run -p api &
sleep 3

# 注册
curl -s -X POST http://0.0.0.0:3000/auth/register \
  -H "Content-Type: application/json" \
  -d '{"account":"testuser","password":"test1234","nickname":"Test"}' | python3 -m json.tool

# 登录
LOGIN=$(curl -s -X POST http://0.0.0.0:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"account":"testuser","password":"test1234"}')
echo "$LOGIN" | python3 -m json.tool

# 获取 me（提取 access_token）
TOKEN=$(echo "$LOGIN" | python3 -c "import sys,json;print(json.load(sys.stdin)['data']['access_token'])")
curl -s http://0.0.0.0:3000/auth/me -H "Authorization: Bearer $TOKEN" | python3 -m json.tool

# 刷新 token
REFRESH=$(echo "$LOGIN" | python3 -c "import sys,json;print(json.load(sys.stdin)['data']['refresh_token'])")
curl -s -X POST http://0.0.0.0:3000/auth/token/refresh \
  -H "Content-Type: application/json" \
  -d "{\"refresh_token\":\"$REFRESH\"}" | python3 -m json.tool

kill %1
```

Expected: 4 个接口均返回 `"code": 0` 的成功响应。

- [ ] **Step 4: Commit**

```bash
git commit -m "chore: final validation after full build and e2e test" --allow-empty
```