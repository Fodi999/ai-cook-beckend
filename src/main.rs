use axum::{
    extract::Extension,
    http::{StatusCode, Method, HeaderValue, HeaderName},
    routing::{get},
    Router,
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
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load configuration
    let config = Config::new()?;
    
    // Initialize database
    let db_pool = db::init_db(&config.database_url).await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&db_pool).await?;

    // Initialize WebSocket manager and realtime service
    let ws_manager = Arc::new(WebSocketManager::new());
    let realtime_service = Arc::new(RealtimeService::new(ws_manager.clone()));
    
    // Start cleanup task for inactive WebSocket connections
    realtime_service.start_cleanup_task();

    // Build our application with routes
    let app = Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1/auth", api::auth::routes())
        .nest("/api/v1/diary", api::diary::routes())
        .nest("/api/v1/fridge", api::fridge::routes())
        .nest("/api/v1/recipes", api::recipes::routes())
        .nest("/api/v1/goals", api::goals::routes())
        .nest("/api/v1/community", api::community::routes())
        .nest("/api/v1/realtime", api::websocket::routes())
        .nest("/api/v1/ai", ai_routes())
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                    "http://localhost:3001".parse::<HeaderValue>().unwrap()
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

    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    
    println!("🚀 IT Cook Backend starting...");
    println!("📡 Server will listen on http://localhost:3002");
    println!("💾 Database connected and migrations applied");
    println!("🔌 WebSocket support enabled at ws://localhost:3002/api/v1/realtime/ws");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    
    println!("✅ IT Cook Backend is running successfully on PORT 3002!");
    println!("🌐 Health check: http://localhost:3002/health");
    println!("📚 API docs: http://localhost:3002/api/v1");
    println!("🔧 CORS enabled for http://localhost:3000");
    
    info!("🚀 IT Cook Backend starting...");
    info!("📡 Server will listen on http://localhost:3002");
    info!("💾 Database connected and migrations applied");
    info!("🔌 WebSocket support enabled at ws://localhost:3002/api/v1/realtime/ws");
    info!("✅ IT Cook Backend is running successfully!");
    info!("🌐 Health check: http://localhost:3002/health");
    info!("📚 API docs: http://localhost:3002/api/v1");
    
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn health_check() -> Result<String, StatusCode> {
    Ok("IT Cook Backend is running! 🍽️\n".to_string())
}

fn ai_routes() -> Router {
    use axum::routing::post;
    
    Router::new()
        .route("/chat", post(api::ai::chat_with_ai))
        .route("/generate-recipe", post(api::ai::generate_recipe))
        .route("/analyze-nutrition", post(api::ai::analyze_nutrition))
        .with_state(AiService::from_env())
}
