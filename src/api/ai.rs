use axum::{
    extract::{State, Json},
    response::Json as ResponseJson,
};
use serde::{Deserialize, Serialize};
use chrono::Timelike;
use rand::Rng;
use crate::services::ai::AiService;
use crate::utils::errors::AppError;

#[derive(Debug, Deserialize)]
pub struct AiChatRequest {
    pub message: String,
    pub context: Option<String>, // Контекст пользователя (цели, предпочтения и т.д.)
}

#[derive(Debug, Serialize, Clone)]
pub struct AiCard {
    pub title: String,
    pub content: String,
    pub emoji: Option<String>,
    pub category: Option<String>, // nutrition, health, recipe, motivation, general
    pub priority: Option<String>, // high, medium, low
}

#[derive(Debug, Serialize)]
pub struct AiChatResponse {
    pub response: String,
    pub suggestions: Option<Vec<String>>, // Дополнительные предложения
    pub cards: Option<Vec<AiCard>>, // Структурированные карточки
}

#[derive(Debug, Deserialize)]
pub struct ProactiveMessageRequest {
    pub user_context: Option<String>, // Контекст пользователя (последняя активность, цели и т.д.)
    pub last_meal_time: Option<String>, // Время последнего приема пищи
    pub mood_level: Option<i32>, // Уровень настроения от 1 до 5
    pub energy_level: Option<i32>, // Уровень энергии от 1 до 5
}

