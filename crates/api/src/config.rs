use once_cell::sync::Lazy;

pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub bind_addr: String,
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    database_url: std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:data.db".into()),
    jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
    bind_addr: std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".into()),
});