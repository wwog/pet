//! DB 层 — 数据库连接管理与 Repository 工厂。
//!
//! `Database` 封装 toasty ORM 的连接池，通过 `models!` 注册所有模型，
//! 并暴露各域的 Repository 实例。调用方只需通过 factory 方法获取
//! repository，无需直接接触 toasty API。
//!
//! # 架构约定
//!
//! - 所有 datetime 字段在 toasty 层以 ISO 8601 `String` 存储，由 `From` mapper
//!   在进出时转换为 `chrono::DateTime<Utc>`。
//! - Repository 持有 `&'a toasty::Db` 引用，不独立管理连接。
//! - `Database` 实现了 `Clone`（toasty 内部 clone 是廉价的），
//!   因此 Repository 方法内通过 `self.db.clone()` 获得可变所有权。

pub mod user;
pub mod family;
pub mod pet;

use crate::user::{UserRepository, SessionRepository};
use crate::family::FamilyRepository;
use crate::pet::PetRepository;

/// 数据库连接管理器，封装 toasty ORM 连接池。
///
/// 注册模型后通过 factory 方法获取各域的 Repository 实例。
pub struct Database {
    db: toasty::Db,
}

/// 构建并连接 `toasty::Db`，注册全部模型。
///
/// server 和 migrate binary 共用此函数，保证两端的模型注册一致。
/// `Database::connect` 也复用此函数以避免注册逻辑重复。
pub async fn build_db(url: &str) -> Result<toasty::Db, toasty::Error> {
    toasty::Db::builder()
        .models(toasty::models!(
            crate::user::User,
            crate::user::UserSession,
            crate::family::Family,
            crate::family::FamilyMember,
            crate::family::InviteCode,
            crate::family::JoinRequest,
            crate::pet::Pet,
            crate::pet::Breed,
            crate::pet::PersonalityTag,
            crate::pet::PetPersonalityTag
        ))
        .connect(url)
        .await
}

impl Database {
    /// 建立数据库连接并注册所有模型。
    ///
    /// schema 的建立与演进由迁移系统（`toasty-cli` + `toasty/` 迁移文件）负责，
    /// server 在启动时通过 `migration apply` 应用待执行迁移，此处不再建表。
    pub async fn connect(url: &str) -> Result<Self, toasty::Error> {
        let db = build_db(url).await?;
        Ok(Self { db })
    }

    pub fn user_repository(&self) -> UserRepository<'_> {
        UserRepository::new(&self.db)
    }

    pub fn session_repository(&self) -> SessionRepository<'_> {
        SessionRepository::new(&self.db)
    }

    pub fn family_repository(&self) -> FamilyRepository<'_> {
        FamilyRepository::new(&self.db)
    }

    pub fn pet_repository(&self) -> PetRepository<'_> {
        PetRepository::new(&self.db)
    }
}
