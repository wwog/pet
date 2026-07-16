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
    fresh: bool,
}

impl Database {
    /// 建立数据库连接并注册所有模型。
    pub async fn connect(url: &str) -> Result<Self, toasty::Error> {
        // 在连接前判断是否为全新库：sqlite 连接会创建文件，必须先于 connect 采样
        let fresh = is_fresh_sqlite(url);
        let db = toasty::Db::builder()
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
            .await?;
        Ok(Self { db, fresh })
    }

    /// 幂等地初始化数据库 schema。
    ///
    /// toasty 的 `push_schema` 生成的是无条件 `CREATE TABLE`，对已存在表的库会报错，
    /// 因此仅在检测到全新库时执行。返回是否实际创建了 schema。
    pub async fn ensure_schema(&self) -> Result<bool, toasty::Error> {
        if self.fresh {
            self.db.push_schema().await?;
            return Ok(true);
        }
        Ok(false)
    }

    /// 强制创建/迁移数据库 schema（仅应对空库使用，重复调用会报表已存在）。
    pub async fn push_schema(&self) -> Result<(), toasty::Error> {
        self.db.push_schema().await
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

/// 判断 sqlite 连接是否指向一个尚不存在的（全新）数据库。
///
/// - 内存库（`:memory:`）每次进程都是全新的，返回 `true`。
/// - 文件库按文件是否已存在判断。
/// - 非 sqlite / 无法识别的 URL 保守地返回 `true`（交由调用方决定是否 push）。
fn is_fresh_sqlite(url: &str) -> bool {
    let rest = url.strip_prefix("sqlite:").unwrap_or(url);
    if rest.is_empty() || rest.contains(":memory:") {
        return true;
    }
    let path = rest.trim_start_matches("//");
    let path = path.split(['?', '#']).next().unwrap_or(path);
    !std::path::Path::new(path).exists()
}
