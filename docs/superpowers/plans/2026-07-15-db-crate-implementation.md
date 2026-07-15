# DB Crate 实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 实现 auth + family + pet 三个核心域的 DDD 分层数据库层：domain 层定义 repository trait，db 层用 toasty ORM 实现。

**Architecture:** domain 层保持纯净，只定义实体和 `#[async_trait]` repository trait；db 层持有 toasty 模型（`#[derive(toasty::Model)]`），通过 `From` mapper 转换为 domain 实体，通过 `Database` 连接管理器暴露 repository 工厂方法。datetime 字段在 toasty 层存为 ISO 8601 `String`，mapper 中转换为 `chrono::DateTime<Utc>`。

**Tech Stack:** toasty 0.8.0 (sqlite), async-trait, uuid, chrono, serde, thiserror

---

### Task 1: 更新根 workspace Cargo.toml

**Files:**
- Modify: `Cargo.toml`

- [ ] **Step 1: 添加 workspace 依赖**

把根 `Cargo.toml` 的 `[workspace.dependencies]` 替换为以下内容：

```toml
[workspace.dependencies]
domain = { path = "crates/domain" }
db = { path = "crates/db" }

serde = { version = "1.0.228", features = ["derive"] }
serde_json = "1.0.150"
tokio = { version = "1.52.3", features = ['full'] }
uuid = { version = "1.23.5", features = ["v4", "serde"] }
chrono = { version = "0.4.45", features = ["serde"] }
thiserror = "2.0.18"
anyhow = "1.0.103"
tracing = "0.1.44"
tracing-subscriber = { version = "0.3.23", features = ["env-filter"] }
axum = "0.8.9"
toasty = { version = "0.8.0", features = ["sqlite"] }
async-trait = "0.1.88"
```

- [ ] **Step 2: 验证 Cargo.toml 语法**

```bash
cargo metadata --no-deps 2>&1 | head -5
```
Expected: 成功输出 JSON（以 `{` 开头），无错误。

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml
git commit -m "chore: add async-trait, toasty sqlite feature, db path to workspace deps"
```

---

### Task 2: 更新 domain crate Cargo.toml

**Files:**
- Modify: `crates/domain/Cargo.toml`

- [ ] **Step 1: 添加 async-trait 依赖**

把 `crates/domain/Cargo.toml` 替换为：

```toml
[package]
name = "domain"
version = "0.1.0"
edition.workspace = true

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
thiserror = { workspace = true }
async-trait = { workspace = true }
```

- [ ] **Step 2: Commit**

```bash
git add crates/domain/Cargo.toml
git commit -m "chore(domain): add async-trait dependency for repository traits"
```

---

### Task 3: 更新 db crate Cargo.toml

**Files:**
- Modify: `crates/db/Cargo.toml`

- [ ] **Step 1: 添加 domain 和 async-trait 依赖**

把 `crates/db/Cargo.toml` 替换为：

```toml
[package]
name = "db"
version = "0.1.0"
edition.workspace = true

[dependencies]
domain = { workspace = true }
toasty = { workspace = true }
uuid = { workspace = true }
async-trait = { workspace = true }
```

- [ ] **Step 2: Commit**

```bash
git add crates/db/Cargo.toml
git commit -m "chore(db): add domain and async-trait dependencies"
```

---

### Task 4: 重写 domain user.rs（实体 + repository trait）

**Files:**
- Modify: `crates/domain/src/user.rs`

- [ ] **Step 1: 写入完整的 User 域模型**

用以下内容替换 `crates/domain/src/user.rs`：

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub phone: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub wechat_open_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: DateTime<Utc>,
    pub device_id: Option<String>,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: User) -> AppResult<User>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<User>>;
    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<User>>;
    async fn find_by_wechat_open_id(&self, open_id: &str) -> AppResult<Option<User>>;
    async fn update_nickname(&self, id: Uuid, nickname: &str) -> AppResult<()>;
    async fn update_avatar(&self, id: Uuid, avatar: &str) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: UserSession) -> AppResult<UserSession>;
    async fn find_by_refresh_token(&self, token: &str) -> AppResult<Option<UserSession>>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn delete_by_user(&self, user_id: Uuid) -> AppResult<()>;
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/domain/src/user.rs
git commit -m "feat(domain): define User entity and UserRepository/SessionRepository traits"
```

