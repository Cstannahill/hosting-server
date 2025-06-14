Here’s a thorough, production-quality doc for **`15-webhook-handler.md`** (Rust-based webhook handler service), matching the level of detail of your previous docs.

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
