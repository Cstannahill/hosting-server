FROM rust:1.76 as builder
WORKDIR /usr/src/webhook_handler
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/webhook_handler/target/release/webhook_handler /usr/local/bin/webhook_handler
COPY config.toml /app/config.toml
CMD ["webhook_handler"]
