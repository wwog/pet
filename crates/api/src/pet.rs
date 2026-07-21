use utoipa_axum::router::{OpenApiRouter, UtoipaMethodRouter};
use utoipa_axum::routes;

use crate::app_state::SharedState;

mod dto;
mod handler;

pub fn router() -> OpenApiRouter<SharedState> {
    let list_pets: UtoipaMethodRouter<SharedState> = routes!(handler::list_pets);
    OpenApiRouter::<SharedState>::new()
        .routes(list_pets)
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

/// 后台管理专用路由（品种增删/导出）。由 routes.rs 挂载到 `/admin`。
pub fn admin_router() -> OpenApiRouter<SharedState> {
    let create_breed: UtoipaMethodRouter<SharedState> = routes!(handler::create_breed);
    OpenApiRouter::<SharedState>::new()
        .routes(create_breed)
        .routes(routes!(handler::delete_breed))
        .routes(routes!(handler::export_breeds))
}
