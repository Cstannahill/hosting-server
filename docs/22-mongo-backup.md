# 22. Rust Service: MongoDB Backup

## Purpose

This service periodically runs `mongodump` to create compressed backups of your MongoDB database. It mirrors the Postgres backup scheduler but targets MongoDB so the FARM stack has reliable data snapshots.

## Key Features

- Cron based schedule using `tokio-cron-scheduler`.
- Stores archives in a configurable directory.
- Uses the official `mongodump` client inside the container.

## Docker Compose

```yaml
  mongo_backup:
    build: ./services/mongo_backup
    container_name: mongo_backup
    depends_on:
      - mongo
    volumes:
      - ./backups:/app/backups
    environment:
      - MONGO_URI=mongodb://mongo:27017
      - BACKUP_DIR=/app/backups
      - BACKUP_SCHEDULE=0 3 * * *
```

## Environment Variables

- `MONGO_URI` – connection string for the database
- `BACKUP_DIR` – where archives are written
- `BACKUP_SCHEDULE` – cron expression (default `0 3 * * *`)

## Usage

1. Ensure MongoDB tools are installed in the container.
2. Start the service with Docker Compose.
3. Backups will appear in the configured directory as `YYYYMMDDHHMMSS_dump.gz`.

## Next Service

[23-websocket-broadcast.md](./23-websocket-broadcast.md): real-time WebSocket broadcast service written in Rust.
