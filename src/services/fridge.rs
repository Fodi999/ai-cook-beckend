use uuid::Uuid;
use chrono::Utc;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use crate::{
    models::fridge::{FridgeItem, CreateFridgeItem, FridgeCategory, FoodWaste, CreateFoodWaste, ExpenseAnalytics, EconomyInsights, CategoryExpense, WasteByReason, WasteReason},
    utils::errors::AppError,
};

// Глобальное хранилище для mock данных
static MOCK_STORAGE: Lazy<Arc<Mutex<HashMap<Uuid, Vec<FridgeItem>>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

// Глобальное хранилище для отходов
static WASTE_STORAGE: Lazy<Arc<Mutex<HashMap<Uuid, Vec<FoodWaste>>>>> = 
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
            price_per_unit: item_data.price_per_unit,
            total_price: item_data.total_price,
            expiry_date: item_data.expiry_date,
            purchase_date: item_data.purchase_date,
            notes: item_data.notes,
            location: item_data.location,
            // Новые поля для диетических ограничений
            contains_allergens: item_data.contains_allergens,
            contains_intolerances: item_data.contains_intolerances,
            suitable_for_diets: item_data.suitable_for_diets,
            ingredients: item_data.ingredients,
            nutritional_info: item_data.nutritional_info,
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
            price_per_unit: payload.price_per_unit,
            total_price: payload.total_price,
            expiry_date: payload.expiry_date,
            purchase_date: old_item.purchase_date, // Оставляем оригинальную дату покупки
            notes: payload.notes,
            location: payload.location,
            // Новые поля для диетических ограничений
            contains_allergens: payload.contains_allergens.unwrap_or_default(),
            contains_intolerances: payload.contains_intolerances.unwrap_or_default(),
            suitable_for_diets: payload.suitable_for_diets.unwrap_or_default(),
            ingredients: payload.ingredients,
            nutritional_info: payload.nutritional_info,
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

    // Новые методы для работы с отходами и аналитикой
    pub async fn add_waste(&self, waste_data: CreateFoodWaste) -> Result<FoodWaste, AppError> {
        let waste_id = Uuid::new_v4();
        let now = Utc::now();

        let waste = FoodWaste {
            id: waste_id,
            user_id: waste_data.user_id,
            original_item_id: waste_data.original_item_id,
            name: waste_data.name,
            brand: waste_data.brand,
            wasted_quantity: waste_data.wasted_quantity,
            unit: waste_data.unit,
            category: waste_data.category,
            waste_reason: waste_data.waste_reason,
            wasted_value: waste_data.wasted_value,
            waste_date: now,
            notes: waste_data.notes,
            created_at: now,
        };

        // Сохраняем в mock хранилище отходов
        let mut storage = WASTE_STORAGE.lock().unwrap();
        let user_waste = storage.entry(waste_data.user_id).or_insert_with(Vec::new);
        user_waste.push(waste.clone());

        Ok(waste)
    }

    pub async fn get_waste_history(&self, user_id: Uuid, start_date: Option<chrono::DateTime<Utc>>, end_date: Option<chrono::DateTime<Utc>>) -> Result<Vec<FoodWaste>, AppError> {
        let storage = WASTE_STORAGE.lock().unwrap();
        let user_waste = storage.get(&user_id).cloned().unwrap_or_default();

        let filtered_waste: Vec<FoodWaste> = user_waste
            .into_iter()
            .filter(|waste| {
                if let Some(start) = start_date {
                    if waste.waste_date < start {
                        return false;
                    }
                }
                if let Some(end) = end_date {
                    if waste.waste_date > end {
                        return false;
                    }
                }
                true
            })
            .collect();

        Ok(filtered_waste)
    }

    pub async fn get_expense_analytics(&self, user_id: Uuid, period: &str) -> Result<ExpenseAnalytics, AppError> {
        let now = Utc::now();
        let (start_date, end_date) = match period {
            "day" => (now - chrono::Duration::days(1), now),
            "week" => (now - chrono::Duration::weeks(1), now),
            "month" => (now - chrono::Duration::days(30), now),
            _ => (now - chrono::Duration::weeks(1), now),
        };

        // Получаем продукты за период
        let storage = MOCK_STORAGE.lock().unwrap();
        let user_items = storage.get(&user_id).cloned().unwrap_or_default();
        
        let items_in_period: Vec<&FridgeItem> = user_items
            .iter()
            .filter(|item| item.purchase_date >= start_date && item.purchase_date <= end_date)
            .collect();

        // Получаем отходы за период
        let waste_storage = WASTE_STORAGE.lock().unwrap();
        let user_waste = waste_storage.get(&user_id).cloned().unwrap_or_default();
        
        let waste_in_period: Vec<&FoodWaste> = user_waste
            .iter()
            .filter(|waste| waste.waste_date >= start_date && waste.waste_date <= end_date)
            .collect();

        // Рассчитываем аналитику
        let total_purchased: f32 = items_in_period.iter()
            .map(|item| item.calculate_total_value())
            .sum();

        let total_wasted: f32 = waste_in_period.iter()
            .map(|waste| waste.wasted_value.unwrap_or(0.0))
            .sum();

        let waste_percentage = if total_purchased > 0.0 {
            (total_wasted / total_purchased) * 100.0
        } else {
            0.0
        };

        let savings_potential = total_wasted;

        // Группируем по категориям
        let mut category_map: HashMap<FridgeCategory, (f32, f32)> = HashMap::new();
        
        for item in &items_in_period {
            let entry = category_map.entry(item.category.clone()).or_insert((0.0, 0.0));
            entry.0 += item.calculate_total_value();
        }

        for waste in &waste_in_period {
            let entry = category_map.entry(waste.category.clone()).or_insert((0.0, 0.0));
            entry.1 += waste.wasted_value.unwrap_or(0.0);
        }

        let category_breakdown: Vec<CategoryExpense> = category_map
            .into_iter()
            .map(|(category, (purchased, wasted))| {
                let waste_percentage = if purchased > 0.0 {
                    (wasted / purchased) * 100.0
                } else {
                    0.0
                };
                CategoryExpense {
                    category,
                    purchased,
                    wasted,
                    waste_percentage,
                }
            })
            .collect();

        // Группируем отходы по причинам
        let mut reason_map: HashMap<WasteReason, f32> = HashMap::new();
        for waste in &waste_in_period {
            let entry = reason_map.entry(waste.waste_reason.clone()).or_insert(0.0);
            *entry += waste.wasted_value.unwrap_or(0.0);
        }

        let waste_by_reason: Vec<WasteByReason> = reason_map
            .into_iter()
            .map(|(reason, amount)| {
                let percentage = if total_wasted > 0.0 {
                    (amount / total_wasted) * 100.0
                } else {
                    0.0
                };
                WasteByReason {
                    reason,
                    amount,
                    percentage,
                }
            })
            .collect();

        Ok(ExpenseAnalytics {
            period: period.to_string(),
            start_date,
            end_date,
            total_purchased,
            total_wasted,
            waste_percentage,
            savings_potential,
            category_breakdown,
            waste_by_reason,
        })
    }

    pub async fn get_economy_insights(&self, user_id: Uuid) -> Result<EconomyInsights, AppError> {
        // Получаем аналитику за месяц
        let analytics = self.get_expense_analytics(user_id, "month").await?;
        
        // Находим категорию с наибольшими отходами
        let most_wasted_category = analytics.category_breakdown
            .iter()
            .max_by(|a, b| a.wasted.partial_cmp(&b.wasted).unwrap_or(std::cmp::Ordering::Equal))
            .map(|c| c.category.clone());

        // Находим категорию с наименьшими отходами (лучшую)
        let best_category = analytics.category_breakdown
            .iter()
            .filter(|c| c.purchased > 0.0)
            .min_by(|a, b| a.waste_percentage.partial_cmp(&b.waste_percentage).unwrap_or(std::cmp::Ordering::Equal))
            .map(|c| c.category.clone());

        // Генерируем советы
        let mut tips = Vec::new();
        
        if analytics.waste_percentage > 20.0 {
            tips.push("Попробуйте покупать меньше продуктов за раз".to_string());
        }
        
        if let Some(ref category) = most_wasted_category {
            tips.push(format!("Обратите внимание на хранение продуктов категории {:?}", category));
        }
        
        if analytics.waste_percentage < 10.0 {
            tips.push("Отличная работа! Вы эффективно используете продукты".to_string());
        }

        tips.push("Проверяйте сроки годности при покупке".to_string());
        tips.push("Планируйте меню заранее".to_string());

        Ok(EconomyInsights {
            total_savings_this_month: analytics.total_purchased - analytics.total_wasted,
            avg_waste_percentage: analytics.waste_percentage,
            most_wasted_category,
            best_category,
            tips,
        })
    }
}
