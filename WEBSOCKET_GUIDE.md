# 🔌 WebSocket Real-time Features - IT Cook Backend

## 🚀 Новая функциональность: WebSocket Support

Мы добавили полноценную поддержку WebSocket для real-time уведомлений и интерактивных возможностей!

---

## 📡 WebSocket Endpoints

### Подключение к WebSocket
```
ws://localhost:3000/api/v1/realtime/ws
```

**Требования:**
- JWT токен в заголовке Authorization
- Поддержка WebSocket протокола

### Статистика подключений
```
GET /api/v1/realtime/stats
```

---

## 🎯 Типы событий

### 📢 Community Events
- **NewCommunityPost** - Новый пост в сообществе
- **PostLiked** - Лайк поста
- **NewComment** - Новый комментарий
- **NewFollower** - Новый подписчик

### 🧊 Fridge Events  
- **ExpiringItems** - Уведомления о скоропортящихся продуктах

### 🏆 Goals Events
- **GoalAchieved** - Достижение цели

### 🤖 AI Events
- **RecipeGenerated** - AI рецепт готов

### 🔔 System Events
- **SystemNotification** - Системные уведомления
- **Heartbeat** - Проверка соединения

---

## 💻 Frontend Integration

### JavaScript/TypeScript Example

```typescript
class ITCookWebSocket {
  private ws: WebSocket | null = null;
  private token: string;

  constructor(token: string) {
    this.token = token;
  }

  connect() {
    const wsUrl = `ws://localhost:3000/api/v1/realtime/ws`;
    
    this.ws = new WebSocket(wsUrl, [], {
      headers: {
        'Authorization': `Bearer ${this.token}`
      }
    });

    this.ws.onopen = () => {
      console.log('🔌 WebSocket connected');
      this.sendHeartbeat();
    };

    this.ws.onmessage = (event) => {
      const data = JSON.parse(event.data);
      this.handleEvent(data);
    };

    this.ws.onclose = () => {
      console.log('🔌 WebSocket disconnected');
      // Автоматическое переподключение
      setTimeout(() => this.connect(), 5000);
    };

    this.ws.onerror = (error) => {
      console.error('🔌 WebSocket error:', error);
    };
  }

  private handleEvent(event: WebSocketEvent) {
    switch (event.type) {
      case 'NewCommunityPost':
        this.onNewPost(event.data);
        break;
      case 'ExpiringItems':
        this.onExpiringItems(event.data);
        break;
      case 'GoalAchieved':
        this.onGoalAchieved(event.data);
        break;
      case 'SystemNotification':
        this.onSystemNotification(event.data);
        break;
      case 'Heartbeat':
        // Отвечаем на heartbeat
        this.sendHeartbeat();
        break;
      default:
        console.log('Unknown event:', event);
    }
  }

  private sendHeartbeat() {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify({ type: 'Heartbeat' }));
    }
  }

  private onNewPost(data: any) {
    // Показываем уведомление о новом посте
    this.showNotification('📢 Новый пост', `${data.author_name}: ${data.content}`);
  }

  private onExpiringItems(data: any) {
    // Показываем уведомление о скоропортящихся продуктах
    const message = `У вас ${data.items.length} продуктов истекает в ближайшие дни`;
    this.showNotification('🧊 Холодильник', message);
  }

  private onGoalAchieved(data: any) {
    // Показываем поздравление с достижением цели
    this.showNotification('🏆 Цель достигнута!', `Поздравляем с достижением: ${data.title}`);
  }

  private onSystemNotification(data: any) {
    // Системные уведомления
    this.showNotification(data.title, data.message);
  }

  private showNotification(title: string, message: string) {
    // Реализация показа уведомлений (например, через toast)
    if ('Notification' in window && Notification.permission === 'granted') {
      new Notification(title, { body: message });
    }
    console.log(`🔔 ${title}: ${message}`);
  }

  subscribe(channels: string[]) {
    this.send({ type: 'Subscribe', channels });
  }

  startTyping(postId: string) {
    this.send({ type: 'TypingStart', post_id: postId });
  }

  stopTyping(postId: string) {
    this.send({ type: 'TypingStop', post_id: postId });
  }

  private send(message: any) {
    if (this.ws?.readyState === WebSocket.OPEN) {
      this.ws.send(JSON.stringify(message));
    }
  }

  disconnect() {
    this.ws?.close();
  }
}

