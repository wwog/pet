use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

// 统一 camelCase 输出，对齐 api_doc 中 petId / breedId / birthYear 等字段名约定。
// 请求体同样使用 camelCase 反序列化，便于前端直接传 JSON。

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePetRequest {
    pub name: String,
    pub emoji: Option<String>,
    pub gender: String,
    pub birth_year: i32,
    pub birth_month: Option<i32>,
    pub birth_approximate: Option<bool>,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: String,
    #[serde(default)]
    pub personality_tag_ids: Vec<String>,
    #[serde(default)]
    pub custom_tags: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreatePetResponse {
    pub pet_id: Uuid,
    pub name: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePetRequest {
    pub name: Option<String>,
    pub gender: Option<String>,
    pub birth_year: Option<i32>,
    pub birth_month: Option<i32>,
    pub birth_approximate: Option<bool>,
    pub neuter_status: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePetAppearanceRequest {
    pub breed_id: Option<String>,
    pub coat_color: Option<String>,
    pub coat_pattern: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePetPersonalityRequest {
    pub personality_tag_ids: Vec<String>,
    #[serde(default)]
    pub custom_tags: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityTagDto {
    pub tag_id: String,
    pub name: String,
    pub category: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePetPersonalityResponse {
    pub pet_id: Uuid,
    pub personality_tags: Vec<PersonalityTagDto>,
    pub custom_tags: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PetListItem {
    pub pet_id: Uuid,
    pub name: String,
    pub breed: Option<String>,
    pub breed_id: String,
    pub gender: String,
    pub birth_date: Option<String>,
    pub birth_approximate: bool,
    pub neuter_status: String,
    pub avatar: Option<String>,
    pub age_text: String,
    pub companion_days: i64,
    pub weight_kg: Option<f64>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PetListResponse {
    pub list: Vec<PetListItem>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PetDetail {
    pub pet_id: Uuid,
    pub family_id: Uuid,
    pub name: String,
    pub emoji: Option<String>,
    pub gender: String,
    pub birth_date: Option<String>,
    pub birth_approximate: bool,
    pub birth_year: i32,
    pub birth_month: Option<i32>,
    pub breed_id: String,
    pub breed_name: Option<String>,
    pub breed_name_cn: Option<String>,
    pub breed_size: Option<String>,
    pub breed_coat_type: Option<String>,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: String,
    pub avatar: Option<String>,
    pub age_text: String,
    pub companion_days: i64,
    pub created_at: String,
    pub personality_tags: Vec<PersonalityTagDto>,
    pub custom_tags: Vec<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct DeletePetResponse {
    pub pet_id: Uuid,
    pub deleted_at: String,
    pub archive_retention_days: u32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PetStatsResponse {
    pub pet_id: Uuid,
    pub companion_days: i64,
    pub total_walks: u32,
    pub total_walk_distance_km: f64,
    pub total_photos: u32,
    pub total_health_records: u32,
    pub total_diary_entries: u32,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BreedDto {
    pub breed_id: String,
    pub name: String,
    pub name_cn: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BreedListResponse {
    pub list: Vec<BreedDto>,
    pub total: u64,
    pub page: u32,
    pub page_size: u32,
    pub has_more: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateBreedRequest {
    /// 物种，dog/cat/rabbit/...
    pub species: String,
    /// 英文名（AKC/CFA 官方名），用于生成 id
    pub name: String,
    /// 中文名
    pub name_cn: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
    pub origin: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BreedResponse {
    pub breed_id: String,
    pub species: String,
    pub name: String,
    pub name_cn: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
    pub origin: Option<String>,
}

/// 导出 seed JSON 的单条记录，字段顺序与 data/breeds/*.json 保持一致。
#[derive(Debug, Serialize, ToSchema)]
pub struct BreedSeedItem {
    pub id: String,
    pub species: String,
    pub name: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
    pub origin: Option<String>,
    pub name_cn: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityTagCategory {
    pub category_id: String,
    pub category_name: String,
    pub tags: Vec<PersonalityTagSimple>,
    pub allow_custom: Option<bool>,
    pub custom_placeholder: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityTagSimple {
    pub tag_id: String,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonalityTagsResponse {
    pub categories: Vec<PersonalityTagCategory>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct BreedQueryParams {
    pub species: Option<String>,
    pub keyword: Option<String>,
    pub size: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}
