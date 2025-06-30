use axum::{
    extract::{Extension, Json, Path, Query},
    response::Json as ResponseJson,
    routing::{get, post, put, delete},
    Router,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::{
    db::DbPool,
    models::fridge::{FridgeItem, CreateFridgeItem, FridgeCategory},
    services::{auth::Claims, fridge::FridgeService, ai::AiService},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(add_item))
        .route("/", get(get_items))
        .route("/{id}", get(get_item))
        .route("/{id}", put(update_item))
        .route("/{id}", delete(remove_item))
        .route("/suggestions", get(get_recipe_suggestions))
        .route("/expiring", get(get_expiring_items))
        .route("/categories", get(get_categories))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFridgeItemRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub brand: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub location: Option<String>, // "fridge", "freezer", "pantry"
}

#[derive(Debug, Deserialize)]
pub struct FridgeQueryParams {
    pub category: Option<FridgeCategory>,
    pub location: Option<String>,
    pub expiring_days: Option<i32>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FridgeItemResponse {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub location: Option<String>,
    pub days_until_expiry: Option<i32>,
    pub is_expired: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<FridgeItem> for FridgeItemResponse {
    fn from(item: FridgeItem) -> Self {
        let now = Utc::now();
        let days_until_expiry = item.expiry_date.map(|exp| {
            (exp - now).num_days() as i32
        });
        let is_expired = days_until_expiry.map_or(false, |days| days < 0);

        Self {
            id: item.id,
            name: item.name,
            brand: item.brand,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
            expiry_date: item.expiry_date,
            purchase_date: Some(item.purchase_date),
            notes: item.notes,
            location: item.location,
            days_until_expiry,
            is_expired,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RecipeSuggestion {
    pub recipe_name: String,
    pub ingredients_available: Vec<String>,
    pub ingredients_needed: Vec<String>,
    pub preparation_time: Option<i32>,
    pub difficulty: Option<String>,
    pub instructions: Option<String>,
    pub ai_generated: bool,
}

pub async fn add_item(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreateFridgeItemRequest>,
) -> Result<ResponseJson<FridgeItemResponse>, AppError> {
    payload.validate()?;

    let create_item = CreateFridgeItem {
        user_id: claims.sub,
        name: payload.name,
        brand: payload.brand,
        quantity: payload.quantity,
        unit: payload.unit,
        category: payload.category,
        expiry_date: payload.expiry_date,
        purchase_date: payload.purchase_date.unwrap_or_else(Utc::now),
        notes: payload.notes,
        location: payload.location,
    };

    let fridge_service = FridgeService::new(pool);
    let item = fridge_service.add_item(create_item).await?;

    Ok(ResponseJson(item.into()))
}

pub async fn get_items(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<FridgeQueryParams>,
) -> Result<ResponseJson<Vec<FridgeItemResponse>>, AppError> {
    let fridge_service = FridgeService::new(pool);
    let items = fridge_service.get_user_items(
        claims.sub,
        params.category,
        params.location,
        params.search,
    ).await?;

    let response: Vec<FridgeItemResponse> = items.into_iter().map(Into::into).collect();
    Ok(ResponseJson(response))
}

pub async fn get_item(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<FridgeItemResponse>, AppError> {
    let fridge_service = FridgeService::new(pool);
    let item = fridge_service.get_item_by_id(id, claims.sub).await?;

    Ok(ResponseJson(item.into()))
}

pub async fn update_item(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateFridgeItemRequest>,
) -> Result<ResponseJson<FridgeItemResponse>, AppError> {
    payload.validate()?;

    let fridge_service = FridgeService::new(pool);
    let item = fridge_service.update_item(id, claims.sub, payload).await?;

    Ok(ResponseJson(item.into()))
}

pub async fn remove_item(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let fridge_service = FridgeService::new(pool);
    fridge_service.remove_item(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Item removed successfully"})))
}

pub async fn get_recipe_suggestions(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<RecipeSuggestion>>, AppError> {
    let fridge_service = FridgeService::new(pool);
    let ai_service = AiService::from_env();
    
    let available_items = fridge_service.get_user_items(claims.sub, None, None, None).await?;
    let suggestions = ai_service.generate_recipe_suggestions(available_items).await?;

    Ok(ResponseJson(suggestions))
}

pub async fn get_expiring_items(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<FridgeQueryParams>,
) -> Result<ResponseJson<Vec<FridgeItemResponse>>, AppError> {
    let days = params.expiring_days.unwrap_or(3);
    
    let fridge_service = FridgeService::new(pool);
    let items = fridge_service.get_expiring_items(claims.sub, Some(days as u32)).await?;

    let response: Vec<FridgeItemResponse> = items.into_iter().map(Into::into).collect();
    Ok(ResponseJson(response))
}

pub async fn get_categories() -> Result<ResponseJson<Vec<FridgeCategory>>, AppError> {
    Ok(ResponseJson(vec![
        FridgeCategory::Dairy,
        FridgeCategory::Meat,
        FridgeCategory::Fish,
        FridgeCategory::Vegetables,
        FridgeCategory::Fruits,
        FridgeCategory::Grains,
        FridgeCategory::Beverages,
        FridgeCategory::Condiments,
        FridgeCategory::Snacks,
        FridgeCategory::Other,
    ]))
}
