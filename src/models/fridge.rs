use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "fridge_category", rename_all = "lowercase")]
pub enum FridgeCategory {
    Dairy,
    Meat,
    Fish,
    Vegetables,
    Fruits,
    Grains,
    Beverages,
    Condiments,
    Snacks,
    Other,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FridgeItem {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub location: Option<String>, // "fridge", "freezer", "pantry"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFridgeItem {
    pub user_id: Uuid,
    pub name: String,
    pub brand: Option<String>,
    pub quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFridgeItem {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub quantity: Option<f32>,
    pub unit: Option<String>,
    pub category: Option<FridgeCategory>,
    pub expiry_date: Option<Option<DateTime<Utc>>>,
    pub notes: Option<Option<String>>,
    pub location: Option<Option<String>>,
}

impl FridgeItem {
    pub fn is_expired(&self) -> bool {
        match self.expiry_date {
            Some(expiry) => expiry < Utc::now(),
            None => false,
        }
    }

    pub fn days_until_expiry(&self) -> Option<i32> {
        self.expiry_date.map(|expiry| {
            let now = Utc::now();
            (expiry - now).num_days() as i32
        })
    }

    pub fn is_expiring_soon(&self, days: i32) -> bool {
        match self.days_until_expiry() {
            Some(days_left) => days_left <= days && days_left >= 0,
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FridgeStats {
    pub total_items: i32,
    pub expired_items: i32,
    pub expiring_soon: i32,
    pub categories: Vec<CategoryCount>,
    pub locations: Vec<LocationCount>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryCount {
    pub category: FridgeCategory,
    pub count: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LocationCount {
    pub location: String,
    pub count: i32,
}
