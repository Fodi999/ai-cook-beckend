use serde::{Deserialize, Serialize};
use super::fridge::{Allergen, Intolerance, DietType, FridgeCategory};
use std::collections::HashMap;

// Предустановленные данные для аллергий с детальной информацией
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllergenInfo {
    pub allergen: Allergen,
    pub name_en: String,
    pub name_ru: String,
    pub description: String,
    pub severity: String,
    pub common_sources: Vec<String>,
    pub hidden_sources: Vec<String>, // Скрытые источники аллергена
    pub cross_reactions: Vec<Allergen>, // Перекрестные реакции
}

// Предустановленные данные для непереносимостей
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntoleranceInfo {
    pub intolerance: Intolerance,
    pub name_en: String,
    pub name_ru: String,
    pub description: String,
    pub symptoms: Vec<String>,
    pub avoid_foods: Vec<String>,
    pub safe_alternatives: Vec<String>,
    pub severity_levels: Vec<String>,
}

// Предустановленные данные для диет
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DietInfo {
    pub diet: DietType,
    pub name_en: String,
    pub name_ru: String,
    pub description: String,
    pub principles: Vec<String>,
    pub allowed_foods: Vec<String>,
    pub restricted_foods: Vec<String>,
    pub health_benefits: Vec<String>,
    pub difficulty_level: String, // "Easy", "Medium", "Hard"
}

// Предустановленная информация о продуктах по категориям
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductPreset {
    pub name: String,
    pub category: FridgeCategory,
    pub common_allergens: Vec<Allergen>,
    pub common_intolerances: Vec<Intolerance>,
    pub suitable_diets: Vec<DietType>,
    pub typical_shelf_life_days: Option<i32>,
    pub storage_location: String,
    pub nutritional_highlights: Vec<String>,
}

pub struct FoodPresets;

