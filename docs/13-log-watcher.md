Absolutely! That’s the best way to do it—**one doc at a time, with the same level of detail and structure as your main documentation** so far.
Let’s start with the first one:

---

# **13. Rust Service: Log Watcher / Aggregator**

Below is a **detailed, ready-to-use doc**—you can copy this straight into `docs/13-log-watcher.md`.
Just as before, it includes purpose, features, architecture, usage, configuration, Docker/Compose integration, and ideas for future work.

---

```md
# 13. Rust Service: Log Watcher / Aggregator

## Purpose

This document covers the design, configuration, and usage of a Rust-based log watcher/aggregator microservice.  
The goal is to supplement the Docker Compose stack with real-time log collection, filtering, and (optionally) summary/alerting features for improved observability.

---

## Why Add a Log Watcher?

- **Unified log view:** Aggregate logs from all services for easier debugging.
- **Pattern detection:** Filter and alert on error/warning patterns, security events, or custom triggers.
- **Extendability:** Build a foundation for later alerting, dashboarding, or log analytics.

---

## Key Features

- Tails logs from other containers or shared volumes.
- Filters messages by level (info, warning, error) or by regex/pattern.
- Summarizes logs and/or sends alerts (email, webhook, CLI notification, etc.).
- (Optional) Exposes a REST API or dashboard with recent logs/statistics.

---

## System Architecture

```

+---------------------+       +--------------------+
\|  API (FastAPI)      |----\  |                    |
\|  Web (React)        |----+--| Log Watcher (Rust) |---+--> Alert/Webhook
\|  DB (Postgres)      |----/  |                    |   |
+---------------------+       +--------------------+   |
Aggregates/tails logs        |
\|                      |
v                      v
Summary file/REST API   Dashboard (opt.)

```

- Can tail container logs via Docker API, bind-mounted files, or `/var/lib/docker/containers`.
- Can run as a sidecar service in Docker Compose.

---

## Directory Structure

```

services/
└── log\_watcher/
├── src/
│   └── main.rs
├── Cargo.toml
└── Dockerfile

````

---

## Implementation Outline

