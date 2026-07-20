use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::{Modify, OpenApi};

#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
    info(
        title = "Puppy Life OS API",
        version = "0.1.0",
        description = "小狗人生 - 家庭 AI 宠物管家 API 文档"
    ),
    tags(
        (name = "auth", description = "账号认证"),
        (name = "pet", description = "宠物档案")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        // `components` 在存在被注解的接口时已由 utoipa 初始化，这里补充 bearer 安全方案
        let components = openapi
            .components
            .get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
