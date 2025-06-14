Here’s a **detailed, production-style doc for `14-backup-scheduler.md`** (Rust-based backup scheduler microservice) — consistent with your previous docs, ready for you to use and expand.

---

```md
# 14. Rust Service: Backup Scheduler

## Purpose

This document details the design and integration of a Rust-based microservice for automated, scheduled backups of your Postgres database and/or key data volumes.  
The backup scheduler supplements your Docker Compose stack with robust, programmable data safety.

---

## Why Add a Backup Scheduler?

- **Automate data safety:** No manual backup steps or missed schedules.
- **Reliable recovery:** Easily restore the stack after failures, upgrades, or migrations.
- **Configurable:** Choose backup frequency, storage location, compression, and retention.
- **Extendable:** Add cloud upload (S3, Backblaze), notifications, or retention pruning as you grow.

---

## Key Features

- Runs on a customizable schedule (e.g., cron syntax or interval, configurable at runtime).
- Supports **Postgres DB dumps** (with `pg_dump`) and/or file/directory archiving.
- Compresses and timestamps each backup.
- Stores backups locally, and optionally uploads to remote/cloud.
- Logs backup results, can send notifications (email, webhook, etc.).
- (Optional) Exposes a REST API or metrics endpoint for status/health checks.

---

## System Architecture

```

+----------------+        +---------------------+
\|    DB (Postgres)|<-----+|   Backup Scheduler  |
+----------------+      / |  (Rust Service)     |
+----------------+     /  +---------------------+
\|    File Volume |<---+    |  - Runs on schedule
+----------------+         |  - Stores backups locally
\|  - Optional: Upload to cloud
+----------------------+

```

- Service runs inside Docker Compose, connects to DB and/or file volumes.
- (Optional) Exposes `/metrics` or `/status` for monitoring integration.

---

## Directory Structure

```

services/
└── backup\_scheduler/
├── src/
│   └── main.rs
├── Cargo.toml
├── Dockerfile
└── config.toml (optional)

````

---

## Implementation Outline

- **Language:** Rust (using [tokio](https://tokio.rs/) for async, [cron](https://crates.io/crates/cron) or [job_scheduler](https://crates.io/crates/job_scheduler) for scheduling)
- **DB backups:** Use [`pg_dump`](https://www.postgresql.org/docs/current/app-pgdump.html) via [`std::process::Command`](https://doc.rust-lang.org/std/process/struct.Command.html)
- **Compression:** Use [`flate2`](https://docs.rs/flate2/) or [`tar`](https://docs.rs/tar/) crates for gzipping/tarring.
- **Uploads:** Optional S3/backblaze via [`rusoto_s3`](https://docs.rs/rusoto_s3/)/[`aws-sdk-s3`](https://docs.rs/aws-sdk-s3/), or HTTP clients.
- **Notifications:** [`lettre`](https://docs.rs/lettre/) for email, [`reqwest`](https://docs.rs/reqwest/) for webhooks.

---

## Dockerfile Example

```dockerfile
# services/backup_scheduler/Dockerfile

FROM rust:1.76 as builder

WORKDIR /usr/src/backup_scheduler
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y postgresql-client tar gzip && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /usr/src/backup_scheduler/target/release/backup_scheduler /usr/local/bin/backup_scheduler
COPY config.toml /app/config.toml
CMD ["backup_scheduler"]
````

* Installs `pg_dump` for Postgres backups.
* Optionally copies config.

---

## Docker Compose Integration

Add to `docker-compose.yml`:

```yaml
backup_scheduler:
  build: ./services/backup_scheduler
  container_name: backup_scheduler
  depends_on:
    - db
  volumes:
    - db_data:/db_data:ro              # (Optional) For direct file volume backups
    - ./backups:/app/backups           # Host directory to store backups
  environment:
    - PGHOST=db
    - PGUSER=myuser
    - PGPASSWORD=mypassword
    - PGDATABASE=mydb
    - BACKUP_SCHEDULE=0 2 * * *        # Every day at 2am (cron syntax)
    - BACKUP_TYPE=pgdump,files         # Can be "pgdump", "files", or both
    - BACKUP_DIR=/app/backups
    - ALERT_EMAIL=alerts@example.com
    - S3_BUCKET=your-bucket-name       # (Optional) for cloud storage
    - CONFIG_PATH=/app/config.toml
```

---

## Configuration

* **Via environment variables, CLI flags, or `config.toml`.**
* Example `config.toml`:

  ```toml
  [postgres]
  host = "db"
  user = "myuser"
  password = "mypassword"
  database = "mydb"

  [schedule]
  cron = "0 2 * * *"

  [backup]
  directory = "/app/backups"
  type = ["pgdump", "files"]
  compress = true
  retention_days = 7

  [alerts]
  email = "your@email.com"
  webhook = "https://discord.com/api/webhooks/..."
  ```

---

## Usage

1. **Build locally for dev:**

   ```bash
   cd services/backup_scheduler
   cargo run
   ```
2. **Build container:**

   ```bash
   docker build -t backup-scheduler .
   ```
3. **Run in Compose:**
   `docker-compose up -d backup_scheduler`
4. **Check `./backups/` for backup files.**

---

## Example Backup Output

```
backups/
  ├── pg_backup_2024-06-12T02-00-00.sql.gz
  ├── files_backup_2024-06-12T02-00-00.tar.gz
  └── log.txt
```

---

## Optional Extensions

* Cloud storage upload (AWS S3, Backblaze B2, etc.)
* Prune old backups after X days.
* Push backup status/metrics to monitoring dashboard.
* Expose REST API for on-demand/manual backup trigger.

---

## Troubleshooting

* **Backup fails:** Check logs in `./backups/log.txt` or Compose logs for error details.
* **No files created:** Check schedule and backup directory config.
* **DB auth fails:** Ensure Postgres credentials/environment are correct and DB is up.

---

## Further Reading

* [Rust job\_scheduler crate](https://crates.io/crates/job_scheduler)
* [Postgres Docker backup strategies](https://docs.docker.com/storage/volumes/#backup-restore)
* [Docker Compose named volumes](https://docs.docker.com/compose/compose-file/#volumes)

---

## Next Service

[15-webhook-handler.md](./15-webhook-handler.md): Rust-based webhook listener and automation service.

---

```

---

**Ready for the next doc (15-webhook-handler), or want to tweak this one?**
```
