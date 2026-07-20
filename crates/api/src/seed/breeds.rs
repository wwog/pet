//! 品种 seed — 将预置犬种/猫种数据幂等写入 `breeds` 表。
//!
//! 数据来源：AKC (akc.org) 全量犬种 + CFA (cfainc.org) 全量猫种，
//! 由 `scripts/collect-dog-breeds.mjs` / `scripts/collect-cat-breeds.mjs` 抓取生成，
//! JSON 副本位于 `crates/api/data/breeds/{dogs,cats}.json`，通过 `include_str!` 在
//! 编译期嵌入 binary，保证 seed 独立可分发、无运行时文件依赖。
//!
//! 只写入核心字段 (id / species / name / name_cn / size_category / coat_type / origin)。
//! 中文名 `name_cn` 由 `scripts/translate-breeds.mjs` 填充。

use domain::app::AppError;
use domain::pet::{Breed, BreedRepository, Species};

const DOGS_JSON: &str = include_str!("../../../../data/breeds/dogs.json");
const CATS_JSON: &str = include_str!("../../../../data/breeds/cats.json");

#[derive(serde::Deserialize)]
struct BreedSeed {
    id: String,
    species: String,
    name: String,
    name_cn: String,
    size_category: Option<String>,
    coat_type: Option<String>,
    origin: Option<String>,
}

impl BreedSeed {
    fn into_domain(self) -> Result<Breed, AppError> {
        let species = Species::from_str(&self.species).ok_or_else(|| {
            AppError::Internal(format!("unknown species '{}' in seed", self.species))
        })?;
        Ok(Breed {
            id: self.id,
            species,
            name: self.name,
            name_cn: self.name_cn,
            size_category: self.size_category,
            coat_type: self.coat_type,
            origin: self.origin,
        })
    }
}

fn load_seed(json: &'static str, label: &str) -> Result<Vec<Breed>, AppError> {
    let raw: Vec<BreedSeed> =
        serde_json::from_str(json).map_err(|e| {
            AppError::Internal(format!("failed to parse {label} seed JSON: {e}"))
        })?;

    raw.into_iter()
        .map(|b| b.into_domain())
        .collect::<Result<Vec<_>, _>>()
}

/// 幂等地写入全部品种 seed（犬 + 猫）。
///
/// 返回 `(inserted, skipped)`：已存在的 id 跳过，避免覆盖运营期补充的字段。
pub async fn seed_breeds<R: BreedRepository>(repo: &R) -> Result<(usize, usize), AppError> {
    let mut dogs = load_seed(DOGS_JSON, "dogs")?;
    let mut cats = load_seed(CATS_JSON, "cats")?;
    let total = dogs.len() + cats.len();

    dogs.append(&mut cats);
    let (inserted, skipped) = repo.upsert_many(dogs).await?;

    tracing::info!(
        "breeds seeded: inserted={inserted}, skipped={skipped}, total_in_source={total}"
    );
    Ok((inserted, skipped))
}