// Использование
const wsClient = new ITCookWebSocket(userToken);
wsClient.connect();

// Подписка на каналы
wsClient.subscribe(['community', 'personal']);
```

### React Hook Example

```typescript
import { useEffect, useRef, useState } from 'react';

interface WebSocketEvent {
  type: string;
  data: any;
}

export function useITCookWebSocket(token: string) {
  const [isConnected, setIsConnected] = useState(false);
  const [events, setEvents] = useState<WebSocketEvent[]>([]);
  const wsRef = useRef<WebSocket | null>(null);

  useEffect(() => {
    if (!token) return;

    const connect = () => {
      const ws = new WebSocket(`ws://localhost:3000/api/v1/realtime/ws`);
      
      ws.onopen = () => {
        setIsConnected(true);
        console.log('WebSocket connected');
      };

      ws.onmessage = (event) => {
        const data: WebSocketEvent = JSON.parse(event.data);
        setEvents(prev => [...prev, data]);
      };

      ws.onclose = () => {
        setIsConnected(false);
        console.log('WebSocket disconnected');
        // Переподключение через 5 секунд
        setTimeout(connect, 5000);
      };

      wsRef.current = ws;
    };

    connect();

    return () => {
      wsRef.current?.close();
    };
  }, [token]);

  const sendMessage = (message: any) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  };

  return {
    isConnected,
    events,
    sendMessage,
    clearEvents: () => setEvents([])
  };
}

// Компонент использования
function App() {
  const { isConnected, events, sendMessage } = useITCookWebSocket(userToken);

  return (
    <div>
      <div>Status: {isConnected ? '🟢 Connected' : '🔴 Disconnected'}</div>
      
      {events.map((event, index) => (
        <div key={index} className="notification">
          <strong>{event.type}</strong>: {JSON.stringify(event.data)}
        </div>
      ))}
    </div>
  );
}
```

---

## 🔧 Интеграция с Backend Services

### Отправка уведомлений из сервисов

```rust
// В community сервисе
let realtime_service = Arc::clone(&realtime_service);
realtime_service.notify_new_post(
    post_id,
    author_name,
    content,
).await?;

// В fridge сервисе  
let expiring_items = fridge_service.get_expiring_items(user_id, Some(3)).await?;
realtime_service.notify_expiring_items(user_id, expiring_items).await?;

// В goals сервисе
realtime_service.notify_goal_achieved(
    user_id,
    goal_id,
    "Достигнута цель по весу!".to_string(),
).await?;
```

---

## 🚀 Возможности

### ✅ Реализовано
- [x] WebSocket подключения с JWT аутентификацией
- [x] Real-time уведомления для сообщества
- [x] Уведомления о скоропортящихся продуктах
- [x] Системные уведомления
- [x] Heartbeat для проверки соединения
- [x] Автоматическая очистка неактивных соединений
- [x] Статистика подключений

### 🔄 В разработке
- [ ] Персональные каналы для групповых уведомлений
- [ ] Typing indicators в реальном времени
- [ ] Push уведомления через Service Workers
- [ ] Метрики и аналитика событий

### 🎯 Планируется
- [ ] WebRTC для видео-чатов при готовке
- [ ] Совместное планирование меню
- [ ] Live-трансляции готовки
- [ ] Геолокационные уведомления

---

## 📊 Мониторинг

### Получение статистики
```bash
curl -H "Authorization: Bearer YOUR_TOKEN" \
  http://localhost:3000/api/v1/realtime/stats
```

### Пример ответа
```json
{
  "connected_clients": 15,
  "uptime": "Available in future version",
  "events_sent_today": 0
}
```

---

## 🔒 Безопасность

- JWT токены проверяются при подключении
- Все события проходят валидацию
- Автоматическое отключение неактивных клиентов
- Rate limiting для событий (планируется)

---

## 🐛 Troubleshooting

### Проблемы с подключением
1. Проверьте валидность JWT токена
2. Убедитесь что WebSocket endpoint доступен
3. Проверьте CORS настройки для WebSocket

### Отладка событий
1. Включите логирование WebSocket событий
2. Проверьте формат отправляемых сообщений
3. Мониторьте статистику подключений

---

**WebSocket support ready! 🎉 Наслаждайтесь real-time возможностями IT Cook!**
