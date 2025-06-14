Here’s a detailed, plug-and-play doc for **`18-metrics-exporter.md`** (Rust-based metrics exporter), following the same structure and thoroughness:

---

```md
# 18. Rust Service: Metrics Exporter

## Purpose

This document covers the design and usage of a Rust-based metrics exporter for your stack.  
The service gathers custom metrics (errors, request counts, health, etc.) from your services and exposes them in [Prometheus](https://prometheus.io/docs/concepts/data_model/) format for easy integration with Grafana or any modern observability stack.

---

## Why Add a Metrics Exporter?

- **Unified metrics:** Collect and expose stack-wide stats for dashboards, alerts, and analysis.
- **Observability:** Gain insight into trends, health, and anomalies.
- **Flexibility:** Export any custom metric (not just those exposed by apps).
- **Compatibility:** Prometheus format is the industry standard and easily integrates with Grafana, Alertmanager, and more.

---

## Key Features

- Periodically scrapes or receives metrics from your services:
  - HTTP APIs (e.g., `/health`, `/metrics`)
  - Log files (parse errors/warnings)
  - System stats (CPU, RAM, disk usage)
- Exposes `/metrics` HTTP endpoint in Prometheus-compatible format.
- Supports custom counters, gauges, histograms.
- (Optional) Receives push metrics from services via HTTP POST.
- Configurable scrape interval, endpoints, and metric definitions.
- (Optional) Exposes `/status` or web UI for live stats.

---

## System Architecture

```

+------------+       +-------------------+        +-------------+
\|  API/Web   | <---> | Metrics Exporter  | <----> | Prometheus  |
\|  DB/Rust   |  <--- |   (Rust Service)  |        |  Grafana    |
+------------+       +-------------------+        +-------------+
|<--------- Pulls metrics via HTTP or logs

```

- Prometheus scrapes metrics from the exporter at `/metrics`.
- Exporter collects/aggregates metrics from your other services.

---

## Directory Structure

```

services/
└── metrics\_exporter/
├── src/
│   └── main.rs
├── Cargo.toml
├── Dockerfile
└── config.toml (optional)

````

---

## Implementation Outline

- **Language:** Rust (async for concurrency)
- **Libraries:**
  - [prometheus crate](https://docs.rs/prometheus/) for metrics types/format
  - [tokio](https://tokio.rs/) for async runtime
  - [reqwest](https://docs.rs/reqwest/) for HTTP scrapes
  - [axum](https://github.com/tokio-rs/axum) or [warp](https://github.com/seanmonstar/warp) for HTTP server

---

## Dockerfile Example

```dockerfile
# services/metrics_exporter/Dockerfile

FROM rust:1.76 as builder

WORKDIR /usr/src/metrics_exporter
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/metrics_exporter/target/release/metrics_exporter /usr/local/bin/metrics_exporter
COPY config.toml /app/config.toml
CMD ["metrics_exporter"]
````

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yaml
metrics_exporter:
  build: ./services/metrics_exporter
  container_name: metrics_exporter
  ports:
    - "9300:9300"
  environment:
    - METRICS_CONFIG=/app/config.toml
    - EXPORTER_PORT=9300
```

---

## Configuration

* **Via env vars or config file.**
* Example `config.toml`:

  ```toml
  [scrape]
  interval_secs = 15
  targets = [
    { name = "api", url = "http://api:8000/metrics", type = "prometheus" },
    { name = "web", url = "http://web:3000/metrics", type = "custom" }
  ]

  [system]
  enabled = true
  collect_cpu = true
  collect_ram = true
  collect_disk = false

  [metrics]
  custom_gauges = ["errors", "active_sessions"]
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/metrics_exporter
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t metrics-exporter .
   ```
3. **Run in Compose:**

   ```bash
   docker-compose up -d metrics_exporter
   ```
4. **Point Prometheus to:**
   `http://metrics_exporter:9300/metrics`

---

## Example Prometheus Output

```
# HELP api_requests_total Number of API requests processed
# TYPE api_requests_total counter
api_requests_total 1573

# HELP web_errors_total Number of errors in web logs
# TYPE web_errors_total counter
web_errors_total 4

# HELP process_cpu_seconds_total
# TYPE process_cpu_seconds_total counter
process_cpu_seconds_total 12.34
```

---

## Optional Extensions

* Support push-based metrics (e.g., POST JSON to `/update`).
* Tag metrics with environment, service name, or version.
* Expose `/status` endpoint or HTML dashboard.
* Integrate alerting (send webhook/email on threshold).

---

## Troubleshooting

* **Metrics not updating:** Check scrape interval, endpoints, and Docker networking.
* **Prometheus scrape errors:** Ensure correct URL and that container is running.
* **System metrics missing:** Check container permissions.

---

## Further Reading

* [prometheus crate docs](https://docs.rs/prometheus/)
* [Prometheus getting started](https://prometheus.io/docs/introduction/first_steps/)
* [Grafana + Prometheus](https://grafana.com/docs/grafana/latest/datasources/prometheus/)

---

## Next Service

[19-custom-cli-tool.md](./19-custom-cli-tool.md): Rust-based custom CLI tool for developer/devops automation.

---

```

---

**Ready for the next doc (19-custom-cli-tool), or want to adjust this one?**
```