impl FoodPresets {
    // Получить информацию обо всех аллергенах
    pub fn get_allergen_info() -> Vec<AllergenInfo> {
        vec![
            AllergenInfo {
                allergen: Allergen::Peanuts,
                name_en: "Peanuts".to_string(),
                name_ru: "Арахис".to_string(),
                description: "Один из самых распространенных и серьезных пищевых аллергенов".to_string(),
                severity: "Critical".to_string(),
                common_sources: vec![
                    "Арахисовое масло".to_string(),
                    "Арахисовая паста".to_string(),
                    "Орехи в шоколаде".to_string(),
                    "Азиатская кухня".to_string(),
                ],
                hidden_sources: vec![
                    "Растительное масло".to_string(),
                    "Lecithin".to_string(),
                    "Hydrolyzed vegetable protein".to_string(),
                    "Искусственные ароматизаторы".to_string(),
                ],
                cross_reactions: vec![Allergen::TreeNuts, Allergen::Soy],
            },
            AllergenInfo {
                allergen: Allergen::TreeNuts,
                name_en: "Tree Nuts".to_string(),
                name_ru: "Орехи".to_string(),
                description: "Включает миндаль, грецкие орехи, кешью, фисташки и другие орехи".to_string(),
                severity: "Critical".to_string(),
                common_sources: vec![
                    "Миндаль".to_string(),
                    "Грецкие орехи".to_string(),
                    "Кешью".to_string(),
                    "Фисташки".to_string(),
                    "Ореховое молоко".to_string(),
                ],
                hidden_sources: vec![
                    "Marzipan".to_string(),
                    "Artificial flavoring".to_string(),
                    "Natural flavoring".to_string(),
                    "Некоторые косметические продукты".to_string(),
                ],
                cross_reactions: vec![Allergen::Peanuts],
            },
            AllergenInfo {
                allergen: Allergen::Milk,
                name_en: "Milk".to_string(),
                name_ru: "Молочные продукты".to_string(),
                description: "Аллергия на белки коровьего молока".to_string(),
                severity: "High".to_string(),
                common_sources: vec![
                    "Молоко".to_string(),
                    "Сыр".to_string(),
                    "Йогурт".to_string(),
                    "Сливочное масло".to_string(),
                    "Мороженое".to_string(),
                ],
                hidden_sources: vec![
                    "Казеин".to_string(),
                    "Whey".to_string(),
                    "Lactose".to_string(),
                    "Некоторые лекарства".to_string(),
                ],
                cross_reactions: vec![],
            },
            AllergenInfo {
                allergen: Allergen::Eggs,
                name_en: "Eggs".to_string(),
                name_ru: "Яйца".to_string(),
                description: "Аллергия на белки куриных яиц".to_string(),
                severity: "High".to_string(),
                common_sources: vec![
                    "Куриные яйца".to_string(),
                    "Майонез".to_string(),
                    "Выпечка".to_string(),
                    "Pasta".to_string(),
                ],
                hidden_sources: vec![
                    "Lecithin".to_string(),
                    "Albumin".to_string(),
                    "Некоторые вакцины".to_string(),
                    "Artificial flavoring".to_string(),
                ],
                cross_reactions: vec![],
            },
            AllergenInfo {
                allergen: Allergen::Fish,
                name_en: "Fish".to_string(),
                name_ru: "Рыба".to_string(),
                description: "Аллергия на рыбные белки".to_string(),
                severity: "High".to_string(),
                common_sources: vec![
                    "Все виды рыб".to_string(),
                    "Рыбный соус".to_string(),
                    "Worcestershire sauce".to_string(),
                    "Caesar salad dressing".to_string(),
                ],
                hidden_sources: vec![
                    "Surimi".to_string(),
                    "Anchovies in sauces".to_string(),
                    "Некоторые витамины".to_string(),
                ],
                cross_reactions: vec![Allergen::Shellfish],
            },
            AllergenInfo {
                allergen: Allergen::Shellfish,
                name_en: "Shellfish".to_string(),
                name_ru: "Морепродукты".to_string(),
                description: "Аллергия на ракообразных и моллюсков".to_string(),
                severity: "Critical".to_string(),
                common_sources: vec![
                    "Креветки".to_string(),
                    "Крабы".to_string(),
                    "Лобстеры".to_string(),
                    "Мидии".to_string(),
                    "Устрицы".to_string(),
                ],
                hidden_sources: vec![
                    "Glucosamine".to_string(),
                    "Chitosan".to_string(),
                    "Некоторые добавки".to_string(),
                ],
                cross_reactions: vec![Allergen::Fish],
            },
            AllergenInfo {
                allergen: Allergen::Soy,
                name_en: "Soy".to_string(),
                name_ru: "Соя".to_string(),
                description: "Аллергия на соевые белки".to_string(),
                severity: "Medium".to_string(),
                common_sources: vec![
                    "Соевое молоко".to_string(),
                    "Tofu".to_string(),
                    "Соевый соус".to_string(),
                    "Edamame".to_string(),
                ],
                hidden_sources: vec![
                    "Lecithin".to_string(),
                    "Vegetable oil".to_string(),
                    "Natural flavoring".to_string(),
                    "Vitamin E".to_string(),
                ],
                cross_reactions: vec![Allergen::Peanuts],
            },
            AllergenInfo {
                allergen: Allergen::Wheat,
                name_en: "Wheat".to_string(),
                name_ru: "Пшеница".to_string(),
                description: "Аллергия на пшеничные белки (отличается от целиакии)".to_string(),
                severity: "High".to_string(),
                common_sources: vec![
                    "Хлеб".to_string(),
                    "Pasta".to_string(),
                    "Крупы".to_string(),
                    "Выпечка".to_string(),
                ],
                hidden_sources: vec![
                    "Soy sauce".to_string(),
                    "Modified food starch".to_string(),
                    "Некоторые косметические продукты".to_string(),
                ],
                cross_reactions: vec![],
            },
            AllergenInfo {
                allergen: Allergen::Sesame,
                name_en: "Sesame".to_string(),
                name_ru: "Кунжут".to_string(),
                description: "Аллергия на семена кунжута".to_string(),
                severity: "High".to_string(),
                common_sources: vec![
                    "Tahini".to_string(),
                    "Hummus".to_string(),
                    "Кунжутное масло".to_string(),
                    "Halva".to_string(),
                ],
                hidden_sources: vec![
                    "Flavoring".to_string(),
                    "Некоторые хлебы".to_string(),
                    "Косметические продукты".to_string(),
                ],
                cross_reactions: vec![],
            },
            AllergenInfo {
                allergen: Allergen::Sulfites,
                name_en: "Sulfites".to_string(),
                name_ru: "Сульфиты".to_string(),
                description: "Химические консерванты, могут вызывать аллергические реакции".to_string(),
                severity: "Medium".to_string(),
                common_sources: vec![
                    "Вино".to_string(),
                    "Сухофрукты".to_string(),
                    "Картофельные чипсы".to_string(),
                    "Креветки".to_string(),
                ],
                hidden_sources: vec![
                    "Лекарства".to_string(),
                    "Некоторые соки".to_string(),
                    "Замороженные овощи".to_string(),
                ],
                cross_reactions: vec![],
            },
        ]
    }

