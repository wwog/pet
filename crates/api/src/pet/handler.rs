use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::{Datelike, Utc};
use domain::app::AppError;
use domain::family::{FamilyMemberRepository, FamilyRepository};
use domain::pet::{
    BreedRepository, Gender, NeuterStatus, PersonalityTagRepository, PetRepository, Species,
};
use uuid::Uuid;

use super::dto::*;
use crate::app_state::SharedState;
use crate::auth::middleware::AuthenticatedUser;
use crate::error::{ApiError, ApiResponse, ErrorResponse};

/// 单家庭最多宠物数（业务规则 3003）。
const MAX_PETS_PER_FAMILY: i64 = 5;

/// 删除后的归档保留期（天）。
const DELETE_ARCHIVE_RETENTION_DAYS: u32 = 30;

/// admin 场景下，用户至少属于一个家庭。取第一个家庭作为操作上下文。
async fn resolve_family_id(state: &SharedState, user_id: Uuid) -> Result<Uuid, AppError> {
    let family_repo = state.db.family_repository();
    let families = family_repo.find_by_user(user_id).await?;
    families
        .first()
        .map(|family| family.id)
        .ok_or_else(|| AppError::NotFound("user has no family".into()))
}

/// 校验用户为指定宠物的家庭监护人（P12）。删除宠物仅监护人可操作。
async fn ensure_guardian(
    state: &SharedState,
    user_id: Uuid,
    family_id: Uuid,
) -> Result<(), AppError> {
    let member_repo = state.db.family_member_repository();
    let member = member_repo
        .find_by_user_and_family(user_id, family_id)
        .await?
        .ok_or_else(|| AppError::Unauthorized("not a family member".into()))?;
    if !member.is_guardian {
        return Err(AppError::Unauthorized(
            "only the guardian can perform this action".into(),
        ));
    }
    Ok(())
}

fn parse_gender(s: &str) -> Result<Gender, AppError> {
    Gender::from_str(s)
        .ok_or_else(|| AppError::Validation(format!("invalid gender: {s}")))
}

fn parse_neuter_status(s: &str) -> Result<NeuterStatus, AppError> {
    NeuterStatus::from_str(s)
        .ok_or_else(|| AppError::Validation(format!("invalid neuterStatus: {s}")))
}

fn validate_name(name: &str) -> Result<(), AppError> {
    let trimmed = name.trim();
    if trimmed.is_empty() || trimmed.chars().count() > 10 {
        return Err(AppError::Validation(
            "name must be 1-10 characters".into(),
        ));
    }
    Ok(())
}

fn validate_coat_color(color: &str) -> Result<(), AppError> {
    const ALLOWED: &[&str] = &[
        "cream",
        "tan",
        "brown",
        "black",
        "white",
        "gray",
        "gold",
        "red",
        "choco",
        "merle",
        "fawn",
        "pearl",
    ];
    if !ALLOWED.contains(&color) {
        return Err(AppError::Validation(format!(
            "coatColor must be one of: {}",
            ALLOWED.join(", ")
        )));
    }
    Ok(())
}

fn birth_date_string(year: i32, month: Option<i32>) -> Option<String> {
    month.map(|month| format!("{year:04}-{month:02}-01"))
}

/// 生成 "1岁2个月" 形式的年龄文案；月份缺失时仅显示岁数。
fn age_text(year: i32, month: Option<i32>) -> String {
    let now = Utc::now();
    let mut years = now.year() - year;
    let mut months = match month {
        Some(month) => {
            let mut value = now.month() as i32 - month;
            if value < 0 {
                years -= 1;
                value += 12;
            }
            value
        }
        None => 0,
    };
    if years < 0 {
        years = 0;
        months = 0;
    }
    if years > 0 && months > 0 {
        format!("{years}岁{months}个月")
    } else if years > 0 {
        format!("{years}岁")
    } else {
        format!("{months}个月")
    }
}

