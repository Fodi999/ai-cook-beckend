use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::utils::errors::AppError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
pub struct OpenAIRequest {
    pub model: String,
    pub messages: Vec<AiMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct GroqRequest {
    pub model: String,
    pub messages: Vec<AiMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Serialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Serialize)]
pub struct GeminiRequest {
    pub contents: Vec<GeminiContent>,
    #[serde(rename = "generationConfig")]
    pub generation_config: Option<GeminiGenerationConfig>,
}

#[derive(Debug, Serialize)]
pub struct GeminiGenerationConfig {
    #[serde(rename = "maxOutputTokens")]
    pub max_output_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiResponseContent,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponseContent {
    pub parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponsePart {
    pub text: String,
}

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Deserialize)]
pub struct AiChoice {
    pub message: AiMessage,
    pub finish_reason: String,
}

#[derive(Debug, Deserialize)]
pub struct AiResponse {
    pub choices: Vec<AiChoice>,
}

#[derive(Debug, Clone)]
pub enum AiProvider {
    OpenAI(String),
    Groq(String),
    Gemini(String),
    Mock,
}

#[derive(Debug, Clone)]
pub struct AiService {
    client: Client,
    provider: AiProvider,
}

impl AiService {
    pub fn new(provider: AiProvider) -> Self {
        Self {
            client: Client::new(),
            provider,
        }
    }

    pub fn from_env() -> Self {
        if let Ok(gemini_key) = std::env::var("GEMINI_API_KEY") {
            Self::new(AiProvider::Gemini(gemini_key))
        } else if let Ok(groq_key) = std::env::var("GROQ_API_KEY") {
            Self::new(AiProvider::Groq(groq_key))
        } else if let Ok(openai_key) = std::env::var("OPENAI_API_KEY") {
            Self::new(AiProvider::OpenAI(openai_key))
        } else {
            Self::new(AiProvider::Mock)
        }
    }

    /// Генерация общего ответа от ИИ (для чата)
    pub async fn generate_response(&self, prompt: &str) -> Result<String, AppError> {
        match &self.provider {
            AiProvider::Mock => {
                Ok("Это тестовый ответ от ИИ-помощника. В реальном режиме здесь будет ответ от Gemini API.".to_string())
            },
            AiProvider::Gemini(api_key) => {
                self.call_gemini_api(prompt, api_key, Some(1000)).await
            },
            AiProvider::Groq(api_key) => {
                self.call_groq_api(prompt, api_key, Some(1000)).await
            },
            AiProvider::OpenAI(api_key) => {
                self.call_openai_api(prompt, api_key, Some(1000)).await
            }
        }
    }

