use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::user as domain_user;
use uuid::Uuid;

/// toasty ORM 模型 — 对应 `users` 表。
///
/// datetime 以 ISO 8601 `String` 存储，由 `From` mapper 转换为 `DateTime<Utc>`。
#[derive(Debug, toasty::Model)]
#[table = "users"]
pub struct User {
    #[key]
    #[auto(uuid(v4))]
    pub id: uuid::Uuid,

    #[unique]
    pub account: String,

    pub password_hash: String,

    pub nickname: Option<String>,
    pub avatar: Option<String>,
    #[unique]
    pub wechat_open_id: Option<String>,

    #[index]
    pub role: String,

    pub created_at: String,
}

/// toasty ORM 模型 — 对应 `user_sessions` 表。
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

fn parse_datetime(s: &str) -> chrono::DateTime<chrono::Utc> {
    chrono::DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(|_| chrono::Utc::now())
}

/// toasty → domain 映射：将 ISO 8601 字符串转为 `DateTime<Utc>`。
impl From<User> for domain_user::User {
    fn from(u: User) -> Self {
        domain_user::User {
            id: u.id,
            account: u.account,
            password_hash: u.password_hash,
            nickname: u.nickname,
            avatar: u.avatar,
            wechat_open_id: u.wechat_open_id,
            role: domain_user::Role::from_str(&u.role).unwrap_or(domain_user::Role::User),
            created_at: parse_datetime(&u.created_at),
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
            expires_at: parse_datetime(&s.expires_at),
            device_id: s.device_id,
        }
    }
}

/// 用户 Repository 实现，封装 toasty 查询。
pub struct UserRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> UserRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
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
impl domain_user::UserRepository for UserRepository<'_> {
    async fn create(&self, user: domain_user::User) -> AppResult<domain_user::User> {
        let mut db = self.db.clone();
        let now = user.created_at.to_rfc3339();
        let created = toasty::create!(User {
            id: user.id,
            account: user.account,
            password_hash: user.password_hash,
            nickname: user.nickname,
            avatar: user.avatar,
            wechat_open_id: user.wechat_open_id,
            role: user.role.as_str().to_owned(),
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_user::User>> {
        let mut db = self.db.clone();
        let user = User::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn find_by_account(&self, account: &str) -> AppResult<Option<domain_user::User>> {
        let mut db = self.db.clone();
        let user = User::filter_by_account(account)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn find_by_wechat_open_id(&self, open_id: &str) -> AppResult<Option<domain_user::User>> {
        let mut db = self.db.clone();
        let user = User::filter_by_wechat_open_id(open_id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(user.map(Into::into))
    }

    async fn find_by_role(&self, role: domain_user::Role) -> AppResult<Vec<domain_user::User>> {
        let mut db = self.db.clone();
        let users = User::filter_by_role(role.as_str())
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(users.into_iter().map(Into::into).collect())
    }

    async fn update_nickname(&self, id: Uuid, nickname: &str) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = User::update_by_id(id);
        update.set_nickname(nickname.to_owned());
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn update_avatar(&self, id: Uuid, avatar: &str) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = User::update_by_id(id);
        update.set_avatar(avatar.to_owned());
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        User::delete_by_id(&mut db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl domain_user::SessionRepository for SessionRepository<'_> {
    async fn create(
        &self,
        session: domain_user::UserSession,
    ) -> AppResult<domain_user::UserSession> {
        let mut db = self.db.clone();
        let expires = session.expires_at.to_rfc3339();
        let created = toasty::create!(UserSession {
            id: session.id,
            user_id: session.user_id,
            access_token: session.access_token,
            refresh_token: session.refresh_token,
            expires_at: expires,
            device_id: session.device_id,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_refresh_token(
        &self,
        token: &str,
    ) -> AppResult<Option<domain_user::UserSession>> {
        let mut db = self.db.clone();
        let session = UserSession::filter_by_refresh_token(token)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(session.map(Into::into))
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        UserSession::delete_by_id(&mut db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete_by_user(&self, user_id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        UserSession::filter_by_user_id(user_id)
            .delete()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}