fn companion_days(adopted_at: Option<chrono::DateTime<Utc>>) -> i64 {
    let reference = adopted_at.unwrap_or_else(Utc::now);
    (Utc::now() - reference).num_days().max(0)
}

fn category_display_name(category: &str) -> &'static str {
    match category {
        "social" => "社交性格",
        "behavior" => "行为习惯",
        "emotion" => "情绪偏好",
        "custom" => "更多·自定义",
        _ => "其他",
    }
}

async fn build_pet_detail(
    state: &SharedState,
    pet: domain::pet::Pet,
) -> Result<PetDetail, AppError> {
    let breed_repo = state.db.breed_repository();
    let breed = breed_repo.find_by_id(&pet.breed_id).await?;

    let personality_repo = state.db.personality_tag_repository();
    let pet_tags = personality_repo.find_by_pet(pet.id).await?;

    let all_tags = personality_repo.find_all_categories().await?;
    let tag_lookup: std::collections::HashMap<String, domain::pet::PersonalityTag> = all_tags
        .into_iter()
        .map(|tag| (tag.id.clone(), tag))
        .collect();

    let mut personality_tags: Vec<PersonalityTagDto> = Vec::new();
    let mut custom_tags: Vec<String> = Vec::new();
    for pet_tag in pet_tags {
        if let Some(tag) = tag_lookup.get(&pet_tag.tag_id) {
            if pet_tag.tag_id == "custom" {
                if let Some(custom_name) = pet_tag.custom_name {
                    custom_tags.push(custom_name);
                }
            } else {
                personality_tags.push(PersonalityTagDto {
                    tag_id: tag.id.clone(),
                    name: tag.name.clone(),
                    category: tag.category.clone(),
                });
            }
        } else if let Some(custom_name) = pet_tag.custom_name {
            custom_tags.push(custom_name);
        }
    }

    Ok(PetDetail {
        pet_id: pet.id,
        family_id: pet.family_id,
        name: pet.name,
        emoji: pet.emoji,
        gender: pet.gender.as_str().to_owned(),
        birth_date: birth_date_string(pet.birth_year, pet.birth_month),
        birth_approximate: pet.birth_approximate,
        birth_year: pet.birth_year,
        birth_month: pet.birth_month,
        breed_id: pet.breed_id,
        breed_name: breed.as_ref().map(|b| b.name.clone()),
        breed_size: breed.as_ref().and_then(|b| b.size_category.clone()),
        breed_coat_type: breed.as_ref().and_then(|b| b.coat_type.clone()),
        standard_weight_min: breed.as_ref().and_then(|b| b.standard_weight_min),
        standard_weight_max: breed.as_ref().and_then(|b| b.standard_weight_max),
        life_span_min: breed.as_ref().and_then(|b| b.life_span_min),
        life_span_max: breed.as_ref().and_then(|b| b.life_span_max),
        exercise_needs: breed.as_ref().and_then(|b| b.exercise_needs.clone()),
        coat_color: pet.coat_color,
        coat_pattern: pet.coat_pattern,
        neuter_status: pet.neuter_status.as_str().to_owned(),
        avatar: pet.avatar,
        age_text: age_text(pet.birth_year, pet.birth_month),
        companion_days: companion_days(pet.adopted_at),
        created_at: pet.created_at.to_rfc3339(),
        personality_tags,
        custom_tags,
    })
}

async fn load_pet_owned(
    state: &SharedState,
    pet_id: Uuid,
    user_id: Uuid,
) -> Result<domain::pet::Pet, AppError> {
    let pet_repo = state.db.pet_repository();
    let pet = pet_repo
        .find_by_id(pet_id)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("pet {pet_id} not found")))?;

    // 校验宠物归属的 family 是当前用户所属的家庭，避免越权访问他人宠物。
    let family_repo = state.db.family_repository();
    let user_families = family_repo.find_by_user(user_id).await?;
    if !user_families.iter().any(|family| family.id == pet.family_id) {
        return Err(AppError::NotFound(format!("pet {pet_id} not found")));
    }
    Ok(pet)
}

