use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use domain::user::Role;
use uuid::Uuid;

use super::jwt;
use crate::error::ErrorResponse;

pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub role: Role,
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
                axum::Json(ErrorResponse { code, message: msg }),
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

        let role = Role::from_str(&claims.role).unwrap_or(Role::User);

        Ok(AuthenticatedUser { user_id, role })
    }
}

/// 超级管理员提取器 — 验证 JWT 并确保角色为 SuperAdmin。
///
/// 用于管理后台专属 API 端点，返回 403 如果用户不是超级管理员。
pub struct SuperAdminUser {
    pub user_id: Uuid,
}

impl<S> FromRequestParts<S> for SuperAdminUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, axum::Json<ErrorResponse>);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // 先复用 AuthenticatedUser 的 JWT 验证逻辑
        let authed = AuthenticatedUser::from_request_parts(parts, _state).await?;

        if authed.role != Role::SuperAdmin {
            return Err((
                StatusCode::FORBIDDEN,
                axum::Json(ErrorResponse {
                    code: 1008,
                    message: "仅超级管理员可执行此操作".into(),
                }),
            ));
        }

        Ok(SuperAdminUser {
            user_id: authed.user_id,
        })
    }
}