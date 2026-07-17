use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RegisterResponse {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct UserInfo {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub user_id: Uuid,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
    pub created_at: String,
}