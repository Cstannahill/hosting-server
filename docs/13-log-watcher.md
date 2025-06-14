Here’s a clear and actionable template for **`13-log-watcher.md`** — covering a Rust-based log watcher and aggregator microservice.

---

# 13. Rust Service: Log Watcher / Aggregator

## Purpose
This document outlines the design and configuration of a small Rust service that tails log files, filters messages, and optionally forwards alerts. It can run standalone or as part of the Docker Compose stack.

---

## Why Add a Log Watcher?
- **Unified log view:** Aggregate logs from all services.
- **Pattern detection:** Filter and alert on error/warning patterns or custom triggers.
- **Extendability:** Foundation for future alerting, dashboards, or analytics.

---

## Key Features
- Tails logs from files or container volumes.
- Filters messages by level or regex pattern.
- Optional webhook alert on matched lines.
- Built with async Rust for minimal resource use.

---

## System Architecture
```text
+---------------------+       +--------------------+
|  API (FastAPI)      |----\  |                    |
|  Web (React)        |----+--| Log Watcher (Rust) |---+--> Alert/Webhook
|  DB (Postgres)      |----/  |                    |   |
+---------------------+       +--------------------+   |
                                |                      |
                                v                      v
                        Summary file/REST API   Dashboard (optional)
```

The service reads log output via bind mounts or Docker’s logs directory and can run as a sidecar in Compose.

---

## Directory Structure
```text
services/
└── log_watcher/
    ├── src/
    │   └── main.rs
    ├── Cargo.toml
    └── Dockerfile
```

---

## Implementation Outline
- **Language:** Rust (tokio runtime)
- **Libraries:** `tokio`, `regex`, `serde`, and `reqwest` for optional webhooks.
- Configuration via environment variables or a `config.toml` file.

---

## Dockerfile Example
```dockerfile
# services/log_watcher/Dockerfile
FROM rust:1.76 as builder
WORKDIR /usr/src/log_watcher
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/log_watcher/target/release/log_watcher /usr/local/bin/log_watcher
COPY config.toml /app/config.toml
CMD ["log_watcher"]
```

---

## Docker Compose Integration
```yaml
log_watcher:
  build: ./services/log_watcher
  volumes:
    - /var/lib/docker/containers:/var/lib/docker/containers:ro
  environment:
    - LOG_FILES=/var/lib/docker/containers
    - LOG_LEVELS=error,warning
  depends_on:
    - api
    - web
```

---

## Configuration
Example `config.toml`:
```toml
[log_sources]
files = ["/var/lib/docker/containers/<id>/<id>-json.log"]

[filters]
levels = ["error", "warning"]
regexes = ["panic", "unauthorized", "exception"]

[alerts]
webhook = "https://discord.com/api/webhooks/..."
```

---

## Usage
1. **Run locally:**
   ```bash
   cd services/log_watcher
   cargo run
   ```
2. **Build container:**
   ```bash
   docker build -t log-watcher .
   ```
3. **Start with Compose:**
   ```bash
   docker compose up -d log_watcher
   ```

---

## Example Output
```text
[2024-06-12T01:23:45Z][api][ERROR] Database connection failed: timeout
[2024-06-12T01:23:50Z][web][WARNING] Deprecated API usage detected.
Summary (last 10 min): 5 errors, 2 warnings.
```

---

## Optional Extensions
- Expose a `/logs` or `/summary` endpoint via REST (axum or warp).
- Push summaries to a metrics service or database.
- Pluggable alerting channels (Slack, Discord, etc.).

---

## Further Reading
- [tokio async file IO](https://docs.rs/tokio/latest/tokio/fs/index.html)
- [Docker Compose Volumes Docs](https://docs.docker.com/compose/compose-file/compose-file-v3/#volumes)

---

## Next Service
[14-backup-scheduler.md](./14-backup-scheduler.md): Rust-based backup and snapshot microservice.

---
