use crate::models::health::*;
use crate::models::user::User;
use crate::models::diary::DiaryEntry;
use crate::services::ai::AiService;
use crate::utils::errors::AppError;
use chrono::{DateTime, Utc, Local, Timelike};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct PersonalHealthAssistant {
    ai_service: AiService,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthContext {
    pub user_profile: UserHealthSummary,
    pub recent_wellbeing: Vec<DailyWellbeing>,
    pub recent_nutrition: Vec<NutritionSummary>,
    pub current_time: String,
    pub current_season: String,
    pub weather_context: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserHealthSummary {
    pub name: String,
    pub age: Option<i32>,
    pub fitness_level: FitnessLevel,
    pub sleep_goal: Option<f32>,
    pub water_goal: Option<i32>,
    pub dietary_restrictions: Vec<String>,
    pub health_goals: Vec<String>,
    pub medical_conditions: Vec<String>,
    pub stress_level: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NutritionSummary {
    pub date: DateTime<Utc>,
    pub calories: f32,
    pub protein: f32,
    pub carbs: f32,
    pub fat: f32,
    pub water_ml: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonalizedResponse {
    pub response: String,
    pub insights: Vec<HealthInsight>,
    pub recommendations: Vec<PersonalizedRecommendation>,
    pub mood_check: Option<String>,
    pub encouragement: Option<String>,
    pub next_suggestions: Vec<String>,
}

impl PersonalHealthAssistant {
    pub fn new(ai_service: AiService) -> Self {
        Self { ai_service }
    }

    /// Главная функция - персонализированный ответ на основе контекста здоровья
    pub async fn get_personalized_response(
        &self,
        user_message: &str,
        health_context: &HealthContext,
    ) -> Result<PersonalizedResponse, AppError> {
        let system_prompt = self.build_caring_system_prompt(health_context);
        let full_prompt = format!("{}\n\nСообщение пользователя: {}", system_prompt, user_message);

        let ai_response = self.ai_service.generate_response(&full_prompt).await?;
        
        // Анализируем ответ и генерируем дополнительные инсайты
        let insights = self.generate_health_insights(health_context, user_message).await?;
        let recommendations = self.generate_personalized_recommendations(health_context).await?;
        let mood_check = self.generate_mood_check(health_context);
        let encouragement = self.generate_encouragement(health_context);
        let next_suggestions = self.generate_contextual_suggestions(user_message, health_context);

        Ok(PersonalizedResponse {
            response: ai_response,
            insights,
            recommendations,
            mood_check,
            encouragement,
            next_suggestions,
        })
    }

    /// Создает заботливый системный промпт на основе данных пользователя
    fn build_caring_system_prompt(&self, context: &HealthContext) -> String {
        let user = &context.user_profile;
        let current_time = &context.current_time;
        
        let mut prompt = format!(
            "Ты - заботливый персональный помощник по здоровью для {}. Время сейчас: {}. 
            Ты знаешь пользователя лично и искренне заботишься о его благополучии.",
            user.name, current_time
        );

        // Добавляем персональную информацию
        if let Some(age) = user.age {
            prompt.push_str(&format!(" Пользователю {} лет.", age));
        }

        prompt.push_str(&format!(" Уровень активности: {:?}.", user.fitness_level));

        if !user.health_goals.is_empty() {
            prompt.push_str(&format!(" Цели пользователя: {}.", user.health_goals.join(", ")));
        }

        if !user.medical_conditions.is_empty() {
            prompt.push_str(&format!(" Важно учитывать: {}.", user.medical_conditions.join(", ")));
        }

        if !user.dietary_restrictions.is_empty() {
            prompt.push_str(&format!(" Диетические ограничения: {}.", user.dietary_restrictions.join(", ")));
        }

        // Добавляем контекст недавнего самочувствия
        if let Some(latest_wellbeing) = context.recent_wellbeing.first() {
            prompt.push_str(&format!(
                " Последние показатели: настроение {}/10, энергия {}/10, стресс {}/10, сон {} часов.",
                latest_wellbeing.mood_score.unwrap_or(5),
                latest_wellbeing.energy_level.unwrap_or(5),
                latest_wellbeing.stress_level.unwrap_or(5),
                latest_wellbeing.sleep_hours.unwrap_or(0.0)
            ));
        }

        prompt.push_str("\n\nТвой стиль общения:
        - Теплый, понимающий и поддерживающий
        - Даешь практичные, персонализированные советы
        - Учитываешь эмоциональное состояние
        - Мотивируешь без давления
        - Используешь данные пользователя для точных рекомендаций
        - Проявляешь эмпатию и понимание
        - Предлагаешь конкретные действия, а не общие советы
        
        Отвечай как заботливый друг, который хорошо знает пользователя и искренне хочет помочь.");

        prompt
    }

    /// Анализирует данные и генерирует инсайты о здоровье
    pub async fn generate_health_insights(
        &self,
        context: &HealthContext,
        _user_message: &str,
    ) -> Result<Vec<HealthInsight>, AppError> {
        let mut insights = Vec::new();

        // Анализ сна
        if let Some(recent) = context.recent_wellbeing.first() {
            if let Some(sleep_hours) = recent.sleep_hours {
                if let Some(sleep_goal) = context.user_profile.sleep_goal {
                    if sleep_hours < sleep_goal - 1.0 {
                        insights.push(HealthInsight {
                            id: Uuid::new_v4(),
                            user_id: Uuid::new_v4(), // В реальном приложении - ID пользователя
                            insight_type: InsightType::Sleep,
                            title: "Недостаток сна".to_string(),
                            message: format!(
                                "Вы спали {} часов, что меньше вашей цели в {} часов. Недостаток сна может влиять на настроение, энергию и иммунитет.",
                                sleep_hours, sleep_goal
                            ),
                            priority: if sleep_hours < sleep_goal - 2.0 { Priority::High } else { Priority::Medium },
                            action_items: vec![
                                "Попробуйте лечь спать на 30 минут раньше сегодня".to_string(),
                                "Создайте расслабляющий ритуал перед сном".to_string(),
                                "Избегайте экранов за час до сна".to_string(),
                            ],
                            data_sources: vec!["sleep_tracking".to_string()],
                            created_at: Utc::now(),
                            is_read: false,
                        });
                    }
                }
            }
        }

        // Анализ настроения и стресса
        if let Some(recent) = context.recent_wellbeing.first() {
            if let Some(stress) = recent.stress_level {
                if stress >= 7 {
                    insights.push(HealthInsight {
                        id: Uuid::new_v4(),
                        user_id: Uuid::new_v4(),
                        insight_type: InsightType::Stress,
                        title: "Повышенный уровень стресса".to_string(),
                        message: format!(
                            "Ваш уровень стресса {} из 10 довольно высокий. Это может влиять на сон, аппетит и общее самочувствие.",
                            stress
                        ),
                        priority: if stress >= 8 { Priority::High } else { Priority::Medium },
                        action_items: vec![
                            "Попробуйте 5-минутную дыхательную практику".to_string(),
                            "Сделайте короткую прогулку на свежем воздухе".to_string(),
                            "Выпейте теплый травяной чай".to_string(),
                        ],
                        data_sources: vec!["mood_tracking".to_string()],
                        created_at: Utc::now(),
                        is_read: false,
                    });
                }
            }
        }

        // Анализ гидратации
        if let Some(nutrition) = context.recent_nutrition.first() {
            if let Some(water_goal) = context.user_profile.water_goal {
                if nutrition.water_ml < (water_goal as f32 * 0.7) as i32 {
                    insights.push(HealthInsight {
                        id: Uuid::new_v4(),
                        user_id: Uuid::new_v4(),
                        insight_type: InsightType::Hydration,
                        title: "Недостаточное потребление воды".to_string(),
                        message: format!(
                            "Вы выпили {} мл воды из {} мл цели. Достаточная гидратация важна для энергии и концентрации.",
                            nutrition.water_ml, water_goal
                        ),
                        priority: Priority::Medium,
                        action_items: vec![
                            "Поставьте напоминание пить воду каждый час".to_string(),
                            "Держите бутылку воды на видном месте".to_string(),
                            "Добавьте лимон или мяту для вкуса".to_string(),
                        ],
                        data_sources: vec!["water_tracking".to_string()],
                        created_at: Utc::now(),
                        is_read: false,
                    });
                }
            }
        }

        Ok(insights)
    }

    /// Генерирует персонализированные рекомендации
    pub async fn generate_personalized_recommendations(
        &self,
        context: &HealthContext,
    ) -> Result<Vec<PersonalizedRecommendation>, AppError> {
        let mut recommendations = Vec::new();

        // Рекомендации на основе времени дня
        let hour = Local::now().hour();
        
        match hour {
            6..=9 => {
                recommendations.push(PersonalizedRecommendation {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    category: RecommendationCategory::Routine,
                    title: "Утренний заряд энергии".to_string(),
                    description: "Начните день с простых упражнений и здорового завтрака".to_string(),
                    benefits: vec![
                        "Повышает энергию на весь день".to_string(),
                        "Улучшает настроение".to_string(),
                        "Ускоряет метаболизм".to_string(),
                    ],
                    steps: vec![
                        "Выпейте стакан воды".to_string(),
                        "Сделайте 5 минут растяжки".to_string(),
                        "Позавтракайте с белком и клетчаткой".to_string(),
                    ],
                    frequency: "daily".to_string(),
                    difficulty: 2,
                    estimated_time_minutes: Some(15),
                    created_at: Utc::now(),
                    is_active: true,
                });
            },
            12..=14 => {
                recommendations.push(PersonalizedRecommendation {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    category: RecommendationCategory::Nutrition,
                    title: "Энергетический обед".to_string(),
                    description: "Сбалансированный обед для поддержания энергии во второй половине дня".to_string(),
                    benefits: vec![
                        "Предотвращает послеобеденную усталость".to_string(),
                        "Поддерживает стабильный уровень сахара в крови".to_string(),
                    ],
                    steps: vec![
                        "Включите белок (рыба, курица, бобовые)".to_string(),
                        "Добавьте сложные углеводы (киноа, бурый рис)".to_string(),
                        "Не забудьте про овощи и зелень".to_string(),
                    ],
                    frequency: "daily".to_string(),
                    difficulty: 3,
                    estimated_time_minutes: Some(20),
                    created_at: Utc::now(),
                    is_active: true,
                });
            },
            18..=22 => {
                recommendations.push(PersonalizedRecommendation {
                    id: Uuid::new_v4(),
                    user_id: Uuid::new_v4(),
                    category: RecommendationCategory::Sleep,
                    title: "Подготовка ко сну".to_string(),
                    description: "Создайте идеальные условия для качественного сна".to_string(),
                    benefits: vec![
                        "Улучшает качество сна".to_string(),
                        "Помогает быстрее заснуть".to_string(),
                        "Повышает энергию на следующий день".to_string(),
                    ],
                    steps: vec![
                        "Приглушите свет за 2 часа до сна".to_string(),
                        "Выпейте травяной чай".to_string(),
                        "Почитайте книгу вместо экрана".to_string(),
                        "Проветрите спальню".to_string(),
                    ],
                    frequency: "daily".to_string(),
                    difficulty: 2,
                    estimated_time_minutes: Some(30),
                    created_at: Utc::now(),
                    is_active: true,
                });
            },
            _ => {}
        }

        Ok(recommendations)
    }

    /// Генерирует проверку настроения
    fn generate_mood_check(&self, context: &HealthContext) -> Option<String> {
        if let Some(recent) = context.recent_wellbeing.first() {
            if let Some(mood) = recent.mood_score {
                return match mood {
                    1..=3 => Some("Я вижу, что у вас непростой период. Помните - это временно, и я здесь, чтобы поддержать вас. Что могло бы сейчас помочь вам почувствовать себя чуть лучше?".to_string()),
                    4..=6 => Some("Как ваше настроение сегодня? Если хотите, расскажите, что на душе - иногда просто поговорить уже помогает.".to_string()),
                    7..=8 => Some("Рада видеть, что у вас хорошее настроение! Что помогло вам так прекрасно себя чувствовать?".to_string()),
                    9..=10 => Some("Вы просто сияете сегодня! Это заразительно - делитесь своей радостью с миром! 🌟".to_string()),
                    _ => None,
                };
            }
        }
        Some("Как дела? Как настроение? Я всегда готова выслушать и помочь.".to_string())
    }

    /// Генерирует ободряющее сообщение
    fn generate_encouragement(&self, _context: &HealthContext) -> Option<String> {
        let encouragements = vec![
            "Помните: каждый маленький шаг к здоровью важен. Вы делаете больше, чем думаете! 💪",
            "Ваша забота о себе вдохновляет. Продолжайте в том же духе! ✨",
            "Здоровье - это марафон, не спринт. Будьте терпеливы к себе. 🌱",
            "Каждый день - новая возможность позаботиться о себе. Вы справляетесь отлично! 🌟",
        ];
        
        Some(encouragements[rand::random::<usize>() % encouragements.len()].to_string())
    }

    /// Генерирует контекстные предложения для продолжения разговора
    fn generate_contextual_suggestions(&self, user_message: &str, _context: &HealthContext) -> Vec<String> {
        let message_lower = user_message.to_lowercase();
        
        if message_lower.contains("сон") || message_lower.contains("спать") {
            vec![
                "Как создать идеальную обстановку для сна?".to_string(),
                "Помоги с вечерним ритуалом".to_string(),
                "Что делать при бессоннице?".to_string(),
            ]
        } else if message_lower.contains("стресс") || message_lower.contains("тревог") {
            vec![
                "Научи техникам расслабления".to_string(),
                "Как справляться со стрессом на работе?".to_string(),
                "Дыхательные упражнения для успокоения".to_string(),
            ]
        } else if message_lower.contains("вода") || message_lower.contains("пить") {
            vec![
                "Как не забывать пить воду?".to_string(),
                "Сколько воды мне нужно?".to_string(),
                "Альтернативы обычной воде".to_string(),
            ]
        } else if message_lower.contains("еда") || message_lower.contains("питание") {
            vec![
                "Составь план питания на день".to_string(),
                "Здоровые перекусы для энергии".to_string(),
                "Как готовить быстро и полезно?".to_string(),
            ]
        } else {
            vec![
                "Проанализируй мое самочувствие".to_string(),
                "Дай совет по здоровому образу жизни".to_string(),
                "Как улучшить мое настроение?".to_string(),
                "Составь план заботы о себе".to_string(),
            ]
        }
    }
}
