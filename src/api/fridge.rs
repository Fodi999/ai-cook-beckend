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
    models::{
        fridge::{FridgeItem, CreateFridgeItem, FridgeCategory, FoodWaste, CreateFoodWaste, WasteReason, ExpenseAnalytics, EconomyInsights, Allergen, Intolerance, DietType},
        presets::{FoodPresets, AllergenInfo, IntoleranceInfo, DietInfo, ProductPreset}
    },
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
        .route("/waste", post(add_waste))
        .route("/waste", get(get_waste_history))
        .route("/analytics/expenses", get(get_expense_analytics))
        .route("/analytics/insights", get(get_economy_insights))
}

pub fn public_routes() -> Router {
    Router::new()
        // Публичные endpoints для предустановленных данных (не требуют авторизации)
        .route("/presets/allergens", get(get_allergen_presets))
        .route("/presets/intolerances", get(get_intolerance_presets))
        .route("/presets/diets", get(get_diet_presets))
        .route("/presets/products", get(get_product_presets))
        .route("/presets/products/search", get(search_product_presets))
        .route("/autocomplete", get(get_autocomplete_options))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFridgeItemRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub brand: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub price_per_unit: Option<f32>,
    pub total_price: Option<f32>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub location: Option<String>, // "fridge", "freezer", "pantry"
    // Новые поля для диетических ограничений
    pub contains_allergens: Option<Vec<Allergen>>,
    pub contains_intolerances: Option<Vec<Intolerance>>,
    pub suitable_for_diets: Option<Vec<DietType>>,
    pub ingredients: Option<String>,
    pub nutritional_info: Option<String>,
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
    pub price_per_unit: Option<f32>,
    pub total_price: Option<f32>,
    pub calculated_total_value: f32, // Автоматически рассчитанная стоимость
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
        let calculated_total_value = item.calculate_total_value();

        Self {
            id: item.id,
            name: item.name,
            brand: item.brand,
            quantity: item.quantity,
            unit: item.unit,
            category: item.category,
            price_per_unit: item.price_per_unit,
            total_price: item.total_price,
            calculated_total_value,
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
    println!("🔍 ADD ITEM: Received request from user {}", claims.sub);
    payload.validate()?;

    let create_item = CreateFridgeItem {
        user_id: claims.sub,
        name: payload.name,
        brand: payload.brand,
        quantity: payload.quantity,
        unit: payload.unit,
        category: payload.category,
        price_per_unit: payload.price_per_unit,
        total_price: payload.total_price,
        expiry_date: payload.expiry_date,
        purchase_date: payload.purchase_date.unwrap_or_else(Utc::now),
        notes: payload.notes,
        location: payload.location,
        // Новые поля для диетических ограничений
        contains_allergens: payload.contains_allergens.unwrap_or_default(),
        contains_intolerances: payload.contains_intolerances.unwrap_or_default(),
        suitable_for_diets: payload.suitable_for_diets.unwrap_or_default(),
        ingredients: payload.ingredients,
        nutritional_info: payload.nutritional_info,
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
    println!("🔍 GET ITEMS: Received request from user {}", claims.sub);
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

// Новые handler'ы для отходов и аналитики

#[derive(Debug, Deserialize, Validate)]
pub struct CreateFoodWasteRequest {
    pub original_item_id: Option<Uuid>,
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub brand: Option<String>,
    pub wasted_quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub waste_reason: WasteReason,
    pub wasted_value: Option<f32>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct WasteQueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQueryParams {
    pub period: Option<String>, // "day", "week", "month"
}

pub async fn add_waste(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreateFoodWasteRequest>,
) -> Result<ResponseJson<FoodWaste>, AppError> {
    payload.validate()?;

    let create_waste = CreateFoodWaste {
        user_id: claims.sub,
        original_item_id: payload.original_item_id,
        name: payload.name,
        brand: payload.brand,
        wasted_quantity: payload.wasted_quantity,
        unit: payload.unit,
        category: payload.category,
        waste_reason: payload.waste_reason,
        wasted_value: payload.wasted_value,
        notes: payload.notes,
    };

    let fridge_service = FridgeService::new(pool);
    let waste = fridge_service.add_waste(create_waste).await?;

    Ok(ResponseJson(waste))
}

pub async fn get_waste_history(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<WasteQueryParams>,
) -> Result<ResponseJson<Vec<FoodWaste>>, AppError> {
    let fridge_service = FridgeService::new(pool);
    let waste_history = fridge_service.get_waste_history(
        claims.sub,
        params.start_date,
        params.end_date,
    ).await?;

    Ok(ResponseJson(waste_history))
}

pub async fn get_expense_analytics(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<AnalyticsQueryParams>,
) -> Result<ResponseJson<ExpenseAnalytics>, AppError> {
    let period = params.period.as_deref().unwrap_or("week");
    
    let fridge_service = FridgeService::new(pool);
    let analytics = fridge_service.get_expense_analytics(claims.sub, period).await?;

    Ok(ResponseJson(analytics))
}

pub async fn get_economy_insights(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<EconomyInsights>, AppError> {
    let fridge_service = FridgeService::new(pool);
    let insights = fridge_service.get_economy_insights(claims.sub).await?;

    Ok(ResponseJson(insights))
}

// =============================================================================
// PRESET ENDPOINTS - Работа с предустановленными данными
// =============================================================================

#[derive(Debug, Deserialize)]
pub struct ProductSearchQuery {
    pub name: Option<String>,
    pub category: Option<FridgeCategory>,
    pub diet: Option<DietType>,
    pub without_allergen: Option<Allergen>,
    pub without_intolerance: Option<Intolerance>,
}

/// GET /api/fridge/presets/allergens
/// Получить список всех доступных аллергенов с подробной информацией
pub async fn get_allergen_presets() -> Result<ResponseJson<Vec<AllergenInfo>>, AppError> {
    let allergens = FoodPresets::get_allergen_info();
    Ok(ResponseJson(allergens))
}

/// GET /api/fridge/presets/intolerances
/// Получить список всех доступных непереносимостей с подробной информацией
pub async fn get_intolerance_presets() -> Result<ResponseJson<Vec<IntoleranceInfo>>, AppError> {
    let intolerances = FoodPresets::get_intolerance_info();
    Ok(ResponseJson(intolerances))
}

/// GET /api/fridge/presets/diets
/// Получить список всех доступных диет с подробной информацией
pub async fn get_diet_presets() -> Result<ResponseJson<Vec<DietInfo>>, AppError> {
    let diets = FoodPresets::get_diet_info();
    Ok(ResponseJson(diets))
}

/// GET /api/fridge/presets/products
/// Получить список всех предустановленных продуктов с информацией о диетических ограничениях
pub async fn get_product_presets() -> Result<ResponseJson<Vec<ProductPreset>>, AppError> {
    let products = FoodPresets::get_product_presets();
    Ok(ResponseJson(products))
}

/// GET /api/fridge/presets/products/search?name=&category=&diet=&without_allergen=&without_intolerance=
/// Поиск продуктов по различным критериям
pub async fn search_product_presets(
    Query(query): Query<ProductSearchQuery>,
) -> Result<ResponseJson<Vec<ProductPreset>>, AppError> {
    let mut products = FoodPresets::get_product_presets();

    // Фильтрация по имени
    if let Some(name) = &query.name {
        products.retain(|p| p.name.to_lowercase().contains(&name.to_lowercase()));
    }

    // Фильтрация по категории
    if let Some(category) = &query.category {
        products.retain(|p| p.category == *category);
    }

    // Фильтрация по подходящей диете
    if let Some(diet) = &query.diet {
        products.retain(|p| p.suitable_diets.contains(diet));
    }

    // Фильтрация по исключению аллергена
    if let Some(allergen) = &query.without_allergen {
        products.retain(|p| !p.common_allergens.contains(allergen));
    }

    // Фильтрация по исключению непереносимости
    if let Some(intolerance) = &query.without_intolerance {
        products.retain(|p| !p.common_intolerances.contains(intolerance));
    }

    Ok(ResponseJson(products))
}

// =============================================================================
// UTILITY ENDPOINTS - Дополнительные endpoints для автозаполнения
// =============================================================================

#[derive(Debug, Serialize)]
pub struct AutocompleteResponse {
    pub allergens: Vec<Allergen>,
    pub intolerances: Vec<Intolerance>,
    pub diets: Vec<DietType>,
}

/// GET /api/fridge/autocomplete
/// Получить все доступные опции для автозаполнения форм
pub async fn get_autocomplete_options() -> Result<ResponseJson<AutocompleteResponse>, AppError> {
    let response = AutocompleteResponse {
        allergens: FoodPresets::get_all_allergens(),
        intolerances: FoodPresets::get_all_intolerances(),
        diets: FoodPresets::get_all_diets(),
    };
    
    Ok(ResponseJson(response))
}
