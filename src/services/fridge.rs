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

        // Mock implementation for compilation without database
        Ok(FridgeItem {
            id: item_id,
            user_id: item_data.user_id,
            name: item_data.name,
            brand: item_data.brand,
            quantity: item_data.quantity,
            unit: item_data.unit,
            category: item_data.category,
            expiry_date: item_data.expiry_date,
            purchase_date: item_data.purchase_date,
            notes: item_data.notes,
            location: item_data.location,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_items(&self, _user_id: Uuid, _category: Option<FridgeCategory>, _location: Option<String>, _search: Option<String>) -> Result<Vec<FridgeItem>, AppError> {
        // Mock implementation
        Ok(vec![])
    }

    pub async fn get_item_by_id(&self, _id: Uuid, _user_id: Uuid) -> Result<FridgeItem, AppError> {
        // Mock implementation
        Err(AppError::NotFound("Item not found".to_string()))
    }

    pub async fn update_item(&self, _id: Uuid, _user_id: Uuid, _payload: crate::api::fridge::CreateFridgeItemRequest) -> Result<FridgeItem, AppError> {
        // Mock implementation
        Err(AppError::InternalServerError("Not implemented".to_string()))
    }

    pub async fn remove_item(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation
        Ok(())
    }

    pub async fn get_expiring_items(&self, user_id: Uuid, days_ahead: Option<u32>) -> Result<Vec<FridgeItem>, AppError> {
        let _days = days_ahead.unwrap_or(7);
        
        // Mock implementation - в реальности это будет SQL запрос
        // SELECT * FROM fridge_items 
        // WHERE user_id = $1 
        // AND expiry_date IS NOT NULL 
        // AND expiry_date <= NOW() + INTERVAL '{days} days'
        // ORDER BY expiry_date ASC
        
        let now = Utc::now();
        let mock_items = vec![
            FridgeItem {
                id: Uuid::new_v4(),
                user_id,
                name: "Молоко".to_string(),
                brand: Some("Простоквашино".to_string()),
                quantity: 1.0,
                unit: "л".to_string(),
                category: FridgeCategory::Dairy,
                expiry_date: Some(now + chrono::Duration::days(2)),
                purchase_date: now - chrono::Duration::days(5),
                notes: Some("Скоро истекает".to_string()),
                location: Some("fridge".to_string()),
                created_at: now,
                updated_at: now,
            },
            FridgeItem {
                id: Uuid::new_v4(),
                user_id,
                name: "Йогурт".to_string(),
                brand: Some("Danone".to_string()),
                quantity: 4.0,
                unit: "шт".to_string(),
                category: FridgeCategory::Dairy,
                expiry_date: Some(now + chrono::Duration::days(1)),
                purchase_date: now - chrono::Duration::days(3),
                notes: None,
                location: Some("fridge".to_string()),
                created_at: now,
                updated_at: now,
            },
        ];
        
        Ok(mock_items)
    }

    pub async fn check_and_notify_expiring_items(&self, user_id: Uuid, realtime_service: &crate::services::realtime::RealtimeService) -> Result<(), AppError> {
        let expiring_items = self.get_expiring_items(user_id, Some(3)).await?;
        
        if !expiring_items.is_empty() {
            let notification_items: Vec<crate::services::realtime::ExpiringItem> = expiring_items
                .into_iter()
                .filter_map(|item| {
                    item.expiry_date.map(|exp_date| {
                        let days_left = (exp_date.date_naive() - Utc::now().date_naive()).num_days();
                        crate::services::realtime::ExpiringItem {
                            id: item.id,
                            name: item.name,
                            days_left: days_left.max(0) as u32,
                        }
                    })
                })
                .collect();

            realtime_service.notify_expiring_items(user_id, notification_items).await?;
        }
        
        Ok(())
    }
}
