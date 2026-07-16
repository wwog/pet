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