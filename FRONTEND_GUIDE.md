# IT Cook Backend - Frontend Developer Guide 🚀

Полное руководство для подключения фронтенда к Fridge  Backend API.

## 📋 Содержание

1. [Быстрый старт](#быстрый-старт)
2. [API Overview](#api-overview)
3. [Аутентификация](#аутентификация)
4. [API Endpoints](#api-endpoints)
5. [Модели данных](#модели-данных)
6. [Примеры запросов](#примеры-запросов)
7. [Обработка ошибок](#обработка-ошибок)
8. [WebSocket (планируется)](#websocket)

## 🚀 Быстрый старт

### Backend Setup

1. **Клонируйте и запустите бэкенд:**
```bash
git clone <repository-url>
cd it-cook/backend  # Весь бэкенд теперь в папке backend
cargo run
```

2. **API доступен по адресу:**
```
http://localhost:3000
```

3. **Health Check:**
```bash
curl http://localhost:3000/health
```

### Структура проекта:
```
it-cook/
├── backend/              # 🔧 Весь Rust бэкенд здесь
│   ├── src/             # Исходный код
│   ├── migrations/      # Миграции БД
│   ├── Cargo.toml       # Зависимости Rust
│   ├── .env             # Переменные окружения
│   ├── run.sh          # Скрипт запуска
│   ├── start-backend.sh # Альтернативный скрипт
│   ├── FRONTEND_GUIDE.md # Этот файл
│   ├── README_RU.md     # Документация
│   └── ...             # Другие файлы бэкенда
└── frontend/            # 🎨 Фронтенд (если есть)
```

### Запуск бэкенда:
```bash
# Из папки backend
cd backend
cargo run

# Или используя скрипт
./start-backend.sh
```

### Frontend Setup Examples

#### JavaScript/TypeScript
```typescript
const API_BASE_URL = 'http://localhost:3000/api/v1';

// Основной API клиент
class ITCookAPI {
  private baseURL = API_BASE_URL;
  private token: string | null = null;

  setToken(token: string) {
    this.token = token;
  }

  private async request(endpoint: string, options: RequestInit = {}) {
    const url = `${this.baseURL}${endpoint}`;
    const headers = {
      'Content-Type': 'application/json',
      ...(this.token && { 'Authorization': `Bearer ${this.token}` }),
      ...options.headers,
    };

    const response = await fetch(url, { ...options, headers });
    
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }
    
    return response.json();
  }

  // Auth methods
  async login(email: string, password: string) {
    return this.request('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  async register(userData: RegisterData) {
    return this.request('/auth/register', {
      method: 'POST',
      body: JSON.stringify(userData),
    });
  }
}
```

#### React Hook Example
```typescript
import { useState, useEffect } from 'react';

export const useAuth = () => {
  const [user, setUser] = useState(null);
  const [token, setToken] = useState(localStorage.getItem('token'));

  const login = async (email: string, password: string) => {
    const response = await api.login(email, password);
    setToken(response.token);
    setUser(response.user);
    localStorage.setItem('token', response.token);
  };

  return { user, token, login };
};
```

## 🔐 Аутентификация

### JWT Token Authentication

Все защищенные endpoints требуют JWT токен в заголовке:
```http
Authorization: Bearer your-jwt-token-here
```

### Flow аутентификации:

1. **Регистрация/Вход** → Получение `access_token` и `refresh_token`
2. **Использование** → Добавление `access_token` в заголовки
3. **Обновление** → Использование `refresh_token` для получения нового `access_token`

```typescript
// Пример автоматического обновления токена
const refreshToken = async () => {
  const response = await fetch('/api/v1/auth/refresh', {
    method: 'POST',
    headers: {
      'Authorization': `Bearer ${storedRefreshToken}`
    }
  });
  return response.json();
};
```

## 📡 API Endpoints

### Base URL
```
http://localhost:3000/api/v1
```

### 🔐 Authentication Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/auth/register` | Регистрация пользователя | ❌ |
| POST | `/auth/login` | Вход в систему | ❌ |
| POST | `/auth/refresh` | Обновление токена | ✅ |
| GET | `/auth/me` | Текущий пользователь | ✅ |
| POST | `/auth/logout` | Выход из системы | ✅ |

### 🍽 Food Diary Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/diary` | Добавить запись в дневник | ✅ |
| GET | `/diary` | Получить записи дневника | ✅ |
| GET | `/diary/{id}` | Получить конкретную запись | ✅ |
| PUT | `/diary/{id}` | Обновить запись | ✅ |
| DELETE | `/diary/{id}` | Удалить запись | ✅ |
| GET | `/diary/summary/{date}` | Сводка за день (YYYY-MM-DD) | ✅ |
| GET | `/diary/nutrition/week` | Недельная статистика | ✅ |

### 🧊 Fridge Management Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/fridge` | Добавить продукт в холодильник | ✅ |
| GET | `/fridge` | Список продуктов | ✅ |
| GET | `/fridge/{id}` | Получить продукт | ✅ |
| PUT | `/fridge/{id}` | Обновить продукт | ✅ |
| DELETE | `/fridge/{id}` | Удалить продукт | ✅ |
| GET | `/fridge/suggestions` | AI рекомендации рецептов | ✅ |
| GET | `/fridge/expiring` | Скоропортящиеся продукты | ✅ |
| GET | `/fridge/categories` | Категории продуктов | ✅ |

### 📖 Recipe Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/recipes` | Создать рецепт | ✅ |
| GET | `/recipes` | Список рецептов | ✅ |
| GET | `/recipes/{id}` | Получить рецепт | ✅ |
| PUT | `/recipes/{id}` | Обновить рецепт | ✅ |
| DELETE | `/recipes/{id}` | Удалить рецепт | ✅ |
| POST | `/recipes/{id}/favorite` | Добавить/убрать из избранного | ✅ |
| POST | `/recipes/{id}/rating` | Оценить рецепт | ✅ |
| GET | `/recipes/search` | Поиск рецептов | ✅ |
| POST | `/recipes/generate` | AI генерация рецепта | ✅ |
| GET | `/recipes/popular` | Популярные рецепты | ✅ |
| GET | `/recipes/favorites` | Избранные рецепты | ✅ |

### 📈 Health Goals Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/goals` | Создать цель | ✅ |
| GET | `/goals` | Список целей | ✅ |
| GET | `/goals/{id}` | Получить цель | ✅ |
| PUT | `/goals/{id}` | Обновить цель | ✅ |
| DELETE | `/goals/{id}` | Удалить цель | ✅ |
| POST | `/goals/{id}/progress` | Обновить прогресс | ✅ |
| POST | `/goals/weight` | Записать вес | ✅ |
| GET | `/goals/weight` | История веса | ✅ |
| GET | `/goals/bmr` | Расчет BMR | ✅ |
| GET | `/goals/tdee` | Расчет TDEE | ✅ |
| GET | `/goals/achievements` | Достижения | ✅ |
| GET | `/goals/stats` | Статистика здоровья | ✅ |

### 💬 Community Endpoints

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | `/community/posts` | Создать пост | ✅ |
| GET | `/community/posts` | Лента постов | ✅ |
| GET | `/community/posts/{id}` | Получить пост | ✅ |
| PUT | `/community/posts/{id}` | Обновить пост | ✅ |
| DELETE | `/community/posts/{id}` | Удалить пост | ✅ |
| POST | `/community/posts/{id}/like` | Лайк/дизлайк | ✅ |
| POST | `/community/posts/{id}/comments` | Добавить комментарий | ✅ |
| GET | `/community/posts/{id}/comments` | Получить комментарии | ✅ |
| PUT | `/community/comments/{id}` | Обновить комментарий | ✅ |
| DELETE | `/community/comments/{id}` | Удалить комментарий | ✅ |
| POST | `/community/users/{id}/follow` | Подписаться/отписаться | ✅ |
| GET | `/community/users/{id}/posts` | Посты пользователя | ✅ |
| GET | `/community/users/{id}/followers` | Подписчики | ✅ |
| GET | `/community/users/{id}/following` | Подписки | ✅ |
| GET | `/community/trending` | Популярные посты | ✅ |
| POST | `/community/upload` | Загрузить медиа | ✅ |

## 📊 Модели данных

### User Model
```typescript
interface User {
  id: string;
  email: string;
  first_name: string;
  last_name: string;
  date_of_birth?: string;
  gender?: string;
  height?: number; // см
  weight?: number; // кг
  activity_level?: 'sedentary' | 'lightly_active' | 'moderately_active' | 'very_active' | 'extremely_active';
  role: 'user' | 'admin' | 'moderator';
  avatar_url?: string;
  is_verified: boolean;
  email_verified_at?: string;
  last_login_at?: string;
  created_at: string;
  updated_at: string;
}
```

### DiaryEntry Model
```typescript
interface DiaryEntry {
  id: string;
  user_id: string;
  food_item_id: string;
  meal_type: 'breakfast' | 'lunch' | 'dinner' | 'snack';
  quantity: number;
  unit: string;
  calories: number;
  protein: number;
  fat: number;
  carbs: number;
  fiber?: number;
  sugar?: number;
  sodium?: number;
  eaten_at: string;
  notes?: string;
  created_at: string;
  updated_at: string;
}
```

### FridgeItem Model
```typescript
interface FridgeItem {
  id: string;
  user_id: string;
  name: string;
  brand?: string;
  quantity: number;
  unit: string;
  category: 'vegetables' | 'fruits' | 'meat' | 'dairy' | 'grains' | 'beverages' | 'other';
  expiry_date?: string;
  purchase_date?: string;
  notes?: string;
  location?: 'fridge' | 'freezer' | 'pantry';
  created_at: string;
  updated_at: string;
}
```

### Recipe Model
```typescript
interface Recipe {
  id: string;
  user_id: string;
  title: string;
  description?: string;
  instructions: string[];
  prep_time?: number; // минуты
  cook_time?: number; // минуты
  servings?: number;
  difficulty: 'easy' | 'medium' | 'hard';
  category: 'breakfast' | 'lunch' | 'dinner' | 'snack' | 'dessert' | 'appetizer';
  cuisine?: string;
  tags: string[];
  image_url?: string;
  nutrition?: NutritionInfo;
  ingredients: RecipeIngredient[];
  is_public: boolean;
  rating_avg?: number;
  rating_count: number;
  created_at: string;
  updated_at: string;
}

interface RecipeIngredient {
  id: string;
  recipe_id: string;
  food_item_id: string;
  quantity: number;
  unit: string;
  notes?: string;
}
```

### Goal Model
```typescript
interface Goal {
  id: string;
  user_id: string;
  title: string;
  description?: string;
  goal_type: 'weight' | 'calories' | 'exercise' | 'water' | 'sleep' | 'custom';
  target_value: number;
  current_value?: number;
  unit: string;
  target_date?: string;
  daily_target?: number;
  weekly_target?: number;
  status: 'active' | 'completed' | 'paused' | 'cancelled';
  created_at: string;
  updated_at: string;
}
```

### Post Model
```typescript
interface Post {
  id: string;
  user_id: string;
  content: string;
  post_type: 'recipe' | 'achievement' | 'general' | 'question';
  recipe_id?: string;
  goal_id?: string;
  media_urls: string[];
  likes_count: number;
  comments_count: number;
  is_liked?: boolean; // for current user
  created_at: string;
  updated_at: string;
  user: User; // populated
}
```

## 🔍 Примеры запросов

### Authentication Examples

#### Регистрация
```typescript
const register = async (userData: {
  email: string;
  password: string;
  first_name: string;
  last_name: string;
  date_of_birth?: string;
  gender?: string;
  height?: number;
  weight?: number;
  activity_level?: string;
}) => {
  const response = await fetch('/api/v1/auth/register', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(userData),
  });
  return response.json();
};

// Response:
{
  "user": { /* User object */ },
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "refresh_token": "refresh_token_here"
}
```

#### Вход
```typescript
const login = async (email: string, password: string) => {
  const response = await fetch('/api/v1/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ email, password }),
  });
  return response.json();
};
```

### Diary Examples

#### Добавить запись в дневник
```typescript
const addDiaryEntry = async (entry: {
  food_item_id: string;
  meal_type: 'breakfast' | 'lunch' | 'dinner' | 'snack';
  quantity: number;
  unit: string;
  eaten_at?: string; // ISO timestamp
  notes?: string;
}) => {
  const response = await fetch('/api/v1/diary', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(entry),
  });
  return response.json();
};
```

#### Получить дневник за день
```typescript
const getDailySummary = async (date: string) => {
  const response = await fetch(`/api/v1/diary/summary/${date}`, {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return response.json();
};

// Response:
{
  "date": "2025-06-30",
  "total_calories": 1850,
  "total_protein": 85.5,
  "total_fat": 65.2,
  "total_carbs": 220.8,
  "entries": [/* DiaryEntry objects */],
  "goals": {
    "calories": 2000,
    "protein": 100,
    "fat": 70,
    "carbs": 250
  }
}
```

### Fridge Examples

#### Добавить продукт в холодильник
```typescript
const addFridgeItem = async (item: {
  name: string;
  brand?: string;
  quantity: number;
  unit: string;
  category: string;
  expiry_date?: string;
  location?: string;
  notes?: string;
}) => {
  const response = await fetch('/api/v1/fridge', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(item),
  });
  return response.json();
};
```

#### Получить AI рекомендации рецептов
```typescript
const getRecipeSuggestions = async () => {
  const response = await fetch('/api/v1/fridge/suggestions', {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return response.json();
};

// Response:
{
  "suggestions": [
    {
      "title": "Паста с курицей и овощами",
      "description": "Быстрый и полезный ужин",
      "ingredients_available": ["курица", "помидоры", "лук"],
      "ingredients_missing": ["паста", "сыр"],
      "difficulty": "easy",
      "prep_time": 25
    }
  ]
}
```

### Recipe Examples

#### Создать рецепт
```typescript
const createRecipe = async (recipe: {
  title: string;
  description?: string;
  instructions: string[];
  prep_time?: number;
  cook_time?: number;
  servings?: number;
  difficulty: 'easy' | 'medium' | 'hard';
  category: string;
  cuisine?: string;
  tags: string[];
  ingredients: Array<{
    food_item_id: string;
    quantity: number;
    unit: string;
    notes?: string;
  }>;
  is_public: boolean;
}) => {
  const response = await fetch('/api/v1/recipes', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(recipe),
  });
  return response.json();
};
```

#### AI генерация рецепта
```typescript
const generateRecipe = async (prompt: {
  ingredients?: string[];
  cuisine?: string;
  dietary_restrictions?: string[];
  prep_time?: number;
  difficulty?: string;
  servings?: number;
}) => {
  const response = await fetch('/api/v1/recipes/generate', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(prompt),
  });
  return response.json();
};
```

### Goals Examples

#### Создать цель
```typescript
const createGoal = async (goal: {
  title: string;
  description?: string;
  goal_type: 'weight' | 'calories' | 'exercise' | 'water' | 'sleep' | 'custom';
  target_value: number;
  unit: string;
  target_date?: string;
  daily_target?: number;
  weekly_target?: number;
}) => {
  const response = await fetch('/api/v1/goals', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(goal),
  });
  return response.json();
};
```

#### Расчет BMR и TDEE
```typescript
const calculateBMR = async () => {
  const response = await fetch('/api/v1/goals/bmr', {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return response.json();
};

// Response:
{
  "bmr": 1650.5,
  "method": "mifflin_st_jeor",
  "user_data": {
    "age": 25,
    "weight": 70,
    "height": 175,
    "gender": "male"
  }
}

const calculateTDEE = async () => {
  const response = await fetch('/api/v1/goals/tdee', {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return response.json();
};

// Response:
{
  "tdee": 2310.7,
  "bmr": 1650.5,
  "activity_multiplier": 1.4,
  "activity_level": "lightly_active"
}
```

### Community Examples

#### Создать пост
```typescript
const createPost = async (post: {
  content: string;
  post_type: 'recipe' | 'achievement' | 'general' | 'question';
  recipe_id?: string;
  goal_id?: string;
  media_urls?: string[];
}) => {
  const response = await fetch('/api/v1/community/posts', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`,
    },
    body: JSON.stringify(post),
  });
  return response.json();
};
```

#### Получить ленту
```typescript
const getFeed = async (params?: {
  limit?: number;
  offset?: number;
  post_type?: string;
}) => {
  const searchParams = new URLSearchParams(params);
  const response = await fetch(`/api/v1/community/posts?${searchParams}`, {
    headers: { 'Authorization': `Bearer ${token}` },
  });
  return response.json();
};
```

## ⚠️ Обработка ошибок

API возвращает ошибки в стандартном формате:

```typescript
interface APIError {
  error: string;
  message: string;
  details?: any;
}
```

### HTTP Status Codes

| Code | Description |
|------|-------------|
| 200 | Успешный запрос |
| 201 | Ресурс создан |
| 400 | Неверный запрос |
| 401 | Не авторизован |
| 403 | Доступ запрещен |
| 404 | Ресурс не найден |
| 422 | Ошибка валидации |
| 500 | Внутренняя ошибка сервера |

### Примеры ошибок

```typescript
// 401 Unauthorized
{
  "error": "Unauthorized",
  "message": "Missing authorization token"
}

// 422 Validation Error
{
  "error": "ValidationError",
  "message": "Invalid input data",
  "details": {
    "email": ["Email is required"],
    "password": ["Password must be at least 6 characters"]
  }
}

// 404 Not Found
{
  "error": "NotFound",
  "message": "Recipe not found"
}
```

### Error Handling в клиенте

```typescript
const handleAPIError = (error: any) => {
  if (error.status === 401) {
    // Redirect to login
    window.location.href = '/login';
  } else if (error.status === 422) {
    // Show validation errors
    showValidationErrors(error.details);
  } else {
    // Show generic error
    showError(error.message);
  }
};

const apiCall = async () => {
  try {
    const response = await fetch('/api/v1/some-endpoint');
    if (!response.ok) {
      const error = await response.json();
      handleAPIError({ ...error, status: response.status });
      return;
    }
    return response.json();
  } catch (error) {
    handleAPIError({ message: 'Network error', status: 0 });
  }
};
```

## 🔮 Query Parameters

Многие GET endpoints поддерживают query parameters для фильтрации и пагинации:

### Общие параметры
```typescript
interface CommonQueryParams {
  limit?: number;      // количество элементов (default: 20, max: 100)
  offset?: number;     // сдвиг для пагинации (default: 0)
  sort_by?: string;    // поле для сортировки
  sort_order?: 'asc' | 'desc'; // направление сортировки
}
```

### Diary endpoints
```typescript
// GET /api/v1/diary
interface DiaryQueryParams extends CommonQueryParams {
  date_from?: string;  // YYYY-MM-DD
  date_to?: string;    // YYYY-MM-DD
  meal_type?: 'breakfast' | 'lunch' | 'dinner' | 'snack';
}
```

### Recipe endpoints
```typescript
// GET /api/v1/recipes/search
interface RecipeSearchParams extends CommonQueryParams {
  q?: string;          // поисковый запрос
  category?: string;   // категория рецепта
  difficulty?: 'easy' | 'medium' | 'hard';
  max_prep_time?: number; // максимальное время приготовления
  cuisine?: string;    // кухня
  tags?: string[];     // теги (comma-separated)
}
```

## 🚀 WebSocket (Планируется)

В будущих версиях будет добавлена поддержка WebSocket для:

- Real-time уведомления
- Live updates ленты сообщества
- Уведомления о скоропортящихся продуктах
- Синхронизация между устройствами

```typescript
// Планируемый API для WebSocket
const socket = new WebSocket('ws://localhost:3000/ws');

socket.onmessage = (event) => {
  const data = JSON.parse(event.data);
  switch (data.type) {
    case 'notification':
      showNotification(data.payload);
      break;
    case 'feed_update':
      updateFeed(data.payload);
      break;
  }
};
```

## 🔌 WebSocket Real-time Support

### Новая возможность: Real-time уведомления!

#### WebSocket Endpoint
```
ws://localhost:3000/api/v1/realtime/ws
```

#### Типы событий:
- **NewCommunityPost** - Новые посты в сообществе
- **PostLiked** - Лайки постов
- **ExpiringItems** - Уведомления о скоропортящихся продуктах  
- **GoalAchieved** - Достижения целей
- **SystemNotification** - Системные уведомления
- **Heartbeat** - Проверка соединения

#### JavaScript подключение:
```javascript
const ws = new WebSocket('ws://localhost:3000/api/v1/realtime/ws');
ws.onopen = () => console.log('🔌 Connected to IT Cook real-time');
ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('Real-time event:', data);
};
```

📚 **Подробная документация**: См. файл `WEBSOCKET_GUIDE.md`

---

## 🛠️ Полезные утилиты

### TypeScript типы
```typescript
// Сохраните в types/api.ts
export interface ApiResponse<T> {
  data: T;
  message?: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  total: number;
  limit: number;
  offset: number;
  has_more: boolean;
}

export interface LoginResponse {
  user: User;
  token: string;
  refresh_token: string;
}
```

### React Context для API
```typescript
// contexts/ApiContext.tsx
import React, { createContext, useContext, useCallback } from 'react';

interface ApiContextType {
  request: (endpoint: string, options?: RequestInit) => Promise<any>;
  setToken: (token: string) => void;
}

const ApiContext = createContext<ApiContextType | null>(null);

export const ApiProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [token, setTokenState] = useState<string | null>(
    localStorage.getItem('token')
  );

  const request = useCallback(async (endpoint: string, options: RequestInit = {}) => {
    const url = `${API_BASE_URL}${endpoint}`;
    const headers = {
      'Content-Type': 'application/json',
      ...(token && { 'Authorization': `Bearer ${token}` }),
      ...options.headers,
    };

    const response = await fetch(url, { ...options, headers });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(error.message || 'API Error');
    }
    
    return response.json();
  }, [token]);

  const setToken = useCallback((newToken: string) => {
    setTokenState(newToken);
    localStorage.setItem('token', newToken);
  }, []);

  return (
    <ApiContext.Provider value={{ request, setToken }}>
      {children}
    </ApiContext.Provider>
  );
};

export const useApi = () => {
  const context = useContext(ApiContext);
  if (!context) {
    throw new Error('useApi must be used within ApiProvider');
  }
  return context;
};
```

## 📱 Рекомендации для мобильных приложений

### React Native
```typescript
// Используйте AsyncStorage вместо localStorage
import AsyncStorage from '@react-native-async-storage/async-storage';

const storeToken = async (token: string) => {
  await AsyncStorage.setItem('token', token);
};

const getToken = async () => {
  return await AsyncStorage.getItem('token');
};
```

### Flutter
```dart
// pubspec.yaml
dependencies:
  http: ^0.13.0
  shared_preferences: ^2.0.0

// api_service.dart
class ApiService {
  static const String baseUrl = 'http://localhost:3000/api/v1';
  
  Future<Map<String, dynamic>> login(String email, String password) async {
    final response = await http.post(
      Uri.parse('$baseUrl/auth/login'),
      headers: {'Content-Type': 'application/json'},
      body: jsonEncode({'email': email, 'password': password}),
    );
    
    if (response.statusCode == 200) {
      return jsonDecode(response.body);
    } else {
      throw Exception('Login failed');
    }
  }
}
```

## 🧪 Тестирование API

### Postman Collection
Создайте Postman коллекцию для тестирования:

```json
{
  "info": {
    "name": "IT Cook Backend API",
    "description": "API collection for IT Cook Backend"
  },
  "variable": [
    {
      "key": "baseUrl",
      "value": "http://localhost:3000/api/v1"
    },
    {
      "key": "token",
      "value": ""
    }
  ]
}
```

### Curl Examples
```bash
# Health check
curl http://localhost:3000/health

# Login
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'

# Get diary entries
curl http://localhost:3000/api/v1/diary \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"

# Add fridge item
curl -X POST http://localhost:3000/api/v1/fridge \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer YOUR_TOKEN_HERE" \
  -d '{"name":"Milk","quantity":1,"unit":"liter","category":"dairy"}'
```

## 🔧 Примеры тестирования API

### Проверка работоспособности
```bash
# Проверка что сервер запущен
curl http://localhost:3000/health
# Ожидаемый ответ: "IT Cook Backend is running! 🍽️"

# Запуск бэкенда из папки backend
cd backend
cargo run

# Или используя скрипт
./start-backend.sh
```

# Проверка регистрации (если пользователь уже существует)
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123","first_name":"Test","last_name":"User"}'
# Ожидаемый ответ при существующем пользователе: {"error":{"details":"Bad request: Email already registered","message":"Bad request"}}

# Успешный логин
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"password123"}'
# Ожидаемый ответ: {"access_token":"...","refresh_token":"...","user":{...}}
```
