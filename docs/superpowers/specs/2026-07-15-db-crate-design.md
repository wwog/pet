# DB Crate 开发设计文档

> **日期**: 2026-07-15
> **范围**: auth + family + pet 三个核心域的数据库层
> **架构**: DDD 分层 — domain 层定义 repository trait，db 层用 toasty ORM 实现

---

## 1. 背景与问题

### 1.1 当前状态

- **domain crate**: 定义了 `User` 实体和 `AppError`/`AppResult` 错误体系，但 User 模型过时（仅 `username/password_hash/created_at`），缺少 phone、nickname、avatar、wechat_open_id 等 API 文档要求的字段
- **db crate**: 使用 toasty ORM，有一个 `User` 模型映射到 `pet_user` 表，但与 domain 层字段不一致，无 repository trait 抽象，无 domain 层依赖
- **api crate**: 几乎空壳，仅有 tracing 初始化
- **缺少核心域模型**: family、pet 等核心域无模型定义

### 1.2 核心问题

1. domain User 模型与 API 文档需求脱节
2. db 层与 domain 层脱节：toasty 模型和 domain 模型字段不一致，无映射逻辑
3. 缺少 Repository Trait：DDD 要求 domain 层定义 repository trait，db 层实现它
4. db crate 没有依赖 domain：违反 DDD 依赖方向
5. 缺少核心域模型：family、pet、health 等无模型定义

---

## 2. 架构设计

### 2.1 依赖方向

```
api → domain
api → db
db  → domain
```

domain 层保持纯净，只依赖 serde/uuid/chrono/thiserror/async-trait。db 层新增 domain 依赖，继续使用 toasty。toasty 模型定义在 db 层内部，通过 mapper 函数转换为 domain 实体。

### 2.2 方案选择：Repository Trait 抽象

在 domain 层为每个聚合根定义 `async trait` repository（如 `UserRepository`, `FamilyRepository`, `PetRepository`），db 层用 toasty 实现这些 trait。

- db 层有自己的 toasty 模型（`#[derive(toasty::Model)]`）
- 通过 mapper 函数（`impl From<DbModel> for DomainEntity`）转换
- domain 层零基础设施依赖，可测试性高

### 2.3 Crate 结构

```
crates/domain/
├── Cargo.toml          # 新增 async-trait
└── src/
    ├── lib.rs          # pub mod user; pub mod family; pub mod pet; pub mod app;
    ├── app.rs          # AppError, AppResult（现有）
    ├── user.rs         # User 实体 + UserRepository trait + SessionRepository trait
    ├── family.rs       # Family, FamilyMember, Permissions, InviteCode, JoinRequest + repository traits
    └── pet.rs          # Pet, Breed, PersonalityTag + repository traits

crates/db/
├── Cargo.toml          # 新增 domain 依赖，toasty 启用 sqlite feature
└── src/
    ├── lib.rs          # Database 结构体 + 连接管理 + repository 工厂方法
    ├── user.rs         # toasty User 模型 + UserRepository 实现 + mapper
    ├── family.rs       # toasty Family/FamilyMember/InviteCode/JoinRequest 模型 + mapper + repository 实现
    └── pet.rs          # toasty Pet/Breed/PetPersonalityTag 模型 + mapper + repository 实现
```

### 2.4 DB Crate 连接管理

db crate 暴露 `Database` 结构体，持有 `toasty::Db`，作为所有 repository 实现的共享连接入口：

```rust
pub struct Database {
    db: toasty::Db,
}

impl Database {
    pub async fn connect(url: &str) -> Result<Self, toasty::Error>;
    pub async fn push_schema(&self) -> Result<(), toasty::Error>;
    pub fn user_repository(&self) -> UserRepository<'_>;
    pub fn family_repository(&self) -> FamilyRepository<'_>;
    pub fn pet_repository(&self) -> PetRepository<'_>;
}
```

每个 repository 持有 `&toasty::Db` 引用，实现 domain 层对应的 trait。api 层通过 `Database` 获取 repository，注入到 handler 中。

---

## 3. Domain 层模型设计