#[derive(Debug, Serialize, Clone)]
pub struct AiProactiveMessage {
    pub message: String,
    pub trigger_type: String, // breakfast, sleep, mood, activity, nutrition, motivation
    pub urgency: String, // high, medium, low
    pub cards: Option<Vec<AiCard>>,
    pub suggestions: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct RecipeGenerationRequest {
    pub ingredients: Vec<String>,
    pub dietary_restrictions: Option<Vec<String>>,
    pub cuisine_type: Option<String>,
    pub cooking_time: Option<i32>, // в минутах
    pub difficulty: Option<String>,
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
    
    // Генерируем карточки для структурированного ответа
    let cards = generate_response_cards(&request.message, &request.context);

    Ok(ResponseJson(AiChatResponse {
        response: ai_response,
        suggestions: Some(suggestions),
        cards,
    }))
}

/// Генерирует предложения для продолжения разговора
fn generate_suggestions(user_message: &str, _ai_response: &str) -> Vec<String> {
    let user_lower = user_message.to_lowercase();
    
    if user_lower.contains("суп") {
        vec![
            "Как сделать суп более густым?".to_string(),
            "Какие специи лучше добавить?".to_string(),
            "Покажи другие варианты супов".to_string(),
        ]
    } else if user_lower.contains("салат") {
        vec![
            "Какую заправку лучше использовать?".to_string(),
            "Как сохранить салат свежим?".to_string(),
            "Покажи теплые салаты".to_string(),
        ]
    } else if user_lower.contains("мясо") || user_lower.contains("курица") || user_lower.contains("говядина") {
        vec![
            "Как правильно мариновать мясо?".to_string(),
            "Покажи гарниры к мясу".to_string(),
            "Как проверить готовность мяса?".to_string(),
        ]
    } else if user_lower.contains("рецепт") || user_lower.contains("готовить") {
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

/// Генерирует структурированные карточки для ответа
fn generate_response_cards(user_message: &str, _context: &Option<String>) -> Option<Vec<AiCard>> {
    let user_lower = user_message.to_lowercase();
    
    // Проверяем конкретные блюда
    if user_lower.contains("суп") {
        Some(vec![
            AiCard {
                title: "🍜 Куриный суп готов!".to_string(),
                content: "Полноценное блюдо с высоким содержанием белка".to_string(),
                emoji: Some("🍜".to_string()),
                category: Some("recipe".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "🥄 Совет по подаче".to_string(),
                content: "Подавайте с зеленью и сухариками для лучшего вкуса".to_string(),
                emoji: Some("🥄".to_string()),
                category: Some("general".to_string()),
                priority: Some("medium".to_string()),
            },
            AiCard {
                title: "📊 Пищевая ценность".to_string(),
                content: "~300 ккал, 25г белка - отличный баланс питательных веществ".to_string(),
                emoji: Some("📊".to_string()),
                category: Some("nutrition".to_string()),
                priority: Some("medium".to_string()),
            },
        ])
    } else if user_lower.contains("салат") {
        Some(vec![
            AiCard {
                title: "🥗 Свежий салат".to_string(),
                content: "Легкое и полезное блюдо с витаминами".to_string(),
                emoji: Some("🥗".to_string()),
                category: Some("recipe".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "🌿 Заправка".to_string(),
                content: "Используйте оливковое масло и лимонный сок".to_string(),
                emoji: Some("🌿".to_string()),
                category: Some("general".to_string()),
                priority: Some("medium".to_string()),
            },
        ])
    } else if user_lower.contains("мясо") || user_lower.contains("курица") || user_lower.contains("говядина") {
        Some(vec![
            AiCard {
                title: "🍖 Мясное блюдо".to_string(),
                content: "Отличный источник белка для роста мышц".to_string(),
                emoji: Some("🍖".to_string()),
                category: Some("recipe".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "🔥 Способ приготовления".to_string(),
                content: "Запекание или гриль сохранят больше питательных веществ".to_string(),
                emoji: Some("🔥".to_string()),
                category: Some("health".to_string()),
                priority: Some("medium".to_string()),
            },
        ])
    } else if user_lower.contains("рецепт") || user_lower.contains("готовить") || user_lower.contains("приготовить") {
        Some(vec![
            AiCard {
                title: "🍳 Совет по готовке".to_string(),
                content: "Всегда разогревайте сковороду перед добавлением масла".to_string(),
                emoji: Some("🍳".to_string()),
                category: Some("recipe".to_string()),
                priority: Some("medium".to_string()),
            },
            AiCard {
                title: "⏱️ Экономия времени".to_string(),
                content: "Подготовьте все ингредиенты заранее".to_string(),
                emoji: Some("⏱️".to_string()),
                category: Some("general".to_string()),
                priority: Some("low".to_string()),
            },
        ])
    } else if user_lower.contains("диета") || user_lower.contains("похудеть") {
        Some(vec![
            AiCard {
                title: "🥗 Здоровое питание".to_string(),
                content: "Увеличьте потребление овощей и белка".to_string(),
                emoji: Some("🥗".to_string()),
                category: Some("nutrition".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "💧 Гидратация".to_string(),
                content: "Пейте больше воды в течение дня".to_string(),
                emoji: Some("💧".to_string()),
                category: Some("health".to_string()),
                priority: Some("medium".to_string()),
            },
        ])
    } else if user_lower.contains("привет") || user_lower.contains("здравствуй") {
        Some(vec![
            AiCard {
                title: "👋 Добро пожаловать!".to_string(),
                content: "Я ваш персональный кулинарный помощник. Чем могу помочь?".to_string(),
                emoji: Some("👋".to_string()),
                category: Some("general".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "✨ Начните с целей".to_string(),
                content: "Расскажите мне о ваших кулинарных предпочтениях".to_string(),
                emoji: Some("✨".to_string()),
                category: Some("motivation".to_string()),
                priority: Some("medium".to_string()),
            },
        ])
    } else {
        // Возвращаем базовые карточки для любого запроса
        Some(vec![
            AiCard {
                title: "💡 Кулинарный совет".to_string(),
                content: "Попробуйте добавить свежую зелень для улучшения вкуса".to_string(),
                emoji: Some("💡".to_string()),
                category: Some("general".to_string()),
                priority: Some("medium".to_string()),
            },
            AiCard {
                title: "🍽️ Подача блюда".to_string(),
                content: "Красивая подача делает еду еще вкуснее".to_string(),
                emoji: Some("🍽️".to_string()),
                category: Some("general".to_string()),
                priority: Some("low".to_string()),
            },
        ])
    }
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
        cards: Some(vec![
            AiCard {
                title: "🍳 Рецепт готов!".to_string(),
                content: "Ваш персональный рецепт на основе выбранных ингредиентов".to_string(),
                emoji: Some("🍳".to_string()),
                category: Some("recipe".to_string()),
                priority: Some("high".to_string()),
            },
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
        cards: Some(vec![
            AiCard {
                title: "📊 Анализ питания".to_string(),
                content: "Подробная информация о пищевой ценности вашего блюда".to_string(),
                emoji: Some("📊".to_string()),
                category: Some("nutrition".to_string()),
                priority: Some("high".to_string()),
            },
        ]),
    }))
}

/// Генерирует активное сообщение от ИИ при заходе в профиль
pub async fn generate_proactive_message(
    _state: State<AiService>,
    Json(request): Json<ProactiveMessageRequest>,
) -> Result<ResponseJson<AiProactiveMessage>, AppError> {
    
    // Получаем текущий час для контекстных сообщений
    let current_hour = chrono::Utc::now().hour();
    
    // Генерируем активное сообщение на основе времени и контекста
    let proactive_message = generate_contextual_proactive_message(current_hour, &request);
    
    Ok(ResponseJson(proactive_message))
}

/// Генерирует контекстное активное сообщение на основе времени и активности пользователя
fn generate_contextual_proactive_message(hour: u32, request: &ProactiveMessageRequest) -> AiProactiveMessage {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Проверяем настроение пользователя для более персонализированных сообщений
    let is_low_mood = request.mood_level.map_or(false, |mood| mood <= 3);
    let is_low_energy = request.energy_level.map_or(false, |energy| energy <= 3);
    
    // Утренние сообщения (6:00 - 11:00)
    if hour >= 6 && hour < 11 {
        let morning_messages = if is_low_energy {
            vec![
                AiProactiveMessage {
                    message: "🌅 Вижу, энергии маловато с утра! Хочешь рецепт энергетического завтрака за 5 минут? Он зарядит тебя на весь день!".to_string(),
                    trigger_type: "breakfast".to_string(),
                    urgency: "high".to_string(),
                    cards: Some(vec![
                        AiCard {
                            title: "⚡ Энергетический завтрак".to_string(),
                            content: "Банановый смузи с овсянкой и орехами - мгновенный заряд!".to_string(),
                            emoji: Some("⚡".to_string()),
                            category: Some("recipe".to_string()),
                            priority: Some("high".to_string()),
                        },
                        AiCard {
                            title: "☕ Бодрящий напиток".to_string(),
                            content: "Зеленый чай с имбирем разгонит метаболизм".to_string(),
                            emoji: Some("☕".to_string()),
                            category: Some("health".to_string()),
                            priority: Some("high".to_string()),
                        },
                    ]),
                    suggestions: Some(vec![
                        "Энергетические завтраки".to_string(),
                        "Натуральные энергетики".to_string(),
                        "Быстрые рецепты на утро".to_string(),
                    ]),
                },
            ]
        } else {
            vec![
                AiProactiveMessage {
                    message: "☀️ Как спалось? Качественный сон - это основа твоей энергии и правильного аппетита на весь день!".to_string(),
                    trigger_type: "sleep".to_string(),
                    urgency: "medium".to_string(),
                    cards: Some(vec![
                        AiCard {
                            title: "💤 Сон и питание".to_string(),
                            content: "Недосып увеличивает тягу к сладкому на 30%".to_string(),
                            emoji: Some("💤".to_string()),
                            category: Some("health".to_string()),
                            priority: Some("high".to_string()),
                        },
                        AiCard {
                            title: "🥛 Сбалансированный завтрак".to_string(),
                            content: "Белки + сложные углеводы = стабильная энергия".to_string(),
                            emoji: Some("🥛".to_string()),
                            category: Some("nutrition".to_string()),
                            priority: Some("medium".to_string()),
                        },
                    ]),
                    suggestions: Some(vec![
                        "Продукты для хорошего сна".to_string(),
                        "Сбалансированные завтраки".to_string(),
                        "Режим питания и сна".to_string(),
                    ]),
                },
                AiProactiveMessage {
                    message: "🌅 Доброе утро! Я вижу, ты не завтракал. Хочешь рецепт за 5 минут? Быстро, вкусно и полезно!".to_string(),
                    trigger_type: "breakfast".to_string(),
                    urgency: "medium".to_string(),
                    cards: Some(vec![
                        AiCard {
                            title: "⏱️ 5-минутный завтрак".to_string(),
                            content: "Авокадо тост с яйцом - готов моментально!".to_string(),
                            emoji: Some("⏱️".to_string()),
                            category: Some("recipe".to_string()),
                            priority: Some("high".to_string()),
                        },
                        AiCard {
                            title: "🍌 Быстрая альтернатива".to_string(),
                            content: "Греческий йогурт с ягодами и мёдом".to_string(),
                            emoji: Some("🍌".to_string()),
                            category: Some("recipe".to_string()),
                            priority: Some("medium".to_string()),
                        },
                    ]),
                    suggestions: Some(vec![
                        "Быстрые завтраки".to_string(),
                        "Что пить с утра?".to_string(),
                        "Полезные перекусы".to_string(),
                    ]),
                },
            ]
        };
        return morning_messages[rng.gen_range(0..morning_messages.len())].clone();
    }
    
    // Дневные сообщения (11:00 - 17:00)
    if hour >= 11 && hour < 17 {
        let day_messages = if is_low_mood {
            vec![
                AiProactiveMessage {
                    message: "😔 Настроение на 3/5? Понимаю... Предлагаю сходить в парк! 🌳 Свежий воздух и движение творят чудеса с настроением!".to_string(),
                    trigger_type: "mood".to_string(),
                    urgency: "high".to_string(),
                    cards: Some(vec![
                        AiCard {
                            title: "🌳 Сила природы".to_string(),
                            content: "15 минут на свежем воздухе повышают настроение на 40%".to_string(),
                            emoji: Some("🌳".to_string()),
                            category: Some("motivation".to_string()),
                            priority: Some("high".to_string()),
                        },
                        AiCard {
                            title: "🍫 Натуральные антидепрессанты".to_string(),
                            content: "Темный шоколад и орехи стимулируют выработку серотонина".to_string(),
                            emoji: Some("🍫".to_string()),
                            category: Some("health".to_string()),
                            priority: Some("medium".to_string()),
                        },
                    ]),
                    suggestions: Some(vec![
                        "Продукты для настроения".to_string(),
                        "Активности на свежем воздухе".to_string(),
                        "Быстрые упражнения".to_string(),
                    ]),
                },
            ]
        } else {
            vec![
                AiProactiveMessage {
                    message: "🌞 День в разгаре! Как твоя энергия? Если чувствуешь спад, предлагаю здоровый перекус для подзарядки!".to_string(),
                    trigger_type: "energy".to_string(),
                    urgency: "medium".to_string(),
                    cards: Some(vec![
                        AiCard {
                            title: "🥜 Энергетический перекус".to_string(),
                            content: "Миндаль + сухофрукты = природная энергия без сахарных скачков".to_string(),
                            emoji: Some("🥜".to_string()),
                            category: Some("nutrition".to_string()),
                            priority: Some("high".to_string()),
                        },
                        AiCard {
                            title: "🚶‍♂️ Микро-активность".to_string(),
                            content: "5-минутная разминка лучше кофе для концентрации".to_string(),
                            emoji: Some("🚶‍♂️".to_string()),
                            category: Some("motivation".to_string()),
                            priority: Some("medium".to_string()),
                        },
                    ]),
                    suggestions: Some(vec![
                        "Здоровые перекусы".to_string(),
                        "Быстрые упражнения".to_string(),
                        "Полезные сладости".to_string(),
                    ]),
                },
            ]
        };
        return day_messages[rng.gen_range(0..day_messages.len())].clone();
    }
    
    // Вечерние сообщения (17:00 - 22:00)
    if hour >= 17 && hour < 22 {
        let evening_messages = vec![
            AiProactiveMessage {
                message: "🌅 День подходит к концу! Как прошел твой план питания? Давай подведем итоги и подготовимся к завтрашнему дню.".to_string(),
                trigger_type: "nutrition".to_string(),
                urgency: "medium".to_string(),
                cards: Some(vec![
                    AiCard {
                        title: "📊 Итоги дня".to_string(),
                        content: "Проанализируй баланс белков, жиров и углеводов за сегодня".to_string(),
                        emoji: Some("📊".to_string()),
                        category: Some("nutrition".to_string()),
                        priority: Some("high".to_string()),
                    },
                    AiCard {
                        title: "🌙 Легкий ужин".to_string(),
                        content: "Ужинай за 2-3 часа до сна для лучшего восстановления".to_string(),
                        emoji: Some("🌙".to_string()),
                        category: Some("health".to_string()),
                        priority: Some("medium".to_string()),
                    },
                ]),
                suggestions: Some(vec![
                    "Анализ питания за день".to_string(),
                    "Легкие ужины".to_string(),
                    "План на завтра".to_string(),
                ]),
            },
            AiProactiveMessage {
                message: "🎯 Отличная работа сегодня! Помни: каждое здоровое решение - это шаг к твоей цели. Гордись собой!".to_string(),
                trigger_type: "motivation".to_string(),
                urgency: "low".to_string(),
                cards: Some(vec![
                    AiCard {
                        title: "🏆 Ты молодец!".to_string(),
                        content: "Каждый правильный выбор в питании приближает к результату".to_string(),
                        emoji: Some("🏆".to_string()),
                        category: Some("motivation".to_string()),
                        priority: Some("high".to_string()),
                    },
                    AiCard {
                        title: "📅 Завтрашние цели".to_string(),
                        content: "Планирование ужина - залог успешного завтрашнего дня".to_string(),
                        emoji: Some("📅".to_string()),
                        category: Some("general".to_string()),
                        priority: Some("medium".to_string()),
                    },
                ]),
                suggestions: Some(vec![
                    "Мои достижения".to_string(),
                    "Планирование завтра".to_string(),
                    "Мотивационные советы".to_string(),
                ]),
            },
        ];
        return evening_messages[rng.gen_range(0..evening_messages.len())].clone();
    }
    
    // Ночные/поздние сообщения (22:00 - 6:00)
    AiProactiveMessage {
        message: "🌙 Довольно поздно! Хороший сон - основа здорового питания завтра. Может, пора отдохнуть?".to_string(),
        trigger_type: "sleep".to_string(),
        urgency: "high".to_string(),
        cards: Some(vec![
            AiCard {
                title: "😴 Важность сна".to_string(),
                content: "7-8 часов сна помогают контролировать аппетит и вес".to_string(),
                emoji: Some("😴".to_string()),
                category: Some("health".to_string()),
                priority: Some("high".to_string()),
            },
            AiCard {
                title: "🛏️ Подготовка ко сну".to_string(),
                content: "Травяной чай и отказ от экранов за час до сна".to_string(),
                emoji: Some("🛏️".to_string()),
                category: Some("health".to_string()),
                priority: Some("medium".to_string()),
            },
        ]),
        suggestions: Some(vec![
            "Продукты для сна".to_string(),
            "Вечерние ритуалы".to_string(),
            "Режим отдыха".to_string(),
        ]),
    }
}