#[utoipa::path(
    get,
    path = "/pets",
    tag = "pet",
    operation_id = "list_pets",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "当前家庭所有宠物", body = ApiResponse<PetListResponse>),
        (status = 401, description = "未认证", body = ErrorResponse),
        (status = 404, description = "用户未加入家庭", body = ErrorResponse),
    )
)]
pub async fn list_pets(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
) -> Result<Json<ApiResponse<PetListResponse>>, ApiError> {
    let family_id = resolve_family_id(&state, user_id).await?;

    let pet_repo = state.db.pet_repository();
    let pets = pet_repo.find_by_family(family_id).await?;

    let breed_repo = state.db.breed_repository();
    let mut list: Vec<PetListItem> = Vec::with_capacity(pets.len());
    for pet in pets {
        let breed = breed_repo.find_by_id(&pet.breed_id).await?;
        list.push(PetListItem {
            pet_id: pet.id,
            name: pet.name,
            breed: breed.as_ref().map(|b| b.name.clone()),
            breed_id: pet.breed_id,
            gender: pet.gender.as_str().to_owned(),
            birth_date: birth_date_string(pet.birth_year, pet.birth_month),
            birth_approximate: pet.birth_approximate,
            neuter_status: pet.neuter_status.as_str().to_owned(),
            avatar: pet.avatar,
            age_text: age_text(pet.birth_year, pet.birth_month),
            companion_days: companion_days(pet.adopted_at),
            weight_kg: None,
        });
    }

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: PetListResponse { list },
    }))
}

#[utoipa::path(
    get,
    path = "/pets/{petId}",
    tag = "pet",
    operation_id = "get_pet",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    responses(
        (status = 200, description = "宠物详情", body = ApiResponse<PetDetail>),
        (status = 401, description = "未认证", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn get_pet(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
) -> Result<Json<ApiResponse<PetDetail>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let pet = load_pet_owned(&state, pet_id, user_id).await?;
    let detail = build_pet_detail(&state, pet).await?;
    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: detail,
    }))
}

#[utoipa::path(
    post,
    path = "/pets",
    tag = "pet",
    operation_id = "create_pet",
    security(("bearer_auth" = [])),
    request_body = CreatePetRequest,
    responses(
        (status = 201, description = "创建成功", body = ApiResponse<CreatePetResponse>),
        (status = 400, description = "参数校验失败", body = ErrorResponse),
        (status = 409, description = "宠物数量已达上限（5只/家庭）", body = ErrorResponse),
    )
)]
pub async fn create_pet(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Json(body): Json<CreatePetRequest>,
) -> Result<(StatusCode, Json<ApiResponse<CreatePetResponse>>), ApiError> {
    validate_name(&body.name)?;
    validate_coat_color(&body.coat_color)?;
    let gender = parse_gender(&body.gender)?;
    let neuter_status = parse_neuter_status(&body.neuter_status)?;

    let family_id = resolve_family_id(&state, user_id).await?;

    let pet_repo = state.db.pet_repository();
    let count = pet_repo.count_by_family(family_id).await?;
    if count >= MAX_PETS_PER_FAMILY {
        return Err(AppError::Conflict(format!(
            "宠物数量已达上限（{MAX_PETS_PER_FAMILY}只/家庭）"
        ))
        .into());
    }

    let breed_repo = state.db.breed_repository();
    if breed_repo.find_by_id(&body.breed_id).await?.is_none() {
        return Err(AppError::Validation(format!(
            "breedId '{}' does not exist",
            body.breed_id
        ))
        .into());
    }

    let now = Utc::now();
    let pet_id = Uuid::new_v4();
    let pet = domain::pet::Pet {
        id: pet_id,
        family_id,
        name: body.name.trim().to_owned(),
        emoji: body.emoji,
        gender,
        birth_year: body.birth_year,
        birth_month: body.birth_month,
        birth_approximate: body.birth_approximate.unwrap_or(false),
        species: Species::Dog,
        breed_id: body.breed_id,
        coat_color: body.coat_color,
        coat_pattern: body.coat_pattern,
        neuter_status,
        avatar: None,
        acquisition_source: None,
        source_org: None,
        adopted_at: Some(now),
        extra_attributes: None,
        created_at: now,
    };
    let saved = pet_repo.create(pet).await?;

    let personality_repo = state.db.personality_tag_repository();
    personality_repo
        .set_pet_tags(saved.id, body.personality_tag_ids, body.custom_tags)
        .await?;

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            code: 0,
            message: "success".into(),
            data: CreatePetResponse {
                pet_id: saved.id,
                name: saved.name,
                created_at: saved.created_at.to_rfc3339(),
            },
        }),
    ))
}

