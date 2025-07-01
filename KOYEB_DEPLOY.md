# Деплой IT Cook Backend на Koyeb

## Шаг 1: Подготовка репозитория

1. Убедитесь, что backend загружен на GitHub
2. Файлы готовы: `Dockerfile`, `koyeb.yml`, `.dockerignore`

## Шаг 2: Настройка базы данных PostgreSQL

Создайте PostgreSQL базу данных на одном из провайдеров:
- **Neon** (рекомендуется): https://neon.tech/
- **Supabase**: https://supabase.com/
- **Railway**: https://railway.app/

Получите `DATABASE_URL` в формате:
```
postgresql://username:password@hostname:port/database_name
```

## Шаг 3: Регистрация на Koyeb

1. Перейдите на https://koyeb.com/
2. Зарегистрируйтесь или войдите в аккаунт
3. Подключите GitHub аккаунт

## Шаг 4: Создание приложения на Koyeb

1. Нажмите "Create App"
2. Выберите "Deploy from GitHub"
3. Выберите репозиторий с backend
4. Настройте параметры:

### Build настройки:
- **Source**: GitHub repository
- **Branch**: main/master
- **Build command**: Автоматически определится из Dockerfile
- **Run command**: `./itcook-backend`

### Environment Variables:
Добавьте следующие переменные окружения:

```bash
DATABASE_URL=postgresql://your_db_connection_string
JWT_SECRET=your-secure-jwt-secret-key-min-32-chars
OPENAI_API_KEY=your-openai-api-key (опционально)
CLOUDINARY_URL=your-cloudinary-url (опционально)
RUST_LOG=info
ITCOOK_PORT=3000
```

### Instance настройки:
- **Region**: Frankfurt (fra) или ближайший
- **Instance type**: Nano (для начала) или Micro
- **Port**: 3000
- **Health check**: 
  - Path: `/health`
  - Port: 3000

## Шаг 5: Деплой

1. Нажмите "Deploy"
2. Дождитесь завершения build процесса
3. Проверьте логи на наличие ошибок

## Шаг 6: Проверка

После успешного деплоя:

1. **Health check**: `https://your-app.koyeb.app/health`
2. **API документация**: `https://your-app.koyeb.app/api/v1/`

## Шаг 7: Обновление фронтенда

Обновите переменные окружения фронтенда на Vercel:

```bash
NEXT_PUBLIC_API_URL=https://your-app.koyeb.app
```

## Возможные проблемы и решения

### 1. Build fails
- Проверьте Dockerfile
- Убедитесь, что все зависимости указаны в Cargo.toml

### 2. Database connection error
- Проверьте DATABASE_URL
- Убедитесь, что база данных доступна

### 3. Health check fails
- Проверьте, что приложение запускается на порту 3000
- Убедитесь, что endpoint `/health` доступен

### 4. Memory/CPU limits
- Увеличьте instance type с Nano на Micro или Small

## Полезные команды

### Локальная проверка Docker образа:
```bash
# Build
docker build -t itcook-backend .

# Run
docker run -p 3000:3000 \
  -e DATABASE_URL="your_db_url" \
  -e JWT_SECRET="your_jwt_secret" \
  itcook-backend

# Test
curl http://localhost:3000/health
```

### Проверка переменных окружения:
```bash
# В контейнере
echo $DATABASE_URL
echo $JWT_SECRET
echo $ITCOOK_PORT
```

## Мониторинг

После деплоя следите за:
- CPU и Memory usage в Koyeb dashboard
- Логи приложения
- Response time API endpoints
- Database connections

## Следующие шаги

1. Настройте домен (опционально)
2. Настройте SSL сертификат (автоматически)
3. Настройте мониторинг и алерты
4. Проведите нагрузочное тестирование