    // Получить информацию обо всех непереносимостях
    pub fn get_intolerance_info() -> Vec<IntoleranceInfo> {
        vec![
            IntoleranceInfo {
                intolerance: Intolerance::Lactose,
                name_en: "Lactose Intolerance".to_string(),
                name_ru: "Непереносимость лактозы".to_string(),
                description: "Неспособность переваривать молочный сахар".to_string(),
                symptoms: vec![
                    "Вздутие живота".to_string(),
                    "Диарея".to_string(),
                    "Газы".to_string(),
                    "Боли в животе".to_string(),
                ],
                avoid_foods: vec![
                    "Молоко".to_string(),
                    "Сливки".to_string(),
                    "Мороженое".to_string(),
                    "Некоторые сыры".to_string(),
                ],
                safe_alternatives: vec![
                    "Безлактозное молоко".to_string(),
                    "Растительные молочные продукты".to_string(),
                    "Твердые сыры".to_string(),
                    "Йогурт с живыми культурами".to_string(),
                ],
                severity_levels: vec!["Mild".to_string(), "Moderate".to_string(), "Severe".to_string()],
            },
            IntoleranceInfo {
                intolerance: Intolerance::Gluten,
                name_en: "Gluten Intolerance".to_string(),
                name_ru: "Непереносимость глютена".to_string(),
                description: "Неспособность переваривать глютен (может включать целиакию)".to_string(),
                symptoms: vec![
                    "Боли в животе".to_string(),
                    "Вздутие".to_string(),
                    "Диарея".to_string(),
                    "Усталость".to_string(),
                    "Головные боли".to_string(),
                ],
                avoid_foods: vec![
                    "Пшеница".to_string(),
                    "Рожь".to_string(),
                    "Ячмень".to_string(),
                    "Хлеб".to_string(),
                    "Pasta".to_string(),
                ],
                safe_alternatives: vec![
                    "Рис".to_string(),
                    "Кукуруза".to_string(),
                    "Киноа".to_string(),
                    "Гречка".to_string(),
                    "Безглютеновые продукты".to_string(),
                ],
                severity_levels: vec!["Sensitivity".to_string(), "Intolerance".to_string(), "Celiac Disease".to_string()],
            },
            IntoleranceInfo {
                intolerance: Intolerance::Fructose,
                name_en: "Fructose Intolerance".to_string(),
                name_ru: "Непереносимость фруктозы".to_string(),
                description: "Трудности с перевариванием фруктозы".to_string(),
                symptoms: vec![
                    "Боли в животе".to_string(),
                    "Вздутие".to_string(),
                    "Диарея".to_string(),
                    "Тошнота".to_string(),
                ],
                avoid_foods: vec![
                    "Яблоки".to_string(),
                    "Груши".to_string(),
                    "Мед".to_string(),
                    "Агава".to_string(),
                    "Сладкие напитки".to_string(),
                ],
                safe_alternatives: vec![
                    "Бананы".to_string(),
                    "Виноград".to_string(),
                    "Апельсины".to_string(),
                    "Глюкоза".to_string(),
                ],
                severity_levels: vec!["Mild".to_string(), "Moderate".to_string(), "Severe".to_string()],
            },
            IntoleranceInfo {
                intolerance: Intolerance::Histamine,
                name_en: "Histamine Intolerance".to_string(),
                name_ru: "Непереносимость гистамина".to_string(),
                description: "Неспособность расщеплять гистамин в организме".to_string(),
                symptoms: vec![
                    "Головные боли".to_string(),
                    "Покраснение кожи".to_string(),
                    "Заложенность носа".to_string(),
                    "Боли в животе".to_string(),
                ],
                avoid_foods: vec![
                    "Выдержанные сыры".to_string(),
                    "Вино".to_string(),
                    "Ферментированные продукты".to_string(),
                    "Копчености".to_string(),
                ],
                safe_alternatives: vec![
                    "Свежие продукты".to_string(),
                    "Молодые сыры".to_string(),
                    "Свежее мясо".to_string(),
                    "Рис".to_string(),
                ],
                severity_levels: vec!["Mild".to_string(), "Moderate".to_string(), "Severe".to_string()],
            },
            IntoleranceInfo {
                intolerance: Intolerance::FODMAP,
                name_en: "FODMAP Intolerance".to_string(),
                name_ru: "Непереносимость FODMAP".to_string(),
                description: "Непереносимость ферментируемых углеводов".to_string(),
                symptoms: vec![
                    "Синдром раздраженного кишечника".to_string(),
                    "Вздутие".to_string(),
                    "Газы".to_string(),
                    "Боли в животе".to_string(),
                ],
                avoid_foods: vec![
                    "Лук".to_string(),
                    "Чеснок".to_string(),
                    "Яблоки".to_string(),
                    "Бобовые".to_string(),
                    "Пшеница".to_string(),
                ],
                safe_alternatives: vec![
                    "Морковь".to_string(),
                    "Картофель".to_string(),
                    "Рис".to_string(),
                    "Бананы".to_string(),
                ],
                severity_levels: vec!["Mild".to_string(), "Moderate".to_string(), "Severe".to_string()],
            },
        ]
    }