#[utoipa::path(
    put,
    path = "/pets/{petId}",
    tag = "pet",
    operation_id = "update_pet",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    request_body = UpdatePetRequest,
    responses(
        (status = 200, description = "更新后的宠物详情", body = ApiResponse<PetDetail>),
        (status = 400, description = "参数校验失败", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn update_pet(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
    Json(body): Json<UpdatePetRequest>,
) -> Result<Json<ApiResponse<PetDetail>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let mut pet = load_pet_owned(&state, pet_id, user_id).await?;

    if let Some(name) = body.name {
        validate_name(&name)?;
        pet.name = name.trim().to_owned();
    }
    if let Some(gender_str) = body.gender {
        pet.gender = parse_gender(&gender_str)?;
    }
    if let Some(birth_year) = body.birth_year {
        pet.birth_year = birth_year;
    }
    if let Some(birth_month) = body.birth_month {
        pet.birth_month = Some(birth_month);
    }
    if let Some(birth_approximate) = body.birth_approximate {
        pet.birth_approximate = birth_approximate;
    }
    if let Some(neuter_status_str) = body.neuter_status {
        pet.neuter_status = parse_neuter_status(&neuter_status_str)?;
    }

    let pet_repo = state.db.pet_repository();
    pet_repo.update(pet.clone()).await?;
    let detail = build_pet_detail(&state, pet).await?;
    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: detail,
    }))
}

#[utoipa::path(
    put,
    path = "/pets/{petId}/appearance",
    tag = "pet",
    operation_id = "update_pet_appearance",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    request_body = UpdatePetAppearanceRequest,
    responses(
        (status = 200, description = "更新后的宠物详情", body = ApiResponse<PetDetail>),
        (status = 400, description = "参数校验失败", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn update_pet_appearance(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
    Json(body): Json<UpdatePetAppearanceRequest>,
) -> Result<Json<ApiResponse<PetDetail>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let mut pet = load_pet_owned(&state, pet_id, user_id).await?;

    if let Some(breed_id) = body.breed_id {
        let breed_repo = state.db.breed_repository();
        if breed_repo.find_by_id(&breed_id).await?.is_none() {
            return Err(AppError::Validation(format!(
                "breedId '{breed_id}' does not exist"
            ))
            .into());
        }
        pet.breed_id = breed_id;
    }
    if let Some(coat_color) = body.coat_color {
        validate_coat_color(&coat_color)?;
        pet.coat_color = coat_color;
    }
    if let Some(coat_pattern) = body.coat_pattern {
        pet.coat_pattern = Some(coat_pattern);
    }

    let pet_repo = state.db.pet_repository();
    pet_repo.update(pet.clone()).await?;
    let detail = build_pet_detail(&state, pet).await?;
    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: detail,
    }))
}

