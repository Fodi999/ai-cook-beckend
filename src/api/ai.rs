use axum::{
    extract::{State, Json},
    http::StatusCode,
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use crate::services::ai::AiService;
use crate::utils::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct AiChatRequest {
    pub message: String,
    pub context: Option<String>, // Контекст пользователя (цели, предпочтения и т.д.)
}

#[derive(Debug, Serialize)]
pub struct AiChatResponse {
    pub response: String,
    pub suggestions: Option<Vec<String>>, // Дополнительные предложения
}

/// Обработчик для общения с ИИ-помощником
pub async fn chat_with_ai(
    State(ai_service): State<AiService>,
    Json(request): Json<AiChatRequest>,
) -> Result<ResponseJson<AiChatResponse>, AppError> {
    // Формируем контекстный промпт
    let context_prompt = if let Some(context) = &request.context {
        format!(
            "Контекст пользователя: {}. Вопрос пользователя: {}",
            context,
            request.message
        )
    } else {
        format!(
            "Ты - ИИ помощник в кулинарном приложении IT Cook. Помогай пользователям с рецептами, советами по готовке, планированию питания и достижению целей. Вопрос: {}",
            request.message
        )
    };

    // Получаем ответ от ИИ
    let ai_response = ai_service.generate_response(&context_prompt).await?;
    
    // Генерируем дополнительные предложения на основе ответа
    let suggestions = generate_suggestions(&request.message, &ai_response);

    Ok(ResponseJson(AiChatResponse {
        response: ai_response,
        suggestions: Some(suggestions),
    }))
}

/// Генерирует предложения для продолжения разговора
fn generate_suggestions(user_message: &str, ai_response: &str) -> Vec<String> {
    let user_lower = user_message.to_lowercase();
    
    if user_lower.contains("рецепт") || user_lower.contains("готовить") {
        vec![
            "Покажи пошаговые инструкции".to_string(),
            "Какие альтернативные ингредиенты можно использовать?".to_string(),
            "Сколько времени займет приготовление?".to_string(),
        ]
    } else if user_lower.contains("диета") || user_lower.contains("похудеть") {
        vec![
            "Составь план питания на неделю".to_string(),
            "Какие продукты лучше исключить?".to_string(),
            "Как контролировать калории?".to_string(),
        ]
    } else if user_lower.contains("продукт") || user_lower.contains("ингредиент") {
        vec![
            "Что можно приготовить из этих продуктов?".to_string(),
            "Как долго хранятся эти продукты?".to_string(),
            "Чем можно заменить этот ингредиент?".to_string(),
        ]
    } else {
        vec![
            "Расскажи больше об этом".to_string(),
            "Покажи похожие рецепты".to_string(),
            "Дай еще один совет".to_string(),
        ]
    }
}

#[derive(Debug, Deserialize)]
pub struct RecipeGenerationRequest {
    pub ingredients: Vec<String>,
    pub dietary_restrictions: Option<Vec<String>>,
    pub cuisine_type: Option<String>,
    pub cooking_time: Option<i32>, // в минутах
    pub difficulty: Option<String>,
}

/// Генерация рецепта на основе ингредиентов и предпочтений
pub async fn generate_recipe(
    State(ai_service): State<AiService>,
    Json(request): Json<RecipeGenerationRequest>,
) -> Result<ResponseJson<AiChatResponse>, AppError> {
    let mut prompt = format!(
        "Создай подробный рецепт используя эти ингредиенты: {}",
        request.ingredients.join(", ")
    );

    if let Some(restrictions) = &request.dietary_restrictions {
        prompt.push_str(&format!(". Учти диетические ограничения: {}", restrictions.join(", ")));
    }

    if let Some(cuisine) = &request.cuisine_type {
        prompt.push_str(&format!(". Стиль кухни: {}", cuisine));
    }

    if let Some(time) = request.cooking_time {
        prompt.push_str(&format!(". Время приготовления не более {} минут", time));
    }

    if let Some(difficulty) = &request.difficulty {
        prompt.push_str(&format!(". Уровень сложности: {}", difficulty));
    }

    prompt.push_str(". Предоставь: название, список ингредиентов с количествами, пошаговые инструкции, время приготовления, и советы по подаче.");

    let ai_response = ai_service.generate_response(&prompt).await?;

    Ok(ResponseJson(AiChatResponse {
        response: ai_response,
        suggestions: Some(vec![
            "Изменить ингредиенты".to_string(),
            "Упростить рецепт".to_string(),
            "Добавить пищевую ценность".to_string(),
        ]),
    }))
}

#[derive(Debug, Deserialize)]
pub struct NutritionAnalysisRequest {
    pub recipe_text: String,
    pub servings: Option<i32>,
}

/// Анализ пищевой ценности рецепта
pub async fn analyze_nutrition(
    State(ai_service): State<AiService>,
    Json(request): Json<NutritionAnalysisRequest>,
) -> Result<ResponseJson<AiChatResponse>, AppError> {
    let servings = request.servings.unwrap_or(1);
    
    let prompt = format!(
        "Проанализируй пищевую ценность этого рецепта на {} порций и предоставь приблизительные данные о калориях, белках, жирах, углеводах, витаминах и минералах. Рецепт: {}",
        servings,
        request.recipe_text
    );

    let ai_response = ai_service.generate_response(&prompt).await?;

    Ok(ResponseJson(AiChatResponse {
        response: ai_response,
        suggestions: Some(vec![
            "Как снизить калорийность?".to_string(),
            "Добавить больше белка".to_string(),
            "Сделать более полезным".to_string(),
        ]),
    }))
}
