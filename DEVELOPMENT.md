# IT Cook Backend - Development Guide

## Quick Start

1. **Clone and Setup**
   ```bash
   cd "/Users/dmitrijfomin/Desktop/it cook"
   cp .env.example .env
   # Edit .env with your database configuration
   ```

2. **Database Setup**
   ```bash
   # Create PostgreSQL database
   createdb itcook
   
   # Set DATABASE_URL in .env
   echo "DATABASE_URL=postgresql://username:password@localhost/itcook" >> .env
   ```

3. **Build and Run**
   ```bash
   ./run.sh
   # Or manually:
   cargo build --release
   cargo run --release
   ```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── config.rs            # Configuration management
├── db.rs               # Database connection setup
├── api/                # HTTP API endpoints
│   ├── auth.rs         # Authentication endpoints
│   ├── diary.rs        # Food diary endpoints
│   ├── fridge.rs       # Fridge management endpoints
│   ├── recipes.rs      # Recipe endpoints
│   ├── goals.rs        # Health goals endpoints
│   └── community.rs    # Community features endpoints
├── models/             # Data models
│   ├── user.rs         # User model and types
│   ├── diary.rs        # Diary entry models
│   ├── fridge.rs       # Fridge item models
│   ├── recipe.rs       # Recipe models
│   ├── goal.rs         # Goal models
│   └── community.rs    # Community models
├── services/           # Business logic services
│   ├── auth.rs         # Authentication service (JWT, password hashing)
│   ├── diary.rs        # Diary management service (implemented)
│   ├── fridge.rs       # Fridge management service (implemented)
│   ├── recipe.rs       # Recipe service
│   ├── goal.rs         # Goal tracking service
│   ├── community.rs    # Community service
│   ├── ai.rs           # AI service (OpenAI integration)
│   ├── health.rs       # Health calculations service
│   └── media.rs        # File upload service (implemented)
├── middleware/         # HTTP middleware
│   └── mod.rs          # JWT authentication middleware
└── utils/              # Utility modules
    └── errors.rs       # Error handling
```

## Implemented Features

### ✅ Authentication Service
- User registration with password hashing (bcrypt)
- Login with JWT token generation
- Token refresh functionality
- User profile management
- Session management with database storage

### ✅ Diary Service
- Add food diary entries with nutritional information
- Get entries by date, meal type, or user
- Update and delete entries
- Calculate daily nutrition summaries
- Weekly nutrition tracking
- Search functionality

### ✅ Fridge Service
- Add items to fridge with expiry dates
- Get items by category, location, or search
- Update item quantities and details
- Delete items
- Track expiring items
- Consume items (reduce quantities)

### ✅ Media Service
- File upload with size validation
- Support for multiple image formats
- User-specific file organization
- File deletion functionality
- Recipe image uploads

### ✅ AI Service (Basic)
- OpenAI integration structure
- Recipe suggestions from fridge items
- Mock responses when API key not configured
- Extensible for nutrition analysis and meal planning

## API Endpoints

### Authentication
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/refresh` - Refresh JWT token
- `POST /api/v1/auth/logout` - User logout
- `GET /api/v1/auth/profile` - Get user profile
- `PUT /api/v1/auth/profile` - Update user profile

### Food Diary
- `POST /api/v1/diary/entries` - Add diary entry
- `GET /api/v1/diary/entries` - Get user entries
- `GET /api/v1/diary/entries/{id}` - Get specific entry
- `PUT /api/v1/diary/entries/{id}` - Update entry
- `DELETE /api/v1/diary/entries/{id}` - Delete entry
- `GET /api/v1/diary/summary/{date}` - Get daily nutrition summary
- `GET /api/v1/diary/weekly` - Get weekly nutrition

### Fridge Management
- `POST /api/v1/fridge/items` - Add fridge item
- `GET /api/v1/fridge/items` - Get user items
- `GET /api/v1/fridge/items/{id}` - Get specific item
- `PUT /api/v1/fridge/items/{id}` - Update item
- `DELETE /api/v1/fridge/items/{id}` - Delete item
- `GET /api/v1/fridge/expiring` - Get expiring items
- `POST /api/v1/fridge/suggestions` - Get recipe suggestions

### Community (Endpoints defined, services pending)
- `GET /api/v1/community/posts` - Get community posts
- `POST /api/v1/community/posts` - Create post
- `POST /api/v1/community/posts/{id}/like` - Like post
- `POST /api/v1/community/upload` - Upload media

## Database Schema

The project includes comprehensive PostgreSQL migrations:

- **Users table**: Authentication and profile data
- **Diary entries**: Food consumption tracking
- **Fridge items**: Inventory management
- **Recipes**: Recipe storage and sharing
- **Goals**: Health and fitness goals
- **Community**: Posts, comments, likes, follows
- **User sessions**: JWT refresh token management

## Configuration

Environment variables (see `.env.example`):

```bash
DATABASE_URL=postgresql://user:password@localhost/itcook
JWT_SECRET=your-secret-key-here
OPENAI_API_KEY=your-openai-api-key (optional)
CLOUDINARY_URL=your-cloudinary-url (optional)
PORT=3000
```

## Next Steps

### 🔄 Pending Implementation
1. **Recipe Service**: Full recipe CRUD operations
2. **Goal Service**: Health goal tracking and progress
3. **Community Service**: Posts, comments, social features
4. **Health Service**: BMR, calorie calculations
5. **Real AI Integration**: OpenAI API implementation
6. **File Upload**: Cloudinary/S3 integration
7. **Testing**: Unit and integration tests
8. **Documentation**: API documentation with Swagger

### 🚀 Future Enhancements
1. **WebSocket Support**: Real-time notifications
2. **Microservices**: Split into separate services
3. **Caching**: Redis integration
4. **Search**: Elasticsearch for food/recipe search
5. **Analytics**: User behavior tracking
6. **Mobile API**: Optimized mobile endpoints

## Development Commands

```bash
# Check code quality
cargo check
cargo clippy

# Run tests (when implemented)
cargo test

# Build for production
cargo build --release

# Format code
cargo fmt

# Run migrations
sqlx migrate run

# Generate new migration
sqlx migrate add migration_name
```

## Health Check

The server exposes a health check endpoint:
- `GET /health` - Returns server status

Server runs on `http://localhost:3000` by default.

## Architecture Notes

- **Modular Design**: Clean separation between API, business logic, and data layers
- **Error Handling**: Comprehensive error types with proper HTTP status mapping
- **Security**: JWT authentication, password hashing, input validation
- **Database**: PostgreSQL with SQLx for type-safe queries
- **Async/Await**: Full async support with Tokio runtime
- **CORS**: Configured for frontend integration
- **Logging**: Structured logging with tracing crate
