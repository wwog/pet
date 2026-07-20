use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app::AppResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// 宠物物种。用于区分犬、猫、兔、鸟等多物种档案。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Species {
    Dog,
    Cat,
    Rabbit,
    Bird,
    Rodent,
    Reptile,
    Fish,
    Other,
}

impl Species {
    pub fn as_str(&self) -> &'static str {
        match self {
            Species::Dog => "dog",
            Species::Cat => "cat",
            Species::Rabbit => "rabbit",
            Species::Bird => "bird",
            Species::Rodent => "rodent",
            Species::Reptile => "reptile",
            Species::Fish => "fish",
            Species::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "dog" => Some(Species::Dog),
            "cat" => Some(Species::Cat),
            "rabbit" => Some(Species::Rabbit),
            "bird" => Some(Species::Bird),
            "rodent" => Some(Species::Rodent),
            "reptile" => Some(Species::Reptile),
            "fish" => Some(Species::Fish),
            "other" => Some(Species::Other),
            _ => None,
        }
    }
}

/// 宠物入家来源类型,用于记录领养/购买/救助等途径。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AcquisitionSource {
    Adoption,
    Purchase,
    Rescue,
    Gift,
    Found,
    Bred,
    Other,
}

impl AcquisitionSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            AcquisitionSource::Adoption => "adoption",
            AcquisitionSource::Purchase => "purchase",
            AcquisitionSource::Rescue => "rescue",
            AcquisitionSource::Gift => "gift",
            AcquisitionSource::Found => "found",
            AcquisitionSource::Bred => "bred",
            AcquisitionSource::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "adoption" => Some(AcquisitionSource::Adoption),
            "purchase" => Some(AcquisitionSource::Purchase),
            "rescue" => Some(AcquisitionSource::Rescue),
            "gift" => Some(AcquisitionSource::Gift),
            "found" => Some(AcquisitionSource::Found),
            "bred" => Some(AcquisitionSource::Bred),
            "other" => Some(AcquisitionSource::Other),
            _ => None,
        }
    }
}

/// 宠物档案(核心聚合根)。
///
/// `extra_attributes` 以 JSON 承载各物种特有字段,约定如下(不强校验):
/// - 狗: `{ "chip_number": "...", "work_dog_license": "...", "bloodline_cert": "..." }`
/// - 猫: `{ "indoor_only": true, "declawed": false, "ear_tipped": false }`
/// - 兔: `{ "housing": "free_roam" | "cage" | "pen" }`
/// - 鸟: `{ "leg_ring_id": "...", "wing_clipped": true }`
/// - 其他物种:自由扩展。
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
    pub species: Species,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: NeuterStatus,
    pub avatar: Option<String>,
    pub acquisition_source: Option<AcquisitionSource>,
    pub source_org: Option<String>,
    pub adopted_at: Option<DateTime<Utc>>,
    pub extra_attributes: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

/// 宠物品种库。多物种共用一张表,通过 `species` 区分。
///
/// 数据来自 AKC (狗) / CFA (猫) 官方品种列表,仅包含核心字段。
/// 前端展示用 `name` (英文) 或 `name_cn` (中文)。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breed {
    pub id: String,
    pub species: Species,
    pub name: String,
    pub name_cn: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
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
        species: Species,
        keyword: &str,
        size: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<Breed>, u64)>;

    /// 幂等地批量写入品种 seed 数据。
    ///
    /// 已存在的 id 跳过；不存在的则插入。返回 (inserted, skipped) 计数。
    async fn upsert_many(&self, breeds: Vec<Breed>) -> AppResult<(usize, usize)>;
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
