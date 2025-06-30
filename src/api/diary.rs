use axum::{
    extract::{Extension, Json, Path, Query},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

use crate::{
    db::DbPool,
    models::diary::{DiaryEntry, CreateDiaryEntry, NutritionSummary},
    services::{auth::Claims, diary::DiaryService},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_entry))
        .route("/", get(get_entries))
        .route("/{id}", get(get_entry))
        .route("/{id}", put(update_entry))
        .route("/{id}", delete(delete_entry))
        .route("/summary/{date}", get(get_daily_summary))
        .route("/nutrition/week", get(get_weekly_nutrition))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateDiaryEntryRequest {
    pub food_name: String,
    pub brand: Option<String>,
    pub portion_size: f32,
    pub unit: String, // "g", "ml", "piece", etc.
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fiber_per_100g: Option<f32>,
    pub sugar_per_100g: Option<f32>,
    pub sodium_per_100g: Option<f32>,
    pub meal_type: String, // "breakfast", "lunch", "dinner", "snack"
    pub consumed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct DiaryQueryParams {
    pub date: Option<NaiveDate>,
    pub meal_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct DiaryEntryResponse {
    pub id: Uuid,
    pub food_name: String,
    pub brand: Option<String>,
    pub portion_size: f32,
    pub unit: String,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_fat: f32,
    pub total_carbs: f32,
    pub total_fiber: Option<f32>,
    pub total_sugar: Option<f32>,
    pub total_sodium: Option<f32>,
    pub meal_type: String,
    pub consumed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl From<DiaryEntry> for DiaryEntryResponse {
    fn from(entry: DiaryEntry) -> Self {
        let multiplier = entry.portion_size / 100.0;
        
        Self {
            id: entry.id,
            food_name: entry.food_name,
            brand: entry.brand,
            portion_size: entry.portion_size,
            unit: entry.unit,
            total_calories: entry.calories_per_100g * multiplier,
            total_protein: entry.protein_per_100g * multiplier,
            total_fat: entry.fat_per_100g * multiplier,
            total_carbs: entry.carbs_per_100g * multiplier,
            total_fiber: entry.fiber_per_100g.map(|f| f * multiplier),
            total_sugar: entry.sugar_per_100g.map(|s| s * multiplier),
            total_sodium: entry.sodium_per_100g.map(|s| s * multiplier),
            meal_type: entry.meal_type,
            consumed_at: entry.consumed_at,
            created_at: entry.created_at,
        }
    }
}

pub async fn create_entry(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreateDiaryEntryRequest>,
) -> Result<ResponseJson<DiaryEntryResponse>, AppError> {
    payload.validate()?;

    let create_entry = CreateDiaryEntry {
        user_id: claims.sub,
        food_name: payload.food_name,
        brand: payload.brand,
        portion_size: payload.portion_size,
        unit: payload.unit,
        calories_per_100g: payload.calories_per_100g,
        protein_per_100g: payload.protein_per_100g,
        fat_per_100g: payload.fat_per_100g,
        carbs_per_100g: payload.carbs_per_100g,
        fiber_per_100g: payload.fiber_per_100g,
        sugar_per_100g: payload.sugar_per_100g,
        sodium_per_100g: payload.sodium_per_100g,
        meal_type: payload.meal_type,
        consumed_at: payload.consumed_at.unwrap_or_else(Utc::now),
    };

    let diary_service = DiaryService::new(pool);
    let entry = diary_service.create_entry(create_entry).await?;

    Ok(ResponseJson(entry.into()))
}

pub async fn get_entries(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<DiaryQueryParams>,
) -> Result<ResponseJson<Vec<DiaryEntryResponse>>, AppError> {
    let diary_service = DiaryService::new(pool);
    let entries = diary_service.get_user_entries(
        claims.sub,
        params.date,
        params.meal_type,
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
    ).await?;

    let response: Vec<DiaryEntryResponse> = entries.into_iter().map(Into::into).collect();
    Ok(ResponseJson(response))
}

pub async fn get_entry(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<DiaryEntryResponse>, AppError> {
    let diary_service = DiaryService::new(pool);
    let entry = diary_service.get_entry_by_id(id, claims.sub).await?;

    Ok(ResponseJson(entry.into()))
}

pub async fn update_entry(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateDiaryEntryRequest>,
) -> Result<ResponseJson<DiaryEntryResponse>, AppError> {
    payload.validate()?;

    let diary_service = DiaryService::new(pool);
    let entry = diary_service.update_entry(id, claims.sub, payload).await?;

    Ok(ResponseJson(entry.into()))
}

pub async fn delete_entry(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let diary_service = DiaryService::new(pool);
    diary_service.delete_entry(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Entry deleted successfully"})))
}

pub async fn get_daily_summary(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(date): Path<NaiveDate>,
) -> Result<ResponseJson<NutritionSummary>, AppError> {
    let diary_service = DiaryService::new(pool);
    let summary = diary_service.get_daily_summary(claims.sub, date).await?;

    Ok(ResponseJson(summary))
}

pub async fn get_weekly_nutrition(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<NutritionSummary>>, AppError> {
    let diary_service = DiaryService::new(pool);
    let summaries = diary_service.get_weekly_nutrition(claims.sub).await?;

    Ok(ResponseJson(summaries))
}
