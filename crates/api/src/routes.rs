//! 按受众分组组装路由：admin（后台）/ app（客户端）/ common（两端共用）。
//!
//! 通过 `utoipa_axum` 的 `.nest(prefix, router)` 挂载，前缀同时作用于
//! axum 路由与 OpenAPI 文档路径。handler 的 `#[utoipa::path]` 路径应使用
//! 组内相对路径，由 nest 自动补全前缀。

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::app_state::SharedState;
use crate::{auth, pet};

/// 后台管理端路由 — 仅 SuperAdmin 可访问。
///
/// 登录入口 `POST /admin/login`（在 `admin_login` handler 内校验角色），
/// 其余管理端点未来通过 `SuperAdminUser` 提取器保护。
pub fn admin_router() -> OpenApiRouter<SharedState> {
    OpenApiRouter::new().routes(routes!(auth::handler::admin_login))
}

/// 客户端路由 — 面向终端用户（client_app）。
///
/// 包含客户端认证（注册 / 登录）与宠物档案的全部 CRUD。
pub fn app_router() -> OpenApiRouter<SharedState> {
    OpenApiRouter::new()
        .routes(routes!(auth::handler::register))
        .routes(routes!(auth::handler::login))
        .merge(pet::router())
}

/// 两端共用路由 — 后台与客户端均会调用。
///
/// 当前包含 `GET /common/auth/me` 与 `POST /common/auth/token/refresh`。
pub fn common_router() -> OpenApiRouter<SharedState> {
    OpenApiRouter::new()
        .routes(routes!(auth::handler::refresh_token))
        .routes(routes!(auth::handler::get_me))
}
