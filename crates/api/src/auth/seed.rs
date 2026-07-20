//! 超级管理员 seed — 在数据库迁移就绪后幂等地写入超级管理员账号。
//!
//! 每次启动后端时调用 `ensure_super_admin`，若账号已存在则跳过，
//! 不存在则用 argon2id 哈希密码后创建。保证首次部署即可登录后台。

use chrono::Utc;
use domain::app::AppError;
use domain::user::{Role, UserRepository};
use uuid::Uuid;

use super::handler::hash_password;
use crate::app_state::SharedState;

/// 超级管理员账号（硬编码，后续可移入 config）。
const SUPER_ADMIN_ACCOUNT: &str = "871782513";
/// 超级管理员初始密码。
const SUPER_ADMIN_PASSWORD: &str = "Pet12345.";
/// 超级管理员昵称。
const SUPER_ADMIN_NICKNAME: &str = "超级管理员";

/// 幂等地确保超级管理员账号存在。
///
/// 在启动时 `migration apply` 之后调用。若账号已存在则跳过；否则用 argon2id 哈希
/// 密码后创建 `Role::SuperAdmin` 用户。
pub async fn ensure_super_admin(state: &SharedState) -> Result<(), AppError> {
    let user_repo = state.db.user_repository();
    if user_repo
        .find_by_account(SUPER_ADMIN_ACCOUNT)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let password_hash = hash_password(SUPER_ADMIN_PASSWORD)?;
    let now = Utc::now();
    let user = domain::user::User {
        id: Uuid::new_v4(),
        account: SUPER_ADMIN_ACCOUNT.to_owned(),
        password_hash,
        nickname: Some(SUPER_ADMIN_NICKNAME.to_owned()),
        avatar: None,
        wechat_open_id: None,
        role: Role::SuperAdmin,
        created_at: now,
    };
    user_repo.create(user).await?;
    tracing::info!("super admin seeded: account={SUPER_ADMIN_ACCOUNT}");
    Ok(())
}
