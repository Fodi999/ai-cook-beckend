#!/bin/bash

# IT Cook Backend Runner
# Этот скрипт запускает бэкенд из папки backend

set -e

echo "🍽️ IT Cook Backend Runner"
echo "=========================="

# Проверяем, что мы в папке backend
if [ ! -f "Cargo.toml" ]; then
    echo "❌ Ошибка: файл 'Cargo.toml' не найден"
    echo "   Убедитесь, что вы запускаете скрипт из папки backend"
    exit 1
fi

echo "🔧 Запускаем IT Cook Backend..."
cargo run
