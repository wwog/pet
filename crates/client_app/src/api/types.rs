use serde::{Deserialize, Serialize};

/// 后端统一响应壳:成功时 data 为实际数据
#[derive(Debug, Deserialize)]
pub struct ApiResponse<T> {
    pub code: u16,
    pub message: String,
    pub data: T,
}

/// 后端错误响应(无 data 字段)
#[derive(Debug, Deserialize)]
pub struct ApiErrorResponse {
    pub code: u16,
    pub message: String,
}

/// 前端用错误类型
#[derive(Debug, Clone)]
pub enum ApiError {
    /// 后端业务错误(code + message)
    Server(u16, String),
    /// 网络错误 / 反序列化失败
    Network(String),
    /// 401 未授权(触发 refresh 流程)
    Unauthorized,
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::Server(code, msg) => write!(f, "[{code}] {msg}"),
            ApiError::Network(msg) => write!(f, "网络错误: {msg}"),
            ApiError::Unauthorized => write!(f, "未授权"),
        }
    }
}

impl std::error::Error for ApiError {}

// ── 请求 DTO ──────────────────────────────────────────
#[derive(Debug, Serialize)]
pub struct LoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterRequest {
    pub account: String,
    pub password: String,
    pub nickname: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

// ── 响应 DTO(字段与后端 dto.rs 对齐,snake_case)─────
#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub user_id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
    pub user: UserInfo,
}

#[derive(Debug, Deserialize)]
pub struct RegisterResponse {
    pub user_id: String,
    pub account: String,
    pub nickname: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshResponse {
    pub access_token: String,
    pub expires_in: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MeResponse {
    pub user_id: String,
    pub account: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub role: String,
    pub created_at: String,
}
