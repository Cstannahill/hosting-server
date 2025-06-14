FROM rust:1.76 as builder
WORKDIR /usr/src/log_watcher
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/log_watcher/target/release/log_watcher /usr/local/bin/log_watcher
COPY config.toml /app/config.toml
CMD ["log_watcher"]
