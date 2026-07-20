//! seed_breeds binary — 将预置犬种 / 猫种数据幂等写入 `breeds` 表。
//!
//! 用法（在项目根目录执行）：
//!   cargo run -p api --bin seed_breeds
//!
//! 复用 db crate 的 `build_db` 注册全部模型，先应用待执行迁移确保 schema 就绪，
//! 再调用 `seed::breeds::seed_breeds` 写入。数据库 URL 与 server 保持一致。
//!
//! 幂等：已存在的 id 跳过，不会覆盖运营期补充的 `pinyin` / `initial` / 数值字段。

use toasty_cli::{Config, ToastyCli};

#[path = "../seed/breeds.rs"]
mod breeds_seed;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".into());
    let config = Config::load()?;
    let db_handle = db::build_db(&url).await?;

    ToastyCli::with_config(db_handle, config)
        .parse_from(["toasty", "migration", "apply"])
        .await
        .map_err(|e| anyhow::anyhow!("failed to apply migrations: {e}"))?;

    let database = db::Database::connect(&url).await?;
    let breed_repo = database.breed_repository();

    let (inserted, skipped) = breeds_seed::seed_breeds(&breed_repo).await?;
    println!("breeds seeded: inserted={inserted}, skipped={skipped}");
    Ok(())
}
