use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::SharedState;

pub mod dto;
pub mod handler;
pub mod jwt;
pub mod middleware;
pub mod seed;

pub fn router() -> OpenApiRouter<SharedState> {
    OpenApiRouter::new()
        .routes(routes!(handler::register))
        .routes(routes!(handler::login))
        .routes(routes!(handler::refresh_token))
        .routes(routes!(handler::get_me))
}