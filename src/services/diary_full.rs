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

        // For now, return a mock entry since we need a database to use query macros
        // In production, uncomment the sqlx query below once database is set up
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

        /* TODO: Use this once DATABASE_URL is available
        let entry = sqlx::query_as!(
            DiaryEntry,
            r#"
            INSERT INTO diary_entries (
                id, user_id, food_name, brand, portion_size, unit,
                calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                fiber_per_100g, sugar_per_100g, sodium_per_100g,
                meal_type, consumed_at, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)
            RETURNING id, user_id, food_name, brand, portion_size, unit,
                      calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                      fiber_per_100g, sugar_per_100g, sodium_per_100g,
                      meal_type, consumed_at, created_at, updated_at
            "#,
            entry_id,
            entry_data.user_id,
            entry_data.food_name,
            entry_data.brand,
            entry_data.portion_size,
            entry_data.unit,
            entry_data.calories_per_100g,
            entry_data.protein_per_100g,
            entry_data.fat_per_100g,
            entry_data.carbs_per_100g,
            entry_data.fiber_per_100g,
            entry_data.sugar_per_100g,
            entry_data.sodium_per_100g,
            entry_data.meal_type,
            entry_data.consumed_at,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(entry)
        */
    }

    pub async fn get_user_entries(&self, user_id: Uuid, date: Option<NaiveDate>, meal_type: Option<String>, limit: i64, offset: i64) -> Result<Vec<DiaryEntry>, AppError> {
        let entries = match (date, meal_type) {
            (Some(date), Some(meal_type)) => {
                let start_of_day = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                let end_of_day = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                
                sqlx::query_as!(
                    DiaryEntry,
                    r#"
                    SELECT id, user_id, food_name, brand, portion_size, unit,
                           calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                           fiber_per_100g, sugar_per_100g, sodium_per_100g,
                           meal_type, consumed_at, created_at, updated_at
                    FROM diary_entries
                    WHERE user_id = $1 AND consumed_at >= $2 AND consumed_at <= $3 AND meal_type = $4
                    ORDER BY consumed_at DESC
                    LIMIT $5 OFFSET $6
                    "#,
                    user_id,
                    start_of_day,
                    end_of_day,
                    meal_type,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
            },
            (Some(date), None) => {
                let start_of_day = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
                let end_of_day = date.and_hms_opt(23, 59, 59).unwrap().and_utc();
                
                sqlx::query_as!(
                    DiaryEntry,
                    r#"
                    SELECT id, user_id, food_name, brand, portion_size, unit,
                           calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                           fiber_per_100g, sugar_per_100g, sodium_per_100g,
                           meal_type, consumed_at, created_at, updated_at
                    FROM diary_entries
                    WHERE user_id = $1 AND consumed_at >= $2 AND consumed_at <= $3
                    ORDER BY consumed_at DESC
                    LIMIT $4 OFFSET $5
                    "#,
                    user_id,
                    start_of_day,
                    end_of_day,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
            },
            (None, Some(meal_type)) => {
                sqlx::query_as!(
                    DiaryEntry,
                    r#"
                    SELECT id, user_id, food_name, brand, portion_size, unit,
                           calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                           fiber_per_100g, sugar_per_100g, sodium_per_100g,
                           meal_type, consumed_at, created_at, updated_at
                    FROM diary_entries
                    WHERE user_id = $1 AND meal_type = $2
                    ORDER BY consumed_at DESC
                    LIMIT $3 OFFSET $4
                    "#,
                    user_id,
                    meal_type,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
            },
            (None, None) => {
                sqlx::query_as!(
                    DiaryEntry,
                    r#"
                    SELECT id, user_id, food_name, brand, portion_size, unit,
                           calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                           fiber_per_100g, sugar_per_100g, sodium_per_100g,
                           meal_type, consumed_at, created_at, updated_at
                    FROM diary_entries
                    WHERE user_id = $1
                    ORDER BY consumed_at DESC
                    LIMIT $2 OFFSET $3
                    "#,
                    user_id,
                    limit,
                    offset
                )
                .fetch_all(&self.pool)
                .await
            }
        };

        entries.map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_entry_by_id(&self, id: Uuid, user_id: Uuid) -> Result<DiaryEntry, AppError> {
        let entry = sqlx::query_as!(
            DiaryEntry,
            r#"
            SELECT id, user_id, food_name, brand, portion_size, unit,
                   calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                   fiber_per_100g, sugar_per_100g, sodium_per_100g,
                   meal_type, consumed_at, created_at, updated_at
            FROM diary_entries
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Diary entry not found".to_string()))?;

        Ok(entry)
    }

    pub async fn update_entry(&self, id: Uuid, user_id: Uuid, payload: crate::api::diary::CreateDiaryEntryRequest) -> Result<DiaryEntry, AppError> {
        let now = Utc::now();

        let entry = sqlx::query_as!(
            DiaryEntry,
            r#"
            UPDATE diary_entries SET
                food_name = $3,
                brand = $4,
                portion_size = $5,
                unit = $6,
                calories_per_100g = $7,
                protein_per_100g = $8,
                fat_per_100g = $9,
                carbs_per_100g = $10,
                fiber_per_100g = $11,
                sugar_per_100g = $12,
                sodium_per_100g = $13,
                meal_type = $14,
                consumed_at = $15,
                updated_at = $16
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, food_name, brand, portion_size, unit,
                      calories_per_100g, protein_per_100g, fat_per_100g, carbs_per_100g,
                      fiber_per_100g, sugar_per_100g, sodium_per_100g,
                      meal_type, consumed_at, created_at, updated_at
            "#,
            id,
            user_id,
            payload.food_name,
            payload.brand,
            payload.portion_size,
            payload.unit,
            payload.calories_per_100g,
            payload.protein_per_100g,
            payload.fat_per_100g,
            payload.carbs_per_100g,
            payload.fiber_per_100g,
            payload.sugar_per_100g,
            payload.sodium_per_100g,
            payload.meal_type,
            payload.consumed_at,
            now
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Diary entry not found".to_string()))?;

        Ok(entry)
    }

    pub async fn delete_entry(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            "DELETE FROM diary_entries WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Diary entry not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_daily_summary(&self, user_id: Uuid, date: NaiveDate) -> Result<NutritionSummary, AppError> {
        let start_of_day = date.and_hms_opt(0, 0, 0).unwrap().and_utc();
        let end_of_day = date.and_hms_opt(23, 59, 59).unwrap().and_utc();

        let summary = sqlx::query!(
            r#"
            SELECT 
                COALESCE(SUM((calories_per_100g * portion_size) / 100), 0) as total_calories,
                COALESCE(SUM((protein_per_100g * portion_size) / 100), 0) as total_protein,
                COALESCE(SUM((fat_per_100g * portion_size) / 100), 0) as total_fat,
                COALESCE(SUM((carbs_per_100g * portion_size) / 100), 0) as total_carbs,
                COALESCE(SUM((fiber_per_100g * portion_size) / 100), 0) as total_fiber,
                COALESCE(SUM((sugar_per_100g * portion_size) / 100), 0) as total_sugar,
                COALESCE(SUM((sodium_per_100g * portion_size) / 100), 0) as total_sodium,
                COUNT(*) as total_entries
            FROM diary_entries
            WHERE user_id = $1 AND consumed_at >= $2 AND consumed_at <= $3
            "#,
            user_id,
            start_of_day,
            end_of_day
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(NutritionSummary {
            date,
            total_calories: summary.total_calories.unwrap_or(0.0) as f32,
            total_protein: summary.total_protein.unwrap_or(0.0) as f32,
            total_fat: summary.total_fat.unwrap_or(0.0) as f32,
            total_carbs: summary.total_carbs.unwrap_or(0.0) as f32,
            total_fiber: summary.total_fiber.unwrap_or(0.0) as f32,
            total_sugar: summary.total_sugar.unwrap_or(0.0) as f32,
            total_sodium: summary.total_sodium.unwrap_or(0.0) as f32,
            meal_breakdown: Vec::new(), // TODO: implement meal breakdown calculation
            calorie_goal: None,
            protein_goal: None,
            fat_goal: None,
            carbs_goal: None,
        })
    }

    pub async fn get_weekly_nutrition(&self, user_id: Uuid) -> Result<Vec<NutritionSummary>, AppError> {
        // Get nutrition summaries for the last 7 days
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
