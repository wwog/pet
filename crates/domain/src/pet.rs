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

/// 宠物档案（核心聚合根）。
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
