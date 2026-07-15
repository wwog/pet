use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

/// 用户实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    /// 手机号（唯一）
    pub phone: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    /// 微信 OpenID（唯一）
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
    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<User>>;
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
