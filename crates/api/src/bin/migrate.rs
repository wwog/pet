//! migrate binary — 项目内的 toasty 迁移 CLI。
//!
//! 用法（在项目根目录执行）：
//!   cargo run -p api --bin migrate -- migration generate --name <name>
//!   cargo run -p api --bin migrate -- migration apply
//!   cargo run -p api --bin migrate -- migration snapshot
//!   cargo run -p api --bin migrate -- migration reset
//!
//! 复用 db crate 的 `build_db` 注册全部模型，保证与 server 跑的是同一份 schema。
//! 数据库 URL 从 `DATABASE_URL` 环境变量读取（默认 `sqlite:data.db`），
//! 与 server 的 `config::CONFIG.database_url` 保持一致。

use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".into());
    let config = Config::load()?;
    let db = db::build_db(&url).await?;

    ToastyCli::with_config(db, config).parse_and_run().await?;

    Ok(())
}