---

### Task 5: 创建 domain family.rs（家庭域模型）

**Files:**
- Create: `crates/domain/src/family.rs`

- [ ] **Step 1: 写入 Family 域模型**

创建 `crates/domain/src/family.rs`：

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub city: Option<String>,
    pub guardian_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub is_guardian: bool,
    pub permissions: Permissions,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Permissions(u16);

impl Default for Permissions {
    fn default() -> Self {
        Self(0)
    }
}

impl Permissions {
    pub fn from_bits(bits: u16) -> Self {
        Self(bits & 0x0FFF)
    }

    pub fn bits(&self) -> u16 {
        self.0
    }

    pub fn is_enabled(&self, index: u8) -> bool {
        if index < 1 || index > 12 {
            return false;
        }
        (self.0 >> (index - 1)) & 1 == 1
    }

    pub fn enable(&mut self, index: u8) {
        if index >= 1 && index <= 12 {
            self.0 |= 1 << (index - 1);
        }
    }

    pub fn disable(&mut self, index: u8) {
        if index >= 1 && index <= 12 {
            self.0 &= !(1 << (index - 1));
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JoinStatus {
    Pending,
    Approved,
    Rejected,
}

impl JoinStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JoinStatus::Pending => "pending",
            JoinStatus::Approved => "approved",
            JoinStatus::Rejected => "rejected",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(JoinStatus::Pending),
            "approved" => Some(JoinStatus::Approved),
            "rejected" => Some(JoinStatus::Rejected),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteCode {
    pub id: Uuid,
    pub family_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRequest {
    pub id: Uuid,
    pub family_id: Uuid,
    pub applicant_id: Uuid,
    pub selected_role: String,
    pub note: Option<String>,
    pub status: JoinStatus,
    pub submitted_at: DateTime<Utc>,
}

#[async_trait]
pub trait FamilyRepository: Send + Sync {
    async fn create(&self, family: Family) -> AppResult<Family>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Family>>;
    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<Family>>;
    async fn update(&self, family: Family) -> AppResult<()>;
}

#[async_trait]
pub trait FamilyMemberRepository: Send + Sync {
    async fn create(&self, member: FamilyMember) -> AppResult<FamilyMember>;
    async fn find_by_family(&self, family_id: Uuid) -> AppResult<Vec<FamilyMember>>;
    async fn find_by_user_and_family(
        &self,
        user_id: Uuid,
        family_id: Uuid,
    ) -> AppResult<Option<FamilyMember>>;
    async fn update_role(&self, id: Uuid, role: &str) -> AppResult<()>;
    async fn update_permissions(&self, id: Uuid, permissions: Permissions) -> AppResult<()>;
    async fn transfer_guardian(
        &self,
        family_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait InviteCodeRepository: Send + Sync {
    async fn create(&self, invite_code: InviteCode) -> AppResult<InviteCode>;
    async fn find_by_code(&self, code: &str) -> AppResult<Option<InviteCode>>;
    async fn mark_used(&self, id: Uuid, used_by: Uuid) -> AppResult<()>;
}

#[async_trait]
pub trait JoinRequestRepository: Send + Sync {
    async fn create(&self, request: JoinRequest) -> AppResult<JoinRequest>;
    async fn find_by_family(
        &self,
        family_id: Uuid,
        status: Option<JoinStatus>,
    ) -> AppResult<Vec<JoinRequest>>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<JoinRequest>>;
    async fn update_status(&self, id: Uuid, status: JoinStatus) -> AppResult<()>;
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/domain/src/family.rs
git commit -m "feat(domain): define Family domain entities and repository traits"
```

---

### Task 6: 创建 domain pet.rs（宠物域模型）

**Files:**
- Create: `crates/domain/src/pet.rs`

- [ ] **Step 1: 写入 Pet 域模型**

创建 `crates/domain/src/pet.rs`：

```rust
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn as_str(&self) -> &'static str {
        match self {
            Gender::Male => "male",
            Gender::Female => "female",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "male" => Some(Gender::Male),
            "female" => Some(Gender::Female),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NeuterStatus {
    Neutered,
    Intact,
    Planned,
}

impl NeuterStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            NeuterStatus::Neutered => "neutered",
            NeuterStatus::Intact => "intact",
            NeuterStatus::Planned => "planned",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "neutered" => Some(NeuterStatus::Neutered),
            "intact" => Some(NeuterStatus::Intact),
            "planned" => Some(NeuterStatus::Planned),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pet {
    pub id: Uuid,
    pub family_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
    pub gender: Gender,
    pub birth_year: i32,
    pub birth_month: Option<i32>,
    pub birth_approximate: bool,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: NeuterStatus,
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub name: String,
    pub pinyin: String,
    pub initial: char,
    pub size_category: String,
    pub coat_type: String,
    pub standard_weight_min: f64,
    pub standard_weight_max: f64,
    pub life_span_min: i32,
    pub life_span_max: i32,
    pub exercise_needs: String,
    pub icon: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTag {
    pub id: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PetPersonalityTag {
    pub pet_id: Uuid,
    pub tag_id: String,
    pub custom_name: Option<String>,
}

#[async_trait]
pub trait PetRepository: Send + Sync {
    async fn create(&self, pet: Pet) -> AppResult<Pet>;
    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<Pet>>;
    async fn find_by_family(&self, family_id: Uuid) -> AppResult<Vec<Pet>>;
    async fn update(&self, pet: Pet) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
    async fn count_by_family(&self, family_id: Uuid) -> AppResult<i64>;
}

#[async_trait]
pub trait BreedRepository: Send + Sync {
    async fn find_by_id(&self, id: &str) -> AppResult<Option<Breed>>;
    async fn search(
        &self,
        keyword: &str,
        size: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<Breed>, u64)>;
}

#[async_trait]
pub trait PersonalityTagRepository: Send + Sync {
    async fn find_all_categories(&self) -> AppResult<Vec<PersonalityTag>>;
    async fn find_by_pet(&self, pet_id: Uuid) -> AppResult<Vec<PetPersonalityTag>>;
    async fn set_pet_tags(
        &self,
        pet_id: Uuid,
        tag_ids: Vec<String>,
        custom_tags: Vec<String>,
    ) -> AppResult<()>;
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/domain/src/pet.rs
git commit -m "feat(domain): define Pet domain entities and repository traits"
```

---

### Task 7: 更新 domain lib.rs（导出新模块）

**Files:**
- Modify: `crates/domain/src/lib.rs`

- [ ] **Step 1: 更新模块导出**

用以下内容替换 `crates/domain/src/lib.rs`：

```rust
pub mod app;
pub mod user;
pub mod family;
pub mod pet;
```

- [ ] **Step 2: Commit**

```bash
git add crates/domain/src/lib.rs
git commit -m "feat(domain): export new family and pet modules"
```

---

### Task 8: 重写 db lib.rs（Database 连接管理器）

**Files:**
- Modify: `crates/db/src/lib.rs`

- [ ] **Step 1: 写入 Database 连接管理器**

用以下内容替换 `crates/db/src/lib.rs`：

```rust
pub mod user;
pub mod family;
pub mod pet;

use crate::user::UserRepository;
use crate::family::FamilyRepository;
use crate::pet::PetRepository;

pub struct Database {
    db: toasty::Db,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, toasty::Error> {
        let db = toasty::Db::builder()
            .models(toasty::models!(crate::user::User, crate::user::UserSession))
            .models(toasty::models!(crate::family::Family, crate::family::FamilyMember, crate::family::InviteCode, crate::family::JoinRequest))
            .models(toasty::models!(crate::pet::Pet, crate::pet::Breed, crate::pet::PersonalityTag, crate::pet::PetPersonalityTag))
            .connect(url)
            .await?;
        Ok(Self { db })
    }

    pub async fn push_schema(&self) -> Result<(), toasty::Error> {
        self.db.push_schema().await
    }

    pub fn user_repository(&self) -> UserRepository<'_> {
        UserRepository::new(&self.db)
    }

    pub fn family_repository(&self) -> FamilyRepository<'_> {
        FamilyRepository::new(&self.db)
    }

    pub fn pet_repository(&self) -> PetRepository<'_> {
        PetRepository::new(&self.db)
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/db/src/lib.rs
git commit -m "feat(db): add Database connection manager with repository factory methods"
```

---

### Task 9: 重写 db user.rs（toasty 模型 + mapper + repository 实现）

**Files:**
- Modify: `crates/db/src/user.rs`

- [ ] **Step 1: 写入 toasty User 模型、mapper 和 repository 实现**

用以下内容替换 `crates/db/src/user.rs`：

```rust
use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::user as domain_user;
use uuid::Uuid;

use crate::Database;

#[derive(Debug, toasty::Model)]
#[table = "users"]
pub struct User {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[unique]
    pub phone: String,

    pub nickname: Option<String>,
    pub avatar: Option<String>,
    #[unique]
    pub wechat_open_id: Option<String>,

    pub created_at: String,
}

#[derive(Debug, toasty::Model)]
#[table = "user_sessions"]
pub struct UserSession {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub user_id: uuid::Uuid,

    #[unique]
    pub refresh_token: String,

    pub access_token: String,
    pub expires_at: String,
    pub device_id: Option<String>,
}

impl From<User> for domain_user::User {
    fn from(u: User) -> Self {
        domain_user::User {
            id: u.id,
            phone: u.phone,
            nickname: u.nickname,
            avatar: u.avatar,
            wechat_open_id: u.wechat_open_id,
            created_at: chrono::DateTime::parse_from_rfc3339(&u.created_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

impl From<UserSession> for domain_user::UserSession {
    fn from(s: UserSession) -> Self {
        domain_user::UserSession {
            id: s.id,
            user_id: s.user_id,
            access_token: s.access_token,
            refresh_token: s.refresh_token,
            expires_at: chrono::DateTime::parse_from_rfc3339(&s.expires_at)
                .map(|dt| dt.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            device_id: s.device_id,
        }
    }
}

pub struct UserRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_user::UserRepository for UserRepository<'_> {
    async fn create(&self, user: domain_user::User) -> AppResult<domain_user::User> {
        let now = user.created_at.to_rfc3339();
        let created = toasty::create!(User {
            id: user.id,
            phone: user.phone,
            nickname: user.nickname,
            avatar: user.avatar,
            wechat_open_id: user.wechat_open_id,
            created_at: now,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_user::User>> {
        let user = User::get_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<domain_user::User>> {
        let user = User::filter_by_phone(phone)
            .first()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn find_by_wechat_open_id(&self, open_id: &str) -> AppResult<Option<domain_user::User>> {
        let user = User::filter_by_wechat_open_id(open_id)
            .first()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn update_nickname(&self, id: Uuid, nickname: &str) -> AppResult<()> {
        User::update_by_id(id)
            .set(User::NICKNAME, nickname.to_owned())
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn update_avatar(&self, id: Uuid, avatar: &str) -> AppResult<()> {
        User::update_by_id(id)
            .set(User::AVATAR, avatar.to_owned())
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        User::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

pub struct SessionRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> SessionRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_user::SessionRepository for SessionRepository<'_> {
    async fn create(&self, session: domain_user::UserSession) -> AppResult<domain_user::UserSession> {
        let expires = session.expires_at.to_rfc3339();
        let created = toasty::create!(UserSession {
            id: session.id,
            user_id: session.user_id,
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_at: expires,
            device_id: session.device_id,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_refresh_token(&self, token: &str) -> AppResult<Option<domain_user::UserSession>> {
        let session = UserSession::filter_by_refresh_token(token)
            .first()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(session.map(Into::into))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        UserSession::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_user(&self, user_id: Uuid) -> AppResult<()> {
        UserSession::filter_by_user_id(user_id)
            .delete()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/db/src/user.rs
git commit -m "feat(db): implement User and UserSession toasty models with repository"
```

---

### Task 10: 创建 db family.rs（toasty 模型 + mapper + repository 实现）

**Files:**
- Create: `crates/db/src/family.rs`

- [ ] **Step 1: 写入 Family toasty 模型和 repository 实现**

创建 `crates/db/src/family.rs`：

```rust
use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::family::{self as domain_family, JoinStatus, Permissions};
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[table = "families"]
pub struct Family {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    pub name: String,
    pub avatar: Option<String>,
    pub city: Option<String>,

    #[index]
    pub guardian_id: uuid::Uuid,

    pub created_at: String,
}

#[derive(Debug, toasty::Model)]
#[table = "family_members"]
pub struct FamilyMember {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub family_id: uuid::Uuid,

    #[index]
    pub user_id: uuid::Uuid,

    pub role: String,
    pub is_guardian: bool,
    pub permissions: i32,
    pub joined_at: String,
}

#[derive(Debug, toasty::Model)]
#[table = "invite_codes"]
pub struct InviteCode {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub family_id: uuid::Uuid,

    #[unique]
    pub code: String,

    pub expires_at: String,
    pub used_by: Option<uuid::Uuid>,
}

#[derive(Debug, toasty::Model)]
#[table = "join_requests"]
pub struct JoinRequest {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub family_id: uuid::Uuid,

    pub applicant_id: uuid::Uuid,
    pub selected_role: String,
    pub note: Option<String>,
    pub status: String,
    pub submitted_at: String,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<Family> for domain_family::Family {
    fn from(f: Family) -> Self {
        domain_family::Family {
            id: f.id,
            name: f.name,
            avatar: f.avatar,
            city: f.city,
            guardian_id: f.guardian_id,
            created_at: parse_datetime(&f.created_at),
        }
    }
}

impl From<FamilyMember> for domain_family::FamilyMember {
    fn from(m: FamilyMember) -> Self {
        domain_family::FamilyMember {
            id: m.id,
            family_id: m.family_id,
            user_id: m.user_id,
            role: m.role,
            is_guardian: m.is_guardian,
            permissions: Permissions::from_bits(m.permissions as u16),
            joined_at: parse_datetime(&m.joined_at),
        }
    }
}

impl From<InviteCode> for domain_family::InviteCode {
    fn from(i: InviteCode) -> Self {
        domain_family::InviteCode {
            id: i.id,
            family_id: i.family_id,
            code: i.code,
            expires_at: parse_datetime(&i.expires_at),
            used_by: i.used_by,
        }
    }
}

impl From<JoinRequest> for domain_family::JoinRequest {
    fn from(j: JoinRequest) -> Self {
        domain_family::JoinRequest {
            id: j.id,
            family_id: j.family_id,
            applicant_id: j.applicant_id,
            selected_role: j.selected_role,
            note: j.note,
            status: JoinStatus::from_str(&j.status).unwrap_or(JoinStatus::Pending),
            submitted_at: parse_datetime(&j.submitted_at),
        }
    }
}

pub struct FamilyRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> FamilyRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_family::FamilyRepository for FamilyRepository<'_> {
    async fn create(&self, family: domain_family::Family) -> AppResult<domain_family::Family> {
        let now = family.created_at.to_rfc3339();
        let created = toasty::create!(Family {
            id: family.id,
            name: family.name,
            avatar: family.avatar,
            city: family.city,
            guardian_id: family.guardian_id,
            created_at: now,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_family::Family>> {
        let family = Family::get_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(family.map(Into::into))
    }

    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<domain_family::Family>> {
        let members = FamilyMember::filter_by_user_id(user_id)
            .all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut families = Vec::new();
        for member in members {
            if let Some(family) = Family::get_by_id(member.family_id)
                .exec(self.db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
            {
                families.push(family.into());
            }
        }
        Ok(families)
    }

    async fn update(&self, family: domain_family::Family) -> AppResult<()> {
        Family::update_by_id(family.id)
            .set(Family::NAME, family.name)
            .set(Family::AVATAR, family.avatar)
            .set(Family::CITY, family.city)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

pub struct FamilyMemberRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> FamilyMemberRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_family::FamilyMemberRepository for FamilyMemberRepository<'_> {
    async fn create(
        &self,
        member: domain_family::FamilyMember,
    ) -> AppResult<domain_family::FamilyMember> {
        let joined = member.joined_at.to_rfc3339();
        let created = toasty::create!(FamilyMember {
            id: member.id,
            family_id: member.family_id,
            user_id: member.user_id,
            role: member.role,
            is_guardian: member.is_guardian,
            permissions: member.permissions.bits() as i32,
            joined_at: joined,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_family(
        &self,
        family_id: Uuid,
    ) -> AppResult<Vec<domain_family::FamilyMember>> {
        let members = FamilyMember::filter_by_family_id(family_id)
            .all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(members.into_iter().map(Into::into).collect())
    }

    async fn find_by_user_and_family(
        &self,
        user_id: Uuid,
        family_id: Uuid,
    ) -> AppResult<Option<domain_family::FamilyMember>> {
        // Use query to filter by both fields
        let member = FamilyMember::filter(
            FamilyMember::USER_ID
                .eq(user_id)
                .and(FamilyMember::FAMILY_ID.eq(family_id)),
        )
        .first()
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(member.map(Into::into))
    }

    async fn update_role(&self, id: Uuid, role: &str) -> AppResult<()> {
        FamilyMember::update_by_id(id)
            .set(FamilyMember::ROLE, role.to_owned())
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn update_permissions(
        &self,
        id: Uuid,
        permissions: Permissions,
    ) -> AppResult<()> {
        FamilyMember::update_by_id(id)
            .set(FamilyMember::PERMISSIONS, permissions.bits() as i32)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn transfer_guardian(
        &self,
        family_id: Uuid,
        from_user_id: Uuid,
        to_user_id: Uuid,
    ) -> AppResult<()> {
        // Demote old guardian
        let old_member = self
            .find_by_user_and_family(from_user_id, family_id)
            .await?;
        if let Some(member) = old_member {
            FamilyMember::update_by_id(member.id)
                .set(FamilyMember::IS_GUARDIAN, false)
                .exec(self.db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // Promote new guardian
        let new_member = self
            .find_by_user_and_family(to_user_id, family_id)
            .await?;
        if let Some(member) = new_member {
            FamilyMember::update_by_id(member.id)
                .set(FamilyMember::IS_GUARDIAN, true)
                .exec(self.db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // Update family guardian_id
        Family::update_by_id(family_id)
            .set(Family::GUARDIAN_ID, to_user_id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        FamilyMember::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

pub struct InviteCodeRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> InviteCodeRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_family::InviteCodeRepository for InviteCodeRepository<'_> {
    async fn create(
        &self,
        invite_code: domain_family::InviteCode,
    ) -> AppResult<domain_family::InviteCode> {
        let expires = invite_code.expires_at.to_rfc3339();
        let created = toasty::create!(InviteCode {
            id: invite_code.id,
            family_id: invite_code.family_id,
            code: invite_code.code,
            expires_at: expires,
            used_by: invite_code.used_by,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_code(&self, code: &str) -> AppResult<Option<domain_family::InviteCode>> {
        let invite = InviteCode::filter_by_code(code)
            .first()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(invite.map(Into::into))
    }

    async fn mark_used(&self, id: Uuid, used_by: Uuid) -> AppResult<()> {
        InviteCode::update_by_id(id)
            .set(InviteCode::USED_BY, Some(used_by))
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

pub struct JoinRequestRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> JoinRequestRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_family::JoinRequestRepository for JoinRequestRepository<'_> {
    async fn create(
        &self,
        request: domain_family::JoinRequest,
    ) -> AppResult<domain_family::JoinRequest> {
        let submitted = request.submitted_at.to_rfc3339();
        let created = toasty::create!(JoinRequest {
            id: request.id,
            family_id: request.family_id,
            applicant_id: request.applicant_id,
            selected_role: request.selected_role,
            note: request.note,
            status: request.status.as_str().to_owned(),
            submitted_at: submitted,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_family(
        &self,
        family_id: Uuid,
        status: Option<JoinStatus>,
    ) -> AppResult<Vec<domain_family::JoinRequest>> {
        let query = JoinRequest::filter_by_family_id(family_id);
        let results = match status {
            Some(s) => {
                query
                    .filter(JoinRequest::STATUS.eq(s.as_str()))
                    .all()
                    .exec(self.db)
                    .await
            }
            None => query.all().exec(self.db).await,
        }
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(results.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_family::JoinRequest>> {
        let request = JoinRequest::get_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(request.map(Into::into))
    }

    async fn update_status(&self, id: Uuid, status: JoinStatus) -> AppResult<()> {
        JoinRequest::update_by_id(id)
            .set(JoinRequest::STATUS, status.as_str().to_owned())
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/db/src/family.rs
git commit -m "feat(db): implement Family toasty models with repository"
```

---

### Task 11: 创建 db pet.rs（toasty 模型 + mapper + repository 实现）

**Files:**
- Create: `crates/db/src/pet.rs`

- [ ] **Step 1: 写入 Pet toasty 模型和 repository 实现**

创建 `crates/db/src/pet.rs`：

```rust
use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::pet::{self as domain_pet, Gender, NeuterStatus};
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[table = "pets"]
pub struct Pet {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub family_id: uuid::Uuid,

    pub name: String,
    pub emoji: Option<String>,
    pub gender: String,
    pub birth_year: i32,
    pub birth_month: Option<i32>,
    pub birth_approximate: bool,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: String,
    pub avatar: Option<String>,

    pub created_at: String,
}

#[derive(Debug, toasty::Model)]
#[table = "breeds"]
pub struct Breed {
    #[key]
    pub id: String,

    pub name: String,
    pub pinyin: String,
    pub initial: String,
    pub size_category: String,
    pub coat_type: String,
    pub standard_weight_min: f64,
    pub standard_weight_max: f64,
    pub life_span_min: i32,
    pub life_span_max: i32,
    pub exercise_needs: String,
    pub icon: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, toasty::Model)]
#[table = "personality_tags"]
pub struct PersonalityTag {
    #[key]
    pub id: String,

    pub name: String,
    pub category: String,
}

#[derive(Debug, toasty::Model)]
#[table = "pet_personality_tags"]
pub struct PetPersonalityTag {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[index]
    pub pet_id: uuid::Uuid,

    pub tag_id: String,
    pub custom_name: Option<String>,
}

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

impl From<Pet> for domain_pet::Pet {
    fn from(p: Pet) -> Self {
        domain_pet::Pet {
            id: p.id,
            family_id: p.family_id,
            name: p.name,
            emoji: p.emoji,
            gender: Gender::from_str(&p.gender).unwrap_or(Gender::Male),
            birth_year: p.birth_year,
            birth_month: p.birth_month,
            birth_approximate: p.birth_approximate,
            breed_id: p.breed_id,
            coat_color: p.coat_color,
            coat_pattern: p.coat_pattern,
            neuter_status: NeuterStatus::from_str(&p.neuter_status)
                .unwrap_or(NeuterStatus::Intact),
            avatar: p.avatar,
            created_at: parse_datetime(&p.created_at),
        }
    }
}

impl From<Breed> for domain_pet::Breed {
    fn from(b: Breed) -> Self {
        domain_pet::Breed {
            id: b.id,
            name: b.name,
            pinyin: b.pinyin,
            initial: b.initial.chars().next().unwrap_or('?'),
            size_category: b.size_category,
            coat_type: b.coat_type,
            standard_weight_min: b.standard_weight_min,
            standard_weight_max: b.standard_weight_max,
            life_span_min: b.life_span_min,
            life_span_max: b.life_span_max,
            exercise_needs: b.exercise_needs,
            icon: b.icon,
            origin: b.origin,
        }
    }
}

impl From<PersonalityTag> for domain_pet::PersonalityTag {
    fn from(t: PersonalityTag) -> Self {
        domain_pet::PersonalityTag {
            id: t.id,
            name: t.name,
            category: t.category,
        }
    }
}

impl From<PetPersonalityTag> for domain_pet::PetPersonalityTag {
    fn from(t: PetPersonalityTag) -> Self {
        domain_pet::PetPersonalityTag {
            pet_id: t.pet_id,
            tag_id: t.tag_id,
            custom_name: t.custom_name,
        }
    }
}

pub struct PetRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> PetRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_pet::PetRepository for PetRepository<'_> {
    async fn create(&self, pet: domain_pet::Pet) -> AppResult<domain_pet::Pet> {
        let now = pet.created_at.to_rfc3339();
        let created = toasty::create!(Pet {
            id: pet.id,
            family_id: pet.family_id,
            name: pet.name,
            emoji: pet.emoji,
            gender: pet.gender.as_str().to_owned(),
            birth_year: pet.birth_year,
            birth_month: pet.birth_month,
            birth_approximate: pet.birth_approximate,
            breed_id: pet.breed_id,
            coat_color: pet.coat_color,
            coat_pattern: pet.coat_pattern,
            neuter_status: pet.neuter_status.as_str().to_owned(),
            avatar: pet.avatar,
            created_at: now,
        })
        .exec(self.db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_pet::Pet>> {
        let pet = Pet::get_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(pet.map(Into::into))
    }

    async fn find_by_family(&self, family_id: Uuid) -> AppResult<Vec<domain_pet::Pet>> {
        let pets = Pet::filter_by_family_id(family_id)
            .all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(pets.into_iter().map(Into::into).collect())
    }

    async fn update(&self, pet: domain_pet::Pet) -> AppResult<()> {
        Pet::update_by_id(pet.id)
            .set(Pet::NAME, pet.name)
            .set(Pet::EMOJI, pet.emoji)
            .set(Pet::GENDER, pet.gender.as_str().to_owned())
            .set(Pet::BIRTH_YEAR, pet.birth_year)
            .set(Pet::BIRTH_MONTH, pet.birth_month)
            .set(Pet::BIRTH_APPROXIMATE, pet.birth_approximate)
            .set(Pet::BREED_ID, pet.breed_id)
            .set(Pet::COAT_COLOR, pet.coat_color)
            .set(Pet::COAT_PATTERN, pet.coat_pattern)
            .set(Pet::NEUTER_STATUS, pet.neuter_status.as_str().to_owned())
            .set(Pet::AVATAR, pet.avatar)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        Pet::delete_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn count_by_family(&self, family_id: Uuid) -> AppResult<i64> {
        let count = Pet::filter_by_family_id(family_id)
            .count()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(count as i64)
    }
}

pub struct BreedRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> BreedRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_pet::BreedRepository for BreedRepository<'_> {
    async fn find_by_id(&self, id: &str) -> AppResult<Option<domain_pet::Breed>> {
        let breed = Breed::get_by_id(id)
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(breed.map(Into::into))
    }

    async fn search(
        &self,
        keyword: &str,
        size: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<domain_pet::Breed>, u64)> {
        let mut query = Breed::all();

        if !keyword.is_empty() {
            let lower = keyword.to_lowercase();
            query = query.filter(
                Breed::NAME
                    .like(format!("%{}%", lower))
                    .or(Breed::PINYIN.like(format!("%{}%", lower)))
                    .or(Breed::INITIAL.eq(lower.clone())),
            );
        }

        if let Some(size_cat) = size {
            query = query.filter(Breed::SIZE_CATEGORY.eq(size_cat.to_owned()));
        }

        let total = query
            .count()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let offset = ((page.saturating_sub(1)) * page_size) as usize;
        let results = query
            .limit(page_size as usize)
            .offset(offset)
            .all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((results.into_iter().map(Into::into).collect(), total))
    }
}

pub struct PersonalityTagRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> PersonalityTagRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_pet::PersonalityTagRepository for PersonalityTagRepository<'_> {
    async fn find_all_categories(&self) -> AppResult<Vec<domain_pet::PersonalityTag>> {
        let tags = PersonalityTag::all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(tags.into_iter().map(Into::into).collect())
    }

    async fn find_by_pet(&self, pet_id: Uuid) -> AppResult<Vec<domain_pet::PetPersonalityTag>> {
        let tags = PetPersonalityTag::filter_by_pet_id(pet_id)
            .all()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(tags.into_iter().map(Into::into).collect())
    }

    async fn set_pet_tags(
        &self,
        pet_id: Uuid,
        tag_ids: Vec<String>,
        custom_tags: Vec<String>,
    ) -> AppResult<()> {
        // Delete existing tags
        PetPersonalityTag::filter_by_pet_id(pet_id)
            .delete()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Insert preset tags
        for tag_id in &tag_ids {
            toasty::create!(PetPersonalityTag {
                pet_id,
                tag_id: tag_id.clone(),
                custom_name: None,
            })
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // Insert custom tags
        for name in &custom_tags {
            toasty::create!(PetPersonalityTag {
                pet_id,
                tag_id: String::from("custom"),
                custom_name: Some(name.clone()),
            })
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok(())
    }
}
```

- [ ] **Step 2: Commit**

```bash
git add crates/db/src/pet.rs
git commit -m "feat(db): implement Pet toasty models with repository"
```

---

### Task 12: 验证编译

**Files:**
- None (verification only)

- [ ] **Step 1: 运行 cargo check**

```bash
cargo check 2>&1
```

Expected: 所有 crate 编译通过，无错误。

如果编译失败，根据错误信息修正对应文件后再运行。常见可能需调整的问题：
- toasty 生成的字段常量名可能与代码中的不一致（如 `User::NICKNAME` 可能被生成或其他命名），需要根据编译错误调整
- `operator` 或 `like` 等方法可能需要特定导入

- [ ] **Step 2: Commit（如果有点修正的话）**

```bash
git add -A && git commit -m "chore: fix compilation issues from cargo check"
```
