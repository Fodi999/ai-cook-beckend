use axum::{
    extract::{State, Json, Path},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

use crate::services::personal_health_assistant::{PersonalHealthAssistant, HealthContext, UserHealthSummary, NutritionSummary, PersonalizedResponse};
use crate::services::ai::AiService;
use crate::models::health::*;
use crate::utils::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct PersonalChatRequest {
    pub message: String,
    pub include_health_context: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct WellbeingCheckRequest {
    pub mood_score: Option<i32>,
    pub energy_level: Option<i32>,
    pub stress_level: Option<i32>,
    pub sleep_hours: Option<f32>,
    pub sleep_quality: Option<i32>,
    pub water_intake_ml: Option<i32>,
    pub exercise_minutes: Option<i32>,
    pub notes: Option<String>,
    pub symptoms: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct HealthDashboardResponse {
    pub current_wellbeing: Option<DailyWellbeing>,
    pub insights: Vec<HealthInsight>,
    pub recommendations: Vec<PersonalizedRecommendation>,
    pub weekly_trends: WeeklyTrends,
    pub motivational_message: String,
}

#[derive(Debug, Serialize)]
pub struct WeeklyTrends {
    pub avg_mood: f32,
    pub avg_energy: f32,
    pub avg_stress: f32,
    pub avg_sleep: f32,
    pub total_water_ml: i32,
    pub total_exercise_minutes: i32,
}

/// Персонализированный чат с заботливым ИИ-помощником
pub async fn personal_health_chat(
    State(ai_service): State<AiService>,
    Json(request): Json<PersonalChatRequest>,
) -> Result<ResponseJson<PersonalizedResponse>, AppError> {
    let assistant = PersonalHealthAssistant::new(ai_service);
    
    // В реальном приложении здесь бы загружались данные пользователя из БД
    let health_context = create_mock_health_context();
    
    let response = assistant.get_personalized_response(&request.message, &health_context).await?;
    
    Ok(ResponseJson(response))
}

/// Ежедневная проверка самочувствия
pub async fn daily_wellbeing_check(
    State(ai_service): State<AiService>,
    Json(request): Json<WellbeingCheckRequest>,
) -> Result<ResponseJson<PersonalizedResponse>, AppError> {
    let assistant = PersonalHealthAssistant::new(ai_service);
    
    // Создаем запись о самочувствии
    let wellbeing = DailyWellbeing {
        id: Uuid::new_v4(),
        user_id: Uuid::new_v4(), // В реальном приложении - ID из токена
        date: Utc::now(),
        mood_score: request.mood_score,
        energy_level: request.energy_level,
        stress_level: request.stress_level,
        sleep_hours: request.sleep_hours,
        sleep_quality: request.sleep_quality,
        water_intake_ml: request.water_intake_ml,
        exercise_minutes: request.exercise_minutes,
        notes: request.notes,
        symptoms: request.symptoms,
        created_at: Utc::now(),
    };
    
    // В реальном приложении здесь сохранялось бы в БД
    
    // Генерируем персонализированный ответ на основе данных
    let health_context = create_health_context_from_wellbeing(&wellbeing);
    let message = generate_wellbeing_summary(&wellbeing);
    
    let response = assistant.get_personalized_response(&message, &health_context).await?;
    
    Ok(ResponseJson(response))
}

/// Панель здоровья с инсайтами и рекомендациями
pub async fn health_dashboard(
    State(ai_service): State<AiService>,
) -> Result<ResponseJson<HealthDashboardResponse>, AppError> {
    let assistant = PersonalHealthAssistant::new(ai_service);
    
    // В реальном приложении загружались бы данные пользователя
    let health_context = create_mock_health_context();
    
    let insights = assistant.generate_health_insights(&health_context, "").await?;
    let recommendations = assistant.generate_personalized_recommendations(&health_context).await?;
    
    let dashboard = HealthDashboardResponse {
        current_wellbeing: health_context.recent_wellbeing.first().cloned(),
        insights,
        recommendations,
        weekly_trends: WeeklyTrends {
            avg_mood: 7.2,
            avg_energy: 6.8,
            avg_stress: 4.1,
            avg_sleep: 7.5,
            total_water_ml: 14000,
            total_exercise_minutes: 180,
        },
        motivational_message: "Вы заботитесь о своем здоровье уже 7 дней подряд! Это отличная привычка. 🌟".to_string(),
    };
    
    Ok(ResponseJson(dashboard))
}

/// Получить персонализированные рекомендации
pub async fn get_recommendations(
    State(ai_service): State<AiService>,
) -> Result<ResponseJson<Vec<PersonalizedRecommendation>>, AppError> {
    let assistant = PersonalHealthAssistant::new(ai_service);
    let health_context = create_mock_health_context();
    
    let recommendations = assistant.generate_personalized_recommendations(&health_context).await?;
    
    Ok(ResponseJson(recommendations))
}

/// Анализ настроения и предложения
pub async fn mood_analysis(
    State(ai_service): State<AiService>,
    Json(mood_data): Json<serde_json::Value>,
) -> Result<ResponseJson<PersonalizedResponse>, AppError> {
    let assistant = PersonalHealthAssistant::new(ai_service);
    
    let mood_score = mood_data["mood_score"].as_i64().unwrap_or(5) as i32;
    let notes = mood_data["notes"].as_str().unwrap_or("");
    
    let message = format!(
        "Мое настроение сегодня {} из 10. Заметки: {}. Помоги разобраться с эмоциями и дай советы.",
        mood_score, notes
    );
    
    let health_context = create_mock_health_context();
    let response = assistant.get_personalized_response(&message, &health_context).await?;
    
    Ok(ResponseJson(response))
}

// Вспомогательные функции

fn create_mock_health_context() -> HealthContext {
    HealthContext {
        user_profile: UserHealthSummary {
            name: "Александра".to_string(),
            age: Some(28),
            fitness_level: FitnessLevel::ModeratelyActive,
            sleep_goal: Some(8.0),
            water_goal: Some(2000),
            dietary_restrictions: vec!["без глютена".to_string()],
            health_goals: vec![
                "Улучшить качество сна".to_string(),
                "Снизить уровень стресса".to_string(),
                "Больше двигаться".to_string(),
            ],
            medical_conditions: vec![],
            stress_level: Some(6),
        },
        recent_wellbeing: vec![
            DailyWellbeing {
                id: Uuid::new_v4(),
                user_id: Uuid::new_v4(),
                date: Utc::now(),
                mood_score: Some(7),
                energy_level: Some(6),
                stress_level: Some(5),
                sleep_hours: Some(7.2),
                sleep_quality: Some(7),
                water_intake_ml: Some(1800),
                exercise_minutes: Some(30),
                notes: None,
                symptoms: vec![],
                created_at: Utc::now(),
            }
        ],
        recent_nutrition: vec![
            NutritionSummary {
                date: Utc::now(),
                calories: 1850.0,
                protein: 85.0,
                carbs: 220.0,
                fat: 65.0,
                water_ml: 1800,
            }
        ],
        current_time: chrono::Local::now().format("%H:%M").to_string(),
        current_season: "Лето".to_string(),
        weather_context: Some("Солнечно, +25°C".to_string()),
    }
}

fn create_health_context_from_wellbeing(wellbeing: &DailyWellbeing) -> HealthContext {
    HealthContext {
        user_profile: UserHealthSummary {
            name: "Пользователь".to_string(),
            age: Some(30),
            fitness_level: FitnessLevel::ModeratelyActive,
            sleep_goal: Some(8.0),
            water_goal: Some(2000),
            dietary_restrictions: vec![],
            health_goals: vec!["Улучшить общее самочувствие".to_string()],
            medical_conditions: vec![],
            stress_level: wellbeing.stress_level,
        },
        recent_wellbeing: vec![wellbeing.clone()],
        recent_nutrition: vec![],
        current_time: chrono::Local::now().format("%H:%M").to_string(),
        current_season: "Лето".to_string(),
        weather_context: None,
    }
}

fn generate_wellbeing_summary(wellbeing: &DailyWellbeing) -> String {
    let mut summary = "Вот мои показатели на сегодня:".to_string();
    
    if let Some(mood) = wellbeing.mood_score {
        summary.push_str(&format!(" настроение {}/10,", mood));
    }
    if let Some(energy) = wellbeing.energy_level {
        summary.push_str(&format!(" энергия {}/10,", energy));
    }
    if let Some(stress) = wellbeing.stress_level {
        summary.push_str(&format!(" стресс {}/10,", stress));
    }
    if let Some(sleep) = wellbeing.sleep_hours {
        summary.push_str(&format!(" спал {} часов,", sleep));
    }
    if let Some(water) = wellbeing.water_intake_ml {
        summary.push_str(&format!(" выпил {} мл воды,", water));
    }
    
    if !wellbeing.symptoms.is_empty() {
        summary.push_str(&format!(" симптомы: {}.", wellbeing.symptoms.join(", ")));
    }
    
    if let Some(notes) = &wellbeing.notes {
        summary.push_str(&format!(" Заметки: {}", notes));
    }
    
    summary.push_str(" Проанализируй мое состояние и дай рекомендации.");
    summary
}
