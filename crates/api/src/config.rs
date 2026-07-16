use once_cell::sync::Lazy;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
    pub app_env: String,
    pub enable_docs: bool,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    // 加载 .env 文件（优先级：当前目录 > 父级目录），不存在时静默跳过
    match dotenvy::dotenv() {
        Ok(path) => eprintln!("info: loaded env from {}", path.display()),
        Err(dotenvy::Error::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            // .env 文件不存在，只使用系统环境变量
        }
        Err(err) => eprintln!("warn: failed to load .env: {err}"),
    }

    let app_env = std::env::var("APP_ENV").unwrap_or_else(|_| "development".into());

    // 文档默认在非生产环境开放；ENABLE_DOCS 可显式覆盖（true/false/1/0）
    let enable_docs = match std::env::var("ENABLE_DOCS") {
        Ok(value) => matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes"),
        Err(_) => app_env != "production",
    };

    Config {
        database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".into()),
        jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
        app_env,
        enable_docs,
    }
});