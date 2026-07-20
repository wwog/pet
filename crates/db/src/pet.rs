use async_trait::async_trait;
use domain::app::{AppError, AppResult};
use domain::pet::{self as domain_pet, AcquisitionSource, Gender, NeuterStatus, Species};
use uuid::Uuid;

/// toasty ORM 模型 — 对应 `pets` 表。
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
    pub species: String,
    pub breed_id: String,
    pub coat_color: String,
    pub coat_pattern: Option<String>,
    pub neuter_status: String,
    pub avatar: Option<String>,
    pub acquisition_source: Option<String>,
    pub source_org: Option<String>,
    pub adopted_at: Option<String>,
    pub extra_attributes: Option<String>,

    pub created_at: String,
}

#[derive(Debug, toasty::Model)]
#[table = "breeds"]
pub struct Breed {
    #[key]
    pub id: String,

    pub species: String,
    pub name: String,
    pub pinyin: String,
    pub initial: String,
    pub size_category: Option<String>,
    pub coat_type: Option<String>,
    pub standard_weight_min: Option<f64>,
    pub standard_weight_max: Option<f64>,
    pub life_span_min: Option<i32>,
    pub life_span_max: Option<i32>,
    pub exercise_needs: Option<String>,
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

fn serialize_extra(attr: &Option<serde_json::Value>) -> Option<String> {
    attr.as_ref().map(|v| v.to_string())
}

fn deserialize_extra(s: &Option<String>) -> Option<serde_json::Value> {
    s.as_ref().and_then(|raw| serde_json::from_str(raw).ok())
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
            species: Species::from_str(&p.species).unwrap_or(Species::Dog),
            breed_id: p.breed_id,
            coat_color: p.coat_color,
            coat_pattern: p.coat_pattern,
            neuter_status: NeuterStatus::from_str(&p.neuter_status)
                .unwrap_or(NeuterStatus::Intact),
            avatar: p.avatar,
            acquisition_source: p
                .acquisition_source
                .as_deref()
                .and_then(AcquisitionSource::from_str),
            source_org: p.source_org,
            adopted_at: p.adopted_at.as_deref().map(parse_datetime),
            extra_attributes: deserialize_extra(&p.extra_attributes),
            created_at: parse_datetime(&p.created_at),
        }
    }
}

