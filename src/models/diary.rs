use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DiaryEntry {
    pub id: Uuid,
    pub user_id: Uuid,
    pub food_name: String,
    pub brand: Option<String>,
    pub portion_size: f32,
    pub unit: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fiber_per_100g: Option<f32>,
    pub sugar_per_100g: Option<f32>,
    pub sodium_per_100g: Option<f32>,
    pub meal_type: String,
    pub consumed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDiaryEntry {
    pub user_id: Uuid,
    pub food_name: String,
    pub brand: Option<String>,
    pub portion_size: f32,
    pub unit: String,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fiber_per_100g: Option<f32>,
    pub sugar_per_100g: Option<f32>,
    pub sodium_per_100g: Option<f32>,
    pub meal_type: String,
    pub consumed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct NutritionSummary {
    pub date: NaiveDate,
    pub total_calories: f32,
    pub total_protein: f32,
    pub total_fat: f32,
    pub total_carbs: f32,
    pub total_fiber: f32,
    pub total_sugar: f32,
    pub total_sodium: f32,
    pub meal_breakdown: Vec<MealSummary>,
    pub calorie_goal: Option<f32>,
    pub protein_goal: Option<f32>,
    pub fat_goal: Option<f32>,
    pub carbs_goal: Option<f32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MealSummary {
    pub meal_type: String,
    pub calories: f32,
    pub protein: f32,
    pub fat: f32,
    pub carbs: f32,
    pub entries_count: i32,
}

impl DiaryEntry {
    pub fn calculate_nutrition(&self) -> (f32, f32, f32, f32) {
        let multiplier = self.portion_size / 100.0;
        (
            self.calories_per_100g * multiplier,
            self.protein_per_100g * multiplier,
            self.fat_per_100g * multiplier,
            self.carbs_per_100g * multiplier,
        )
    }

    pub fn total_calories(&self) -> f32 {
        self.calories_per_100g * (self.portion_size / 100.0)
    }

    pub fn total_protein(&self) -> f32 {
        self.protein_per_100g * (self.portion_size / 100.0)
    }

    pub fn total_fat(&self) -> f32 {
        self.fat_per_100g * (self.portion_size / 100.0)
    }

    pub fn total_carbs(&self) -> f32 {
        self.carbs_per_100g * (self.portion_size / 100.0)
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct FoodItem {
    pub id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fiber_per_100g: Option<f32>,
    pub sugar_per_100g: Option<f32>,
    pub sodium_per_100g: Option<f32>,
    pub verified: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFoodItem {
    pub name: String,
    pub brand: Option<String>,
    pub barcode: Option<String>,
    pub calories_per_100g: f32,
    pub protein_per_100g: f32,
    pub fat_per_100g: f32,
    pub carbs_per_100g: f32,
    pub fiber_per_100g: Option<f32>,
    pub sugar_per_100g: Option<f32>,
    pub sodium_per_100g: Option<f32>,
    pub created_by: Uuid,
}
