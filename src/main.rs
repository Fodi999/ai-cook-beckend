use axum::{
    extract::Extension,
    http::{StatusCode, Method, HeaderValue, HeaderName},
    routing::{get},
    Router,
    middleware as axum_middleware,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing::{info, instrument};

mod api;
mod db;
mod models;
mod services;
mod utils;
mod config;
mod middleware;

use config::Config;
use services::ai::AiService;
use services::realtime::{WebSocketManager, RealtimeService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up panic hook to capture panics
    std::panic::set_hook(Box::new(|panic_info| {
        println!("💥 PANIC OCCURRED: {}", panic_info);
        if let Some(location) = panic_info.location() {
            println!("💥 Panic location: {}:{}", location.file(), location.line());
        }
        std::process::exit(1);
    }));

    // Initialize tracing FIRST
    println!("🔧 Initializing tracing subscriber...");
    
    // Try to initialize tracing with error handling
    match std::panic::catch_unwind(|| {
        tracing_subscriber::fmt::init();
    }) {
        Ok(_) => println!("✅ Tracing subscriber initialized"),
        Err(_) => {
            println!("❌ Failed to initialize tracing subscriber");
            return Err("Failed to initialize tracing".into());
        }
    }

    println!("🚀 IT Cook Backend initializing...");
    println!("🔍 Debug: Main function started successfully");
    
    // Check all environment variables
    println!("🔍 Debug: Checking environment variables...");
    println!("PORT: {}", std::env::var("PORT").unwrap_or("not set".to_string()));
    println!("RUST_LOG: {}", std::env::var("RUST_LOG").unwrap_or("not set".to_string()));
    println!("DATABASE_URL: {}", std::env::var("DATABASE_URL").map(|url| format!("{}...", &url[..std::cmp::min(50, url.len())])).unwrap_or("not set".to_string()));
    println!("JWT_SECRET: {}", std::env::var("JWT_SECRET").map(|secret| format!("{}...", &secret[..std::cmp::min(10, secret.len())])).unwrap_or("not set".to_string()));
    println!("SQLX_OFFLINE: {}", std::env::var("SQLX_OFFLINE").unwrap_or("not set".to_string()));
    
    // Load configuration
    println!("📁 Loading configuration...");
    let config = match Config::new() {
        Ok(config) => {
            println!("✅ Configuration loaded successfully");
            println!("🔗 Database URL: {}...", &config.database_url[..std::cmp::min(50, config.database_url.len())]);
            config
        },
        Err(e) => {
            println!("❌ Failed to load configuration: {}", e);
            return Err(e.into());
        }
    };
    
    // Initialize database
    println!("💾 Connecting to database...");
    println!("🔗 Database URL: {}...", &config.database_url[..std::cmp::min(50, config.database_url.len())]);
    let db_pool = match db::init_db(&config.database_url).await {
        Ok(pool) => {
            println!("✅ Database connected successfully");
            
            // Test database connection
            match sqlx::query("SELECT 1").fetch_one(&pool).await {
                Ok(_) => println!("✅ Database query test successful"),
                Err(e) => {
                    println!("❌ Database query test failed: {}", e);
                    return Err(format!("Database query test failed: {}", e).into());
                }
            }
            
            pool
        },
        Err(e) => {
            println!("❌ Failed to connect to database: {}", e);
            println!("🔍 Database URL format check: starts with 'postgresql://'? {}", config.database_url.starts_with("postgresql://"));
            return Err(format!("Database connection failed: {}", e).into());
        }
    };
    
    // Run migrations - закомментировано, так как миграции уже применены
    // sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Initialize WebSocket manager and realtime service
    let ws_manager = Arc::new(WebSocketManager::new());
    let realtime_service = Arc::new(RealtimeService::new(ws_manager.clone()));
    
    // Start cleanup task for inactive WebSocket connections
    realtime_service.start_cleanup_task();

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        // Публичные роуты аутентификации (не требуют токена)
        .nest("/api/v1/auth", api::auth::routes())
        // Публичные роуты для предустановленных данных холодильника
        // .nest("/api/v1/fridge", api::fridge::public_routes())
        // Защищенные роуты аутентификации (требуют токена)
        .nest("/api/v1/auth", api::auth::protected_routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        // Остальные защищенные роуты (требуют токена)
        .nest("/api/v1/diary", api::diary::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/fridge", api::fridge::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/recipes", api::recipes::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/goals", api::goals::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/community", api::community::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/realtime", api::websocket::routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/ai", ai_routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .nest("/api/v1/health", health_routes()
            .layer(axum_middleware::from_fn_with_state(db_pool.clone(), middleware::auth_middleware)))
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:3001".parse::<HeaderValue>().unwrap(),
                    "https://ai-cook-frontend.vercel.app".parse::<HeaderValue>().unwrap(),
                    "https://ai-cook-frontend-git-main-fodis-projects-dcba8b75.vercel.app".parse::<HeaderValue>().unwrap()
                ])
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
                .allow_headers([
                    HeaderName::from_static("authorization"),
                    HeaderName::from_static("content-type"),
                    HeaderName::from_static("x-requested-with"),
                ])
                .allow_credentials(true)
        )
        .layer(Extension(db_pool))
        .layer(Extension(config))
        .layer(Extension(ws_manager))
        .layer(Extension(realtime_service));

    // Получаем порт из переменной окружения PORT или используем значение по умолчанию
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);
    
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    
    println!("🚀 IT Cook Backend starting...");
    println!("📡 Server will listen on http://0.0.0.0:{}", port);
    println!("💾 Database connected and migrations applied");
    println!("🔌 WebSocket support enabled at ws://0.0.0.0:{}/api/v1/realtime/ws", port);
    
    println!("✅ IT Cook Backend is running successfully on PORT {}!", port);
    println!("🌐 Health check: http://0.0.0.0:{}/health", port);
    println!("📚 API docs: http://0.0.0.0:{}/api/v1", port);
    println!("🔧 CORS enabled for production");
    
    info!("🚀 IT Cook Backend starting...");
    info!("📡 Server will listen on http://0.0.0.0:{}", port);
    info!("💾 Database connected and migrations applied");
    info!("🔌 WebSocket support enabled at ws://0.0.0.0:{}/api/v1/realtime/ws", port);
    info!("✅ IT Cook Backend is running successfully!");
    info!("🌐 Health check: http://0.0.0.0:{}/health", port);
    info!("📚 API docs: http://0.0.0.0:{}/api/v1", port);
    
    println!("🌐 Starting server on http://0.0.0.0:{}", port);
    
    match axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await 
    {
        Ok(_) => {
            println!("✅ Server stopped gracefully");
            Ok(())
        },
        Err(e) => {
            println!("❌ Server error: {}", e);
            Err(e.into())
        }
    }
}

