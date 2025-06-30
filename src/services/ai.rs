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
                category: crate::models::recipe::RecipeCategory::Dinner,
                difficulty: crate::models::recipe::DifficultyLevel::Easy,
                prep_time_minutes: max_prep_time.or(Some(30)),
                cook_time_minutes: Some(20),
                servings: servings.or(Some(4)),
                instructions: format!("Mock instructions for {} using ingredients: {}", description, available_ingredients.join(", ")),
                ingredients: vec![],
                tags: vec!["AI-generated".to_string()],
                nutrition_per_serving: None,
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

pub struct GeneratedRecipe {
    pub name: String,
    pub description: String,
    pub category: crate::models::recipe::RecipeCategory,
    pub difficulty: crate::models::recipe::DifficultyLevel,
    pub prep_time_minutes: Option<i32>,
    pub cook_time_minutes: Option<i32>,
    pub servings: Option<i32>,
    pub instructions: String,
    pub ingredients: Vec<crate::api::recipes::CreateRecipeIngredientRequest>,
    pub tags: Vec<String>,
    pub nutrition_per_serving: Option<crate::api::recipes::NutritionInfoRequest>,
}