### 3.1 User 域（auth 模块）

```rust
pub struct User {
    pub id: Uuid,
    pub phone: String,
    pub nickname: Option<String>,
    pub avatar: Option<String>,
    pub wechat_open_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

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

### 3.2 Family 域（family 模块）

```rust
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub city: Option<String>,
    pub guardian_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct FamilyMember {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
    pub is_guardian: bool,
    pub permissions: Permissions,
    pub joined_at: DateTime<Utc>,
}

pub struct Permissions(u16);  // 位域: P1=bit0 ... P12=bit11

pub struct InviteCode {
    pub id: Uuid,
    pub family_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
    pub used_by: Option<Uuid>,
}

pub struct JoinRequest {
    pub id: Uuid,
    pub family_id: Uuid,
    pub applicant_id: Uuid,
    pub selected_role: String,
    pub note: Option<String>,
    pub status: JoinStatus,  // Pending, Approved, Rejected
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
    async fn find_by_user_and_family(&self, user_id: Uuid, family_id: Uuid) -> AppResult<Option<FamilyMember>>;
    async fn update_role(&self, id: Uuid, role: &str) -> AppResult<()>;
    async fn update_permissions(&self, id: Uuid, permissions: Permissions) -> AppResult<()>;
    async fn transfer_guardian(&self, family_id: Uuid, old_guardian_id: Uuid, new_guardian_id: Uuid) -> AppResult<()>;
    async fn delete(&self, id: Uuid) -> AppResult<()>;
}
```

### 3.3 Pet 域（pet 模块）

```rust
pub struct Pet {
    pub id: Uuid,
    pub family_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
    pub gender: Gender,          // Male, Female
    pub birth_year: i32,
    pub birth_month: Option<i32>,
    pub birth_approximate: bool,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: NeuterStatus, // Neutered, Intact, Planned
    pub avatar: Option<String>,
    pub created_at: DateTime<Utc>,
}

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

pub struct PersonalityTag {
    pub id: String,
    pub name: String,
    pub category: String,  // social, behavior, emotion, custom
}

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
    async fn search(&self, keyword: &str, size: Option<&str>, page: u32, page_size: u32) -> AppResult<(Vec<Breed>, u64)>;
}

#[async_trait]
pub trait PersonalityTagRepository: Send + Sync {
    async fn find_all_categories(&self) -> AppResult<Vec<PersonalityTag>>;
    async fn find_by_pet(&self, pet_id: Uuid) -> AppResult<Vec<PetPersonalityTag>>;
    async fn set_pet_tags(&self, pet_id: Uuid, tag_ids: Vec<String>, custom_tags: Vec<String>) -> AppResult<()>;
}
```

---

## 4. DB 层 toasty 模型与 Mapper

### 4.1 toasty 模型定义

每个 toasty 模型对应一张表，字段使用 toasty 属性宏。模型字段与 domain 实体保持一一对应，但使用 toasty 的类型系统（如 `toasty::Instant` 替代 `chrono::DateTime<Utc>`）。

**User toasty 模型:**
```rust
#[derive(Debug, toasty::Model)]
#[table = "users"]
pub struct User {
    #[key]
    #[auto(uuid(v4))]
    id: uuid::Uuid,

    #[unique]
    phone: String,

    nickname: Option<String>,
    avatar: Option<String>,
    #[unique]
    wechat_open_id: Option<String>,

