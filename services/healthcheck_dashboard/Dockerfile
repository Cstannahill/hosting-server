FROM rust:1.76 as builder
WORKDIR /usr/src/healthcheck_dashboard
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /usr/src/healthcheck_dashboard/target/release/healthcheck_dashboard /usr/local/bin/healthcheck_dashboard
COPY config.toml /app/config.toml
CMD ["healthcheck_dashboard"]
