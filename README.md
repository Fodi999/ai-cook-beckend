# IT Cook Backend 🍽️

A comprehensive nutrition and food management backend built with Rust, featuring food diary tracking, fridge management, recipe suggestions, and AI-powered meal planning.

## ✨ Features

### 🔐 Authentication & User Management
- Secure user registration and login with JWT
- Password hashing with bcrypt
- User profiles with health metrics
- Session management with refresh tokens

### 📱 Food Diary
- Track daily food consumption with detailed nutrition
- Meal categorization (breakfast, lunch, dinner, snacks)
- Daily and weekly nutrition summaries
- Search and filter food entries

### 🥬 Smart Fridge Management
- Track food inventory with expiry dates
- Categorize items by type and location
- Expiration alerts and notifications
- AI-powered recipe suggestions based on available ingredients

### 🤖 AI Integration
- Recipe suggestions from fridge contents
- Nutrition analysis for food items
- Meal planning assistance
- Cooking tips and recommendations

### 🍳 Recipe Management
- Store and organize recipes
- Difficulty levels and preparation times
- Ingredient tracking and substitutions
- Community recipe sharing

### 📁 Media Upload
- Image upload for recipes and profiles
- File validation and processing
- Organized file storage system

## 🚀 Quick Start

1. **Prerequisites**
   - Rust 1.70+
   - PostgreSQL 12+
   - Git

2. **Installation**
   ```bash
   git clone <repository-url>
   cd it-cook-backend
   cp .env.example .env
   # Edit .env with your database configuration
   ```

3. **Database Setup**
   ```bash
   createdb itcook
   # Update DATABASE_URL in .env
   ```

4. **Run the Server**
   ```bash
   ./run.sh
   # Or manually:
   cargo run --release
   ```

The server will start on `http://localhost:3000`

## 📡 API Endpoints

### Authentication
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login  
- `GET /api/v1/auth/profile` - Get user profile
- `PUT /api/v1/auth/profile` - Update profile

### Food Diary
- `POST /api/v1/diary/entries` - Add food entry
- `GET /api/v1/diary/entries` - Get diary entries
- `GET /api/v1/diary/summary/{date}` - Daily nutrition summary

### Fridge Management  
- `POST /api/v1/fridge/items` - Add fridge item
- `GET /api/v1/fridge/items` - Get fridge inventory
- `GET /api/v1/fridge/expiring` - Get expiring items
- `POST /api/v1/fridge/suggestions` - Get recipe suggestions

### Health Check
- `GET /health` - Server status

## 🏗️ Architecture

```
src/
├── api/           # HTTP endpoints and request handling
├── services/      # Business logic and external integrations  
├── models/        # Data structures and database models
├── middleware/    # Authentication and request processing
├── utils/         # Error handling and utilities
└── main.rs        # Application entry point
```

## 🛠️ Technology Stack

- **Framework**: Axum (async HTTP framework)
- **Database**: PostgreSQL with SQLx
- **Authentication**: JWT with bcrypt password hashing
- **AI Integration**: OpenAI API support
- **Logging**: Tracing for structured logging
- **Validation**: Request validation and sanitization

## 🔧 Configuration

Environment variables:

```bash
DATABASE_URL=postgresql://user:password@localhost/itcook
JWT_SECRET=your-jwt-secret-key
OPENAI_API_KEY=your-openai-api-key (optional)
PORT=3000
```

## 📊 Database Schema

The application uses PostgreSQL with migrations for:
- User accounts and authentication
- Food diary entries with nutrition data
- Fridge inventory management
- Recipe storage and metadata
- Health goals and progress tracking
- Community features (posts, comments, likes)

## 🔮 Development Status

### ✅ Implemented
- [x] Authentication service with JWT
- [x] Food diary with nutrition tracking
- [x] Fridge management system
- [x] Media upload functionality
- [x] Basic AI service structure
- [x] Database migrations and models
- [x] Error handling and validation
- [x] API route structure

### 🚧 In Progress
- [ ] Complete recipe CRUD operations
- [ ] Health goal tracking
- [ ] Community features
- [ ] Full OpenAI integration
- [ ] Image processing and optimization

### 📋 Planned
- [ ] WebSocket real-time features
- [ ] Advanced search functionality
- [ ] Analytics and reporting
- [ ] Mobile-optimized endpoints
- [ ] Microservices architecture

## 🧪 Testing

```bash
# Run tests (when implemented)
cargo test

# Check code quality
cargo check
cargo clippy

# Format code  
cargo fmt
```

## 📚 Documentation

- [Development Guide](DEVELOPMENT.md) - Detailed development information
- [API Documentation](docs/api.md) - Complete API reference (coming soon)

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Implement your changes
4. Add tests for new functionality
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🆘 Support

For questions and support:
- Create an issue on GitHub
- Check the [Development Guide](DEVELOPMENT.md)
- Review the API documentation

---

Built with ❤️ using Rust and modern web technologies.
