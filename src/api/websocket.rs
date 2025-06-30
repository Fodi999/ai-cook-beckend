use std::sync::Arc;
use axum::{
    extract::{Extension, WebSocketUpgrade},
    response::Response,
    routing::get,
    Router,
};
use serde::Serialize;

use crate::services::{
    auth::Claims,
    realtime::{WebSocketManager, handle_websocket, RealtimeService},
};

pub fn routes() -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .route("/stats", get(get_realtime_stats))
}

/// WebSocket endpoint для подключения клиентов
async fn websocket_handler(
    ws: WebSocketUpgrade,
    claims: Claims,
    Extension(ws_manager): Extension<Arc<WebSocketManager>>,
) -> Response {
    ws.on_upgrade(move |socket| handle_websocket(socket, claims, ws_manager))
}

/// Получение статистики WebSocket подключений
async fn get_realtime_stats(
    _claims: Claims,
    Extension(realtime_service): Extension<Arc<RealtimeService>>,
) -> Result<axum::Json<RealtimeStatsResponse>, crate::utils::errors::AppError> {
    let stats = realtime_service.get_stats().await;
    
    Ok(axum::Json(RealtimeStatsResponse {
        connected_clients: stats.connected_clients,
        uptime: "Available in future version".to_string(),
        events_sent_today: 0, // TODO: Implement metrics
    }))
}

#[derive(Serialize)]
struct RealtimeStatsResponse {
    connected_clients: usize,
    uptime: String,
    events_sent_today: u64,
}
