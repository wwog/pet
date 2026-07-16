use std::sync::Arc;

pub struct AppState {
    pub db: db::Database,
}

pub type SharedState = Arc<AppState>;