    // Получить информацию обо всех диетах
    pub fn get_diet_info() -> Vec<DietInfo> {
        vec![
            DietInfo {
                diet: DietType::Vegan,
                name_en: "Vegan".to_string(),
                name_ru: "Веганская".to_string(),
                description: "Исключает все продукты животного происхождения".to_string(),
                principles: vec![
                    "Никаких продуктов животного происхождения".to_string(),
                    "Растительная пища только".to_string(),
                    "Этические соображения".to_string(),
                ],
                allowed_foods: vec![
                    "Овощи".to_string(),
                    "Фрукты".to_string(),
                    "Зерновые".to_string(),
                    "Бобовые".to_string(),
                    "Орехи и семена".to_string(),
                    "Растительные масла".to_string(),
                ],
                restricted_foods: vec![
                    "Мясо".to_string(),
                    "Рыба".to_string(),
                    "Молочные продукты".to_string(),
                    "Яйца".to_string(),
                    "Мед".to_string(),
                    "Желатин".to_string(),
                ],
                health_benefits: vec![
                    "Снижение риска сердечных заболеваний".to_string(),
                    "Контроль веса".to_string(),
                    "Улучшение пищеварения".to_string(),
                    "Снижение воспаления".to_string(),
                ],
                difficulty_level: "Medium".to_string(),
            },
            DietInfo {
                diet: DietType::Vegetarian,
                name_en: "Vegetarian".to_string(),
                name_ru: "Вегетарианская".to_string(),
                description: "Исключает мясо и рыбу, но допускает молочные продукты и яйца".to_string(),
                principles: vec![
                    "Никакого мяса и рыбы".to_string(),
                    "Молочные продукты и яйца разрешены".to_string(),
                    "Акцент на растительной пище".to_string(),
                ],
                allowed_foods: vec![
                    "Овощи".to_string(),
                    "Фрукты".to_string(),
                    "Молочные продукты".to_string(),
                    "Яйца".to_string(),
                    "Зерновые".to_string(),
                    "Бобовые".to_string(),
                ],
                restricted_foods: vec![
                    "Мясо".to_string(),
                    "Рыба".to_string(),
                    "Морепродукты".to_string(),
                    "Желатин".to_string(),
                ],
                health_benefits: vec![
                    "Снижение риска рака".to_string(),
                    "Здоровье сердца".to_string(),
                    "Контроль веса".to_string(),
                    "Долголетие".to_string(),
                ],
                difficulty_level: "Easy".to_string(),
            },
            DietInfo {
                diet: DietType::Keto,
                name_en: "Ketogenic".to_string(),
                name_ru: "Кетогенная".to_string(),
                description: "Высокожировая, низкоуглеводная диета".to_string(),
                principles: vec![
                    "70-80% жиров".to_string(),
                    "20-25% белков".to_string(),
                    "5-10% углеводов".to_string(),
                    "Кетоз".to_string(),
                ],
                allowed_foods: vec![
                    "Мясо".to_string(),
                    "Рыба".to_string(),
                    "Яйца".to_string(),
                    "Сливочное масло".to_string(),
                    "Авокадо".to_string(),
                    "Орехи".to_string(),
                    "Зеленые овощи".to_string(),
                ],
                restricted_foods: vec![
                    "Хлеб".to_string(),
                    "Pasta".to_string(),
                    "Рис".to_string(),
                    "Картофель".to_string(),
                    "Сахар".to_string(),
                    "Фрукты (большинство)".to_string(),
                ],
                health_benefits: vec![
                    "Быстрое похудение".to_string(),
                    "Улучшение метаболизма".to_string(),
                    "Ментальная ясность".to_string(),
                    "Контроль аппетита".to_string(),
                ],
                difficulty_level: "Hard".to_string(),
            },
            DietInfo {
                diet: DietType::GlutenFree,
                name_en: "Gluten-Free".to_string(),
                name_ru: "Безглютеновая".to_string(),
                description: "Исключает все продукты, содержащие глютен".to_string(),
                principles: vec![
                    "Никакого глютена".to_string(),
                    "Безопасность для целиакии".to_string(),
                    "Альтернативные зерновые".to_string(),
                ],
                allowed_foods: vec![
                    "Рис".to_string(),
                    "Кукуруза".to_string(),
                    "Киноа".to_string(),
                    "Мясо".to_string(),
                    "Рыба".to_string(),
                    "Овощи".to_string(),
                    "Фрукты".to_string(),
                ],
                restricted_foods: vec![
                    "Пшеница".to_string(),
                    "Рожь".to_string(),
                    "Ячмень".to_string(),
                    "Хлеб".to_string(),
                    "Pasta".to_string(),
                    "Пиво".to_string(),
                ],
                health_benefits: vec![
                    "Лечение целиакии".to_string(),
                    "Уменьшение воспаления".to_string(),
                    "Улучшение пищеварения".to_string(),
                    "Повышение энергии".to_string(),
                ],
                difficulty_level: "Medium".to_string(),
            },
            DietInfo {
                diet: DietType::Mediterranean,
                name_en: "Mediterranean".to_string(),
                name_ru: "Средиземноморская".to_string(),
                description: "Основана на традиционном питании средиземноморских стран".to_string(),
                principles: vec![
                    "Оливковое масло как основной жир".to_string(),
                    "Много рыбы".to_string(),
                    "Свежие овощи и фрукты".to_string(),
                    "Умеренное количество красного вина".to_string(),
                ],
                allowed_foods: vec![
                    "Рыба".to_string(),
                    "Оливковое масло".to_string(),
                    "Овощи".to_string(),
                    "Фрукты".to_string(),
                    "Орехи".to_string(),
                    "Цельные зерна".to_string(),
                    "Бобовые".to_string(),
                ],
                restricted_foods: vec![
                    "Красное мясо (ограничено)".to_string(),
                    "Обработанные продукты".to_string(),
                    "Рафинированный сахар".to_string(),
                    "Транс-жиры".to_string(),
                ],
                health_benefits: vec![
                    "Здоровье сердца".to_string(),
                    "Снижение воспаления".to_string(),
                    "Улучшение когнитивных функций".to_string(),
                    "Долголетие".to_string(),
                ],
                difficulty_level: "Easy".to_string(),
            },
            DietInfo {
                diet: DietType::Paleo,
                name_en: "Paleolithic".to_string(),
                name_ru: "Палеолитическая".to_string(),
                description: "Основана на предполагаемом рационе древних охотников-собирателей".to_string(),
                principles: vec![
                    "Только натуральные продукты".to_string(),
                    "Никаких обработанных продуктов".to_string(),
                    "Как питались наши предки".to_string(),
                ],
                allowed_foods: vec![
                    "Мясо".to_string(),
                    "Рыба".to_string(),
                    "Яйца".to_string(),
                    "Овощи".to_string(),
                    "Фрукты".to_string(),
                    "Орехи".to_string(),
                    "Семена".to_string(),
                ],
                restricted_foods: vec![
                    "Зерновые".to_string(),
                    "Бобовые".to_string(),
                    "Молочные продукты".to_string(),
                    "Сахар".to_string(),
                    "Обработанные продукты".to_string(),
                ],
                health_benefits: vec![
                    "Снижение воспаления".to_string(),
                    "Контроль веса".to_string(),
                    "Стабильный уровень сахара".to_string(),
                    "Улучшение пищеварения".to_string(),
                ],
                difficulty_level: "Medium".to_string(),
            },
        ]
    }

