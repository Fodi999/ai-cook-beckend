use uuid::Uuid;
use chrono::Utc;
use std::fmt;
use crate::{
    models::recipe::{CreateRecipe, RecipeCategory, DifficultyLevel},
    api::recipes::{RecipeResponse, RecipeIngredientResponse, NutritionInfoResponse, CreateRecipeIngredientRequest, NutritionInfoRequest},
    utils::errors::AppError,
};

// Display implementations for enums
impl fmt::Display for RecipeCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RecipeCategory::Breakfast => write!(f, "breakfast"),
            RecipeCategory::Lunch => write!(f, "lunch"),
            RecipeCategory::Dinner => write!(f, "dinner"),
            RecipeCategory::Snack => write!(f, "snack"),
            RecipeCategory::Dessert => write!(f, "dessert"),
            RecipeCategory::Appetizer => write!(f, "appetizer"),
            RecipeCategory::Beverage => write!(f, "beverage"),
            RecipeCategory::Other => write!(f, "other"),
        }
    }
}

impl fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DifficultyLevel::Easy => write!(f, "easy"),
            DifficultyLevel::Medium => write!(f, "medium"),
            DifficultyLevel::Hard => write!(f, "hard"),
        }
    }
}

pub struct RecipeService {
    pool: crate::db::DbPool,
}

impl RecipeService {
    pub fn new(pool: crate::db::DbPool) -> Self {
        Self { pool }
    }

