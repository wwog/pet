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
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    tracing::info!("connecting to database: {}", CONFIG.database_url);
    let database = db::Database::connect(&CONFIG.database_url)
        .await
        .expect("failed to connect to database");

    database
        .ensure_schema()
        .await
        .expect("failed to initialize database schema");

    let state = Arc::new(AppState { db: database });

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
