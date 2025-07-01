use uuid::Uuid;
use chrono::Utc;
use crate::{
    models::fridge::{FridgeItem, CreateFridgeItem, FridgeCategory},
    utils::errors::AppError,
};

pub struct FridgeService {
    pool: crate::db::DbPool,
}

impl FridgeService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { pool }
    }

    pub async fn add_item(&self, item_data: CreateFridgeItem) -> Result<FridgeItem, AppError> {
        let item_id = Uuid::new_v4();
        let now = Utc::now();

        let item = sqlx::query_as!(
            FridgeItem,
            r#"
            INSERT INTO fridge_items (
                id, user_id, name, brand, quantity, unit, category,
                expiry_date, purchase_date, notes, location, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, user_id, name, brand, quantity, unit, 
                      category as "category: FridgeCategory",
                      expiry_date, purchase_date, notes, location, created_at, updated_at
            "#,
            item_id,
            item_data.user_id,
            item_data.name,
            item_data.brand,
            item_data.quantity,
            item_data.unit,
            item_data.category as FridgeCategory,
            item_data.expiry_date,
            item_data.purchase_date,
            item_data.notes,
            item_data.location,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(item)
    }

    pub async fn get_user_items(&self, user_id: Uuid, category: Option<FridgeCategory>, location: Option<String>, search: Option<String>) -> Result<Vec<FridgeItem>, AppError> {
        let items = match (category, location, search) {
            (Some(cat), Some(loc), Some(search_term)) => {
                let search_pattern = format!("%{}%", search_term.to_lowercase());
                sqlx::query_as!(
                    FridgeItem,
                    r#"
                    SELECT id, user_id, name, brand, quantity, unit,
                           category as "category: FridgeCategory",
                           expiry_date, purchase_date, notes, location, created_at, updated_at
                    FROM fridge_items
                    WHERE user_id = $1 AND category = $2 AND location = $3 
                          AND (LOWER(name) LIKE $4 OR LOWER(brand) LIKE $4)
                    ORDER BY expiry_date ASC
                    "#,
                    user_id,
                    cat as FridgeCategory,
                    loc,
                    search_pattern
                )
                .fetch_all(&self.pool)
                .await
            },
            (Some(cat), None, None) => {
                sqlx::query_as!(
                    FridgeItem,
                    r#"
                    SELECT id, user_id, name, brand, quantity, unit,
                           category as "category: FridgeCategory",
                           expiry_date, purchase_date, notes, location, created_at, updated_at
                    FROM fridge_items
                    WHERE user_id = $1 AND category = $2
                    ORDER BY expiry_date ASC
                    "#,
                    user_id,
                    cat as FridgeCategory
                )
                .fetch_all(&self.pool)
                .await
            },
            (None, None, Some(search_term)) => {
                let search_pattern = format!("%{}%", search_term.to_lowercase());
                sqlx::query_as!(
                    FridgeItem,
                    r#"
                    SELECT id, user_id, name, brand, quantity, unit,
                           category as "category: FridgeCategory",
                           expiry_date, purchase_date, notes, location, created_at, updated_at
                    FROM fridge_items
                    WHERE user_id = $1 AND (LOWER(name) LIKE $2 OR LOWER(brand) LIKE $2)
                    ORDER BY name ASC
                    "#,
                    user_id,
                    search_pattern
                )
                .fetch_all(&self.pool)
                .await
            },
            _ => {
                sqlx::query_as!(
                    FridgeItem,
                    r#"
                    SELECT id, user_id, name, brand, quantity, unit,
                           category as "category: FridgeCategory",
                           expiry_date, purchase_date, notes, location, created_at, updated_at
                    FROM fridge_items
                    WHERE user_id = $1
                    ORDER BY expiry_date ASC
                    "#,
                    user_id
                )
                .fetch_all(&self.pool)
                .await
            }
        };

        items.map_err(|e| AppError::Database(e.to_string()))
    }

    pub async fn get_item_by_id(&self, id: Uuid, user_id: Uuid) -> Result<FridgeItem, AppError> {
        let item = sqlx::query_as!(
            FridgeItem,
            r#"
            SELECT id, user_id, name, brand, quantity, unit,
                   category as "category: FridgeCategory",
                   expiry_date, purchase_date, notes, location, created_at, updated_at
            FROM fridge_items
            WHERE id = $1 AND user_id = $2
            "#,
            id,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Fridge item not found".to_string()))?;

        Ok(item)
    }

    pub async fn update_item(&self, id: Uuid, user_id: Uuid, payload: crate::api::fridge::CreateFridgeItemRequest) -> Result<FridgeItem, AppError> {
        let now = Utc::now();

        let item = sqlx::query_as!(
            FridgeItem,
            r#"
            UPDATE fridge_items SET
                name = $3,
                brand = $4,
                quantity = $5,
                unit = $6,
                category = $7,
                expiry_date = $8,
                notes = $9,
                location = $10,
                updated_at = $11
            WHERE id = $1 AND user_id = $2
            RETURNING id, user_id, name, brand, quantity, unit,
                      category as "category: FridgeCategory",
                      expiry_date, purchase_date, notes, location, created_at, updated_at
            "#,
            id,
            user_id,
            payload.name,
            payload.brand,
            payload.quantity,
            payload.unit,
            payload.category as FridgeCategory,
            payload.expiry_date,
            payload.notes,
            payload.location,
            now
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?
        .ok_or_else(|| AppError::NotFound("Fridge item not found".to_string()))?;

        Ok(item)
    }

    pub async fn remove_item(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query!(
            "DELETE FROM fridge_items WHERE id = $1 AND user_id = $2",
            id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("Fridge item not found".to_string()));
        }

        Ok(())
    }

    pub async fn get_expiring_items(&self, user_id: Uuid, days: i32) -> Result<Vec<FridgeItem>, AppError> {
        let future_date = Utc::now() + chrono::Duration::days(days as i64);

        let items = sqlx::query_as!(
            FridgeItem,
            r#"
            SELECT id, user_id, name, brand, quantity, unit,
                   category as "category: FridgeCategory",
                   expiry_date, purchase_date, notes, location, created_at, updated_at
            FROM fridge_items
            WHERE user_id = $1 AND expiry_date IS NOT NULL 
                  AND expiry_date >= NOW() AND expiry_date <= $2
            ORDER BY expiry_date ASC
            "#,
            user_id,
            future_date
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| AppError::Database(e.to_string()))?;

        Ok(items)
    }

    pub async fn check_and_notify_expiring_items(&self, user_id: Uuid) -> Result<Vec<FridgeItem>, AppError> {
        self.get_expiring_items(user_id, 3).await // Продукты, истекающие в ближайшие 3 дня
    }
}