    pub async fn create_recipe(
        &self, 
        recipe: CreateRecipe, 
        ingredients: Vec<CreateRecipeIngredientRequest>, 
        nutrition: Option<NutritionInfoRequest>
    ) -> Result<RecipeResponse, AppError> {
        // Mock implementation - in production, this would use actual database operations
        let recipe_id = Uuid::new_v4();
        
        Ok(RecipeResponse {
            id: recipe_id,
            name: recipe.name,
            description: recipe.description,
            category: recipe.category,
            difficulty: recipe.difficulty,
            prep_time_minutes: recipe.prep_time_minutes,
            cook_time_minutes: recipe.cook_time_minutes,
            total_time_minutes: match (recipe.prep_time_minutes, recipe.cook_time_minutes) {
                (Some(prep), Some(cook)) => Some(prep + cook),
                (Some(prep), None) => Some(prep),
                (None, Some(cook)) => Some(cook),
                (None, None) => None,
            },
            servings: recipe.servings,
            instructions: recipe.instructions,
            ingredients: ingredients.into_iter().map(|ing| RecipeIngredientResponse {
                name: ing.name,
                quantity: ing.quantity,
                unit: ing.unit,
                notes: ing.notes,
            }).collect(),
            tags: recipe.tags,
            image_url: recipe.image_url,
            source_url: recipe.source_url,
            nutrition_per_serving: nutrition.map(|n| NutritionInfoResponse {
                calories: n.calories,
                protein: n.protein,
                fat: n.fat,
                carbs: n.carbs,
                fiber: n.fiber,
                sugar: n.sugar,
                sodium: n.sodium,
            }),
            average_rating: Some(0.0),
            ratings_count: 0,
            is_favorite: false,
            created_by: recipe.created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn get_recipes(
        &self,
        user_id: Option<Uuid>,
        _category: Option<RecipeCategory>,
        _difficulty: Option<DifficultyLevel>,
        _max_prep_time: Option<i32>,
        _max_cook_time: Option<i32>,
        _search: Option<String>,
        _tags: Option<String>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RecipeResponse>, AppError> {
        // Mock implementation - return sample recipes
        self.get_mock_recipes(user_id, limit, offset).await
    }

    pub async fn get_recipe_by_id(&self, id: Uuid, user_id: Option<Uuid>) -> Result<RecipeResponse, AppError> {
        // Mock implementation
        self.get_mock_recipe(id, user_id).await
    }

    pub async fn update_recipe(
        &self,
        id: Uuid,
        user_id: Uuid,
        payload: crate::api::recipes::CreateRecipeRequest,
    ) -> Result<RecipeResponse, AppError> {
        // Mock implementation - in production, verify ownership and update database
        Ok(RecipeResponse {
            id,
            name: payload.name,
            description: payload.description,
            category: payload.category,
            difficulty: payload.difficulty,
            prep_time_minutes: payload.prep_time_minutes,
            cook_time_minutes: payload.cook_time_minutes,
            total_time_minutes: match (payload.prep_time_minutes, payload.cook_time_minutes) {
                (Some(prep), Some(cook)) => Some(prep + cook),
                (Some(prep), None) => Some(prep),
                (None, Some(cook)) => Some(cook),
                (None, None) => None,
            },
            servings: payload.servings,
            instructions: payload.instructions,
            ingredients: payload.ingredients.into_iter().map(|ing| RecipeIngredientResponse {
                name: ing.name,
                quantity: ing.quantity,
                unit: ing.unit,
                notes: ing.notes,
            }).collect(),
            tags: payload.tags,
            image_url: payload.image_url,
            source_url: payload.source_url,
            nutrition_per_serving: payload.nutrition_per_serving.map(|n| NutritionInfoResponse {
                calories: n.calories,
                protein: n.protein,
                fat: n.fat,
                carbs: n.carbs,
                fiber: n.fiber,
                sugar: n.sugar,
                sodium: n.sodium,
            }),
            average_rating: Some(4.2),
            ratings_count: 15,
            is_favorite: true,
            created_by: user_id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    pub async fn delete_recipe(&self, _id: Uuid, _user_id: Uuid) -> Result<(), AppError> {
        // Mock implementation - in production, verify ownership and delete from database
        Ok(())
    }

    pub async fn toggle_favorite(&self, _recipe_id: Uuid, _user_id: Uuid) -> Result<bool, AppError> {
        // Mock implementation - in production, check if already favorited and toggle
        Ok(true) // Return true indicating it's now favorited
    }

    pub async fn rate_recipe(
        &self,
        _recipe_id: Uuid,
        _user_id: Uuid,
        rating: i32,
        _comment: Option<String>,
    ) -> Result<(), AppError> {
        if !(1..=5).contains(&rating) {
            return Err(AppError::BadRequest("Rating must be between 1 and 5".to_string()));
        }
        
        // Mock implementation - in production, upsert rating in database
        Ok(())
    }

    pub async fn search_recipes(
        &self,
        query: String,
        user_id: Option<Uuid>,
        category: Option<RecipeCategory>,
        difficulty: Option<DifficultyLevel>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<RecipeResponse>, AppError> {
        self.get_recipes(
            user_id,
            category,
            difficulty,
            None,
            None,
            Some(query),
            None,
            limit,
            offset,
        ).await
    }

    pub async fn get_popular_recipes(&self, user_id: Option<Uuid>) -> Result<Vec<RecipeResponse>, AppError> {
        self.get_mock_recipes(user_id, 10, 0).await
    }

    pub async fn get_favorite_recipes(&self, user_id: Uuid) -> Result<Vec<RecipeResponse>, AppError> {
        self.get_mock_recipes(Some(user_id), 20, 0).await
    }

    // Mock implementations for testing without database
    async fn get_mock_recipe(&self, id: Uuid, user_id: Option<Uuid>) -> Result<RecipeResponse, AppError> {
        Ok(RecipeResponse {
            id,
            name: "Mock Chicken Pasta".to_string(),
            description: Some("Delicious pasta with chicken and vegetables".to_string()),
            category: RecipeCategory::Dinner,
            difficulty: DifficultyLevel::Medium,
            prep_time_minutes: Some(20),
            cook_time_minutes: Some(30),
            total_time_minutes: Some(50),
            servings: Some(4),
            instructions: "1. Cook pasta\n2. Cook chicken\n3. Mix together".to_string(),
            ingredients: vec![
                RecipeIngredientResponse {
                    name: "Pasta".to_string(),
                    quantity: 300.0,
                    unit: "g".to_string(),
                    notes: None,
                },
                RecipeIngredientResponse {
                    name: "Chicken breast".to_string(),
                    quantity: 500.0,
                    unit: "g".to_string(),
                    notes: Some("Cut into pieces".to_string()),
                },
            ],
            tags: vec!["pasta".to_string(), "chicken".to_string(), "easy".to_string()],
            image_url: Some("https://example.com/image.jpg".to_string()),
            source_url: None,
            nutrition_per_serving: Some(NutritionInfoResponse {
                calories: Some(450.0),
                protein: Some(35.0),
                fat: Some(12.0),
                carbs: Some(55.0),
                fiber: Some(3.0),
                sugar: Some(5.0),
                sodium: Some(800.0),
            }),
            average_rating: Some(4.5),
            ratings_count: 23,
            is_favorite: user_id.is_some(),
            created_by: user_id.unwrap_or_else(Uuid::new_v4),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }

    async fn get_mock_recipes(&self, user_id: Option<Uuid>, limit: i64, offset: i64) -> Result<Vec<RecipeResponse>, AppError> {
        let mut recipes = vec![];
        
        // Generate different mock recipes
        for i in 0..std::cmp::min(limit, 10) {
            let recipe_id = Uuid::new_v4();
            let recipe = RecipeResponse {
                id: recipe_id,
                name: format!("Mock Recipe {}", i + 1),
                description: Some(format!("Description for recipe {}", i + 1)),
                category: match i % 4 {
                    0 => RecipeCategory::Breakfast,
                    1 => RecipeCategory::Lunch,
                    2 => RecipeCategory::Dinner,
                    _ => RecipeCategory::Snack,
                },
                difficulty: match i % 3 {
                    0 => DifficultyLevel::Easy,
                    1 => DifficultyLevel::Medium,
                    _ => DifficultyLevel::Hard,
                },
                prep_time_minutes: Some(10 + (i as i32 * 5)),
                cook_time_minutes: Some(20 + (i as i32 * 10)),
                total_time_minutes: Some(30 + (i as i32 * 15)),
                servings: Some(2 + (i as i32)),
                instructions: format!("Instructions for recipe {}", i + 1),
                ingredients: vec![
                    RecipeIngredientResponse {
                        name: format!("Ingredient {}", i + 1),
                        quantity: 100.0 + (i as f32 * 50.0),
                        unit: "g".to_string(),
                        notes: None,
                    },
                ],
                tags: vec![format!("tag{}", i + 1)],
                image_url: Some(format!("https://example.com/image{}.jpg", i + 1)),
                source_url: None,
                nutrition_per_serving: Some(NutritionInfoResponse {
                    calories: Some(300.0 + (i as f32 * 50.0)),
                    protein: Some(20.0 + (i as f32 * 5.0)),
                    fat: Some(10.0 + (i as f32 * 2.0)),
                    carbs: Some(40.0 + (i as f32 * 10.0)),
                    fiber: Some(5.0),
                    sugar: Some(8.0),
                    sodium: Some(600.0),
                }),
                average_rating: Some(3.0 + (i as f32 * 0.5)),
                ratings_count: (i as i32 + 1) * 3,
                is_favorite: i % 2 == 0,
                created_by: user_id.unwrap_or_else(Uuid::new_v4),
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };
            recipes.push(recipe);
        }
        
        let start = offset as usize;
        let end = std::cmp::min(start + limit as usize, recipes.len());
        
        if start >= recipes.len() {
            Ok(vec![])
        } else {
            Ok(recipes[start..end].to_vec())
        }
    }
}