- **Language:** Rust (async recommended for real-time)
- **Libraries:**  
  - [tokio](https://tokio.rs/) or [async-std](https://async.rs/) for async file/network IO
  - [regex](https://docs.rs/regex/latest/regex/) for filtering
  - [serde](https://serde.rs/) for config/log parsing
  - [reqwest](https://docs.rs/reqwest/latest/reqwest/) or [lettre](https://docs.rs/lettre/) for alerts
  - [warp](https://github.com/seanmonstar/warp) or [axum](https://github.com/tokio-rs/axum) for optional REST API

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
# Copy optional config file
COPY config.toml /app/config.toml
CMD ["log_watcher"]
````

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yamlHere’s a thorough, production-quality doc for **`15-webhook-handler.md`** (Rust-based webhook handler service), matching the level of detail of your previous docs.

---

```md
# 15. Rust Service: Webhook Handler

## Purpose

This document describes the design and usage of a Rust-based webhook handler microservice.  
The webhook handler listens for HTTP webhook events (e.g., GitHub, GitLab) and triggers programmable actions in your stack—such as auto-deploys, notifications, or running scripts.

---

## Why Add a Webhook Handler?

- **Enable GitOps:** Automate deployments or reloads when code is pushed.
- **Integrate with external services:** React to events from GitHub, CI/CD, or 3rd-party systems.
- **Programmable automation:** Trigger any container/script/task in response to a webhook.

---

## Key Features

- Listens for HTTP POST webhook requests on a configurable port.
- Verifies webhook signatures for security (supports shared secrets).
- Configurable: supports multiple event types (e.g., push, release, PR).
- Triggers programmable actions:
  - Runs shell scripts/commands inside the container or on the host.
  - Calls Docker Compose commands (deploy, restart, etc.).
  - Sends notifications (email, Slack, Discord, etc.).
- Logs all events and actions for auditability.

---

## System Architecture

```

\[GitHub/GitLab/Other] ---> \[Webhook Handler (Rust)] ---> \[Script | Compose | Notification | Other Service]

```
- Can be used for auto-deploy, cache purge, notifications, or custom event flows.

---

## Directory Structure

```

services/
└── webhook\_handler/
├── src/
│   └── main.rs
├── Cargo.toml
├── Dockerfile
└── config.toml (optional)

````

---

## Implementation Outline

- **Language:** Rust (with async HTTP server)
- **Libraries:**
  - [axum](https://github.com/tokio-rs/axum) or [warp](https://github.com/seanmonstar/warp) for HTTP
  - [serde](https://serde.rs/) for JSON parsing
  - [hmac](https://docs.rs/hmac/) for webhook signature validation
  - [tokio](https://tokio.rs/) for async runtime
  - [std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html) for running scripts

---

## Dockerfile Example

```dockerfile
# services/webhook_handler/Dockerfile

FROM rust:1.76 as builder

WORKDIR /usr/src/webhook_handler
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/webhook_handler/target/release/webhook_handler /usr/local/bin/webhook_handler
COPY config.toml /app/config.toml
CMD ["webhook_handler"]
````

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yaml
webhook_handler:
  build: ./services/webhook_handler
  container_name: webhook_handler
  ports:
    - "9000:9000"
  volumes:
    - ./scripts:/app/scripts:ro     # Optional: mount action scripts
  environment:
    - WEBHOOK_PORT=9000
    - WEBHOOK_SECRET=your_shared_secret
    - ACTION_SCRIPT=/app/scripts/deploy.sh
    - CONFIG_PATH=/app/config.toml
```

---

## Configuration

* **Via environment variables or config file.**
* Example `config.toml`:

  ```toml
  [server]
  port = 9000
  secret = "your_shared_secret"

  [actions]
  push_event_script = "/app/scripts/deploy.sh"
  notify_webhook = "https://discord.com/api/webhooks/..."

  [filters]
  allowed_repos = ["yourorg/yourrepo"]
  allowed_events = ["push", "release"]
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/webhook_handler
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t webhook-handler .
   ```
3. **Run in Compose:**

   ```bash
   docker-compose up -d webhook_handler
   ```
4. **Configure GitHub/GitLab/other service to send webhooks to `http://yourhost:9000/`**

---

## Example Actions

* On `push` event:

  * Validates signature.
  * Runs `/app/scripts/deploy.sh`.
  * Sends POST to Discord/Slack webhook.
* On other events:

  * Logs payload for audit/troubleshooting.

---

## Security Notes

* Always use HTTPS in production.
* Always use a strong, random `WEBHOOK_SECRET`—never expose this in public repos.
* Validate sender IP or source for added safety.

---

## Example Log Output

```
[2024-06-12T13:03:00Z] [INFO] Received push event for repo yourorg/yourrepo.
[2024-06-12T13:03:01Z] [INFO] Signature validated.
[2024-06-12T13:03:01Z] [INFO] Running deploy.sh...
[2024-06-12T13:03:15Z] [INFO] Deployment completed.
[2024-06-12T13:03:15Z] [INFO] Notification sent to Discord.
```

---

## Optional Extensions

* Support more event types (PR, release, issue, etc.).
* Trigger Compose or Docker commands via Docker API.
* Expose REST API for monitoring or manual trigger.
* Add retry/backoff on action failure.

---

## Further Reading

* [GitHub Webhooks docs](https://docs.github.com/en/developers/webhooks-and-events/webhooks/about-webhooks)
* [Rust axum HTTP server example](https://github.com/tokio-rs/axum)
* [Verifying webhook signatures (GitHub)](https://docs.github.com/en/developers/webhooks-and-events/webhooks/securing-your-webhooks)

---

## Next Service

[16-healthcheck-dashboard.md](./16-healthcheck-dashboard.md): Rust-based health check aggregator/dashboard.

---

```

---

**Ready for the next doc (16-healthcheck-dashboard), or any tweaks to this one?**
```

log_watcher:
  build: ./services/log_watcher
  container_name: log_watcher
  # Option 1: Bind Docker logs directory (Linux/macOS only)
  volumes:
    - /var/lib/docker/containers:/var/lib/docker/containers:ro
  # Option 2: Or bind-mount log files exported by other services
    # - ./logs:/logs:ro
  environment:
    - LOG_FILTER=error,warning
    - ALERT_EMAIL=alerts@example.com
    - CONFIG_PATH=/app/config.toml
  depends_on:
    - api
    - web
```

---

## Configuration

* **Via environment variables, CLI args, or a config file (`config.toml`).**
* Example `config.toml`:

  ```toml
  [log_sources]
  docker_logs_dir = "/var/lib/docker/containers"
  extra_files = ["/logs/web.log", "/logs/api.log"]

  [filters]
  levels = ["error", "warning"]
  regexes = ["panic", "unauthorized", "exception"]

  [alerts]
  email = "your@email.com"
  webhook = "https://discord.com/api/webhooks/..."
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/log_watcher
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t log-watcher .
   ```
3. **Run in Compose:**
   `docker-compose up -d log_watcher`

---

## Example Output

```
[2024-06-12T01:23:45Z][api][ERROR] Database connection failed: timeout
[2024-06-12T01:23:50Z][web][WARNING] Deprecated API usage detected.
Summary (last 10 min): 5 errors, 2 warnings.
Alert sent to: alerts@example.com
```

---

## Optional Extensions

* Expose `/logs` or `/summary` endpoint via REST API for easy dashboarding.
* Push log summaries to a database or metrics service.
* Add UI/dashboard (with React or TUI, terminal UI).
* Pluggable alerting (Slack, Discord, PagerDuty, etc.).

---

## Further Reading

* [Reading Docker logs with Rust (stackoverflow)](https://stackoverflow.com/questions/35474072/how-to-read-docker-container-logs)
* [tokio async file IO](https://docs.rs/tokio/latest/tokio/fs/index.html)
* [Docker Compose Volumes Docs](https://docs.docker.com/compose/compose-file/compose-file-v3/#volumes)

---

## Next Service

[14-backup-scheduler.md](./14-backup-scheduler.md): Rust-based backup and snapshot microservice.

---

```

---

**Ready to move to doc 14 (Backup Scheduler), or any tweaks to this one first?**
```
