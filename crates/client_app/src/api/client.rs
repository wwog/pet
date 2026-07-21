use once_cell::sync::Lazy;
use reqwest::{Client, StatusCode};

use super::types::{ApiError, ApiErrorResponse, ApiResponse};
use super::url::abs_url;

static HTTP: Lazy<Option<Client>> = Lazy::new(|| Client::builder().build().ok());

fn client() -> Result<Client, ApiError> {
    HTTP.clone().ok_or_else(|| ApiError::Network("无法创建 HTTP client".into()))
}

/// 发送 POST 请求并解析后端 ApiResponse<T>。
/// 失败(非 2xx)时返回 ApiError。
pub async fn post<B: serde::Serialize, T: serde::de::DeserializeOwned>(
    path: &str,
    body: &B,
) -> Result<T, ApiError> {
    let resp = client()?
        .post(abs_url(path))
        .json(body)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    parse_response(resp).await
}

/// 发送 GET 请求(带 Bearer token),解析 ApiResponse<T>。
pub async fn get_with_token<T: serde::de::DeserializeOwned>(
    path: &str,
    token: &str,
) -> Result<T, ApiError> {
    let resp = client()?
        .get(abs_url(path))
        .bearer_auth(token)
        .send()
        .await
        .map_err(|e| ApiError::Network(e.to_string()))?;
    parse_response(resp).await
}

async fn parse_response<T: serde::de::DeserializeOwned>(
    resp: reqwest::Response,
) -> Result<T, ApiError> {
    let status = resp.status();
    if status == StatusCode::UNAUTHORIZED {
        return Err(ApiError::Unauthorized);
    }
    if !status.is_success() {
        let err: ApiErrorResponse = resp
            .json()
            .await
            .map_err(|e| ApiError::Network(format!("解析错误响应失败: {e}")))?;
        return Err(ApiError::Server(err.code, err.message));
    }
    let body: ApiResponse<T> = resp
        .json()
        .await
        .map_err(|e| ApiError::Network(format!("解析响应失败: {e}")))?;
    Ok(body.data)
}

// ── 业务接口封装 ──────────────────────────────────────
use super::types::*;

pub async fn login(account: String, password: String) -> Result<LoginResponse, ApiError> {
    post("/app/auth/login", &LoginRequest { account, password }).await
}

pub async fn register(
    account: String,
    password: String,
    nickname: String,
) -> Result<RegisterResponse, ApiError> {
    post(
        "/app/auth/register",
        &RegisterRequest {
            account,
            password,
            nickname,
        },
    )
    .await
}

pub async fn refresh(refresh_token: String) -> Result<RefreshResponse, ApiError> {
    post("/common/auth/token/refresh", &RefreshRequest { refresh_token }).await
}

pub async fn me(access_token: &str) -> Result<MeResponse, ApiError> {
    get_with_token("/common/auth/me", access_token).await
}
