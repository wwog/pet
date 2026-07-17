use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Role {
    User,
    Admin,
    SuperAdmin,
}

impl Role {
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::User => "user",
            Role::Admin => "admin",
            Role::SuperAdmin => "super_admin",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "user" => Some(Role::User),
            "admin" => Some(Role::Admin),
            "super_admin" => Some(Role::SuperAdmin),
            _ => None,
        }
    }

    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin | Role::SuperAdmin)
    }
}

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
    pub role: Role,
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
    async fn find_by_role(&self, role: Role) -> AppResult<Vec<User>>;
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