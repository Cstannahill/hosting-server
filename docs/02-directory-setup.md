# 02. Directory Structure & Setup

## Purpose

This document outlines the repository layout and explains why a monorepo is used for all services. A consistent structure keeps the project maintainable as more apps and infrastructure are added.

## Why Monorepo?

- **Centralized management:** Source, infrastructure, and documentation live in a single repository.
- **Atomic commits:** Code, infrastructure, and docs can be updated together.
- **Shared dependencies:** Reuse configuration, scripts, or libraries between apps.
- **Simple orchestration:** All services start together with Docker Compose.

## Top-Level Structure

```
my-hosting-platform/
├── apps/
│   ├── web/                  # Frontend app (Vite/React/TypeScript)
│   └── api/                  # Backend API (FastAPI/Python)
├── services/                 # Additional utilities written in Rust
├── nginx/
│   └── nginx.conf            # Reverse proxy config
├── docs/                     # Project documentation
├── docker-compose.yml        # Multi-app orchestration config
├── .env.example              # Example environment configuration
└── README.md                 # Project overview and quickstart
```

## Directory/Component Breakdown

### `/apps/`
Contains all application code.

- **`web/`** – React/Vite frontend served via NGINX.
- **`api/`** – FastAPI backend exposing REST endpoints.

### `/services/`
Rust utilities such as the log watcher.

### `/nginx/`
Holds the reverse proxy configuration used by Docker Compose.

### `/docs/`
Living documentation for every setup step. Keep this folder up to date as new services are added.

### `docker-compose.yml`
Orchestrates all containers for local development.

### `.env.example`
Sample environment variables. Copy to `.env` and edit for local values. **Never commit real secrets.**

### `README.md`
High-level introduction and quick-start instructions.

## Best Practices

- Use clear, descriptive folder names.
- Keep documentation updated as new services are introduced.
- Store any shared libraries in a dedicated location if needed.

## Next Steps

- [x] [03-dockerizing-api.md](./03-dockerizing-api.md): Dockerizing the backend (FastAPI) app.


