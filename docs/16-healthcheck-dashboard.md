Here’s a thorough, ready-to-use doc for **`16-healthcheck-dashboard.md`** (Rust-based healthcheck/dashboard service), consistent with the depth and organization of your earlier docs:

---

```md
# 16. Rust Service: Healthcheck & Dashboard

## Purpose

This document outlines the design, configuration, and usage of a Rust-based healthcheck service and optional dashboard.  
The service continuously checks the health/status of your platform’s containers (API, web, DB, custom services) and exposes a `/health` API endpoint, optionally providing a simple web dashboard for fast status checks.

---

## Why Add a Healthcheck Service?

- **Centralized status:** Monitor all your critical services from one endpoint.
- **Automation:** Enable other services or CI/CD to quickly check stack health.
- **User-friendly:** Optionally present status visually via a minimal dashboard UI.
- **Early warning:** Detect issues before users or automation are affected.

---

## Key Features

- Configurable, periodic checks (HTTP, TCP, or even custom command).
- Supports checking:
  - HTTP endpoints (e.g., `/api/health`, `/`)
  - Database connection status (optional)
  - Custom ports/services
- Aggregates results and exposes a `/health` endpoint (JSON, e.g. for load balancers/monitors).
- (Optional) Minimal web dashboard page showing live status.
- (Optional) Alerting on failures (Slack, webhook, email).

---

## System Architecture

```

+-------------------+     +-------------------------+
\|    Web Frontend   | <-- |                         |
\|    API Backend    | <-- |  Healthcheck Dashboard  | <-- \[User/CI/Uptime Robot]
\|    DB, Rust Svc   | <-- |   (Rust Service)        |
+-------------------+     +-------------------------+
|<------- Health status, periodic polling

```

- The Healthcheck service can run anywhere in the stack and ping containers by Docker Compose service name.

---

## Directory Structure

```

services/
└── healthcheck\_dashboard/
├── src/
│   └── main.rs
├── Cargo.toml
├── Dockerfile
└── config.toml (optional)

````

---

## Implementation Outline

- **Language:** Rust (async for parallel checks)
- **Libraries:**
  - [tokio](https://tokio.rs/) for async runtime
  - [reqwest](https://docs.rs/reqwest/) for HTTP health checks
  - [deadpool-postgres](https://docs.rs/deadpool-postgres/) or `tokio-postgres` for DB checks
  - [axum](https://github.com/tokio-rs/axum) or [warp](https://github.com/seanmonstar/warp) for REST/web dashboard

---

## Dockerfile Example

```dockerfile
# services/healthcheck_dashboard/Dockerfile

FROM rust:1.76 as builder

WORKDIR /usr/src/healthcheck_dashboard
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/healthcheck_dashboard/target/release/healthcheck_dashboard /usr/local/bin/healthcheck_dashboard
COPY config.toml /app/config.toml
CMD ["healthcheck_dashboard"]
````

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yaml
healthcheck_dashboard:
  build: ./services/healthcheck_dashboard
  container_name: healthcheck_dashboard
  ports:
    - "9100:9100"
  environment:
    - HEALTHCHECK_CONFIG=/app/config.toml
    - DASHBOARD_PORT=9100
```

---

## Configuration

* **Via env vars or config file.**
* Example `config.toml`:

  ```toml
  [checks]
  interval_secs = 10
  endpoints = [
    { name = "web", url = "http://web:3000/", type = "http" },
    { name = "api", url = "http://api:8000/health", type = "http" },
    { name = "db",  url = "postgres://myuser:mypassword@db:5432/mydb", type = "postgres" }
  ]

  [dashboard]
  enabled = true
  port = 9100

  [alerts]
  enabled = false
  webhook = ""
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/healthcheck_dashboard
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t healthcheck-dashboard .
   ```
3. **Run in Compose:**

   ```bash
   docker-compose up -d healthcheck_dashboard
   ```
4. **Check health endpoint:**

   * [http://localhost:9100/health](http://localhost:9100/health) (JSON)
   * [http://localhost:9100/dashboard](http://localhost:9100/dashboard) (UI, if enabled)

---

## Example `/health` JSON Output

```json
{
  "timestamp": "2024-06-12T16:15:23Z",
  "status": "healthy",
  "services": {
    "web": { "ok": true, "response_time_ms": 24 },
    "api": { "ok": true, "response_time_ms": 17 },
    "db":  { "ok": true, "response_time_ms": 31 }
  }
}
```

---

## Example Dashboard UI

* Simple HTML page with green/yellow/red status indicators.
* (Optional) Live auto-refresh, error detail display.

---

## Optional Extensions

* Push failing status to Slack/Discord/webhook/email.
* Support custom shell checks or advanced port checks.
* Expose metrics endpoint (Prometheus format).

---

## Troubleshooting

* **Service not reachable:** Check Docker Compose networking and correct URLs.
* **False negatives:** Ensure endpoints are correct and allow access from healthcheck container.
* **Dashboard not available:** Ensure `[dashboard].enabled = true` and port mapping is correct.

---

## Further Reading

* [axum web framework](https://github.com/tokio-rs/axum)
* [tokio async runtime](https://tokio.rs/)
* [Healthcheck best practices](https://microservices.io/patterns/observability/health-check-api.html)

---

## Next Service

[17-static-file-proxy.md](./17-static-file-proxy.md): Rust-based static file server or programmable reverse proxy.

---

```

---

**Ready for the next doc (17-static-file-proxy), or any tweaks to this one?**
```