#[instrument]
async fn health_check() -> Result<String, StatusCode> {
    Ok("IT Cook Backend is running! 🍽️\n".to_string())
}

fn ai_routes() -> Router {
    use axum::routing::{get, post};
    
    Router::new()
        .route("/chat", post(api::ai::chat_with_ai))
        .route("/generate-recipe", post(api::ai::generate_recipe))
        .route("/analyze-nutrition", post(api::ai::analyze_nutrition))
        .route("/proactive-message", post(api::ai::generate_proactive_message))
        // Новые маршруты для интеграции с холодильником
        .route("/fridge/analyze", post(api::ai::analyze_fridge))
        .route("/fridge/recipes", post(api::ai::generate_fridge_recipes))
        .route("/fridge/report", get(api::ai::fridge_quick_report))
        .with_state(AiService::from_env())
}

fn health_routes() -> Router {
    use axum::routing::{get, post};
    
    Router::new()
        .route("/chat", post(api::personal_health::personal_health_chat))
        .route("/wellbeing", post(api::personal_health::daily_wellbeing_check))
        .route("/dashboard", get(api::personal_health::health_dashboard))
        .route("/recommendations", get(api::personal_health::get_recommendations))
        .route("/mood-analysis", post(api::personal_health::mood_analysis))
        .with_state(AiService::from_env())
}