    pub async fn generate_recipe_suggestions(&self, items: Vec<crate::models::fridge::FridgeItem>) -> Result<Vec<crate::api::fridge::RecipeSuggestion>, AppError> {
        let ingredient_names: Vec<String> = items.iter().map(|item| item.name.clone()).collect();
        
        match &self.provider {
            AiProvider::Mock => {
                return Ok(vec![
                    crate::api::fridge::RecipeSuggestion {
                        recipe_name: "Mock Recipe 1".to_string(),
                        ingredients_available: ingredient_names.clone(),
                        ingredients_needed: vec!["Salt".to_string(), "Pepper".to_string()],
                        preparation_time: Some(30),
                        difficulty: Some("Easy".to_string()),
                        instructions: Some(format!("A delicious recipe using {}", ingredient_names.join(", "))),
                        ai_generated: true,
                    },
                    crate::api::fridge::RecipeSuggestion {
                        recipe_name: "Mock Recipe 2".to_string(),
                        ingredients_available: ingredient_names.clone(),
                        ingredients_needed: vec!["Oil".to_string(), "Garlic".to_string()],
                        preparation_time: Some(45),
                        difficulty: Some("Medium".to_string()),
                        instructions: Some(format!("Another great recipe with {}", ingredient_names.join(", "))),
                        ai_generated: true,
                    },
                ]);
            },
            AiProvider::Gemini(api_key) => {
                let prompt = format!(
                    "Given these ingredients: {}, suggest 2-3 simple recipes. For each recipe provide: name, ingredients needed (beyond what's available), preparation time, difficulty level, and brief instructions.",
                    ingredient_names.join(", ")
                );
                
                let response = self.call_gemini_api(&prompt, api_key, Some(500)).await?;
                
                // Simple fallback if parsing fails
                Ok(vec![
                    crate::api::fridge::RecipeSuggestion {
                        recipe_name: "Gemini Generated Recipe".to_string(),
                        ingredients_available: ingredient_names.clone(),
                        ingredients_needed: vec!["Check AI response".to_string()],
                        preparation_time: Some(30),
                        difficulty: Some("Medium".to_string()),
                        instructions: Some(response),
                        ai_generated: true,
                    },
                ])
            },
            AiProvider::Groq(api_key) => {
                let prompt = format!(
                    "Given these ingredients: {}, suggest 2-3 simple recipes. For each recipe provide: name, ingredients needed (beyond what's available), preparation time, difficulty level, and brief instructions. Format as JSON.",
                    ingredient_names.join(", ")
                );
                
                let response = self.call_groq_api(&prompt, api_key, Some(500)).await?;
                
                // Simple fallback if parsing fails
                Ok(vec![
                    crate::api::fridge::RecipeSuggestion {
                        recipe_name: "AI Generated Recipe".to_string(),
                        ingredients_available: ingredient_names.clone(),
                        ingredients_needed: vec!["Check AI response".to_string()],
                        preparation_time: Some(30),
                        difficulty: Some("Medium".to_string()),
                        instructions: Some(response),
                        ai_generated: true,
                    },
                ])
            },
            AiProvider::OpenAI(api_key) => {
                let prompt = format!(
                    "Given these ingredients: {}, suggest 2-3 simple recipes. For each recipe provide: name, ingredients needed (beyond what's available), preparation time, difficulty level, and brief instructions.",
                    ingredient_names.join(", ")
                );
                
                let response = self.call_openai_api(&prompt, api_key, Some(500)).await?;
                
                // Simple fallback if parsing fails
                Ok(vec![
                    crate::api::fridge::RecipeSuggestion {
                        recipe_name: "AI Generated Recipe".to_string(),
                        ingredients_available: ingredient_names.clone(),
                        ingredients_needed: vec!["Check AI response".to_string()],
                        preparation_time: Some(30),
                        difficulty: Some("Medium".to_string()),
                        instructions: Some(response),
                        ai_generated: true,
                    },
                ])
            }
        }
    }

    async fn call_groq_api(&self, prompt: &str, api_key: &str, max_tokens: Option<u32>) -> Result<String, AppError> {
        let request = GroqRequest {
            model: "llama-3.1-8b-instant".to_string(), // Free Groq model
            messages: vec![
                AiMessage {
                    role: "system".to_string(),
                    content: "You are a helpful cooking assistant. Provide practical, easy-to-follow recipes.".to_string(),
                },
                AiMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            max_tokens,
            temperature: Some(0.7),
        };

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Groq API request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ExternalService(format!(
                "Groq API returned status: {}",
                response.status()
            )));
        }

        let ai_response: AiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse Groq response: {}", e)))?;

