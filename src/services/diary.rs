use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::{
    models::diary::{DiaryEntry, CreateDiaryEntry, NutritionSummary, MealSummary},
    utils::errors::AppError,
};

pub struct DiaryService {
    pool: crate::db::DbPool,
}

impl DiaryService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_entry(&self, entry_data: CreateDiaryEntry) -> Result<DiaryEntry, AppError> {
        let entry_id = Uuid::new_v4();
        let now = Utc::now();

        // Mock implementation for compilation without database
        // TODO: Replace with real database operations when DATABASE_URL is available
        Ok(DiaryEntry {
            id: entry_id,
            user_id: entry_data.user_id,
            food_name: entry_data.food_name,
            brand: entry_data.brand,
            portion_size: entry_data.portion_size,
            unit: entry_data.unit,
            calories_per_100g: entry_data.calories_per_100g,
            protein_per_100g: entry_data.protein_per_100g,
            fat_per_100g: entry_data.fat_per_100g,
            carbs_per_100g: entry_data.carbs_per_100g,
            fiber_per_100g: entry_data.fiber_per_100g,
            sugar_per_100g: entry_data.sugar_per_100g,
            sodium_per_100g: entry_data.sodium_per_100g,
            meal_type: entry_data.meal_type,
            consumed_at: entry_data.consumed_at,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_entries(&self, _user_id: Uuid, _date: Option<NaiveDate>, _meal_type: Option<String>, _limit: i64, _offset: i64) -> Result<Vec<DiaryEntry>, AppError> {
        // Mock implementation
        Ok(vec![])
    }

    pub async fn get_entry_by_id(&self, _id: Uuid, _user_id: Uuid) -> Result<DiaryEntry, AppError> {
        // Mock implementation
        Err(AppError::NotFound("Entry not found".to_string()))
    }

    pub async fn update_entry(&self, _id: Uuid, _user_id: Uuid, _payload: crate::api::diary::CreateDiaryEntryRequest) -> Result<DiaryEntry, AppError> {
        // Mock implementation
        Err(AppError::InternalServerError("Not implemented".to_string()))
    }

    pub async fn delete_entry(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation
        Ok(())
    }

    pub async fn get_daily_summary(&self, _user_id: Uuid, date: NaiveDate) -> Result<NutritionSummary, AppError> {
        // Mock implementation
        Ok(NutritionSummary {
            date,
            total_calories: 2000.0,
            total_protein: 100.0,
            total_fat: 70.0,
            total_carbs: 250.0,
            total_fiber: 25.0,
            total_sugar: 50.0,
            total_sodium: 2300.0,
            meal_breakdown: vec![],
            calorie_goal: Some(2200.0),
            protein_goal: Some(120.0),
            fat_goal: Some(80.0),
            carbs_goal: Some(300.0),
        })
    }

    pub async fn get_weekly_nutrition(&self, user_id: Uuid) -> Result<Vec<NutritionSummary>, AppError> {
        // Mock implementation - return 7 days of mock data
        let mut summaries = Vec::new();
        let today = chrono::Utc::now().date_naive();
        
        for i in 0..7 {
            let date = today - chrono::Duration::days(i);
            let summary = self.get_daily_summary(user_id, date).await?;
            summaries.push(summary);
        }

        Ok(summaries)
    }
}
