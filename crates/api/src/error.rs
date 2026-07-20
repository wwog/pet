use axum::{http::StatusCode, response::IntoResponse, Json};
use domain::app::AppError;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
}

#[derive(Serialize, ToSchema)]
pub struct ApiResponse<T: Serialize + ToSchema> {
    pub code: u16,
    pub message: String,
    pub data: T,
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
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    code: 1008,
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