impl From<Breed> for domain_pet::Breed {
    fn from(b: Breed) -> Self {
        domain_pet::Breed {
            id: b.id,
            species: Species::from_str(&b.species).unwrap_or(Species::Dog),
            name: b.name,
            pinyin: b.pinyin,
            initial: b.initial.chars().next().unwrap_or('?').to_string(),
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

pub struct BreedRepository<'a> {
    db: &'a toasty::Db,
}

impl<'a> BreedRepository<'a> {
    pub fn new(db: &'a toasty::Db) -> Self {
        Self { db }
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
impl domain_pet::PetRepository for PetRepository<'_> {
    async fn create(&self, pet: domain_pet::Pet) -> AppResult<domain_pet::Pet> {
        let mut db = self.db.clone();
        let now = pet.created_at.to_rfc3339();
        let adopted_at = pet.adopted_at.map(|dt| dt.to_rfc3339());
        let extra = serialize_extra(&pet.extra_attributes);
        let created = toasty::create!(Pet {
            id: pet.id,
            family_id: pet.family_id,
            name: pet.name,
            emoji: pet.emoji,
            gender: pet.gender.as_str().to_owned(),
            birth_year: pet.birth_year,
            birth_month: pet.birth_month,
            birth_approximate: pet.birth_approximate,
            species: pet.species.as_str().to_owned(),
            breed_id: pet.breed_id,
            coat_color: pet.coat_color,
            coat_pattern: pet.coat_pattern,
            neuter_status: pet.neuter_status.as_str().to_owned(),
            avatar: pet.avatar,
            acquisition_source: pet.acquisition_source.map(|s| s.as_str().to_owned()),
            source_org: pet.source_org,
            adopted_at,
            extra_attributes: extra,
            created_at: now,
        })
        .exec(&mut db)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(created.into())
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<domain_pet::Pet>> {
        let mut db = self.db.clone();
        let pet = Pet::filter_by_id(id)
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(pet.map(Into::into))
    }

    async fn find_by_family(&self, family_id: Uuid) -> AppResult<Vec<domain_pet::Pet>> {
        let mut db = self.db.clone();
        let pets = Pet::filter_by_family_id(family_id)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(pets.into_iter().map(Into::into).collect())
    }

    async fn update(&self, pet: domain_pet::Pet) -> AppResult<()> {
        let mut db = self.db.clone();
        let adopted_at = pet.adopted_at.map(|dt| dt.to_rfc3339());
        let extra = serialize_extra(&pet.extra_attributes);
        let mut update = Pet::update_by_id(pet.id);
        update.set_name(pet.name);
        update.set_emoji(pet.emoji);
        update.set_gender(pet.gender.as_str().to_owned());
        update.set_birth_year(pet.birth_year);
        update.set_birth_month(pet.birth_month);
        update.set_birth_approximate(pet.birth_approximate);
        update.set_species(pet.species.as_str().to_owned());
        update.set_breed_id(pet.breed_id);
        update.set_coat_color(pet.coat_color);
        update.set_coat_pattern(pet.coat_pattern);
        update.set_neuter_status(pet.neuter_status.as_str().to_owned());
        update.set_avatar(pet.avatar);
        update.set_acquisition_source(pet.acquisition_source.map(|s| s.as_str().to_owned()));
        update.set_source_org(pet.source_org);
        update.set_adopted_at(adopted_at);
        update.set_extra_attributes(extra);
        update.exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> AppResult<()> {
        let mut db = self.db.clone();
        Pet::delete_by_id(&mut db, id)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(())
    }

    async fn count_by_family(&self, family_id: Uuid) -> AppResult<i64> {
        let mut db = self.db.clone();
        let count = Pet::filter_by_family_id(family_id)
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(count as i64)
    }
}

#[async_trait]
impl domain_pet::BreedRepository for BreedRepository<'_> {
    async fn find_by_id(&self, id: &str) -> AppResult<Option<domain_pet::Breed>> {
        let mut db = self.db.clone();
        let breed = Breed::filter(Breed::fields().id().eq(id))
            .first()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(breed.map(Into::into))
    }

    async fn search(
        &self,
        species: Species,
        keyword: &str,
        size: Option<&str>,
        page: u32,
        page_size: u32,
    ) -> AppResult<(Vec<domain_pet::Breed>, u64)> {
        let mut db = self.db.clone();
        let species_str = species.as_str().to_owned();
        let mut query = Breed::filter(Breed::fields().species().eq(species_str));

        if !keyword.is_empty() {
            let lower = keyword.to_lowercase();
            let like_pat = format!("%{}%", lower);
            query = query.filter(
                Breed::fields()
                    .name()
                    .like(like_pat.clone())
                    .or(Breed::fields().pinyin().like(like_pat.clone()))
                    .or(Breed::fields().initial().eq(lower.clone())),
            );
        }

        // 体型筛选仅对犬种有意义
        if let Some(size_cat) = size {
            query = query.filter(Breed::fields().size_category().eq(size_cat.to_owned()));
        }

        let total = query
            .clone()
            .count()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        let offset = ((page.saturating_sub(1)) * page_size) as usize;
        let results = query
            .limit(page_size as usize)
            .offset(offset)
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        Ok((results.into_iter().map(Into::into).collect(), total))
    }

    async fn upsert_many(
        &self,
        breeds: Vec<domain_pet::Breed>,
    ) -> AppResult<(usize, usize)> {
        let mut db = self.db.clone();

        let existing_ids: std::collections::HashSet<String> = Breed::all()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?
            .into_iter()
            .map(|b| b.id)
            .collect();

        let to_insert: Vec<&domain_pet::Breed> =
            breeds.iter().filter(|b| !existing_ids.contains(&b.id)).collect();

        if to_insert.is_empty() {
            return Ok((0, breeds.len()));
        }

        for breed in &to_insert {
            toasty::create!(Breed {
                id: breed.id.clone(),
                species: breed.species.as_str().to_owned(),
                name: breed.name.clone(),
                pinyin: breed.pinyin.clone(),
                initial: breed.initial.clone(),
                size_category: breed.size_category.clone(),
                coat_type: breed.coat_type.clone(),
                standard_weight_min: breed.standard_weight_min,
                standard_weight_max: breed.standard_weight_max,
                life_span_min: breed.life_span_min,
                life_span_max: breed.life_span_max,
                exercise_needs: breed.exercise_needs.clone(),
                icon: breed.icon.clone(),
                origin: breed.origin.clone(),
            })
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok((to_insert.len(), breeds.len() - to_insert.len()))
    }
}

#[async_trait]
impl domain_pet::PersonalityTagRepository for PersonalityTagRepository<'_> {
    async fn find_all_categories(&self) -> AppResult<Vec<domain_pet::PersonalityTag>> {
        let mut db = self.db.clone();
        let tags = PersonalityTag::all()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        Ok(tags.into_iter().map(Into::into).collect())
    }

    async fn find_by_pet(&self, pet_id: Uuid) -> AppResult<Vec<domain_pet::PetPersonalityTag>> {
        let mut db = self.db.clone();
        let tags = PetPersonalityTag::filter_by_pet_id(pet_id)
            .exec(&mut db)
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
        let mut db = self.db.clone();

        // Delete existing tags
        PetPersonalityTag::filter_by_pet_id(pet_id)
            .delete()
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;

        // Insert preset tags
        for tag_id in &tag_ids {
            toasty::create!(PetPersonalityTag {
                pet_id,
                tag_id: tag_id.clone(),
                custom_name: None,
            })
            .exec(&mut db)
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
            .exec(&mut db)
            .await
            .map_err(|e| AppError::Database(e.to_string()))?;
        }

        Ok(())
    }
}
