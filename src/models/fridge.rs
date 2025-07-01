use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
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
    pub price_per_unit: Option<f32>, // Цена за единицу (кг, л, шт)
    pub total_price: Option<f32>, // Общая стоимость продукта
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub location: Option<String>, // "fridge", "freezer", "pantry"
    // Новые поля для диетических ограничений
    pub contains_allergens: Vec<Allergen>, // Содержит аллергены
    pub contains_intolerances: Vec<Intolerance>, // Содержит непереносимые вещества  
    pub suitable_for_diets: Vec<DietType>, // Подходит для диет
    pub ingredients: Option<String>, // Состав продукта
    pub nutritional_info: Option<String>, // Пищевая ценность
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
    pub price_per_unit: Option<f32>,
    pub total_price: Option<f32>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub purchase_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub location: Option<String>,
    // Новые поля для диетических ограничений
    pub contains_allergens: Vec<Allergen>,
    pub contains_intolerances: Vec<Intolerance>,
    pub suitable_for_diets: Vec<DietType>,
    pub ingredients: Option<String>,
    pub nutritional_info: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateFridgeItem {
    pub name: Option<String>,
    pub brand: Option<String>,
    pub quantity: Option<f32>,
    pub unit: Option<String>,
    pub category: Option<FridgeCategory>,
    pub price_per_unit: Option<f32>,
    pub total_price: Option<f32>,
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

    // Новые методы для расчета стоимости
    pub fn calculate_total_value(&self) -> f32 {
        self.total_price.unwrap_or_else(|| {
            self.price_per_unit.map(|price| price * self.quantity).unwrap_or(0.0)
        })
    }

    pub fn calculate_waste_value(&self, wasted_quantity: f32) -> f32 {
        if self.quantity > 0.0 {
            let value_per_unit = self.calculate_total_value() / self.quantity;
            value_per_unit * wasted_quantity
        } else {
            0.0
        }
    }

    pub fn calculate_remaining_value(&self) -> f32 {
        self.calculate_total_value()
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

// Новые модели для отслеживания отходов и аналитики

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FoodWaste {
    pub id: Uuid,
    pub user_id: Uuid,
    pub original_item_id: Option<Uuid>, // Связь с оригинальным продуктом
    pub name: String,
    pub brand: Option<String>,
    pub wasted_quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub waste_reason: WasteReason,
    pub wasted_value: Option<f32>, // Стоимость выброшенного продукта
    pub waste_date: DateTime<Utc>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "waste_reason", rename_all = "lowercase")]
pub enum WasteReason {
    Expired,      // Истек срок годности
    Spoiled,      // Испортился
    Overcooked,   // Переготовлен
    NotLiked,     // Не понравился
    TooMuch,      // Слишком много приготовили
    Other,        // Другое
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateFoodWaste {
    pub user_id: Uuid,
    pub original_item_id: Option<Uuid>,
    pub name: String,
    pub brand: Option<String>,
    pub wasted_quantity: f32,
    pub unit: String,
    pub category: FridgeCategory,
    pub waste_reason: WasteReason,
    pub wasted_value: Option<f32>,
    pub notes: Option<String>,
}

// Модели для аналитики расходов и экономии
#[derive(Debug, Clone, Serialize)]
pub struct ExpenseAnalytics {
    pub period: String, // "day", "week", "month"
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_purchased: f32,   // Общая сумма купленных продуктов
    pub total_wasted: f32,      // Общая сумма выброшенных продуктов
    pub waste_percentage: f32,  // Процент отходов
    pub savings_potential: f32, // Потенциальная экономия
    pub category_breakdown: Vec<CategoryExpense>,
    pub waste_by_reason: Vec<WasteByReason>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CategoryExpense {
    pub category: FridgeCategory,
    pub purchased: f32,
    pub wasted: f32,
    pub waste_percentage: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct WasteByReason {
    pub reason: WasteReason,
    pub amount: f32,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct DailyExpense {
    pub date: String,
    pub purchased: f32,
    pub wasted: f32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EconomyInsights {
    pub total_savings_this_month: f32,
    pub avg_waste_percentage: f32,
    pub most_wasted_category: Option<FridgeCategory>,
    pub best_category: Option<FridgeCategory>, // Категория с наименьшими отходами
    pub tips: Vec<String>, // Советы по экономии
}

// Новые enum'ы для диетических ограничений и аллергий

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "allergen", rename_all = "lowercase")]
pub enum Allergen {
    Peanuts,      // Арахис
    TreeNuts,     // Орехи
    Milk,         // Молочные продукты
    Eggs,         // Яйца
    Fish,         // Рыба
    Shellfish,    // Морепродукты
    Soy,          // Соя
    Wheat,        // Пшеница
    Sesame,       // Кунжут
    Sulfites,     // Сульфиты
    Celery,       // Сельдерей
    Mustard,      // Горчица
    Lupin,        // Люпин
    Molluscs,     // Моллюски
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "intolerance", rename_all = "lowercase")]
pub enum Intolerance {
    Lactose,      // Лактоза
    Gluten,       // Глютен
    Fructose,     // Фруктоза
    Histamine,    // Гистамин
    Sorbitol,     // Сорбитол
    Sucrose,      // Сахароза
    FODMAP,       // FODMAP
    Caffeine,     // Кофеин
    Alcohol,      // Алкоголь
    Tyramine,     // Тирамин
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Eq, Hash)]
#[sqlx(type_name = "diet_type", rename_all = "lowercase")]
pub enum DietType {
    Vegan,        // Веганская
    Vegetarian,   // Вегетарианская
    GlutenFree,   // Безглютеновая
    DairyFree,    // Безмолочная
    Keto,         // Кето
    Paleo,        // Палео
    Mediterranean,// Средиземноморская
    LowCarb,      // Низкоуглеводная
    LowFat,       // Низкожировая
    Halal,        // Халяль
    Kosher,       // Кошерная
    Raw,          // Сыроедение
    Pescatarian,  // Пескетарианская
    Flexitarian,  // Флекситарианская
}

// Модели для работы с диетическими ограничениями

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DietaryProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub allergies: Vec<Allergen>,
    pub intolerances: Vec<Intolerance>, 
    pub diets: Vec<DietType>,
    pub custom_restrictions: Vec<String>, // Дополнительные ограничения от пользователя
    pub severity_notes: Option<String>, // Заметки о серьезности ограничений
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CreateDietaryProfile {
    pub user_id: Uuid,
    pub allergies: Vec<Allergen>,
    pub intolerances: Vec<Intolerance>,
    pub diets: Vec<DietType>,
    pub custom_restrictions: Vec<String>,
    pub severity_notes: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateDietaryProfile {
    pub allergies: Option<Vec<Allergen>>,
    pub intolerances: Option<Vec<Intolerance>>,
    pub diets: Option<Vec<DietType>>,
    pub custom_restrictions: Option<Vec<String>>,
    pub severity_notes: Option<String>,
}

// Модель для анализа совместимости продуктов с диетой

#[derive(Debug, Clone, Serialize)]
pub struct DietaryCompatibility {
    pub item_id: Uuid,
    pub item_name: String,
    pub is_safe: bool,
    pub compatibility_score: f32, // 0.0 - 1.0
    pub warnings: Vec<DietaryWarning>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DietaryWarning {
    pub warning_type: DietaryWarningType,
    pub severity: WarningSeverity,
    pub message: String,
    pub affected_restriction: String, // Какое ограничение нарушается
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DietaryWarningType {
    Allergy,      // Аллерген
    Intolerance,  // Непереносимость
    DietViolation,// Нарушение диеты
    CrossContamination, // Перекрестное загрязнение
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WarningSeverity {
    Critical,     // Критично (аллергия)
    High,         // Высокая (серьезная непереносимость)
    Medium,       // Средняя (нарушение диеты)
    Low,          // Низкая (предупреждение)
}

// Модель для анализа холодильника на соответствие диете

#[derive(Debug, Clone, Serialize)]
pub struct FridgeComplianceReport {
    pub user_id: Uuid,
    pub analysis_date: DateTime<Utc>,
    pub total_items: usize,
    pub safe_items: usize,
    pub problematic_items: usize,
    pub compliance_percentage: f32,
    pub item_analyses: Vec<DietaryCompatibility>,
    pub overall_recommendations: Vec<String>,
    pub shopping_suggestions: Vec<String>, // Что купить вместо проблемных продуктов
}

// Модель для умных рекомендаций продуктов

#[derive(Debug, Clone, Serialize)]
pub struct SmartFoodSuggestion {
    pub category: FridgeCategory,
    pub suggested_items: Vec<SuggestedItem>,
    pub reasoning: String, // Почему эти продукты рекомендуются
}

#[derive(Debug, Clone, Serialize)]
pub struct SuggestedItem {
    pub name: String,
    pub brand_suggestions: Vec<String>,
    pub why_suitable: String,
    pub estimated_price_range: Option<(f32, f32)>,
    pub nutritional_benefits: Vec<String>,
}
