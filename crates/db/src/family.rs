use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::family::{self as domain_family, JoinStatus, Permissions};
use uuid::Uuid;

/// toasty ORM 模型 — 对应 `families` 表。
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

pub struct FamilyMemberRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> FamilyMemberRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
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

pub struct JoinRequestRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> JoinRequestRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
    }
}

#[async_trait]
impl domain_family::FamilyRepository for FamilyRepository<'_> {
    async fn create(&self, family: domain_family::Family) -> AppResult<domain_family::Family> {
        let mut db = self.db.clone();
        let now = family.created_at.to_rfc3339();
        let created = toasty::create!(Family {
            id: family.id,
            name: family.name,
            avatar: family.avatar,
            city: family.city,
            guardian_id: family.guardian_id,
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_family::Family>> {
        let mut db = self.db.clone();
        let family = Family::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(family.map(Into::into))
    }

    async fn find_by_user(&self, user_id: Uuid) -> AppResult<Vec<domain_family::Family>> {
        let mut db = self.db.clone();
        let members = FamilyMember::filter_by_user_id(user_id)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let mut families = Vec::new();
        for member in members {
            if let Some(family) = Family::filter_by_id(member.family_id)
                .first()
                .exec(&mut db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?
            {
                families.push(family.into());
            }
        }
        Ok(families)
    }

    async fn update(&self, family: domain_family::Family) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = Family::update_by_id(family.id);
        update.set_name(family.name);
        update.set_avatar(family.avatar);
        update.set_city(family.city);
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl domain_family::FamilyMemberRepository for FamilyMemberRepository<'_> {
    async fn create(
        &self,
        member: domain_family::FamilyMember,
    ) -> AppResult<domain_family::FamilyMember> {
        let mut db = self.db.clone();
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
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_family(
        &self,
        family_id: Uuid,
    ) -> AppResult<Vec<domain_family::FamilyMember>> {
        let mut db = self.db.clone();
        let members = FamilyMember::filter_by_family_id(family_id)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(members.into_iter().map(Into::into).collect())
    }

    async fn find_by_user_and_family(
        &self,
        user_id: Uuid,
        family_id: Uuid,
    ) -> AppResult<Option<domain_family::FamilyMember>> {
        let mut db = self.db.clone();
        let member = FamilyMember::filter(
            FamilyMember::fields()
                .user_id()
                .eq(user_id)
                .and(FamilyMember::fields().family_id().eq(family_id)),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(member.map(Into::into))
    }

    async fn update_role(&self, id: Uuid, role: &str) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = FamilyMember::update_by_id(id);
        update.set_role(role.to_owned());
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn update_permissions(&self, id: Uuid, permissions: Permissions) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = FamilyMember::update_by_id(id);
        update.set_permissions(permissions.bits() as i32);
        update.exec(&mut db)
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
        let mut db = self.db.clone();

        // Demote old guardian
        let old_member = FamilyMember::filter(
            FamilyMember::fields()
                .user_id()
                .eq(from_user_id)
                .and(FamilyMember::fields().family_id().eq(family_id)),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(old) = old_member {
            let mut update = FamilyMember::update_by_id(old.id);
            update.set_is_guardian(false);
            update.exec(&mut db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // Promote new guardian
        let new_member = FamilyMember::filter(
            FamilyMember::fields()
                .user_id()
                .eq(to_user_id)
                .and(FamilyMember::fields().family_id().eq(family_id)),
        )
        .first()
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if let Some(new) = new_member {
            let mut update = FamilyMember::update_by_id(new.id);
            update.set_is_guardian(true);
            update.exec(&mut db)
                .await
                .map_err(|e| AppError::Database(e.to_string()))?;
        }

        // Update family guardian_id
        let mut family_update = Family::update_by_id(family_id);
        family_update.set_guardian_id(to_user_id);
        family_update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        FamilyMember::delete_by_id(&mut db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl domain_family::InviteCodeRepository for InviteCodeRepository<'_> {
    async fn create(
        &self,
        invite_code: domain_family::InviteCode,
    ) -> AppResult<domain_family::InviteCode> {
        let mut db = self.db.clone();
        let expires = invite_code.expires_at.to_rfc3339();
        let created = toasty::create!(InviteCode {
            id: invite_code.id,
            family_id: invite_code.family_id,
            code: invite_code.code,
            expires_at: expires,
            used_by: invite_code.used_by,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_code(&self, code: &str) -> AppResult<Option<domain_family::InviteCode>> {
        let mut db = self.db.clone();
        let invite = InviteCode::filter_by_code(code)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(invite.map(Into::into))
    }

    async fn mark_used(&self, id: Uuid, used_by: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = InviteCode::update_by_id(id);
        update.set_used_by(Some(used_by));
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl domain_family::JoinRequestRepository for JoinRequestRepository<'_> {
    async fn create(
        &self,
        request: domain_family::JoinRequest,
    ) -> AppResult<domain_family::JoinRequest> {
        let mut db = self.db.clone();
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
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_family(
        &self,
        family_id: Uuid,
        status: Option<JoinStatus>,
    ) -> AppResult<Vec<domain_family::JoinRequest>> {
        let mut db = self.db.clone();
        let results = match status {
            Some(s) => JoinRequest::filter_by_family_id(family_id)
                .filter(JoinRequest::fields().status().eq(s.as_str()))
                .exec(&mut db)
                .await,
            None => JoinRequest::filter_by_family_id(family_id)
                .exec(&mut db)
                .await,
        }
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(results.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_family::JoinRequest>> {
        let mut db = self.db.clone();
        let request = JoinRequest::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(request.map(Into::into))
    }

    async fn update_status(&self, id: Uuid, status: JoinStatus) -> AppResult<()> {
        let mut db = self.db.clone();
        let mut update = JoinRequest::update_by_id(id);
        update.set_status(status.as_str().to_owned());
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }
}
