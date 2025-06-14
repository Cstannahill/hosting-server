# Hosting Platform Documentation
**Table of Contents & Build Checklist**

This checklist defines the **build order** of the platform. Each item represents a concrete **implementation step**, not just a reading assignment.

> ⚠️ You must actively build each part of the system as you proceed.  
> This is a hands-on checklist — not a theory guide.

---

| Step | Area/Service | Documentation File | Complete? |
|------|--------------|--------------------|-----------|
| 1 | Project Overview & Goals | [01-overview-goals.md](./01-overview-goals.md) | [x] |
| 2 | Directory Structure & Setup | [02-directory-setup.md](./02-directory-setup.md) | [x] |
| 3 | Dockerizing Backend (API) | [03-dockerizing-api.md](./03-dockerizing-api.md) | [x] |
| 4 | Docker Compose Orchestration | [04-docker-compose-orchestration.md](./04-docker-compose-orchestration.md) | [x] |
| 5 | NGINX Reverse Proxy | [05-nginx-reverse-proxy.md](./05-nginx-reverse-proxy.md) | [x] |
| 6 | Environment Variables & Secrets | [06-env-vars-secrets.md](./06-env-vars-secrets.md) | [x] |
| 7 | Persistent Volumes | [07-persistent-volumes.md](./07-persistent-volumes.md) | [x] |
| 8 | Networking & Service Discovery | [08-networking-service-discovery.md](./08-networking-service-discovery.md) | [x] |
| 9 | Operations (Run/Logs/Update) | [09-operations.md](./09-operations.md) | [x] |
| 10 | Local Domains & SSL (Optional) | [10-local-domains-ssl.md](./10-local-domains-ssl.md) | [x] |
| 11 | Monitoring & Logs | [11-monitoring-logs.md](./11-monitoring-logs.md) | [x] |
| 12 | Scaling & Migration | [12-scaling-migration.md](./12-scaling-migration.md) | [x] |
| 13 | Rust: Log Watcher/Aggregator | [13-log-watcher.md](./13-log-watcher.md) | [x] |
| 14 | Rust: Backup Scheduler | [14-backup-scheduler.md](./14-backup-scheduler.md) | [x] |
| 15 | Rust: Webhook Handler | [15-webhook-handler.md](./15-webhook-handler.md) | [x] |
| 16 | Rust: Healthcheck Dashboard | [16-healthcheck-dashboard.md](./16-healthcheck-dashboard.md) | [x] |
| 17 | Rust: Static File Server/Proxy | [17-static-file-proxy.md](./17-static-file-proxy.md) | [x] |
| 18 | Rust: Metrics Exporter | [18-metrics-exporter.md](./18-metrics-exporter.md) | [x] |

| 19 | Rust: Custom CLI Tool | [19-custom-cli-tool.md](./19-custom-cli-tool.md) | [x] |


| 20 | Multi-Language Backends | [20-multi-language-backends.md](./20-multi-language-backends.md) | [x] |
| 21 | Data Capture & Embedding | [21-data-capture.md](./21-data-capture.md) | [x] |


---

### ✅ Completion Instructions

- You must **create and commit actual code/config** for each doc before marking it `[x]`.
- Use the doc as a spec — don’t skip implementation.
- If a doc is unclear, pause and clarify before proceeding.

---

**No doc is considered done until it’s been built.**