    // Предустановленные продукты по категориям
    pub fn get_product_presets() -> Vec<ProductPreset> {
        vec![
            // Молочные продукты
            ProductPreset {
                name: "Молоко коровье".to_string(),
                category: FridgeCategory::Dairy,
                common_allergens: vec![Allergen::Milk],
                common_intolerances: vec![Intolerance::Lactose],
                suitable_diets: vec![DietType::Vegetarian, DietType::Keto],
                typical_shelf_life_days: Some(7),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Кальций".to_string(), "Белок".to_string(), "Витамин B12".to_string()],
            },
            ProductPreset {
                name: "Сыр твердый".to_string(),
                category: FridgeCategory::Dairy,
                common_allergens: vec![Allergen::Milk],
                common_intolerances: vec![], // Твердые сыры содержат мало лактозы
                suitable_diets: vec![DietType::Vegetarian, DietType::Keto, DietType::Mediterranean],
                typical_shelf_life_days: Some(30),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Кальций".to_string(), "Белок".to_string(), "Витамин A".to_string()],
            },
            ProductPreset {
                name: "Йогурт натуральный".to_string(),
                category: FridgeCategory::Dairy,
                common_allergens: vec![Allergen::Milk],
                common_intolerances: vec![], // Живые культуры помогают с лактозой
                suitable_diets: vec![DietType::Vegetarian, DietType::Mediterranean],
                typical_shelf_life_days: Some(14),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Пробиотики".to_string(), "Белок".to_string(), "Кальций".to_string()],
            },
            
            // Мясные продукты
            ProductPreset {
                name: "Куриная грудка".to_string(),
                category: FridgeCategory::Meat,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Keto, DietType::Paleo, DietType::Mediterranean],
                typical_shelf_life_days: Some(3),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Белок".to_string(), "Ниацин".to_string(), "Селен".to_string()],
            },
            ProductPreset {
                name: "Говядина".to_string(),
                category: FridgeCategory::Meat,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Keto, DietType::Paleo],
                typical_shelf_life_days: Some(5),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Железо".to_string(), "Белок".to_string(), "Витамин B12".to_string()],
            },
            
