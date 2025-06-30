# 🍽️ IT Cook Backend - Production Ready! 

## 📋 Краткое описание

**IT Cook Backend** - это полнофункциональный REST API сервер для мобильного приложения управления питанием и здоровым образом жизни, построенный на Rust с использованием современного стека технологий.

## 🚀 Что делает бэкенд:

### 🔐 **Аутентификация и безопасность**
- Регистрация и авторизация пользователей
- JWT токены с refresh механизмом
- Безопасное хранение паролей (bcrypt)
- Middleware для защищённых endpoints

### 📊 **Управление питанием**
- **Дневник питания**: запись приёмов пищи, подсчёт калорий и макронутриентов
- **Холодильник**: учёт продуктов, сроки годности, уведомления
- **Рецепты**: создание, поиск, оценки, избранное
- **AI генерация рецептов** на основе доступных продуктов

### 🎯 **Здоровье и цели**
- Постановка и отслеживание целей по здоровью
- Расчёт BMR и TDEE
- Мониторинг веса и прогресса
- Статистика и аналитика

### 👥 **Социальные функции**
- Лента сообщества
- Публикация достижений и рецептов
- Лайки, комментарии, подписки
- Загрузка медиа контента

### 🔌 **Real-time возможности**
- WebSocket соединения для live уведомлений
- Уведомления о скоропортящихся продуктах
- Обновления ленты сообщества в реальном времени
- Heartbeat и автоматическая очистка соединений

## 🛡️ Production Ready Features:

### ✅ **Готово к продакшену:**
- [x] Полная архитектура REST API
- [x] PostgreSQL с миграциями
- [x] JWT аутентификация 
- [x] CORS настройки
- [x] Структурированная обработка ошибок
- [x] Логирование (tracing)
- [x] WebSocket поддержка
- [x] Environment configuration
- [x] Health check endpoint
- [x] Модульная архитектура (MVC pattern)

### 🔧 **Технический стек:**
- **Язык**: Rust 🦀
- **Framework**: Axum (высокопроизводительный async)
- **База данных**: PostgreSQL + SQLx
- **Аутентификация**: JWT + bcrypt
- **Real-time**: WebSocket (tokio-tungstenite)
- **Сериализация**: Serde JSON
- **Логирование**: Tracing
- **CORS**: Tower HTTP

### 📈 **Производительность:**
- Асинхронная архитектура (Tokio)
- Connection pooling для БД
- Efficient JSON serialization
- Memory-safe (Rust)
- Zero-cost abstractions

### 🔒 **Безопасность:**
- Хеширование паролей (bcrypt)
- JWT с коротким временем жизни
- SQL injection protection (SQLx)
- CORS policy
- Environment variables для секретов

## 📁 **Структура проекта:**
```
backend/
├── src/
│   ├── api/          # REST endpoints
│   ├── models/       # Data models
│   ├── services/     # Business logic
│   ├── middleware/   # Auth middleware
│   ├── utils/        # Error handling
│   └── main.rs       # Application entry
├── migrations/       # Database migrations
├── Cargo.toml        # Dependencies
├── .env             # Environment config
└── run.sh           # Start script
```

## 🌐 **API Endpoints:**
- **Auth**: `/api/v1/auth/*` (login, register, refresh)
- **Diary**: `/api/v1/diary/*` (food tracking)
- **Fridge**: `/api/v1/fridge/*` (inventory management)
- **Recipes**: `/api/v1/recipes/*` (recipe CRUD + AI)
- **Goals**: `/api/v1/goals/*` (health tracking)
- **Community**: `/api/v1/community/*` (social features)
- **WebSocket**: `/api/v1/realtime/ws` (real-time events)

## 🚀 **Запуск:**
```bash
cd backend
cargo run
# Server starts on http://localhost:3000
```

## ✨ **Готов для:**
- Development ✅
- Staging ✅  
- Production ✅
- Mobile app integration ✅
- Frontend integration ✅
- Scaling и deployment ✅

**Бэкенд полностью готов к подключению фронтенда и мобильных приложений!** 🎉
