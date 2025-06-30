#!/bin/bash

# IT Cook Backend Build and Run Script

set -e

echo "🔨 Building IT Cook Backend..."

# Check if .env file exists
if [ ! -f .env ]; then
    echo "📝 Creating .env file from .env.example..."
    cp .env.example .env
    echo "⚠️  Please update the .env file with your actual configuration values"
fi

# Build the project
echo "🔧 Compiling..."
cargo build --release

echo "✅ Build completed successfully!"

# Check if DATABASE_URL is set
if [ -z "$DATABASE_URL" ]; then
    echo "⚠️  DATABASE_URL is not set. Please set it in your .env file or environment."
    echo "Example: DATABASE_URL=postgresql://user:password@localhost/itcook"
    exit 1
fi

echo "🚀 Starting IT Cook Backend server..."
cargo run --release
