# IT Cook Backend 🍽️

Мощный бэкенд на Rust для приложения IT Cook - современной платформы для отслеживания питания, управления холодильником и кулинарного сообщества.

## 🚀 Возможности

### 🔐 Авторизация и безопасность
- JWT-токены с refresh механизмом
- Хеширование паролей с bcrypt
- Защищенные маршруты и middleware
- Управление сессиями

### 🍽 Питание и дневник БЖУ
- Отслеживание калорий, белков, жиров, углеводов
- Дневник питания с разбивкой по приемам пищи
- Еженедельная статистика и анализ
- База данных продуктов питания

### 🧊 Умный холодильник
- Управление продуктами с датами истечения
- Уведомления о скоропортящихся продуктах
- Категоризация по типам и местам хранения
- Предложения рецептов на основе доступных продуктов

### 📖 Система рецептов
- Создание и редактирование рецептов
- Рейтинги и избранное
- Поиск по ингредиентам, категориям, времени готовки
- Расчет пищевой ценности

### 📈 Цели и здоровье
- Постановка целей по весу, калориям, активности
- Автоматический расчет BMR/TDEE
- Отслеживание прогресса с визуализацией
- Система достижений

### 💬 Сообщество и соцсеть
- Лента постов в стиле Threads
- Публикация рецептов, фото, достижений
- Система лайков, комментариев, подписок
- Загрузка медиафайлов

### 🧠 AI-интеграции
- Генерация рецептов через OpenAI
- Умные рекомендации питания
- Анализ пищевых привычек
- Персонализированные советы

## 🛠 Технологический стек

- **Язык**: Rust 🦀
- **Web Framework**: Axum
- **База данных**: PostgreSQL с SQLx
- **Аутентификация**: JWT + bcrypt
- **Валидация**: validator
- **Логирование**: tracing
- **AI**: OpenAI API
- **Медиа**: Cloudinary/S3

## 📦 Структура проекта

```
it-cook/
├── backend/                    # 🔧 Rust бэкенд
│   ├── src/                   # Исходный код
│   │   ├── api/              # API endpoints
│   │   │   ├── auth.rs       # Авторизация
│   │   │   ├── diary.rs      # Дневник питания
│   │   │   ├── fridge.rs     # Холодильник
│   │   │   ├── recipes.rs    # Рецепты
│   │   │   ├── goals.rs      # Цели и здоровье
│   │   │   ├── community.rs  # Сообщество
│   │   │   └── websocket.rs  # WebSocket API
│   │   ├── models/           # Модели данных
│   │   │   ├── user.rs
│   │   │   ├── diary.rs
│   │   │   ├── fridge.rs
│   │   │   ├── recipe.rs
│   │   │   ├── goal.rs
│   │   │   └── community.rs
│   │   ├── services/         # Бизнес-логика
│   │   │   ├── auth.rs
│   │   │   ├── ai.rs         # AI интеграция
│   │   │   ├── realtime.rs   # WebSocket сервис
│   │   │   └── ...
│   │   ├── utils/            # Утилиты
│   │   ├── middleware/       # Middleware
│   │   ├── config.rs         # Конфигурация
│   │   ├── db.rs            # Подключение к БД
│   │   └── main.rs          # Точка входа
│   ├── migrations/           # Миграции PostgreSQL
│   ├── Cargo.toml           # Зависимости Rust
│   ├── .env                 # Переменные окружения
│   ├── .env.example         # Пример файла окружения
│   ├── run.sh              # Скрипт запуска
│   ├── DEVELOPMENT.md       # Документация разработки
│   ├── WEBSOCKET_GUIDE.md   # WebSocket документация
│   ├── PROJECT_STATUS.md    # Статус проекта
│   └── BACKEND_STATUS_REPORT.md # Отчёт по бэкенду
├── FRONTEND_GUIDE.md        # 📚 Документация для фронтенда
├── README_RU.md            # Основная документация
├── README_EN.md            # English documentation
├── README.md               # Project overview
└── start-backend.sh        # Скрипт запуска из корня
```
├── services/              # Бизнес-логика
│   ├── auth.rs
│   ├── diary.rs
│   ├── fridge.rs
│   ├── recipe.rs
│   ├── goal.rs
│   ├── community.rs
│   ├── ai.rs              # AI интеграции
│   ├── health.rs          # Расчеты здоровья
│   └── media.rs           # Загрузка файлов
├── middleware/            # Middleware
└── utils/                 # Утилиты
    └── errors.rs          # Обработка ошибок
