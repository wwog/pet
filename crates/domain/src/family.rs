use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

/// 家庭实体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Family {
    pub id: Uuid,
    pub name: String,
    pub avatar: Option<String>,
    pub city: Option<String>,
    /// 监护人（管理员）用户 ID
    pub guardian_id: Uuid,
    pub created_at: DateTime<Utc>,
}

/// 家庭成员。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FamilyMember {
    pub id: Uuid,
    pub family_id: Uuid,
    pub user_id: Uuid,
    /// 角色名（如"爸爸"、"妈妈"）
    pub role: String,
    /// 是否为监护人（拥有管理权限）
    pub is_guardian: bool,
    /// 精细权限位掩码（12 位，index 1-12）
    pub permissions: Permissions,
    pub joined_at: DateTime<Utc>,
}

/// 精细权限位掩码。
///
/// 使用 u16 的低 12 位（0x0FFF），每位对应一个权限开关。
/// index 1-12 分别对应具体的功能权限。
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

/// 邀请码，用于家庭新成员加入。
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
