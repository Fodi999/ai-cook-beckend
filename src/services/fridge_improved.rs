use uuid::Uuid;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::{
    models::fridge::{FridgeItem, CreateFridgeItem, FridgeCategory},
    utils::errors::AppError,
};

// Глобальное хранилище для mock данных
static MOCK_STORAGE: Lazy<Arc<Mutex<HashMap<Uuid, Vec<FridgeItem>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

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

        let item = FridgeItem {
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
        };

        // Сохраняем в mock хранилище
        let mut storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.entry(item_data.user_id).or_insert_with(Vec::new);
        user_items.push(item.clone());

        Ok(item)
    }

    pub async fn get_user_items(&self, user_id: Uuid, category: Option<FridgeCategory>, location: Option<String>, search: Option<String>) -> Result<Vec<FridgeItem>, AppError> {
        let storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.get(&user_id).cloned().unwrap_or_default();

        // Фильтруем по категории
        let filtered_items: Vec<FridgeItem> = user_items
            .into_iter()
            .filter(|item| {
                // Фильтр по категории
                if let Some(ref cat) = category {
                    if &item.category != cat {
                        return false;
                    }
                }

                // Фильтр по локации
                if let Some(ref loc) = location {
                    if item.location.as_ref() != Some(loc) {
                        return false;
                    }
                }

                // Фильтр по поиску
                if let Some(ref search_term) = search {
                    let search_lower = search_term.to_lowercase();
                    let name_matches = item.name.to_lowercase().contains(&search_lower);
                    let brand_matches = item.brand.as_ref()
                        .map(|b| b.to_lowercase().contains(&search_lower))
                        .unwrap_or(false);
                    
                    if !name_matches && !brand_matches {
                        return false;
                    }
                }

                true
            })
            .collect();

        Ok(filtered_items)
    }

    pub async fn get_item_by_id(&self, id: Uuid, user_id: Uuid) -> Result<FridgeItem, AppError> {
        let storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.get(&user_id).cloned().unwrap_or_default();

        user_items
            .into_iter()
            .find(|item| item.id == id)
            .ok_or_else(|| AppError::NotFound("Item not found".to_string()))
    }

    pub async fn update_item(&self, id: Uuid, user_id: Uuid, payload: crate::api::fridge::CreateFridgeItemRequest) -> Result<FridgeItem, AppError> {
        let mut storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.entry(user_id).or_insert_with(Vec::new);

        let item_index = user_items
            .iter()
            .position(|item| item.id == id)
            .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;

        let now = Utc::now();
        let old_item = &user_items[item_index];

        let updated_item = FridgeItem {
            id: old_item.id,
            user_id: old_item.user_id,
            name: payload.name,
            brand: payload.brand,
            quantity: payload.quantity,
            unit: payload.unit,
            category: payload.category,
            expiry_date: payload.expiry_date,
            purchase_date: old_item.purchase_date, // Оставляем оригинальную дату покупки
            notes: payload.notes,
            location: payload.location,
            created_at: old_item.created_at,
            updated_at: now,
        };

        user_items[item_index] = updated_item.clone();

        Ok(updated_item)
    }

    pub async fn remove_item(&self, id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        let mut storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.entry(user_id).or_insert_with(Vec::new);

        let item_index = user_items
            .iter()
            .position(|item| item.id == id)
            .ok_or_else(|| AppError::NotFound("Item not found".to_string()))?;

        user_items.remove(item_index);

        Ok(())
    }

    pub async fn get_expiring_items(&self, user_id: Uuid, days_ahead: Option<u32>) -> Result<Vec<FridgeItem>, AppError> {
        let days = days_ahead.unwrap_or(7);
        let now = Utc::now();
        let future_date = now + chrono::Duration::days(days as i64);

        let storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.get(&user_id).cloned().unwrap_or_default();

        let expiring_items: Vec<FridgeItem> = user_items
            .into_iter()
            .filter(|item| {
                if let Some(expiry_date) = item.expiry_date {
                    expiry_date >= now && expiry_date <= future_date
                } else {
                    false
                }
            })
            .collect();

        Ok(expiring_items)
    }

    pub async fn check_and_notify_expiring_items(&self, user_id: Uuid) -> Result<Vec<FridgeItem>, AppError> {
        self.get_expiring_items(user_id, Some(3)).await // Продукты, истекающие в ближайшие 3 дня
    }
}
