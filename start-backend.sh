#!/bin/bash

# IT Cook Project Runner
# Этот скрипт запускает бэкенд из корневой папки проекта

set -e

echo "🍽️ IT Cook Project Runner"
echo "=========================="

# Проверяем, что мы в правильной папке
if [ ! -d "backend" ]; then
    echo "❌ Ошибка: папка 'backend' не найдена"
    echo "   Убедитесь, что вы запускаете скрипт из корневой папки проекта"
    exit 1
fi

echo "📁 Переходим в папку backend..."
cd backend

echo "🔧 Запускаем IT Cook Backend..."
cargo run