#[utoipa::path(
    put,
    path = "/pets/{petId}/personality",
    tag = "pet",
    operation_id = "update_pet_personality",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    request_body = UpdatePetPersonalityRequest,
    responses(
        (status = 200, description = "更新后的性格标签", body = ApiResponse<UpdatePetPersonalityResponse>),
        (status = 400, description = "参数校验失败", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn update_pet_personality(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
    Json(body): Json<UpdatePetPersonalityRequest>,
) -> Result<Json<ApiResponse<UpdatePetPersonalityResponse>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let pet = load_pet_owned(&state, pet_id, user_id).await?;

    let personality_repo = state.db.personality_tag_repository();
    personality_repo
        .set_pet_tags(pet.id, body.personality_tag_ids.clone(), body.custom_tags.clone())
        .await?;

    let all_tags = personality_repo.find_all_categories().await?;
    let tag_lookup: std::collections::HashMap<String, domain::pet::PersonalityTag> = all_tags
        .into_iter()
        .map(|tag| (tag.id.clone(), tag))
        .collect();

    let mut personality_tags: Vec<PersonalityTagDto> = Vec::new();
    for tag_id in &body.personality_tag_ids {
        if let Some(tag) = tag_lookup.get(tag_id) {
            personality_tags.push(PersonalityTagDto {
                tag_id: tag.id.clone(),
                name: tag.name.clone(),
                category: tag.category.clone(),
            });
        }
    }

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: UpdatePetPersonalityResponse {
            pet_id: pet.id,
            personality_tags,
            custom_tags: body.custom_tags,
        },
    }))
}