    #[auto]
    created_at: toasty::Instant,
}
```

### 4.2 模型映射关系

| db toasty 模型 | domain 实体 | 说明 |
|---|---|---|
| `db::user::User` | `domain::user::User` | 字段一一对应，mapper 直接转换 |
| `db::user::UserSession` | `domain::user::UserSession` | 同上 |
| `db::family::Family` | `domain::family::Family` | 同上 |
| `db::family::FamilyMember` | `domain::family::FamilyMember` | Permissions 用 u16 位域，toasty 存为 i32 |
| `db::family::InviteCode` | `domain::family::InviteCode` | 同上 |
| `db::family::JoinRequest` | `domain::family::JoinRequest` | JoinStatus 枚举存为 String |
| `db::pet::Pet` | `domain::pet::Pet` | Gender, NeuterStatus 枚举存为 String |
| `db::pet::Breed` | `domain::pet::Breed` | 品种库相对静态 |
| `db::pet::PetPersonalityTag` | `domain::pet::PetPersonalityTag` | 关联表 |

### 4.3 Mapper 函数

每个 db toasty 模型实现 `From<DbModel> for DomainEntity`：

```rust
impl From<db_user::User> for domain::user::User {
    fn from(u: db_user::User) -> Self {
        domain::user::User {
            id: u.id,
            phone: u.phone,
            nickname: u.nickname,
            avatar: u.avatar,
            wechat_open_id: u.wechat_open_id,
            created_at: u.created_at.into(),
        }
    }
}
```

### 4.4 时间类型处理

toasty 使用 `toasty::Instant`。domain 层使用 `chrono::DateTime<Utc>`。mapper 中通过 `.into()` 做类型转换。

### 4.5 错误转换

toasty 的 `toasty::Error` 统一转换为 `AppError::Database(String)`。在 db 层的 mapper 函数中处理：

```rust
fn map_toasty_error(e: toasty::Error) -> AppError {
    match e {
        // 唯一约束冲突 → Conflict
        // 记录不存在 → NotFound
        // 其他 → Database
    }
}
```

### 4.6 Repository 实现示例

```rust
pub struct UserRepository<'a> {
    db: &'a toasty::Db,
}

#[async_trait::async_trait]
impl<'a> domain::user::UserRepository for UserRepository<'a> {
    async fn find_by_phone(&self, phone: &str) -> AppResult<Option<domain::user::User>> {
        let user = db_user::User::filter_by_phone(phone)
            .first()
            .exec(self.db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }
    // ... 其他方法
}
```

---

## 5. Cargo.toml 变更

### 5.1 根 Cargo.toml（workspace.dependencies）

新增：
- `db = { path = "crates/db" }`
- `async-trait = "0.1.85"`
- `toasty` 启用 `sqlite` feature: `toasty = { version = "0.8.0", features = ["sqlite"] }`

### 5.2 crates/domain/Cargo.toml

新增 `async-trait = { workspace = true }`

### 5.3 crates/db/Cargo.toml

新增：
- `domain = { workspace = true }`
- `async-trait = { workspace = true }`

### 5.4 crates/api/Cargo.toml

新增 `db = { workspace = true }`

---

## 6. 实现顺序

1. **更新 workspace.dependencies**: 根 Cargo.toml 添加 async-trait、toasty 启用 sqlite feature、db 路径依赖
2. **更新 domain crate**:
   - `Cargo.toml` 添加 `async-trait`
   - 重写 `user.rs`: User 实体 + UserRepository trait + SessionRepository trait
   - 新建 `family.rs`: Family, FamilyMember, Permissions, InviteCode, JoinRequest + repository traits
   - 新建 `pet.rs`: Pet, Breed, PersonalityTag + repository traits
   - 更新 `lib.rs` 导出新模块
3. **更新 db crate**:
   - `Cargo.toml` 添加 `domain` 和 `async-trait` 依赖
   - `lib.rs`: Database 结构体 + 连接管理 + repository 工厂方法
   - 重写 `user.rs`: toasty User 模型 + mapper + UserRepository 实现
   - 新建 `family.rs`: toasty Family/FamilyMember/InviteCode/JoinRequest 模型 + mapper + repository 实现
   - 新建 `pet.rs`: toasty Pet/Breed/PetPersonalityTag 模型 + mapper + repository 实现
4. **验证编译**: 确保 `cargo check` 全绿

---

## 7. 范围边界

- **本次只做 auth + family + pet 三个核心域**的 db 层
- health 模块等后续按相同模式扩展
- api crate 暂不实现 HTTP 端点，只确保依赖链路能编译
- 品种库数据（Breed）只定义模型和 repository，不预填充数据
- toasty 使用 sqlite feature（开发阶段），后续可切换 postgres
