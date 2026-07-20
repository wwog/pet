use std::sync::Arc;

use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_scalar::{Scalar, Servable};

mod app_state;
mod auth;
mod config;
mod error;
mod openapi;
mod utils;

use app_state::AppState;
use config::CONFIG;
use openapi::ApiDoc;

#[tokio::main]
async fn main() {
    // 支持 `--export-openapi [path]`：仅导出 OpenAPI JSON 到文件后退出，不启动服务器
    let args: Vec<String> = std::env::args().collect();
    if args.iter().any(|arg| arg == "--export-openapi") {
        let out_path: String = args
            .iter()
            .position(|arg| arg == "--export-openapi")
            .and_then(|idx| args.get(idx + 1))
            .filter(|s| !s.starts_with("--"))
            .cloned()
            .unwrap_or_else(|| "packages/admin/openapi.json".into());
        // 走完整 router 合并，才能收集各模块 #[utoipa::path] 注解
        let (_, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
            .merge(auth::router())
            .split_for_parts();
        let json = api
            .to_pretty_json()
            .unwrap_or_else(|e| panic!("failed to serialize openapi: {e}"));
        std::fs::write(&out_path, json)
            .unwrap_or_else(|e| panic!("failed to write {out_path}: {e}"));
        eprintln!("info: openapi spec exported to {out_path}");
        return;
    }

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("connecting to database: {}", CONFIG.database_url);

    // 启动时自动应用待执行迁移。复用 migrate binary 的同一套 ToastyCli 命令，
    // 保证 server 与 migrate CLI 跑的是同一份 schema。迁移历史靠 `__toasty_migrations`
    // 表追踪，已应用的会跳过。
    let migrate_db = db::build_db(&CONFIG.database_url)
        .await
        .expect("failed to connect to database for migration");
    let migrate_config = toasty_cli::Config::load()
        .expect("failed to load Toasty.toml");
    toasty_cli::ToastyCli::with_config(migrate_db, migrate_config)
        .parse_from(["toasty", "migration", "apply"])
        .await
        .expect("failed to apply database migrations");

    let database = db::Database::connect(&CONFIG.database_url)
        .await
        .expect("failed to connect to database");

    let state = Arc::new(AppState { db: database });

    auth::seed::ensure_super_admin(&state)
        .await
        .expect("failed to seed super admin");

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .merge(auth::router())
        .split_for_parts();

    let app = if CONFIG.enable_docs {
        router.merge(Scalar::with_url("/scalar", api))
    } else {
        router
    }
    .with_state(state);

    let port = CONFIG.port;
    let addr = utils::url::bind_addr(port);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind address");

    let lan_addr = utils::url::local_addr(port);
    tracing::info!("listening on http://{} (env: {})", lan_addr.as_deref().unwrap_or("unknown"), CONFIG.app_env);
    tracing::info!("listening on http://localhost:{port}");
    if CONFIG.enable_docs {
        tracing::info!("API docs (Scalar): http://localhost:{port}/scalar");
        if let Some(ref addr) = lan_addr {
            tracing::info!("API docs (Scalar): http://{addr}/scalar");
        }
    } else {
        tracing::info!("API docs disabled (set ENABLE_DOCS=true to enable)");
    }
    axum::serve(listener, app).await.expect("server error");
}
