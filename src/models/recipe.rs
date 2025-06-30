// Placeholder models
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "recipe_category", rename_all = "lowercase")]
pub enum RecipeCategory {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
    Dessert,
    Appetizer,
    Beverage,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "difficulty_level", rename_all = "lowercase")]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Recipe {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: RecipeCategory,
    pub difficulty: DifficultyLevel,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub instructions: String,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub source_url: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateRecipe {
    pub name: String,
    pub description: Option<String>,
    pub category: RecipeCategory,
    pub difficulty: DifficultyLevel,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub instructions: String,
    pub tags: Vec<String>,
    pub image_url: Option<String>,
    pub source_url: Option<String>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub id: Uuid,
    pub recipe_id: Uuid,
    pub name: String,
    pub quantity: f32,
    pub unit: String,
    pub notes: Option<String>,
}
