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
    models::recipe::{Recipe, CreateRecipe, RecipeCategory, DifficultyLevel, RecipeIngredient},
    services::{auth::Claims, recipe::RecipeService, ai::AiService},
    utils::errors::AppError,
};

pub fn routes() -> Router {
    Router::new()
        .route("/", post(create_recipe))
        .route("/", get(get_recipes))
        .route("/{id}", get(get_recipe))
        .route("/{id}", put(update_recipe))
        .route("/{id}", delete(delete_recipe))
        .route("/{id}/favorite", post(toggle_favorite))
        .route("/{id}/rating", post(rate_recipe))
        .route("/search", get(search_recipes))
        .route("/generate", post(generate_ai_recipe))
        .route("/popular", get(get_popular_recipes))
        .route("/favorites", get(get_favorite_recipes))
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRecipeRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    pub description: Option<String>,
    pub category: RecipeCategory,
    pub difficulty: DifficultyLevel,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub instructions: String,
    pub ingredients: Vec<CreateRecipeIngredientRequest>,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub source_url: Option<String>,
    pub nutrition_per_serving: Option<NutritionInfoRequest>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateRecipeIngredientRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct NutritionInfoRequest {
    pub calories: Option<f32>,
    pub protein: Option<f32>,
    pub fat: Option<f32>,
    pub carbs: Option<f32>,
    pub fiber: Option<f32>,
    pub sugar: Option<f32>,
    pub sodium: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeQueryParams {
    pub category: Option<RecipeCategory>,
    pub difficulty: Option<DifficultyLevel>,
    pub max_prep_time: Option<i32>,
    pub max_cook_time: Option<i32>,
    pub search: Option<String>,
    pub tags: Option<String>, // comma-separated
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct GenerateRecipeRequest {
    #[validate(length(min = 10, max = 500))]
    pub description: String,
    pub available_ingredients: Option<Vec<String>>,
    pub dietary_restrictions: Option<Vec<String>>,
    pub max_prep_time: Option<i32>,
    pub servings: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct RatingRequest {
    pub rating: i32, // 1-5
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecipeResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: RecipeCategory,
    pub difficulty: DifficultyLevel,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub total_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub instructions: String,
    pub ingredients: Vec<RecipeIngredientResponse>,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub source_url: Option<String>,
    pub nutrition_per_serving: Option<NutritionInfoResponse>,
    pub average_rating: Option<f32>,
    pub ratings_count: i32,
    pub is_favorite: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RecipeIngredientResponse {
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NutritionInfoResponse {
    pub calories: Option<f32>,
    pub protein: Option<f32>,
    pub fat: Option<f32>,
    pub carbs: Option<f32>,
    pub fiber: Option<f32>,
    pub sugar: Option<f32>,
    pub sodium: Option<f32>,
}

pub async fn create_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<CreateRecipeRequest>,
) -> Result<ResponseJson<RecipeResponse>, AppError> {
    payload.validate()?;

    let create_recipe = CreateRecipe {
        name: payload.name,
        description: payload.description,
        category: payload.category,
        difficulty: payload.difficulty,
        prep_time_minutes: payload.prep_time_minutes,
        cook_time_minutes: payload.cook_time_minutes,
        servings: payload.servings,
        instructions: payload.instructions,
        tags: payload.tags,
        image_url: payload.image_url,
        source_url: payload.source_url,
        created_by: claims.sub,
    };

    let recipe_service = RecipeService::new(pool);
    let recipe = recipe_service.create_recipe(create_recipe, payload.ingredients, payload.nutrition_per_serving).await?;

    Ok(ResponseJson(recipe))
}

pub async fn get_recipes(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<RecipeQueryParams>,
) -> Result<ResponseJson<Vec<RecipeResponse>>, AppError> {
    let recipe_service = RecipeService::new(pool);
    let recipes = recipe_service.get_recipes(
        Some(claims.sub),
        params.category,
        params.difficulty,
        params.max_prep_time,
        params.max_cook_time,
        params.search,
        params.tags,
        params.limit.unwrap_or(20),
        params.offset.unwrap_or(0),
    ).await?;

    Ok(ResponseJson(recipes))
}

pub async fn get_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<RecipeResponse>, AppError> {
    let recipe_service = RecipeService::new(pool);
    let recipe = recipe_service.get_recipe_by_id(id, Some(claims.sub)).await?;

    Ok(ResponseJson(recipe))
}

pub async fn update_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateRecipeRequest>,
) -> Result<ResponseJson<RecipeResponse>, AppError> {
    payload.validate()?;

    let recipe_service = RecipeService::new(pool);
    let recipe = recipe_service.update_recipe(id, claims.sub, payload).await?;

    Ok(ResponseJson(recipe))
}

pub async fn delete_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let recipe_service = RecipeService::new(pool);
    recipe_service.delete_recipe(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Recipe deleted successfully"})))
}

pub async fn toggle_favorite(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    let recipe_service = RecipeService::new(pool);
    let is_favorite = recipe_service.toggle_favorite(id, claims.sub).await?;

    Ok(ResponseJson(serde_json::json!({
        "is_favorite": is_favorite,
        "message": if is_favorite { "Added to favorites" } else { "Removed from favorites" }
    })))
}

pub async fn rate_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Path(id): Path<Uuid>,
    Json(payload): Json<RatingRequest>,
) -> Result<ResponseJson<serde_json::Value>, AppError> {
    if payload.rating < 1 || payload.rating > 5 {
        return Err(AppError::BadRequest("Rating must be between 1 and 5".to_string()));
    }

    let recipe_service = RecipeService::new(pool);
    recipe_service.rate_recipe(id, claims.sub, payload.rating, payload.comment).await?;

    Ok(ResponseJson(serde_json::json!({"message": "Recipe rated successfully"})))
}

pub async fn search_recipes(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Query(params): Query<RecipeQueryParams>,
) -> Result<ResponseJson<Vec<RecipeResponse>>, AppError> {
    let search_query = params.search.unwrap_or_default();
    
    let recipe_service = RecipeService::new(pool);
    let recipes = recipe_service.search_recipes(
        search_query,
        Some(claims.sub),
        params.category,
        params.difficulty,
        params.limit.unwrap_or(20),
        params.offset.unwrap_or(0),
    ).await?;

    Ok(ResponseJson(recipes))
}

pub async fn generate_ai_recipe(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
    Json(payload): Json<GenerateRecipeRequest>,
) -> Result<ResponseJson<RecipeResponse>, AppError> {
    payload.validate()?;

    let ai_service = AiService::from_env();
    let recipe_service = RecipeService::new(pool);
    
    let generated_recipe = ai_service.generate_recipe(
        &payload.description,
        payload.available_ingredients.unwrap_or_default(),
        payload.dietary_restrictions.unwrap_or_default(),
        payload.max_prep_time,
        payload.servings,
    ).await?;

    // Сохраняем AI-сгенерированный рецепт
    let create_recipe = CreateRecipe {
        name: generated_recipe.name,
        description: Some(generated_recipe.description),
        category: generated_recipe.category,
        difficulty: generated_recipe.difficulty,
        prep_time_minutes: generated_recipe.prep_time_minutes,
        cook_time_minutes: generated_recipe.cook_time_minutes,
        servings: generated_recipe.servings,
        instructions: generated_recipe.instructions,
        tags: generated_recipe.tags,
        image_url: None,
        source_url: Some("AI Generated".to_string()),
        created_by: claims.sub,
    };

    let recipe = recipe_service.create_recipe(
        create_recipe,
        generated_recipe.ingredients,
        generated_recipe.nutrition_per_serving,
    ).await?;

    Ok(ResponseJson(recipe))
}

pub async fn get_popular_recipes(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<RecipeResponse>>, AppError> {
    let recipe_service = RecipeService::new(pool);
    let recipes = recipe_service.get_popular_recipes(Some(claims.sub)).await?;

    Ok(ResponseJson(recipes))
}

pub async fn get_favorite_recipes(
    Extension(pool): Extension<DbPool>,
    claims: Claims,
) -> Result<ResponseJson<Vec<RecipeResponse>>, AppError> {
    let recipe_service = RecipeService::new(pool);
    let recipes = recipe_service.get_favorite_recipes(claims.sub).await?;

    Ok(ResponseJson(recipes))
}
