use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::SharedState;

mod dto;
mod handler;

pub fn router() -> OpenApiRouter<SharedState> {
    OpenApiRouter::new()
        .routes(routes!(handler::list_pets))
        .routes(routes!(handler::create_pet))
        .routes(routes!(handler::get_pet))
        .routes(routes!(handler::update_pet))
        .routes(routes!(handler::update_pet_appearance))
        .routes(routes!(handler::update_pet_personality))
        .routes(routes!(handler::delete_pet))
        .routes(routes!(handler::get_pet_stats))
        .routes(routes!(handler::list_pet_breeds))
        .routes(routes!(handler::list_personality_tags))
}
