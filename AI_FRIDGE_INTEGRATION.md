# ИИ Интеграция с Умным Холодильником - IT Cook

## Описание

Интеграция ИИ-помощника с системой умного холодильника в IT Cook позволяет пользователям получать персонализированные отчеты, рекомендации и рецепты на основе содержимого их холодильника.

## Новые API Endpoints

### 1. Анализ Холодильника с ИИ
**POST** `/api/v1/ai/fridge/analyze`

Запрос полного анализа содержимого холодильника с ИИ-помощником.

**Headers:**
```
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Body:**
```json
{
  "analysis_type": "report", // "report", "recipes", "expiry", "waste", "shopping"
  "max_recipes": 3, // Опционально, максимальное количество рецептов
  "include_diet_check": true // Опционально, проверка диетических ограничений
}
```

**Response:**
```json
{
  "summary": "Общий анализ холодильника от ИИ",
  "recommendations": ["Список рекомендаций"],
  "recipes": [
    {
      "name": "Название рецепта",
      "description": "Описание",
      "ingredients": [...],
      "instructions": [...],
      "cook_time": "30 минут",
      "servings": 4,
      "difficulty": "Легко",
      "available_ingredients": ["Что есть в холодильнике"],
      "missing_ingredients": ["Что нужно купить"]
    }
  ],
  "alerts": [
    {
      "alert_type": "Expiring",
      "message": "Молоко истекает через 1 день",
      "item_name": "Молоко",
      "urgency": "High"
    }
  ],
  "insights": ["Полезные аналитические выводы"],
  "cards": [
    {
      "title": "📊 Общий анализ",
      "content": "Содержимое карточки",
      "emoji": "📊",
      "category": "fridge",
      "priority": "high"
    }
  ]
}
```

### 2. Генерация Рецептов из Холодильника
**POST** `/api/v1/ai/fridge/recipes`

Генерация рецептов на основе продуктов в холодильнике.

**Headers:**
```
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

**Body:**
```json
{
  "max_recipes": 5, // Опционально
  "difficulty": "easy", // "easy", "medium", "hard" (опционально)
  "max_cook_time": "30 minutes", // Опционально
  "dietary_restrictions": ["vegetarian", "gluten-free"] // Опционально
}
```

**Response:**
```json
{
  "recipes": [...], // Массив рецептов
  "missing_ingredients_summary": ["Общий список недостающих ингредиентов"],
  "shopping_suggestions": ["Рекомендации по покупкам"],
  "cards": [
    {
      "title": "🍽️ Куриная грудка с овощами",
      "content": "Быстрое блюдо | ⏱️ 20 минут | 👥 2 порции",
      "emoji": "🍽️",
      "category": "recipe",
      "priority": "high"
    }
  ]
}
```

### 3. Быстрый Отчет о Холодильнике
**GET** `/api/v1/ai/fridge/report`

Получение быстрого отчета о состоянии холодильника.

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "summary": "Краткий анализ состояния холодильника",
  "recommendations": ["Список рекомендаций"],
  "recipes": [...], // 2-3 рецепта
  "alerts": [...], // Уведомления
  "insights": [...], // Аналитика
  "cards": [...]
}
```

## Структуры Данных

### FridgeAlert
```json
{
  "alert_type": "Expiring" | "Expired" | "LowStock" | "WastePattern" | "DietViolation",
  "message": "Описание уведомления",
  "item_name": "Название продукта (опционально)",
  "urgency": "Critical" | "High" | "Medium" | "Low"
}
```

### GeneratedRecipe
```json
{
  "name": "Название рецепта",
  "description": "Описание рецепта",
  "ingredients": [
    {
      "name": "Название ингредиента",
      "amount": "количество",
      "unit": "единица измерения",
      "available_in_fridge": true
    }
  ],
  "instructions": ["Пошаговые инструкции"],
  "cook_time": "Время приготовления",
  "servings": 4,
  "difficulty": "Уровень сложности",
  "available_ingredients": ["Что есть в холодильнике"],
  "missing_ingredients": ["Что нужно купить"]
}
```

### AiCard
```json
{
  "title": "Заголовок карточки",
  "content": "Содержимое карточки",
  "emoji": "🍽️",
  "category": "fridge" | "recipe" | "alert" | "shopping",
  "priority": "high" | "medium" | "low"
}
```

## Примеры Использования

### 1. Получение полного отчета о холодильнике
```bash
curl -X GET "http://localhost:3002/api/v1/ai/fridge/report" \
  -H "Authorization: Bearer <jwt_token>"
```

### 2. Анализ с фокусом на рецепты
```bash
curl -X POST "http://localhost:3002/api/v1/ai/fridge/analyze" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "analysis_type": "recipes",
    "max_recipes": 5
  }'
```

### 3. Генерация простых рецептов
```bash
curl -X POST "http://localhost:3002/api/v1/ai/fridge/recipes" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "max_recipes": 3,
    "difficulty": "easy",
    "max_cook_time": "30 minutes"
  }'
```

### 4. Проверка на истечение срока годности
```bash
curl -X POST "http://localhost:3002/api/v1/ai/fridge/analyze" \
  -H "Authorization: Bearer <jwt_token>" \
  -H "Content-Type: application/json" \
  -d '{
    "analysis_type": "expiry"
  }'
```

## Интеграция с Чатом

Новые ИИ функции интегрированы в чат-интерфейс:

1. **Умные Карточки**: Ответы ИИ содержат структурированные карточки с информацией
2. **Персонализированные Рекомендации**: ИИ учитывает содержимое холодильника пользователя
3. **Проактивные Уведомления**: Система может предупреждать о скором истечении продуктов
4. **Рецепты по Запросу**: Пользователь может попросить рецепты в чате, и ИИ предложит варианты на основе имеющихся продуктов

## Особенности

1. **Mock Режим**: В текущей версии ИИ работает в mock-режиме, генерируя структурированные ответы
2. **Диетические Ограничения**: Система учитывает аллергены, непереносимости и диетические предпочтения пользователя
3. **Аналитика Отходов**: ИИ анализирует паттерны пищевых отходов и предлагает способы их сокращения
4. **Умная Аналитика**: Система отслеживает сроки годности и предлагает рецепты для скоропортящихся продуктов

## Статус

✅ **Реализовано:**
- Новые API endpoints для ИИ анализа холодильника
- Структуры данных для рецептов и уведомлений
- Интеграция с существующей системой холодильника
- Mock-генерация рецептов и отчетов

🔄 **В Разработке:**
- Интеграция с реальными LLM провайдерами (OpenAI, Groq)
- Улучшенный парсинг ответов ИИ
- Персонализация на основе профиля пользователя

📋 **Планируется:**
- Push-уведомления на основе ИИ анализа
- Интеграция с календарем питания
- Рекомендации по покупкам с учетом бюджета