        ai_response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| AppError::ExternalService("No response from Groq".to_string()))
    }

    async fn call_openai_api(&self, prompt: &str, api_key: &str, max_tokens: Option<u32>) -> Result<String, AppError> {
        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                AiMessage {
                    role: "system".to_string(),
                    content: "You are a helpful cooking assistant. Provide practical, easy-to-follow recipes.".to_string(),
                },
                AiMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                },
            ],
            max_tokens,
            temperature: Some(0.7),
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI API request failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::ExternalService(format!(
                "OpenAI API returned status: {}",
                response.status()
            )));
        }

        let ai_response: AiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse OpenAI response: {}", e)))?;

        ai_response
            .choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .ok_or_else(|| AppError::ExternalService("No response from OpenAI".to_string()))
    }

    pub async fn generate_recipe(
        &self,
        description: &str,
        available_ingredients: Vec<String>,
        _dietary_restrictions: Vec<String>,
        max_prep_time: Option<i32>,
        servings: Option<i32>,
    ) -> Result<GeneratedRecipe, AppError> {
        if let AiProvider::Mock = &self.provider {
            return Ok(GeneratedRecipe {
                name: format!("Generated Recipe: {}", description),
                description: format!("A recipe based on: {}", description),
                difficulty: "Easy".to_string(),
                cook_time: "20 minutes".to_string(),
                servings: servings.unwrap_or(4) as u8,
                instructions: vec![format!("Mock instructions for {} using ingredients: {}", description, available_ingredients.join(", "))],
                ingredients: vec![],
                available_ingredients,
                missing_ingredients: vec!["Salt".to_string(), "Pepper".to_string()],
            });
        }

        // TODO: Implement real AI integration (OpenAI / Groq)
        Err(AppError::InternalServerError("AI integration not implemented".to_string()))
    }

    async fn call_openai(&self, prompt: &str, max_tokens: Option<u32>) -> Result<String, AppError> {
        let api_key = if let AiProvider::OpenAI(key) = &self.provider {
            key
        } else {
            return Err(AppError::InternalServerError("OpenAI API key not configured".to_string()));
        };

        let request = OpenAIRequest {
            model: "gpt-3.5-turbo".to_string(),
            messages: vec![
                AiMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            max_tokens,
            temperature: Some(0.7),
        };

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("OpenAI API request failed: {}", e)))?;

        let openai_response: AiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse OpenAI response: {}", e)))?;

        let content = openai_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "No response generated".to_string());

        Ok(content)
    }

    async fn call_groq(&self, prompt: &str, max_tokens: Option<u32>) -> Result<String, AppError> {
        let api_key = if let AiProvider::Groq(key) = &self.provider {
            key
        } else {
            return Err(AppError::InternalServerError("Groq API key not configured".to_string()));
        };

        let request = GroqRequest {
            model: "groq-model".to_string(),
            messages: vec![
                AiMessage {
                    role: "user".to_string(),
                    content: prompt.to_string(),
                }
            ],
            max_tokens,
            temperature: Some(0.7),
        };

        let response = self.client
            .post("https://api.groq.com/v1/ai/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Groq API request failed: {}", e)))?;

        let groq_response: AiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse Groq response: {}", e)))?;

        let content = groq_response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .unwrap_or_else(|| "No response generated".to_string());

        Ok(content)
    }

    async fn call_gemini_api(&self, prompt: &str, api_key: &str, max_tokens: Option<u32>) -> Result<String, AppError> {
        let request = GeminiRequest {
            contents: vec![
                GeminiContent {
                    parts: vec![
                        GeminiPart {
                            text: format!("You are a helpful cooking assistant. Provide practical, easy-to-follow recipes. {}", prompt),
                        }
                    ],
                }
            ],
            generation_config: Some(GeminiGenerationConfig {
                max_output_tokens: max_tokens,
                temperature: Some(0.7),
            }),
        };

        let url = format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}", api_key);
        
        let response = self
            .client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| AppError::ExternalService(format!("Gemini API request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::ExternalService(format!(
                "Gemini API returned status: {}, error: {}",
                status,
                error_text
            )));
        }

        let gemini_response: GeminiResponse = response
            .json()
            .await
            .map_err(|e| AppError::ExternalService(format!("Failed to parse Gemini response: {}", e)))?;

        gemini_response
            .candidates
            .into_iter()
            .next()
            .and_then(|candidate| candidate.content.parts.into_iter().next())
            .map(|part| part.text)
            .ok_or_else(|| AppError::ExternalService("No response from Gemini".to_string()))
    }
}

// =============================================================================
// ИНТЕГРАЦИЯ С УМНЫМ ХОЛОДИЛЬНИКОМ
// =============================================================================