#[utoipa::path(
    delete,
    path = "/pets/{petId}",
    tag = "pet",
    operation_id = "delete_pet",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    responses(
        (status = 200, description = "删除成功", body = ApiResponse<DeletePetResponse>),
        (status = 401, description = "仅监护人可删除", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn delete_pet(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
) -> Result<Json<ApiResponse<DeletePetResponse>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let pet = load_pet_owned(&state, pet_id, user_id).await?;
    ensure_guardian(&state, user_id, pet.family_id).await?;

    let pet_repo = state.db.pet_repository();
    pet_repo.delete(pet.id).await?;

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: DeletePetResponse {
            pet_id: pet.id,
            deleted_at: Utc::now().to_rfc3339(),
            archive_retention_days: DELETE_ARCHIVE_RETENTION_DAYS,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/pets/{petId}/stats",
    tag = "pet",
    operation_id = "get_pet_stats",
    security(("bearer_auth" = [])),
    params(("petId" = String, Path, description = "宠物 ID")),
    responses(
        (status = 200, description = "陪伴统计", body = ApiResponse<PetStatsResponse>),
        (status = 401, description = "未认证", body = ErrorResponse),
        (status = 404, description = "宠物不存在", body = ErrorResponse),
    )
)]
pub async fn get_pet_stats(
    State(state): State<SharedState>,
    AuthenticatedUser { user_id, .. }: AuthenticatedUser,
    Path(pet_id): Path<String>,
) -> Result<Json<ApiResponse<PetStatsResponse>>, ApiError> {
    let pet_id = Uuid::parse_str(&pet_id)
        .map_err(|_| AppError::Validation("invalid petId".into()))?;
    let pet = load_pet_owned(&state, pet_id, user_id).await?;

    // companionDays 来自 adopted_at 的真实计算；其余统计字段对应的表（walks/photos/health/diary）
    // 尚未实现，先返回 0 作为占位，待对应模块就绪后补全。
    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: PetStatsResponse {
            pet_id: pet.id,
            companion_days: companion_days(pet.adopted_at),
            total_walks: 0,
            total_walk_distance_km: 0.0,
            total_photos: 0,
            total_health_records: 0,
            total_diary_entries: 0,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/pets/breeds",
    tag = "pet",
    operation_id = "list_pet_breeds",
    security(("bearer_auth" = [])),
    params(("keyword" = Option<String>, Query, description = "名称/拼音首字母搜索"), ("size" = Option<String>, Query, description = "体型 small/medium/large"), ("page" = Option<u32>, Query, description = "页码，默认 1"), ("pageSize" = Option<u32>, Query, description = "每页条数，默认 20")),
    responses(
        (status = 200, description = "品种列表", body = ApiResponse<BreedListResponse>),
        (status = 401, description = "未认证", body = ErrorResponse),
    )
)]
pub async fn list_pet_breeds(
    State(state): State<SharedState>,
    AuthenticatedUser { .. }: AuthenticatedUser,
    Query(params): Query<BreedQueryParams>,
) -> Result<Json<ApiResponse<BreedListResponse>>, ApiError> {
    let keyword = params.keyword.unwrap_or_default();
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(20).clamp(1, 100);

    let breed_repo = state.db.breed_repository();
    let (breeds, total) = breed_repo
        .search(
            Species::Dog,
            &keyword,
            params.size.as_deref(),
            page,
            page_size,
        )
        .await?;

    let list = breeds
        .into_iter()
        .map(|breed| BreedDto {
            breed_id: breed.id,
            name: breed.name,
            pinyin: breed.pinyin,
            initial: breed.initial,
            size_category: breed.size_category,
            coat_type: breed.coat_type,
            standard_weight_min: breed.standard_weight_min,
            standard_weight_max: breed.standard_weight_max,
            life_span_min: breed.life_span_min,
            life_span_max: breed.life_span_max,
            exercise_needs: breed.exercise_needs,
            icon: breed.icon,
            origin: breed.origin,
        })
        .collect::<Vec<_>>();

    let has_more = (page as u64) * (page_size as u64) < total;

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: BreedListResponse {
            list,
            total,
            page,
            page_size,
            has_more,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/pets/personality-tags",
    tag = "pet",
    operation_id = "list_personality_tags",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "性格标签库", body = ApiResponse<PersonalityTagsResponse>),
        (status = 401, description = "未认证", body = ErrorResponse),
    )
)]
pub async fn list_personality_tags(
    State(state): State<SharedState>,
    AuthenticatedUser { .. }: AuthenticatedUser,
) -> Result<Json<ApiResponse<PersonalityTagsResponse>>, ApiError> {
    let personality_repo = state.db.personality_tag_repository();
    let all_tags = personality_repo.find_all_categories().await?;

    // 数据库中可能未预置 custom 类别的占位行，这里始终补一个 custom 分类入口，
    // 以保证前端可渲染"自定义"区域及其提示文案。
    let mut by_category: std::collections::BTreeMap<String, Vec<domain::pet::PersonalityTag>> =
        std::collections::BTreeMap::new();
    for tag in all_tags {
        by_category
            .entry(tag.category.clone())
            .or_default()
            .push(tag);
    }
    by_category
        .entry("custom".into())
        .or_default();

    let category_order = ["social", "behavior", "emotion", "custom"];
    let mut categories: Vec<PersonalityTagCategory> = category_order
        .iter()
        .filter_map(|category_id| {
            let category_id = *category_id;
            by_category.get(category_id).map(|tags| {
                let allow_custom = category_id == "custom";
                let custom_placeholder = if allow_custom {
                    Some("自定义角色，如 御用铲屎官".to_owned())
                } else {
                    None
                };
                let tag_dtos = tags
                    .iter()
                    .map(|tag| PersonalityTagSimple {
                        tag_id: tag.id.clone(),
                        name: tag.name.clone(),
                    })
                    .collect();
                PersonalityTagCategory {
                    category_id: category_id.to_owned(),
                    category_name: category_display_name(category_id).to_owned(),
                    tags: tag_dtos,
                    allow_custom: Some(allow_custom),
                    custom_placeholder,
                }
            })
        })
        .collect();

    // 兜底：若 DB 中的 category 不在上述顺序内，附加一个分类避免数据丢失。
    for (category_id, tags) in by_category {
        if category_order.contains(&category_id.as_str()) {
            continue;
        }
        let tag_dtos = tags
            .into_iter()
            .map(|tag| PersonalityTagSimple {
                tag_id: tag.id,
                name: tag.name,
            })
            .collect();
        categories.push(PersonalityTagCategory {
            category_id: category_id.clone(),
            category_name: category_display_name(&category_id).to_owned(),
            tags: tag_dtos,
            allow_custom: None,
            custom_placeholder: None,
        });
    }

    Ok(Json(ApiResponse {
        code: 0,
        message: "success".into(),
        data: PersonalityTagsResponse { categories },
    }))
}