```

## 🚀 Быстрый запуск

### Требования
- Rust 1.75+ и Cargo
- PostgreSQL 12+
- (Опционально) Docker и Docker Compose

### Установка и запуск

1. **Клонируйте репозиторий:**
   ```bash
   git clone https://github.com/your-username/it-cook.git
   cd it-cook
   ```

2. **Перейдите в папку бэкенда:**
   ```bash
   cd backend
   ```

3. **Настройте переменные окружения:**
   ```bash
   cp .env.example .env
   # Отредактируйте .env файл с вашими настройками
   ```

4. **Запустите сервер:**
   ```bash
   cargo run
   # или используйте скрипт
   ./run.sh
   ```

5. **Проверьте работоспособность:**
   ```bash
   curl http://localhost:3000/health
   ```

### Переменные окружения

```bash
# База данных
DATABASE_URL=postgresql://username:password@localhost/itcook_db

# JWT Secret (обязательно измените в продакшене!)
JWT_SECRET=your-super-secret-jwt-key

# Внешние AI API
GEMINI_API_KEY=your-gemini-api-key

# Медиа
CLOUDINARY_URL=cloudinary://api_key:api_secret@cloud_name
MEDIA_UPLOAD_DIR=uploads
MAX_FILE_SIZE=10485760

# Сервер
PORT=3000

# Логирование
RUST_LOG=info,itcook_backend=debug
```

## 📚 API Документация

### Основные эндпоинты

#### Авторизация
- `POST /api/v1/auth/register` - Регистрация
- `POST /api/v1/auth/login` - Вход
- `POST /api/v1/auth/refresh` - Обновление токена
- `GET /api/v1/auth/me` - Текущий пользователь

#### Дневник питания
- `POST /api/v1/diary` - Добавить запись
- `GET /api/v1/diary` - Получить записи
- `GET /api/v1/diary/summary/:date` - Сводка за день

#### Холодильник
- `POST /api/v1/fridge` - Добавить продукт
- `GET /api/v1/fridge` - Список продуктов
- `GET /api/v1/fridge/expiring` - Скоропортящиеся
- `GET /api/v1/fridge/suggestions` - Рецепты по продуктам

#### Рецепты
- `POST /api/v1/recipes` - Создать рецепт
- `GET /api/v1/recipes` - Список рецептов
- `POST /api/v1/recipes/generate` - AI генерация
- `GET /api/v1/recipes/popular` - Популярные

#### Цели и здоровье
- `POST /api/v1/goals` - Создать цель
- `GET /api/v1/goals/bmr` - Расчет BMR
- `GET /api/v1/goals/tdee` - Расчет TDEE
- `POST /api/v1/goals/weight` - Запись веса

#### Сообщество
- `POST /api/v1/community/posts` - Создать пост
- `GET /api/v1/community/posts` - Лента
- `POST /api/v1/community/posts/:id/like` - Лайк
- `GET /api/v1/community/trending` - Популярное

## 🔧 Разработка

### Запуск в режиме разработки

```bash
cargo watch -x run
```

### Тестирование

```bash
# Запуск тестов
cargo test

# Тесты с покрытием
cargo tarpaulin --out Html
```

### Проверка кода

```bash
# Линтинг
cargo clippy

# Форматирование
cargo fmt
```

## 🚀 Развертывание

### Docker

```bash
# Сборка образа
docker build -t itcook-backend .

# Запуск с Docker Compose
docker-compose up -d
```

### Продакшен

1. Настройте переменные окружения
2. Используйте PostgreSQL в продакшене
3. Настройте SSL/TLS
4. Настройте логирование
5. Используйте reverse proxy (nginx)

## 🤝 Вклад в проект

1. Форкните репозиторий
2. Создайте ветку для фичи (`git checkout -b feature/amazing-feature`)
3. Коммитите изменения (`git commit -m 'Add amazing feature'`)
4. Пушите ветку (`git push origin feature/amazing-feature`)
5. Откройте Pull Request

## 📝 Лицензия

Этот проект лицензирован под MIT License - см. файл [LICENSE](LICENSE) для деталей.

## 🎯 Дорожная карта

- [ ] WebSocket поддержка для реалтайм уведомлений
- [ ] Микросервисная архитектура
- [ ] GraphQL API
- [ ] Продвинутая AI аналитика
- [ ] Интеграция с фитнес-трекерами
- [ ] Многоязычность
- [ ] Мобильное API

## 📞 Поддержка

Если у вас есть вопросы или предложения, создайте issue в репозитории или свяжитесь с командой разработки.

---

**Сделано с ❤️ и Rust 🦀**
