use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use domain::app::AppError;
use domain::user::{SessionRepository, UserRepository};
use rand_core::OsRng;
use regex::Regex;
use uuid::Uuid;

use super::dto::*;
use super::jwt;
use super::middleware::AuthenticatedUser;
use crate::app_state::SharedState;
use crate::error::{ApiError, ApiResponse, ErrorResponse};

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

#[utoipa::path(
    post,
    path = "/auth/register",
    tag = "auth",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "注册成功", body = ApiResponse<RegisterResponse>),
        (status = 400, description = "参数校验失败", body = ErrorResponse),
        (status = 409, description = "账号已存在", body = ErrorResponse),
    )
)]
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

#[utoipa::path(
    post,
    path = "/auth/login",
    tag = "auth",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "登录成功", body = ApiResponse<LoginResponse>),
        (status = 401, description = "密码错误", body = ErrorResponse),
        (status = 404, description = "账号不存在", body = ErrorResponse),
    )
)]
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

#[utoipa::path(
    post,
    path = "/auth/token/refresh",
    tag = "auth",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "刷新成功", body = ApiResponse<RefreshResponse>),
        (status = 401, description = "refresh token 无效或已过期", body = ErrorResponse),
    )
)]
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

#[utoipa::path(
    get,
    path = "/auth/me",
    tag = "auth",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "当前用户信息", body = ApiResponse<MeResponse>),
        (status = 401, description = "未认证或 token 无效", body = ErrorResponse),
    )
)]
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