# 23. Rust Service: WebSocket Broadcast

## Purpose

Deliver real-time updates from the backend to React clients. FastAPI or other services can POST messages which are instantly pushed to all connected WebSocket clients. Useful for live dashboards, notifications, or ML job progress.

## Key Features

- Lightweight Axum-based WebSocket server.
- `/ws` endpoint for clients to subscribe.
- `/broadcast` HTTP endpoint to push messages.
- In-memory channel with fan-out to all connections.

## Docker Compose

```yaml
  ws_broadcast:
    build: ./services/ws_broadcast
    container_name: ws_broadcast
    ports:
      - "9400:9400"
    environment:
      - WS_PORT=9400
```

## Example Usage

1. Start the service with Docker Compose.
2. React app connects via `ws://host:9400/ws`.
3. FastAPI sends POST requests to `/broadcast` with `{ "message": "text" }`.
4. All WebSocket clients receive the message instantly.

---

Ready to integrate with your FARM stack!
