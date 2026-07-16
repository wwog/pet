use std::sync::Arc;

use axum::Router;
use tracing_subscriber::EnvFilter;

mod app_state;
mod auth;
mod config;
mod error;

use app_state::AppState;
use config::CONFIG;

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
        .push_schema()
        .await
        .expect("failed to push database schema");

    let state = Arc::new(AppState { db: database });

    let app = Router::new()
        .merge(auth::router())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&CONFIG.bind_addr)
        .await
        .expect("failed to bind address");

    tracing::info!("listening on {}", CONFIG.bind_addr);
    axum::serve(listener, app).await.expect("server error");
}