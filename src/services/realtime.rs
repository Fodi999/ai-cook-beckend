use std::collections::HashMap;
use std::sync::Arc;
use axum::extract::WebSocketUpgrade;
use axum::extract::ws::{Message, WebSocket};
use axum::response::Response;
use futures_util::{sink::SinkExt, stream::StreamExt};
use tokio::sync::{broadcast, RwLock};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tracing::{info, warn, error};

use crate::services::auth::Claims;
use crate::utils::errors::AppError;

/// Типы WebSocket событий
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum WebSocketEvent {
    /// Новое сообщение в чате сообщества
    NewCommunityPost {
        post_id: Uuid,
        author_name: String,
        content: String,
        timestamp: DateTime<Utc>,
    },
    /// Новый лайк на пост
    PostLiked {
        post_id: Uuid,
        liker_name: String,
        total_likes: u32,
    },
    /// Новый комментарий
    NewComment {
        post_id: Uuid,
        comment_id: Uuid,
        author_name: String,
        content: String,
    },
    /// Уведомление о скоропортящихся продуктах
    ExpiringItems {
        items: Vec<ExpiringItem>,
        days_left: u32,
    },
    /// Достижение цели
    GoalAchieved {
        goal_id: Uuid,
        title: String,
        achievement_type: String,
    },
    /// Новый подписчик
    NewFollower {
        follower_id: Uuid,
        follower_name: String,
    },
    /// AI рецепт готов
    RecipeGenerated {
        recipe_id: Uuid,
        title: String,
        ingredients_count: u32,
    },
    /// Системное уведомление
    SystemNotification {
        title: String,
        message: String,
        level: NotificationLevel,
    },
    /// Heartbeat для проверки соединения
    Heartbeat {
        timestamp: DateTime<Utc>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpiringItem {
    pub id: Uuid,
    pub name: String,
    pub days_left: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

/// Информация о подключенном клиенте
#[derive(Debug, Clone, Serialize)]
pub struct ConnectedClient {
    pub user_id: Uuid,
    pub user_name: String,
    pub connected_at: DateTime<Utc>,
    pub last_heartbeat: DateTime<Utc>,
}

/// WebSocket сообщение от клиента
#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    Subscribe { channels: Vec<String> },
    Unsubscribe { channels: Vec<String> },
    Heartbeat,
    TypingStart { post_id: Uuid },
    TypingStop { post_id: Uuid },
}

/// WebSocket менеджер для управления соединениями и рассылки событий
pub struct WebSocketManager {
    /// Глобальный канал для рассылки всем подключенным клиентам
    global_sender: broadcast::Sender<WebSocketEvent>,
    /// Клиенты, подключенные к WebSocket
    clients: Arc<RwLock<HashMap<Uuid, ConnectedClient>>>,
    /// Каналы для групповых уведомлений (например, подписчики пользователя)
    channels: Arc<RwLock<HashMap<String, broadcast::Sender<WebSocketEvent>>>>,
}

impl WebSocketManager {
    pub fn new() -> Self {
        let (global_sender, _) = broadcast::channel(1000);
        
        Self {
            global_sender,
            clients: Arc::new(RwLock::new(HashMap::new())),
            channels: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Добавляет нового клиента
    pub async fn add_client(&self, user_id: Uuid, user_name: String) -> broadcast::Receiver<WebSocketEvent> {
        let client = ConnectedClient {
            user_id,
            user_name: user_name.clone(),
            connected_at: Utc::now(),
            last_heartbeat: Utc::now(),
        };

        self.clients.write().await.insert(user_id, client);
        
        info!("WebSocket client connected: {} ({})", user_name, user_id);
        
        // Отправляем приветственное сообщение
        let welcome_event = WebSocketEvent::SystemNotification {
            title: "Добро пожаловать!".to_string(),
            message: "Вы подключились к real-time уведомлениям IT Cook".to_string(),
            level: NotificationLevel::Success,
        };
        
        let _ = self.global_sender.send(welcome_event);
        
        self.global_sender.subscribe()
    }

    /// Удаляет клиента
    pub async fn remove_client(&self, user_id: Uuid) {
        if let Some(client) = self.clients.write().await.remove(&user_id) {
            info!("WebSocket client disconnected: {} ({})", client.user_name, user_id);
        }
    }

    /// Обновляет heartbeat клиента
    pub async fn update_heartbeat(&self, user_id: Uuid) {
        if let Some(client) = self.clients.write().await.get_mut(&user_id) {
            client.last_heartbeat = Utc::now();
        }
    }

    /// Отправляет событие всем подключенным клиентам
    pub async fn broadcast_global(&self, event: WebSocketEvent) -> Result<(), AppError> {
        match self.global_sender.send(event.clone()) {
            Ok(receiver_count) => {
                info!("Broadcasted event to {} clients: {:?}", receiver_count, event);
                Ok(())
            }
            Err(e) => {
                error!("Failed to broadcast event: {}", e);
                Err(AppError::InternalServerError("Failed to broadcast WebSocket event".to_string()))
            }
        }
    }

    /// Отправляет событие конкретному пользователю
    pub async fn send_to_user(&self, _user_id: Uuid, event: WebSocketEvent) -> Result<(), AppError> {
        // Для простоты, отправляем через глобальный канал
        // В более сложной реализации можно создать персональные каналы
        self.broadcast_global(event).await
    }

    /// Отправляет событие группе пользователей (например, подписчикам)
    pub async fn send_to_channel(&self, channel_name: &str, event: WebSocketEvent) -> Result<(), AppError> {
        let channels = self.channels.read().await;
        
        if let Some(sender) = channels.get(channel_name) {
            match sender.send(event.clone()) {
                Ok(receiver_count) => {
                    info!("Sent event to channel '{}' ({} subscribers): {:?}", channel_name, receiver_count, event);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to send to channel '{}': {}", channel_name, e);
                    Err(AppError::InternalServerError("Failed to send to channel".to_string()))
                }
            }
        } else {
            warn!("Channel '{}' not found", channel_name);
            Ok(())
        }
    }

    /// Создает новый канал для группы
    pub async fn create_channel(&self, channel_name: String) -> broadcast::Receiver<WebSocketEvent> {
        let (sender, receiver) = broadcast::channel(100);
        self.channels.write().await.insert(channel_name.clone(), sender);
        info!("Created WebSocket channel: {}", channel_name);
        receiver
    }

    /// Возвращает количество подключенных клиентов
    pub async fn client_count(&self) -> usize {
        self.clients.read().await.len()
    }

    /// Возвращает список подключенных клиентов
    pub async fn get_clients(&self) -> Vec<ConnectedClient> {
        self.clients.read().await.values().cloned().collect()
    }

    /// Очищает неактивные соединения (heartbeat старше 30 секунд)
    pub async fn cleanup_inactive_clients(&self) {
        let now = Utc::now();
        let timeout = chrono::Duration::seconds(30);
        
        let mut clients = self.clients.write().await;
        let inactive_clients: Vec<Uuid> = clients
            .iter()
            .filter(|(_, client)| now.signed_duration_since(client.last_heartbeat) > timeout)
            .map(|(user_id, _)| *user_id)
            .collect();

        for user_id in inactive_clients {
            if let Some(client) = clients.remove(&user_id) {
                warn!("Removed inactive WebSocket client: {} ({})", client.user_name, user_id);
            }
        }
    }
}

impl Default for WebSocketManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Обработчик WebSocket соединения
pub async fn handle_websocket(
    socket: WebSocket,
    claims: Claims,
    ws_manager: Arc<WebSocketManager>,
) {
    let user_id = claims.sub;
    let user_name = format!("{} {}", claims.first_name, claims.last_name);
    
    // Регистрируем клиента и получаем receiver для событий
    let mut receiver = ws_manager.add_client(user_id, user_name.clone()).await;
    
    // Разделяем WebSocket на отправку и получение
    let (mut sender, mut recv) = socket.split();
    
    // Задача для отправки событий клиенту
    let send_task = tokio::spawn(async move {
        while let Ok(event) = receiver.recv().await {
            let message = match serde_json::to_string(&event) {
                Ok(json) => Message::Text(json.into()),
                Err(e) => {
                    error!("Failed to serialize WebSocket event: {}", e);
                    continue;
                }
            };
            
            if sender.send(message).await.is_err() {
                info!("WebSocket send failed, client probably disconnected");
                break;
            }
        }
    });
    
    // Задача для получения сообщений от клиента
    let ws_manager_recv = ws_manager.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(msg) = recv.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Обрабатываем сообщения от клиента
                    if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                        match client_msg {
                            ClientMessage::Heartbeat => {
                                ws_manager_recv.update_heartbeat(user_id).await;
                            }
                            ClientMessage::Subscribe { channels } => {
                                info!("Client {} subscribed to channels: {:?}", user_name, channels);
                                // Здесь можно реализовать подписку на каналы
                            }
                            ClientMessage::Unsubscribe { channels } => {
                                info!("Client {} unsubscribed from channels: {:?}", user_name, channels);
                                // Здесь можно реализовать отписку от каналов
                            }
                            ClientMessage::TypingStart { post_id: _ } => {
                                // Уведомляем других пользователей что кто-то печатает
                                let typing_event = WebSocketEvent::SystemNotification {
                                    title: "Typing".to_string(),
                                    message: format!("{} печатает...", user_name),
                                    level: NotificationLevel::Info,
                                };
                                let _ = ws_manager_recv.broadcast_global(typing_event).await;
                            }
                            ClientMessage::TypingStop { post_id: _ } => {
                                // Можно убрать уведомление о печатании
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("WebSocket connection closed by client");
                    break;
                }
                Ok(Message::Pong(_)) => {
                    ws_manager_recv.update_heartbeat(user_id).await;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    });
    
    // Ждем завершения любой из задач
    tokio::select! {
        _ = send_task => {
            info!("WebSocket send task completed for user {}", user_id);
        }
        _ = recv_task => {
            info!("WebSocket receive task completed for user {}", user_id);
        }
    }
    
    // Убираем клиента из списка подключенных
    ws_manager.remove_client(user_id).await;
}

/// Сервис для интеграции с другими частями приложения
pub struct RealtimeService {
    ws_manager: Arc<WebSocketManager>,
}

impl RealtimeService {
    pub fn new(ws_manager: Arc<WebSocketManager>) -> Self {
        Self { ws_manager }
    }

    /// Уведомляет о новом посте в сообществе
    pub async fn notify_new_post(&self, post_id: Uuid, author_name: String, content: String) -> Result<(), AppError> {
        let event = WebSocketEvent::NewCommunityPost {
            post_id,
            author_name,
            content,
            timestamp: Utc::now(),
        };
        self.ws_manager.broadcast_global(event).await
    }

    /// Уведомляет о лайке поста
    pub async fn notify_post_liked(&self, post_id: Uuid, liker_name: String, total_likes: u32) -> Result<(), AppError> {
        let event = WebSocketEvent::PostLiked {
            post_id,
            liker_name,
            total_likes,
        };
        self.ws_manager.broadcast_global(event).await
    }

    /// Уведомляет о скоропортящихся продуктах
    pub async fn notify_expiring_items(&self, user_id: Uuid, items: Vec<ExpiringItem>) -> Result<(), AppError> {
        if items.is_empty() {
            return Ok(());
        }

        let days_left = items.iter().map(|item| item.days_left).min().unwrap_or(0);
        let event = WebSocketEvent::ExpiringItems { items, days_left };
        
        self.ws_manager.send_to_user(user_id, event).await
    }

    /// Уведомляет о достижении цели
    pub async fn notify_goal_achieved(&self, user_id: Uuid, goal_id: Uuid, title: String) -> Result<(), AppError> {
        let event = WebSocketEvent::GoalAchieved {
            goal_id,
            title,
            achievement_type: "goal_completed".to_string(),
        };
        self.ws_manager.send_to_user(user_id, event).await
    }

    /// Уведомляет о новом подписчике
    pub async fn notify_new_follower(&self, user_id: Uuid, follower_id: Uuid, follower_name: String) -> Result<(), AppError> {
        let event = WebSocketEvent::NewFollower {
            follower_id,
            follower_name,
        };
        self.ws_manager.send_to_user(user_id, event).await
    }

    /// Уведомляет о готовности AI рецепта
    pub async fn notify_recipe_generated(&self, user_id: Uuid, recipe_id: Uuid, title: String, ingredients_count: u32) -> Result<(), AppError> {
        let event = WebSocketEvent::RecipeGenerated {
            recipe_id,
            title,
            ingredients_count,
        };
        self.ws_manager.send_to_user(user_id, event).await
    }

    /// Отправляет системное уведомление
    pub async fn send_system_notification(&self, title: String, message: String, level: NotificationLevel) -> Result<(), AppError> {
        let event = WebSocketEvent::SystemNotification {
            title,
            message,
            level,
        };
        self.ws_manager.broadcast_global(event).await
    }

    /// Запускает периодическую очистку неактивных соединений
    pub fn start_cleanup_task(&self) {
        let ws_manager = self.ws_manager.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(60));
            loop {
                interval.tick().await;
                ws_manager.cleanup_inactive_clients().await;
            }
        });
    }

    /// Отправляет heartbeat всем клиентам
    pub async fn send_heartbeat(&self) -> Result<(), AppError> {
        let event = WebSocketEvent::Heartbeat {
            timestamp: Utc::now(),
        };
        self.ws_manager.broadcast_global(event).await
    }

    /// Возвращает статистику подключений
    pub async fn get_stats(&self) -> RealtimeStats {
        RealtimeStats {
            connected_clients: self.ws_manager.client_count().await,
            clients: self.ws_manager.get_clients().await,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct RealtimeStats {
    pub connected_clients: usize,
    pub clients: Vec<ConnectedClient>,
}