            // Рыба
            ProductPreset {
                name: "Лосось".to_string(),
                category: FridgeCategory::Fish,
                common_allergens: vec![Allergen::Fish],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Keto, DietType::Paleo, DietType::Mediterranean, DietType::Pescatarian],
                typical_shelf_life_days: Some(2),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Омега-3".to_string(), "Белок".to_string(), "Витамин D".to_string()],
            },
            ProductPreset {
                name: "Креветки".to_string(),
                category: FridgeCategory::Fish,
                common_allergens: vec![Allergen::Shellfish],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Keto, DietType::Paleo, DietType::Mediterranean, DietType::Pescatarian],
                typical_shelf_life_days: Some(1),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Белок".to_string(), "Йод".to_string(), "Селен".to_string()],
            },
            
            // Овощи
            ProductPreset {
                name: "Брокколи".to_string(),
                category: FridgeCategory::Vegetables,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::Keto, DietType::Paleo, DietType::Mediterranean, DietType::GlutenFree],
                typical_shelf_life_days: Some(7),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Витамин C".to_string(), "Фолат".to_string(), "Клетчатка".to_string()],
            },
            ProductPreset {
                name: "Авокадо".to_string(),
                category: FridgeCategory::Vegetables,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::Keto, DietType::Paleo, DietType::Mediterranean, DietType::GlutenFree],
                typical_shelf_life_days: Some(5),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Здоровые жиры".to_string(), "Калий".to_string(), "Клетчатка".to_string()],
            },
            
            // Фрукты
            ProductPreset {
                name: "Яблоко".to_string(),
                category: FridgeCategory::Fruits,
                common_allergens: vec![],
                common_intolerances: vec![Intolerance::Fructose, Intolerance::FODMAP],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::Paleo, DietType::Mediterranean, DietType::GlutenFree],
                typical_shelf_life_days: Some(30),
                storage_location: "fridge".to_string(),
                nutritional_highlights: vec!["Клетчатка".to_string(), "Витамин C".to_string(), "Антиоксиданты".to_string()],
            },
            ProductPreset {
                name: "Банан".to_string(),
                category: FridgeCategory::Fruits,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::Paleo, DietType::Mediterranean, DietType::GlutenFree],
                typical_shelf_life_days: Some(7),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Калий".to_string(), "Витамин B6".to_string(), "Энергия".to_string()],
            },
            
            // Зерновые
            ProductPreset {
                name: "Рис белый".to_string(),
                category: FridgeCategory::Grains,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::GlutenFree],
                typical_shelf_life_days: Some(365),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Углеводы".to_string(), "Энергия".to_string()],
            },
            ProductPreset {
                name: "Хлеб пшеничный".to_string(),
                category: FridgeCategory::Grains,
                common_allergens: vec![Allergen::Wheat],
                common_intolerances: vec![Intolerance::Gluten],
                suitable_diets: vec![DietType::Vegetarian],
                typical_shelf_life_days: Some(5),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Углеводы".to_string(), "Клетчатка".to_string(), "B витамины".to_string()],
            },
            ProductPreset {
                name: "Киноа".to_string(),
                category: FridgeCategory::Grains,
                common_allergens: vec![],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::GlutenFree, DietType::Paleo],
                typical_shelf_life_days: Some(365),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Полный белок".to_string(), "Клетчатка".to_string(), "Железо".to_string()],
            },
            
            // Орехи и семена
            ProductPreset {
                name: "Миндаль".to_string(),
                category: FridgeCategory::Snacks,
                common_allergens: vec![Allergen::TreeNuts],
                common_intolerances: vec![],
                suitable_diets: vec![DietType::Vegan, DietType::Vegetarian, DietType::Keto, DietType::Paleo, DietType::Mediterranean, DietType::GlutenFree],
                typical_shelf_life_days: Some(365),
                storage_location: "pantry".to_string(),
                nutritional_highlights: vec!["Здоровые жиры".to_string(), "Витамин E".to_string(), "Магний".to_string()],
            },
        ]
    }

    // Получить информацию о продукте по имени
    pub fn get_product_info(product_name: &str) -> Option<ProductPreset> {
        Self::get_product_presets()
            .into_iter()
            .find(|preset| preset.name.to_lowercase().contains(&product_name.to_lowercase()))
    }

    // Получить продукты, подходящие для определенной диеты
    pub fn get_products_for_diet(diet: &DietType) -> Vec<ProductPreset> {
        Self::get_product_presets()
            .into_iter()
            .filter(|preset| preset.suitable_diets.contains(diet))
            .collect()
    }

    // Получить продукты, не содержащие определенный аллерген
    pub fn get_products_without_allergen(allergen: &Allergen) -> Vec<ProductPreset> {
        Self::get_product_presets()
            .into_iter()
            .filter(|preset| !preset.common_allergens.contains(allergen))
            .collect()
    }

    // Получить продукты, не содержащие определенную непереносимость
    pub fn get_products_without_intolerance(intolerance: &Intolerance) -> Vec<ProductPreset> {
        Self::get_product_presets()
            .into_iter()
            .filter(|preset| !preset.common_intolerances.contains(intolerance))
            .collect()
    }

    // Получить все доступные аллергены для автозаполнения
    pub fn get_all_allergens() -> Vec<Allergen> {
        vec![
            Allergen::Peanuts,
            Allergen::TreeNuts,
            Allergen::Milk,
            Allergen::Eggs,
            Allergen::Fish,
            Allergen::Shellfish,
            Allergen::Soy,
            Allergen::Wheat,
            Allergen::Sesame,
            Allergen::Sulfites,
            Allergen::Celery,
            Allergen::Mustard,
            Allergen::Lupin,
            Allergen::Molluscs,
        ]
    }

    // Получить все доступные непереносимости для автозаполнения
    pub fn get_all_intolerances() -> Vec<Intolerance> {
        vec![
            Intolerance::Lactose,
            Intolerance::Gluten,
            Intolerance::Fructose,
            Intolerance::Histamine,
            Intolerance::Sorbitol,
            Intolerance::Sucrose,
            Intolerance::FODMAP,
            Intolerance::Caffeine,
            Intolerance::Alcohol,
            Intolerance::Tyramine,
        ]
    }

    // Получить все доступные типы диет для автозаполнения
    pub fn get_all_diets() -> Vec<DietType> {
        vec![
            DietType::Vegan,
            DietType::Vegetarian,
            DietType::GlutenFree,
            DietType::DairyFree,
            DietType::Keto,
            DietType::Paleo,
            DietType::Mediterranean,
            DietType::LowCarb,
            DietType::LowFat,
            DietType::Halal,
            DietType::Kosher,
            DietType::Raw,
            DietType::Pescatarian,
            DietType::Flexitarian,
        ]
    }
}
