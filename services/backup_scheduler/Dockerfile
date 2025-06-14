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