use uuid::Uuid;
use crate::{
    models::fridge::{FridgeItem, FoodWaste, Allergen, Intolerance, DietType, ExpenseAnalytics},
    services::fridge::FridgeService,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FridgeAnalysisRequest {
    pub analysis_type: FridgeAnalysisType,
    pub include_recipes: Option<bool>,
    pub dietary_restrictions: Option<Vec<DietaryRestriction>>,
    pub max_recipes: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FridgeAnalysisType {
    FullReport,      // Полный отчет о состоянии холодильника
    RecipeSuggestions, // Только рецепты на основе продуктов
    ExpiryAlert,     // Уведомления о просроченных продуктах
    ShoppingSuggestions, // Предложения для покупок
    WasteAnalysis,   // Анализ пищевых отходов
    DietaryCheck,    // Проверка соответствия диете
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DietaryRestriction {
    pub allergens: Vec<Allergen>,
    pub intolerances: Vec<Intolerance>,
    pub diets: Vec<DietType>,
}

#[derive(Debug, Serialize)]
pub struct FridgeContext {
    pub items: Vec<FridgeItem>,
    pub expiring_items: Vec<FridgeItem>,
    pub recent_waste: Vec<FoodWaste>,
    pub expense_analytics: Option<ExpenseAnalytics>,
    pub user_preferences: Option<DietaryRestriction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SmartFridgeResponse {
    pub analysis_type: FridgeAnalysisType,
    pub summary: String,
    pub recommendations: Vec<String>,
    pub recipes: Option<Vec<GeneratedRecipe>>,
    pub alerts: Vec<FridgeAlert>,
    pub insights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneratedRecipe {
    pub name: String,
    pub description: String,
    pub ingredients: Vec<RecipeIngredient>,
    pub instructions: Vec<String>,
    pub cook_time: String,
    pub servings: u8,
    pub difficulty: String,
    pub available_ingredients: Vec<String>, // Что есть в холодильнике
    pub missing_ingredients: Vec<String>,   // Что нужно докупить
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecipeIngredient {
    pub name: String,
    pub amount: String,
    pub unit: String,
    pub available_in_fridge: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FridgeAlert {
    pub alert_type: AlertType,
    pub message: String,
    pub item_name: Option<String>,
    pub urgency: AlertUrgency,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertType {
    Expiring,     // Продукт скоро испортится
    Expired,      // Продукт уже просрочен
    LowStock,     // Мало продукта
    WastePattern, // Частые отходы этого продукта
    DietViolation, // Продукт не соответствует диете
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertUrgency {
    Critical, // Критично (просрочка, аллергия)
    High,     // Высокая (скоро просрочка)
    Medium,   // Средняя (рекомендация)
    Low,      // Низкая (информация)
}

impl AiService {
    /// Анализ холодильника с ИИ-помощником
    pub async fn analyze_fridge(
        &self,
        user_id: Uuid,
        request: FridgeAnalysisRequest,
        fridge_service: &FridgeService,
    ) -> Result<SmartFridgeResponse, AppError> {
        // Собираем данные о холодильнике
        let fridge_context = self.gather_fridge_context(user_id, fridge_service).await?;
        
        // Генерируем prompt для ИИ
        let prompt = self.build_fridge_analysis_prompt(&request, &fridge_context)?;
        
        // Получаем ответ от ИИ
        let ai_response = self.generate_response(&prompt).await?;
        
        // Парсим и структурируем ответ
        self.parse_fridge_analysis(ai_response, request.analysis_type, &fridge_context).await
    }

    /// Генерация рецептов на основе содержимого холодильника
    pub async fn generate_recipes_from_fridge(
        &self,
        user_id: Uuid,
        max_recipes: Option<u8>,
        dietary_restrictions: Option<DietaryRestriction>,
        fridge_service: &FridgeService,
    ) -> Result<Vec<GeneratedRecipe>, AppError> {
        let fridge_context = self.gather_fridge_context(user_id, fridge_service).await?;
        
        let request = FridgeAnalysisRequest {
            analysis_type: FridgeAnalysisType::RecipeSuggestions,
            include_recipes: Some(true),
            dietary_restrictions: dietary_restrictions.map(|dr| vec![dr]),
            max_recipes,
        };
        
        let response = self.analyze_fridge(user_id, request, fridge_service).await?;
        Ok(response.recipes.unwrap_or_default())
    }

    /// Создание отчета о состоянии холодильника
    pub async fn create_fridge_report(
        &self,
        user_id: Uuid,
        fridge_service: &FridgeService,
    ) -> Result<SmartFridgeResponse, AppError> {
        let request = FridgeAnalysisRequest {
            analysis_type: FridgeAnalysisType::FullReport,
            include_recipes: Some(true),
            dietary_restrictions: None,
            max_recipes: Some(3),
        };
        
        self.analyze_fridge(user_id, request, fridge_service).await
    }

    /// Анализ пищевых отходов с рекомендациями
    pub async fn analyze_food_waste(
        &self,
        user_id: Uuid,
        fridge_service: &FridgeService,
    ) -> Result<SmartFridgeResponse, AppError> {
        let request = FridgeAnalysisRequest {
            analysis_type: FridgeAnalysisType::WasteAnalysis,
            include_recipes: Some(false),
            dietary_restrictions: None,
            max_recipes: None,
        };
        
        self.analyze_fridge(user_id, request, fridge_service).await
    }

    /// Собираем контекст о холодильнике для ИИ
    async fn gather_fridge_context(
        &self,
        user_id: Uuid,
        fridge_service: &FridgeService,
    ) -> Result<FridgeContext, AppError> {
        // Получаем все продукты пользователя
        let items = fridge_service.get_user_items(user_id, None, None, None).await?;
        
        // Получаем продукты, которые скоро истекут
        let expiring_items = fridge_service.get_expiring_items(user_id, Some(7)).await?;
        
        // Получаем недавние отходы (за последнюю неделю)
        let now = chrono::Utc::now();
        let week_ago = now - chrono::Duration::weeks(1);
        let recent_waste = fridge_service.get_waste_history(user_id, Some(week_ago), Some(now)).await?;
        
        // Получаем аналитику расходов
        let expense_analytics = fridge_service.get_expense_analytics(user_id, "month").await.ok();
        
        Ok(FridgeContext {
            items,
            expiring_items,
            recent_waste,
            expense_analytics,
            user_preferences: None, // TODO: Получать из профиля пользователя
        })
    }

    /// Создаем prompt для анализа холодильника
    fn build_fridge_analysis_prompt(
        &self,
        request: &FridgeAnalysisRequest,
        context: &FridgeContext,
    ) -> Result<String, AppError> {
        let mut prompt = String::new();
        
        // Базовая информация о роли ИИ
        prompt.push_str("Ты - умный помощник по питанию и управлению холодильником. ");
        prompt.push_str("Анализируй данные холодильника и предоставляй персонализированные рекомендации.\n\n");
        
        // Добавляем информацию о содержимом холодильника
        prompt.push_str("СОДЕРЖИМОЕ ХОЛОДИЛЬНИКА:\n");
        for item in &context.items {
            prompt.push_str(&format!(
                "- {} ({}): {:.1} {}, категория: {:?}",
                item.name,
                item.brand.as_ref().unwrap_or(&"без бренда".to_string()),
                item.quantity,
                item.unit,
                item.category
            ));
            
            if let Some(expiry) = item.expiry_date {
                let days_left = (expiry - chrono::Utc::now()).num_days();
                if days_left <= 7 {
                    prompt.push_str(&format!(" (истекает через {} дн.)", days_left));
                }
            }
            
            if !item.contains_allergens.is_empty() {
                prompt.push_str(&format!(" [Аллергены: {:?}]", item.contains_allergens));
            }
            
            if !item.suitable_for_diets.is_empty() {
                prompt.push_str(&format!(" [Диеты: {:?}]", item.suitable_for_diets));
            }
            
            prompt.push_str("\n");
        }
        
        // Добавляем информацию о недавних отходах
        if !context.recent_waste.is_empty() {
            prompt.push_str("\nНЕДАВНИЕ ПИЩЕВЫЕ ОТХОДЫ:\n");
            for waste in &context.recent_waste {
                prompt.push_str(&format!(
                    "- {} ({:.1} {}): причина - {:?}\n",
                    waste.name, waste.wasted_quantity, waste.unit, waste.waste_reason
                ));
            }
        }
        
        // Добавляем аналитику расходов
        if let Some(analytics) = &context.expense_analytics {
            prompt.push_str(&format!(
                "\nАНАЛИТИКА ЗА МЕСЯЦ:\n- Потрачено: {:.2} руб.\n- Выброшено: {:.2} руб.\n- Процент отходов: {:.1}%\n",
                analytics.total_purchased,
                analytics.total_wasted,
                analytics.waste_percentage
            ));
        }
        
        // Добавляем специфичные инструкции в зависимости от типа анализа
        match request.analysis_type {
            FridgeAnalysisType::FullReport => {
                prompt.push_str("\nСОЗДАЙ ПОЛНЫЙ ОТЧЕТ:\n");
                prompt.push_str("1. Общее состояние холодильника\n");
                prompt.push_str("2. Продукты, требующие внимания\n");
                prompt.push_str("3. Рекомендации по использованию\n");
                prompt.push_str("4. 2-3 рецепта из имеющихся продуктов\n");
                prompt.push_str("5. Советы по оптимизации\n");
            },
            FridgeAnalysisType::RecipeSuggestions => {
                let max_recipes = request.max_recipes.unwrap_or(5);
                prompt.push_str(&format!("\nПРЕДЛОЖИ {} РЕЦЕПТОВ:\n", max_recipes));
                prompt.push_str("Используй максимально продукты из холодильника.\n");
                prompt.push_str("Для каждого рецепта укажи:\n");
                prompt.push_str("- Название и описание\n");
                prompt.push_str("- Ингредиенты (есть в холодильнике / нужно купить)\n");
                prompt.push_str("- Пошаговые инструкции\n");
                prompt.push_str("- Время приготовления и сложность\n");
            },
            FridgeAnalysisType::ExpiryAlert => {
                prompt.push_str("\nАНАЛИЗ СРОКОВ ГОДНОСТИ:\n");
                prompt.push_str("Проанализируй продукты на предмет скорого истечения срока годности.\n");
                prompt.push_str("Дай рекомендации по приоритетному использованию.\n");
            },
            FridgeAnalysisType::WasteAnalysis => {
                prompt.push_str("\nАНАЛИЗ ПИЩЕВЫХ ОТХОДОВ:\n");
                prompt.push_str("Проанализируй паттерны отходов и дай рекомендации по их уменьшению.\n");
                prompt.push_str("Предложи стратегии более эффективного использования продуктов.\n");
            },
            FridgeAnalysisType::ShoppingSuggestions => {
                prompt.push_str("\nРЕКОМЕНДАЦИИ ДЛЯ ПОКУПОК:\n");
                prompt.push_str("На основе текущего содержимого предложи, что стоит докупить.\n");
                prompt.push_str("Учитывай баланс категорий и отсутствующие основные продукты.\n");
            },
            FridgeAnalysisType::DietaryCheck => {
                prompt.push_str("\nПРОВЕРКА ДИЕТИЧЕСКИХ ОГРАНИЧЕНИЙ:\n");
                prompt.push_str("Проанализируй продукты на соответствие диетическим требованиям.\n");
            },
        }
        
        // Добавляем диетические ограничения
        if let Some(restrictions) = &request.dietary_restrictions {
            prompt.push_str("\nДИЕТИЧЕСКИЕ ОГРАНИЧЕНИЯ:\n");
            for restriction in restrictions {
                if !restriction.allergens.is_empty() {
                    prompt.push_str(&format!("- Аллергии: {:?}\n", restriction.allergens));
                }
                if !restriction.intolerances.is_empty() {
                    prompt.push_str(&format!("- Непереносимости: {:?}\n", restriction.intolerances));
                }
                if !restriction.diets.is_empty() {
                    prompt.push_str(&format!("- Диеты: {:?}\n", restriction.diets));
                }
            }
        }
        
        prompt.push_str("\nОТВЕЧАЙ НА РУССКОМ ЯЗЫКЕ. Будь конкретным и практичным в рекомендациях.");
        
        Ok(prompt)
    }

    /// Парсим ответ ИИ и структурируем его
    async fn parse_fridge_analysis(
        &self,
        ai_response: String,
        analysis_type: FridgeAnalysisType,
        context: &FridgeContext,
    ) -> Result<SmartFridgeResponse, AppError> {
        // В реальной реализации здесь был бы более сложный парсинг
        // Для демонстрации создаем базовую структуру
        
        let mut alerts = Vec::new();
        let mut recommendations = Vec::new();
        let mut insights = Vec::new();
        
        // Анализируем просрочку
        for item in &context.expiring_items {
            if let Some(expiry) = item.expiry_date {
                let days_left = (expiry - chrono::Utc::now()).num_days();
                let urgency = if days_left <= 1 {
                    AlertUrgency::Critical
                } else if days_left <= 3 {
                    AlertUrgency::High
                } else {
                    AlertUrgency::Medium
                };
                
                alerts.push(FridgeAlert {
                    alert_type: AlertType::Expiring,
                    message: format!("{} истекает через {} дн.", item.name, days_left),
                    item_name: Some(item.name.clone()),
                    urgency,
                });
            }
        }
        
        // Добавляем базовые рекомендации
        if !context.items.is_empty() {
            recommendations.push("Используйте продукты с ближайшим сроком годности в первую очередь".to_string());
        }
        
        if !context.recent_waste.is_empty() {
            insights.push(format!("За неделю выброшено {} продуктов", context.recent_waste.len()));
        }
        
        // Генерируем рецепты для соответствующих типов анализа
        let recipes = match analysis_type {
            FridgeAnalysisType::RecipeSuggestions | FridgeAnalysisType::FullReport => {
                Some(self.generate_mock_recipes(&context.items))
            },
            _ => None,
        };
        
        Ok(SmartFridgeResponse {
            analysis_type,
            summary: ai_response,
            recommendations,
            recipes,
            alerts,
            insights,
        })
    }

    /// Генерируем mock-рецепты для демонстрации
    fn generate_mock_recipes(&self, items: &[FridgeItem]) -> Vec<GeneratedRecipe> {
        let mut recipes = Vec::new();
        
        // Простая логика: если есть основные ингредиенты, предлагаем рецепт
        let has_protein = items.iter().any(|item| matches!(item.category, crate::models::fridge::FridgeCategory::Meat | crate::models::fridge::FridgeCategory::Fish));
        let has_vegetables = items.iter().any(|item| matches!(item.category, crate::models::fridge::FridgeCategory::Vegetables));
        let has_grains = items.iter().any(|item| matches!(item.category, crate::models::fridge::FridgeCategory::Grains));
        
        if has_protein && has_vegetables {
            let available_ingredients: Vec<String> = items.iter()
                .filter(|item| matches!(item.category, crate::models::fridge::FridgeCategory::Meat | crate::models::fridge::FridgeCategory::Fish | crate::models::fridge::FridgeCategory::Vegetables))
                .map(|item| item.name.clone())
                .take(4)
                .collect();
                
            recipes.push(GeneratedRecipe {
                name: "Жареное мясо с овощами".to_string(),
                description: "Быстрое и питательное блюдо из имеющихся продуктов".to_string(),
                ingredients: vec![
                    RecipeIngredient {
                        name: "Мясо".to_string(),
                        amount: "200".to_string(),
                        unit: "г".to_string(),
                        available_in_fridge: true,
                    },
                    RecipeIngredient {
                        name: "Овощи".to_string(),
                        amount: "300".to_string(),
                        unit: "г".to_string(),
                        available_in_fridge: true,
                    },
                ],
                instructions: vec![
                    "Нарежьте мясо кусочками".to_string(),
                    "Обжарьте мясо на сковороде 5-7 минут".to_string(),
                    "Добавьте нарезанные овощи".to_string(),
                    "Готовьте еще 10-15 минут до готовности".to_string(),
                ],
                cook_time: "20 минут".to_string(),
                servings: 2,
                difficulty: "Легко".to_string(),
                available_ingredients,
                missing_ingredients: vec!["Растительное масло".to_string(), "Специи".to_string()],
            });
        }
        
        if has_grains && has_vegetables {
            let available_ingredients: Vec<String> = items.iter()
                .filter(|item| matches!(item.category, crate::models::fridge::FridgeCategory::Grains | crate::models::fridge::FridgeCategory::Vegetables))
                .map(|item| item.name.clone())
                .take(3)
                .collect();
                
            recipes.push(GeneratedRecipe {
                name: "Овощной плов".to_string(),
                description: "Вегетарианское блюдо из круп и овощей".to_string(),
                ingredients: vec![
                    RecipeIngredient {
                        name: "Рис".to_string(),
                        amount: "200".to_string(),
                        unit: "г".to_string(),
                        available_in_fridge: true,
                    },
                    RecipeIngredient {
                        name: "Овощи".to_string(),
                        amount: "250".to_string(),
                        unit: "г".to_string(),
                        available_in_fridge: true,
                    },
                ],
                instructions: vec![
                    "Промойте рис до чистой воды".to_string(),
                    "Обжарьте овощи в казане".to_string(),
                    "Добавьте рис и залейте водой".to_string(),
                    "Варите 20 минут под крышкой".to_string(),
                ],
                cook_time: "30 минут".to_string(),
                servings: 3,
                difficulty: "Средне".to_string(),
                available_ingredients,
                missing_ingredients: vec!["Лук".to_string(), "Морковь".to_string()],
            });
        }
        
        recipes
    }
}
